use sea_orm::{
    ColumnTrait, DatabaseConnection, DbErr, EntityTrait, Order, QueryOrder, QuerySelect,
    SqlxPostgresConnector,
};
use sqlx::{query_as, PgPool};

use crate::{
    dto::{comments::Comment, projects::Project},
    model::{comments, projects},
    pagination::Pager,
};

#[derive(Debug)]
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

    pub async fn list(&self, pager: &Pager<i32>) -> sqlx::Result<Vec<Project>> {
        query_as!(
            Project,
            "SELECT * FROM projects WHERE id > $1 ORDER BY id LIMIT $2;",
            pager.after.unwrap_or_default(),
            pager.items.unwrap_or(10),
        )
        .fetch_all(&self.pool)
        .await
    }

    pub async fn get_with_comments(
        &self,
        id: i32,
        pager: &Pager<i32>,
    ) -> Result<Option<(Project, Vec<Comment>)>, DbErr> {
        let mut result: Vec<(projects::Model, Vec<comments::Model>)> =
            projects::Entity::find_by_id(id)
                .find_with_related(comments::Entity)
                .having(comments::Column::Id.gt(pager.after))
                .limit(pager.items.unwrap_or(10) as u64)
                .order_by(comments::Column::DatePosted, Order::Desc)
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
}
