use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;

use crate::{
    db::DbResultExt,
    utils::{err::AppError, response::AppResult},
};

use super::Payload;

#[instrument(skip(pool))]
pub async fn insert_new_user(pool: &PgPool, user: &Payload) -> AppResult<Uuid> {
    sqlx::query!(
        r#"
            INSERT INTO users (username, password, email)
            VALUES ($1, $2, $3)
            RETURNING id
        "#,
        user.username.as_ref(),
        user.password.as_ref(),
        user.email.as_ref()
    )
    .fetch_one(pool)
    .await
    .with_unique_violation(AppError::DuplicatedUser, "Failed create new user")
    .map(|row| row.id)
}
