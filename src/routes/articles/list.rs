use axum::{extract::State, http::StatusCode, response::IntoResponse, Json};
use parse_display::Display;
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::instrument;
use validator::Validate;

use crate::{
    application::AppCtx,
    db::DbResultExt,
    domains::user::Username,
    extractors::{MaybeAuthUser, ValidateJson},
    routes::articles::RawArticleFullCount,
    types::{SearchType, SortingDirection},
    utils::{
        jwt::UserData,
        response::{AppResponse, AppResult},
    },
};

use super::Article;

#[derive(Debug, Deserialize, Serialize, Validate, Default)]
pub struct Payload {
    pub offset: Option<u64>,

    #[validate(range(max = 100))]
    pub limit: Option<u64>,

    #[validate(custom = "crate::parsers::user::validate_username")]
    pub author: Option<Username>,

    #[validate(length(min = 3))]
    pub title: Option<String>,

    #[validate(length(min = 1))]
    pub tag: Option<String>,

    #[validate(length(min = 1))]
    pub order_by: Option<Vec<OrderBy>>,
}

impl Payload {
    fn format_order_by(&self) -> String {
        let mut to_join = vec![];

        if let Some(order_by) = &self.order_by {
            for item in order_by {
                to_join.push(item.to_string());
            }
        } else {
            to_join.push("created_at DESC".to_string());
        }

        to_join.join(", ")
    }
}

#[derive(Debug, Deserialize, Serialize, Display)]
#[serde(untagged)]
pub enum OrderBy {
    #[display("created_at {created_at}")]
    CreatedAt { created_at: SortingDirection },

    #[display("username {username}")]
    Username { username: SortingDirection },
}

#[instrument(skip(ctx))]
pub async fn list_articles(
    MaybeAuthUser(user): MaybeAuthUser<UserData>,
    ctx: State<AppCtx>,
    ValidateJson(query): ValidateJson<Payload>,
) -> AppResponse {
    let data = get_articles_list(&query, &ctx.db).await?;

    Ok((StatusCode::OK, Json(data)).into_response())
}

#[instrument(skip(pool))]
async fn get_articles_list(query: &Payload, pool: &PgPool) -> AppResult<SearchType<Article>> {
    let order_by = query.format_order_by();

    let raw_articles = sqlx::query_as::<_, RawArticleFullCount>(&format!(
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
                    WHERE
                        ($1::TEXT IS NULL OR u.username = $1)
                    AND
                        ($2::TEXT IS NULL OR a.title LIKE $2 || '%')
                    AND
                        ($3::TEXT IS NULL OR a.tags @> array[$3])
                    ORDER BY {order_by}
                    LIMIT $4 OFFSET $5
                )

                SELECT cte.*, (SELECT COUNT(*) FROM cte) AS full_count FROM cte
            "#,
    ))
    .bind(query.author.as_ref().map(|u| u.as_ref()))
    .bind(&query.title)
    .bind(&query.tag)
    .bind(query.limit.unwrap_or(20) as i64)
    .bind(query.offset.unwrap_or(0) as i64)
    .fetch_all(pool)
    .await
    .trace_db("Failed to fetch list of articles")?;

    let mut results = vec![];
    let mut total: i64 = 0;

    for item in raw_articles {
        total = item.full_count.unwrap_or(0);
        results.push(item.into_article());
    }

    Ok(SearchType {
        results,
        total: total as usize,
    })
}
