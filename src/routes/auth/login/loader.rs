use anyhow::Context;
use sqlx::PgPool;

use crate::domains::user::{Password, UserID, Username};

pub struct UserLoginInfo {
    pub id: UserID,
    pub password_hash: Password,
}

pub async fn get_user(pool: &PgPool, username: &Username) -> anyhow::Result<Option<UserLoginInfo>> {
    sqlx::query!(
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
    .context("Failed to fetch user login data")
    .map(|maybe_row| {
        maybe_row.map(|row| UserLoginInfo {
            id: UserID(row.id.to_string()),
            password_hash: Password(row.password),
        })
    })
}
