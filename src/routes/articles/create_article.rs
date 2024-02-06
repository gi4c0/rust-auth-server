use std::str::FromStr;

use anyhow::Context;
use axum::{extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::instrument;
use uuid::Uuid;
use validator::Validate;

use crate::{
    application::AppCtx,
    extractors::{AuthUser, ValidateJson},
    utils::{
        err::{AppError, DbResultExt},
        jwt::UserData,
        response::{AppResponse, AppResult, DataResponse},
    },
};

use super::{Article, RawArticle};

#[derive(Deserialize, Validate, Debug, Serialize)]
pub struct Payload {
    #[validate(length(min = 3))]
    pub title: String,

    #[validate(length(min = 10))]
    pub text: String,

    #[validate(length(min = 1))]
    pub tags: Option<Vec<String>>,
}

#[instrument(skip(ctx))]
pub async fn create_article(
    ctx: State<AppCtx>,
    AuthUser(user): AuthUser<UserData>,
    ValidateJson(payload): ValidateJson<Payload>,
) -> AppResponse {
    let result = insert_new_article(&ctx.db, &payload, &user).await?;

    Ok((StatusCode::CREATED, DataResponse::new(result)).into_response())
}

async fn insert_new_article(
    pool: &PgPool,
    payload: &Payload,
    user: &UserData,
) -> AppResult<Article> {
    let empty_tags = vec![];

    sqlx::query_as!(
        RawArticle,
        r#"
            WITH inserted_article AS (
                INSERT INTO articles (
                    author_id,
                    title,
                    text,
                    tags
                ) VALUES ($1, $2, $3, $4)
                RETURNING id, author_id, title, text, tags, created_at
            )

            SELECT
                inserted_article.*,
                u.username AS author_username
            FROM inserted_article
            JOIN users u ON u.id = $1
        "#,
        Uuid::from_str(user.user_id.as_ref())
            .with_context(|| format!("Invalid uuid for user ID: {}", user.user_id.as_ref()))?,
        &payload.title,
        &payload.text,
        &payload.tags.as_ref().unwrap_or(&empty_tags),
    )
    .fetch_one(pool)
    .await
    .map(RawArticle::into_article)
    .with_unique_violation(AppError::DuplicatedArticle, "Duplicated article title")
}