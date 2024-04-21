use sea_orm::{
    DatabaseConnection, DbErr, EntityTrait, Order, QueryOrder, QuerySelect, SqlxPostgresConnector,
};
use sqlx::{query_as, types::time::OffsetDateTime, PgPool};

use crate::{
    dto::{
        comments::Comment,
        projects::{NewProject, Project},
    },
    model::{comments, projects},
    pagination::Pager,
};

const DEFAULT_PROJECTS_PER_PAGE: i64 = 10;
const DEFAULT_COMMENTS_PER_PAGE: i64 = 10;

#[derive(Clone, Debug)]
pub struct ProjectsRepository {
    pool: PgPool,
    conn: DatabaseConnection,
}

impl ProjectsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            conn: SqlxPostgresConnector::from_sqlx_postgres_pool(pool.clone()),
            pool,
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

    pub async fn create(&self, project: &NewProject) -> sqlx::Result<Project> {
        query_as!(
            Project,
            "INSERT INTO projects (id, name, description, thumbnail_key, project_url) VALUES ($1, \
             $2, $3, $4, $5) RETURNING *;",
            project.id(),
            &project.name,
            &project.description,
            &project.thumbnail_key,
            project.project_url.as_ref()
        )
        .fetch_one(&self.pool)
        .await
    }
}
