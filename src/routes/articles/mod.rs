use axum::{routing::post, Router};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::{
    application::AppCtx,
    domains::user::{UserID, Username},
};

pub mod create_article;
pub mod list;
pub mod subscribe;

use create_article::create_article;
use list::list_articles;

#[derive(Deserialize, Serialize)]
pub struct Article {
    pub id: String,
    pub author: Author,
    pub text: String,
    pub title: String,
    pub tags: Vec<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize)]
pub struct Author {
    pub id: UserID,
    pub username: Username,
}

#[derive(sqlx::FromRow)]
struct RawArticleFullCount {
    full_count: Option<i64>,
    id: Uuid,
    text: String,
    title: String,
    tags: Option<Vec<String>>,
    created_at: NaiveDateTime,
    author_id: Uuid,
    author_username: String,
}

impl RawArticleFullCount {
    fn into_article(self) -> Article {
        return Article {
            id: self.id.to_string(),
            author: Author {
                id: UserID(self.author_id),
                username: Username(self.author_username),
            },
            text: self.text,
            title: self.title,
            tags: self.tags.unwrap_or(vec![]),
            created_at: self.created_at,
        };
    }
}

pub fn routes() -> Router<AppCtx> {
    Router::new()
        .route("/articles", post(create_article))
        .route("/articles/get-articles", post(list_articles))
        .route("/articles/subscribe", post(subscribe::subscribe))
}
