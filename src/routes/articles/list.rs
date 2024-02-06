use axum::{extract::State, http::StatusCode, response::IntoResponse};
use futures::TryStreamExt;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;

use crate::{
    application::AppCtx,
    domains::user::Username,
    extractors::ValidateQuery,
    routes::articles::RawArticle,
    utils::{
        err::DbResultExt,
        response::{AppResponse, AppResult, DataResponse},
    },
};

use super::Article;

#[derive(Debug, Deserialize, Serialize, Validate)]
pub struct Query {
    pub offset: u32,

    #[validate(range(max = 100))]
    pub limit: u32,

    #[validate(custom = "crate::parsers::user::validate_username")]
    pub author: Option<Username>,

    #[validate(length(min = 3))]
    pub title: Option<String>,

    pub tags: Option<Vec<String>>,
}

pub async fn list_articles(
    ctx: State<AppCtx>,
    ValidateQuery(query): ValidateQuery<Query>,
) -> AppResponse {
    let data = get_articles_list(&query, &ctx.db).await?;

    Ok((StatusCode::OK, DataResponse::new(data)).into_response())
}

async fn get_articles_list(query: &Query, pool: &PgPool) -> AppResult<Vec<Article>> {
    sqlx::query_as!(
        RawArticle,
        r#"
            SELECT
                a.id,
                a.title,
                a.text,
                a.tags,
                a.author_id,
                a.created_at,
                u.username AS author_username
            FROM articles a
            JOIN users u ON a.author_id = u.id
            WHERE
                ($1::TEXT IS NULL OR u.username = $1)
            AND
                ($2::TEXT IS NULL OR a.title = $2)
            AND
                ($3::TEXT[] IS NULL OR a.tags @> $3)
            ORDER BY created_at DESC
            LIMIT $4 OFFSET $5;
        "#,
        query.author.as_ref().map(|u| u.as_ref()),
        query.title,
        query.tags.as_deref(),
        query.limit as i64,
        query.offset as i64
    )
    .fetch(pool)
    .map_ok(RawArticle::into_article)
    .try_collect()
    .await
    .trace_db("Failed to fetch list of articles")
}
