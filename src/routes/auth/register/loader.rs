use sqlx::PgPool;
use uuid::Uuid;

use crate::utils::{err::ServerError, response::ServerResult};

use super::Payload;

pub async fn insert_new_user(pool: &PgPool, user: &Payload) -> ServerResult<Uuid> {
    sqlx::query!(
        r#"
            INSERT INTO users (username, password, email)
            VALUES ($1, $2, $3)
            RETURNING id
        "#,
        user.username.as_ref(),
        &user.password,
        user.email.as_ref()
    )
    .fetch_one(pool)
    .await
    .map(|row| row.id)
    .map_err(check_db_error)
}

fn check_db_error(err: sqlx::Error) -> ServerError {
    if let sqlx::Error::Database(e) = &err {
        if e.is_unique_violation() {
            return ServerError::DuplicatedUser;
        }
    }

    anyhow::anyhow!("Failed create a user: {}", err).into()
}
