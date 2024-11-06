use sqlx::{query, query_as, PgPool};

use crate::{
    dto::projects::{
        NewProject, Project, ProjectData, ProjectId, ProjectIndex, ProjectName, ProjectPreview,
    },
    error::AppResult,
    repository::files::FileRepository,
    utils::pagination::{Page, Pager},
};

const DEFAULT_PROJECTS_PER_PAGE: i64 = 12;

#[derive(Clone, Debug)]
pub struct ProjectRepository {
    pool: PgPool,
    file_repo: FileRepository,
}

impl ProjectRepository {
    pub fn new(pool: PgPool, file_repo: FileRepository) -> Self {
        Self { pool, file_repo }
    }

    pub async fn list(&self, pager: &Pager<ProjectIndex>) -> sqlx::Result<Page<ProjectPreview>> {
        let items = pager.items.unwrap_or(DEFAULT_PROJECTS_PER_PAGE);

        let page = match (&pager.before, &pager.after) {
            (Some(index), _) => {
                let mut transaction = self.pool.begin().await?;

                let mut previews = query_as!(
                    ProjectPreview,
                    "SELECT id, name, preview, thumbnail_id, date_posted FROM projects WHERE NOT \
                     deleted AND (date_posted, id) > ($1, $2) ORDER BY (date_posted, id) LIMIT $3;",
                    index.date_posted.as_offset(),
                    &index.id,
                    items + 1,
                )
                .fetch_all(&mut *transaction)
                .await?;

                let has_previous = previews.len() as i64 == items + 1;
                let has_next = !previews.is_empty()
                    && query!(
                        "SELECT id FROM projects WHERE NOT deleted AND (date_posted, id) < ($1, \
                         $2) LIMIT 1;",
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
                    "SELECT id, name, preview, thumbnail_id, date_posted FROM projects WHERE NOT \
                     deleted AND (date_posted, id) < ($1, $2) ORDER BY (date_posted, id) DESC \
                     LIMIT $3;",
                    index.date_posted.as_offset(),
                    &index.id,
                    items + 1,
                )
                .fetch_all(&mut *transaction)
                .await?;

                let has_previous = !previews.is_empty()
                    && query!(
                        "SELECT id FROM projects WHERE NOT deleted AND (date_posted, id) > ($1, \
                         $2) LIMIT 1;",
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
                    "SELECT id, name, preview, thumbnail_id, date_posted FROM projects WHERE NOT \
                     deleted ORDER BY (date_posted, id) DESC LIMIT $1;",
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

    pub async fn read_data(&self, id: &str) -> AppResult<ProjectData> {
        Ok(query_as!(
            ProjectData,
            "SELECT id, name, thumbnail_id, project_url FROM projects WHERE NOT deleted AND id = \
             $1;",
            id,
        )
        .fetch_one(&self.pool)
        .await?)
    }

    pub async fn read_name(&self, id: &str) -> AppResult<ProjectName> {
        let name = query_as!(
            ProjectName,
            "SELECT name FROM projects WHERE NOT deleted AND id = $1;",
            id,
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(name)
    }

    pub async fn read(&self, id: &str) -> AppResult<Option<Project>> {
        let project = query_as!(
            Project,
            "SELECT id, name, thumbnail_id, project_url, date_posted FROM projects WHERE NOT \
             deleted AND id = $1;",
            id
        )
        .fetch_optional(&self.pool)
        .await?;

        Ok(project)
    }

    pub async fn create(&self, new_project: NewProject) -> AppResult<ProjectId> {
        let mut transaction = self.pool.begin().await?;

        let id = new_project.id();

        let project = query_as!(
            ProjectId,
            "INSERT INTO projects (id, name, preview, thumbnail_id, project_url) VALUES ($1, $2, \
             $3, $4, $5) RETURNING id;",
            &id,
            &new_project.name,
            new_project.preview(),
            &new_project.thumbnail_id,
            new_project.project_url.as_ref(),
        )
        .fetch_one(&mut *transaction)
        .await?;

        self.file_repo
            .store_content(&id, &new_project.content)
            .await?;

        transaction.commit().await?;

        Ok(project)
    }

    pub async fn update(&self, id: &str, new_project: NewProject) -> AppResult<()> {
        let mut transaction = self.pool.begin().await?;

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
            .store_content(id, &new_project.content)
            .await?;

        transaction.commit().await?;

        Ok(())
    }

    pub async fn delete(&self, id: &str) -> AppResult<()> {
        query!("UPDATE projects SET deleted = TRUE WHERE id = $1;", id)
            .execute(&self.pool)
            .await?;

        Ok(())
    }
}
