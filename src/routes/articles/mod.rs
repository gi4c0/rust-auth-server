use axum::{routing::post, Router};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

use crate::{
    application::AppCtx,
    domains::user::{UserID, Username},
};

mod create_article;

pub use create_article::{create_article, Payload};

#[derive(Deserialize, Serialize)]
struct Article {
    id: String,
    author: Author,
    text: String,
    title: String,
    tags: Vec<String>,
    created_at: NaiveDateTime,
}

#[derive(Deserialize, Serialize)]
struct Author {
    id: UserID,
    username: Username,
}

#[derive(Deserialize, Serialize)]
struct RawArticle {
    id: String,
    text: String,
    title: String,
    tags: Option<Vec<String>>,
    created_at: NaiveDateTime,
    author_id: String,
    author_username: String,
}

impl RawArticle {
    fn into_article(self) -> Article {
        return Article {
            id: self.id,
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
    Router::new().route("/articles", post(create_article))
}
