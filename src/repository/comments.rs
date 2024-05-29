use sqlx::{query_as, PgPool};
use uuid::Uuid;

use crate::{
    dto::{
        comments::{Comment, NewComment},
        projects::ProjectTime,
    },
    error::AppResult,
    repository::users::UserRepository,
};

#[derive(Clone, Debug)]
pub struct CommentRepository {
    pool: PgPool,
    user_repo: UserRepository,
}

impl CommentRepository {
    pub fn new(pool: PgPool, user_repo: UserRepository) -> Self {
        Self { pool, user_repo }
    }

    pub async fn create(
        &self,
        user_id: Uuid,
        project_id: &str,
        comment: &NewComment,
    ) -> AppResult<Comment> {
        pub struct CommentUserId {
            id: i32,
            user_id: String,
            message: String,
            date_posted: ProjectTime,
        }

        let CommentUserId {
            id,
            user_id,
            message,
            date_posted,
        } = query_as!(
            CommentUserId,
            "INSERT INTO comments (user_id, project_id, message) VALUES ($1, $2, $3) RETURNING \
             id, user_id, message, date_posted;",
            user_id.to_string(),
            project_id,
            &comment.message,
        )
        .fetch_one(&self.pool)
        .await?;

        let user = self.user_repo.get(user_id.parse()?).await?;

        let comment = Comment {
            id,
            user,
            message,
            date_posted,
        };

        Ok(comment)
    }
}
