use sea_orm::{
    DatabaseConnection, EntityTrait, Order, QueryOrder, QuerySelect, SqlxPostgresConnector,
};
use sqlx::{query, query_as, PgPool};
use uuid::Uuid;

use crate::{
    config::AppBucket,
    dto::projects::{
        NewProject, ProjectId, ProjectIndex, ProjectPreview, ProjectView, ProjectWithComments,
    },
    error::AppResult,
    model::{comments, projects},
    repository::files::FileRepository,
    utils::pagination::{Page, Pager},
};

const DEFAULT_PROJECTS_PER_PAGE: i64 = 6;
const DEFAULT_COMMENTS_PER_PAGE: i64 = 10;

#[derive(Clone, Debug)]
pub struct ProjectsRepository {
    pool: PgPool,
    conn: DatabaseConnection,
    file_repo: FileRepository,
}

impl ProjectsRepository {
    pub fn new(pool: PgPool, file_repo: FileRepository) -> Self {
        Self {
            conn: SqlxPostgresConnector::from_sqlx_postgres_pool(pool.clone()),
            pool,
            file_repo,
        }
    }

    pub async fn list(&self, pager: &Pager<ProjectIndex>) -> sqlx::Result<Page<ProjectPreview>> {
        let items = pager.items.unwrap_or(DEFAULT_PROJECTS_PER_PAGE);

        let page = match (&pager.before, &pager.after) {
            (Some(index), _) => {
                let mut transaction = self.pool.begin().await?;

                let mut previews = query_as!(
                    ProjectPreview,
                    "SELECT id, name, preview, thumbnail_id, date_posted FROM projects WHERE \
                     (date_posted, id) > ($1, $2) ORDER BY (date_posted, id) LIMIT $3;",
                    index.date_posted.as_offset(),
                    &index.id,
                    items + 1,
                )
                .fetch_all(&mut *transaction)
                .await?;

                let has_previous = previews.len() as i64 == items + 1;
                let has_next = !previews.is_empty()
                    && query!(
                        "SELECT id FROM projects WHERE (date_posted, id) < ($1, $2) LIMIT 1;",
                        index.date_posted.as_offset(),
                        &index.id,
                    )
                    .fetch_optional(&mut *transaction)
                    .await?
                    .is_some();

                if has_previous {
                    previews.pop();
                }

                previews.reverse();

                transaction.commit().await?;

                Page {
                    items: previews,
                    has_previous,
                    has_next,
                }
            }
            (_, Some(index)) => {
                let mut transaction = self.pool.begin().await?;

                let mut previews = query_as!(
                    ProjectPreview,
                    "SELECT id, name, preview, thumbnail_id, date_posted FROM projects WHERE \
                     (date_posted, id) < ($1, $2) ORDER BY (date_posted, id) DESC LIMIT $3;",
                    index.date_posted.as_offset(),
                    &index.id,
                    items + 1,
                )
                .fetch_all(&mut *transaction)
                .await?;

                let has_previous = !previews.is_empty()
                    && query!(
                        "SELECT id FROM projects WHERE (date_posted, id) > ($1, $2) LIMIT 1;",
                        index.date_posted.as_offset(),
                        &index.id,
                    )
                    .fetch_optional(&mut *transaction)
                    .await?
                    .is_some();
                let has_next = previews.len() as i64 == items + 1;

                if has_next {
                    previews.pop();
                }

                transaction.commit().await?;

                Page {
                    items: previews,
                    has_previous,
                    has_next,
                }
            }
            (None, None) => {
                let mut previews = query_as!(
                    ProjectPreview,
                    "SELECT id, name, preview, thumbnail_id, date_posted FROM projects ORDER BY \
                     (date_posted, id) DESC LIMIT $1;",
                    items + 1,
                )
                .fetch_all(&self.pool)
                .await?;

                let has_next = previews.len() as i64 == items + 1;

                if has_next {
                    previews.pop();
                }

                Page {
                    items: previews,
                    has_previous: false,
                    has_next,
                }
            }
        };

        Ok(page)
    }

    pub async fn get(&self, id: &str) -> AppResult<ProjectView> {
        struct NameUrl {
            name: String,
            content_id: Uuid,
            thumbnail_id: Uuid,
            project_url: Option<String>,
        }

        let NameUrl {
            name,
            content_id,
            thumbnail_id,
            project_url,
        } = query_as!(
            NameUrl,
            "SELECT name, content_id, thumbnail_id, project_url FROM projects WHERE id = $1;",
            id,
        )
        .fetch_one(&self.pool)
        .await?;

        let content = self
            .file_repo
            .retrieve_markdown(content_id, AppBucket::Content)
            .await?;

        Ok(ProjectView {
            name,
            content,
            thumbnail_id,
            project_url,
        })
    }

    pub async fn get_with_comments(
        &self,
        id: &str,
        pager: &Pager<i32>,
    ) -> AppResult<Option<ProjectWithComments>> {
        let mut result: Vec<(projects::Model, Vec<comments::Model>)> =
            projects::Entity::find_by_id(id)
                .find_with_related(comments::Entity)
                // .filter(comments::Column::Id.gt(pager.after))
                // Since the ID is serial, sorting by id or by post time is equivalent
                .order_by(comments::Column::Id, Order::Desc)
                .limit(pager.items.unwrap_or(DEFAULT_COMMENTS_PER_PAGE) as u64)
                .all(&self.conn)
                .await?;

        match result.pop() {
            None => Ok(None),
            Some((project, comments)) => {
                let comments = comments.into_iter().map(Into::into).collect();
                let project =
                    ProjectWithComments::from_model(comments, project, &self.file_repo).await?;
                Ok(Some(project))
            }
        }
    }

    pub async fn create(&self, new_project: NewProject) -> AppResult<ProjectId> {
        let content_id = Uuid::new_v4();

        let mut transaction = self.pool.begin().await?;

        let project = query_as!(
            ProjectId,
            "INSERT INTO projects (id, name, preview, content_id, thumbnail_id, project_url) \
             VALUES ($1, $2, $3, $4, $5, $6) RETURNING id;",
            new_project.id(),
            &new_project.name,
            new_project.preview(),
            content_id,
            &new_project.thumbnail_id,
            new_project.project_url.as_ref(),
        )
        .fetch_one(&mut *transaction)
        .await?;

        self.file_repo
            .store_markdown(content_id, AppBucket::Content, &new_project.content)
            .await?;

        transaction.commit().await?;

        Ok(project)
    }

    pub async fn update(&self, id: &str, new_project: NewProject) -> AppResult<()> {
        struct ContentId {
            content_id: Uuid,
        }

        let mut transaction = self.pool.begin().await?;

        let ContentId { content_id } = query_as!(
            ContentId,
            "SELECT content_id FROM projects WHERE id = $1;",
            id
        )
        .fetch_one(&mut *transaction)
        .await?;

        query!(
            "UPDATE projects SET name = $1, preview = $2, thumbnail_id = $3, project_url = $4 \
             WHERE id = $5;",
            &new_project.name,
            new_project.preview(),
            &new_project.thumbnail_id,
            new_project.project_url.as_ref(),
            id,
        )
        .execute(&mut *transaction)
        .await?;

        self.file_repo
            .store_markdown(content_id, AppBucket::Content, &new_project.content)
            .await?;

        transaction.commit().await?;

        Ok(())
    }
}
