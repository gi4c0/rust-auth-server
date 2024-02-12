use sqlx::PgPool;
use tracing::instrument;

use crate::{
    db::DbResultExt,
    domains::user::{Password, UserID, Username},
    utils::response::AppResult,
};

pub struct UserLoginInfo {
    pub id: UserID,
    pub password_hash: Password,
}

#[instrument(skip(pool))]
pub async fn get_user(pool: &PgPool, username: &Username) -> AppResult<Option<UserLoginInfo>> {
    let result = sqlx::query!(
        r#"
            SELECT
                id,
                password
            FROM
                public.users
            WHERE
                username = $1
        "#,
        username.as_ref()
    )
    .fetch_optional(pool)
    .await
    .trace_db("Failed to fetch user login data")
    .map(|maybe_row| {
        maybe_row.map(|row| UserLoginInfo {
            id: UserID(row.id),
            password_hash: Password(row.password),
        })
    })?;

    Ok(result)
}

/*
#[instrument(skip(pool))]
pub async fn get_user(pool: &PgPool, username: &Username) -> ServerResult<Option<UserLoginInfo>> {
    sqlx::query(
        r#"
            SELEC
                id,
                password
            FROM
                public.users
            WHERE
                username = $1
        "#,
    )
    .bind(username.as_ref())
    .fetch_optional(pool)
    .await
    .trace_db("Failed to fetch user login data")
    .map(|maybe_row| {
        maybe_row.map(|row| UserLoginInfo {
            id: UserID(row.get("id")),
            password_hash: Password(row.get("password")),
        })
    })
}
*/
