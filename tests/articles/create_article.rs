use std::str::FromStr;

use claims::assert_ok;
use lib::{routes::articles::create_article, utils::err::AppError};
use reqwest::StatusCode;
use serde_json::Value;

use crate::helper::TestApp;

#[tokio::test]
async fn create_new_article() {
    let app = TestApp::spawn().await;

    let payload = create_article::Payload {
        title: "A unique title".to_string(),
        text: "A long new article".to_string(),
        tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
    };

    let response = app.create_article(&payload).await;

    assert_eq!(response.status().as_u16(), StatusCode::CREATED);
    let body: Value = response.json().await.unwrap();
    let id = body["data"].as_object().unwrap()["id"].as_str().unwrap();

    assert_ok!(uuid::Uuid::from_str(id));

    app.clean().await;
}

#[tokio::test]
async fn fail_on_creating_article_with_duplicated_title() {
    let app = TestApp::spawn().await;

    let payload = create_article::Payload {
        title: "A unique title".to_string(),
        text: "A long new article".to_string(),
        tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
    };

    let response = app.create_article(&payload).await;
    assert_eq!(response.status().as_u16(), StatusCode::CREATED);

    let response = app.create_article(&payload).await;
    assert_eq!(response.status().as_u16(), StatusCode::BAD_REQUEST);

    let json: Value = response.json().await.unwrap();
    assert_eq!(
        AppError::DuplicatedArticle.to_string(),
        json["message"].as_str().unwrap().to_string()
    );

    app.clean().await;
}
