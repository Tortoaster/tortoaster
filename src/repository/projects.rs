use sqlx::{query_as, PgPool};

use crate::{model::Project, pagination::Pager};

#[derive(Debug)]
pub struct ProjectsRepository(PgPool);

impl ProjectsRepository {
    pub fn new(pool: PgPool) -> Self {
        Self(pool)
    }

    pub async fn list(&self, pager: &Pager<i32>) -> sqlx::Result<Vec<Project>> {
        query_as!(
            Project,
            "SELECT * FROM projects WHERE id > $1 ORDER BY id LIMIT $2;",
            pager.after.unwrap_or_default(),
            pager.items.unwrap_or(10),
        )
        .fetch_all(&self.0)
        .await
    }

    pub async fn get(&self, id: i32) -> sqlx::Result<Option<Project>> {
        query_as!(Project, "SELECT * FROM projects WHERE id = $1;", id)
            .fetch_optional(&self.0)
            .await
    }
}
