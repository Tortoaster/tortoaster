use sea_orm::{
    DatabaseConnection, DbErr, EntityTrait, Order, QueryOrder, QuerySelect, SqlxPostgresConnector,
};
use sqlx::{query_as, types::time::OffsetDateTime, PgPool};
use uuid::Uuid;

use crate::{
    config::AppBucket,
    dto::{
        comments::Comment,
        projects::{NewProject, Project},
    },
    error::AppResult,
    model::{comments, projects},
    pagination::Pager,
    repository::files::FileRepository,
};

const DEFAULT_PROJECTS_PER_PAGE: i64 = 10;
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

    pub async fn list(
        &self,
        pager: &Pager<(OffsetDateTime, String)>,
    ) -> sqlx::Result<Vec<Project>> {
        query_as!(
            Project,
            "SELECT * FROM projects WHERE COALESCE((date_posted, id) < ($1, $2), TRUE) ORDER BY \
             (date_posted, id) DESC LIMIT $3;",
            pager.after.as_ref().map(|columns| &columns.0),
            pager.after.as_ref().map(|columns| &columns.1),
            pager.items.unwrap_or(DEFAULT_PROJECTS_PER_PAGE),
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_with_comments(
        &self,
        id: &str,
        pager: &Pager<i32>,
    ) -> Result<Option<(Project, Vec<Comment>)>, DbErr> {
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
            Some((project, comments)) => Ok(Some((
                project.into(),
                comments.into_iter().map(Into::into).collect(),
            ))),
        }
    }

    pub async fn create(&self, new_project: NewProject) -> AppResult<Project> {
        let content_id = Uuid::new_v4();
        let thumbnail_id = Uuid::new_v4();

        let mut transaction = self.pool.begin().await?;

        let project = query_as!(
            Project,
            "INSERT INTO projects (id, name, preview, content_id, thumbnail_id, project_url) \
             VALUES ($1, $2, $3, $4, $5, $6) RETURNING *;",
            new_project.id(),
            &new_project.name,
            new_project.preview(),
            content_id,
            thumbnail_id,
            new_project.project_url.as_ref()
        )
        .fetch_one(&mut *transaction)
        .await?;

        self.file_repo
            .store_markdown(content_id, AppBucket::Content, &new_project.content)
            .await?;
        self.file_repo
            .store(thumbnail_id, new_project.thumbnail)
            .await?;

        transaction.commit().await?;

        Ok(project)
    }
}
