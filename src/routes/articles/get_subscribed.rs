use axum::{
    extract::{Query, State},
    http::StatusCode,
    response::IntoResponse,
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::instrument;

use crate::{
    application::AppCtx,
    db::{self, DbResultExt},
    domains::user::UserID,
    extractors::AuthUser,
    routes::articles::RawArticleFullCount,
    types::SearchType,
    utils::{
        jwt::UserData,
        response::{AppResponse, AppResult},
    },
};

use super::Article;

#[derive(Serialize, Deserialize, Debug)]
pub struct Payload {
    pub user_id: Option<UserID>,
    pub limit: Option<u64>,
    pub offset: Option<u64>,
}

#[instrument(skip(ctx))]
pub async fn get_subscribed(
    ctx: State<AppCtx>,
    AuthUser(user): AuthUser<UserData>,
    Query(query): Query<Payload>,
) -> AppResponse {
    let data = get_subscribed_articles(&ctx.db, user.user_id, &query).await?;
    Ok((StatusCode::OK, Json(data)).into_response())
}

#[instrument(skip(pool))]
async fn get_subscribed_articles(
    pool: &PgPool,
    user_id: UserID,
    payload: &Payload,
) -> AppResult<SearchType<Article>> {
    let raw_articles = sqlx::query_as!(
        RawArticleFullCount,
        r#"
            WITH cte AS (
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
                JOIN subscriptions s ON s.author_id = a.author_id
                WHERE
                    s.subscriber_id = $1
                AND
                    ($2::UUID IS NULL OR u.id = $2)
                ORDER BY created_at DESC
                LIMIT $3 OFFSET $4
            )

            SELECT cte.*, (SELECT COUNT(*) FROM cte) AS full_count FROM cte
        "#,
        user_id.as_ref(),
        payload.user_id.as_ref().map(|i| i.as_ref()),
        payload.limit.unwrap_or(20) as i64,
        payload.offset.unwrap_or(0) as i64,
    )
    .fetch_all(pool)
    .await
    .trace_db("Failed to fetch list of articles")?;

    Ok(db::into_search_type(raw_articles))
}
