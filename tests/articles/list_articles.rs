use std::time::Duration;

use lib::{
    routes::articles::{create_article, list::Payload, Article},
    types::SearchType,
};
use reqwest::{Response, StatusCode};
use serde::Serialize;
use serde_json::json;

use crate::helper::TestApp;

fn get_payload() -> Vec<create_article::Payload> {
    vec![
        create_article::Payload {
            title: "Around the planet".to_string(),
            text: "Lorem ipsum donna".to_string(),
            tags: Some(vec!["tag1".to_string()]),
        },
        create_article::Payload {
            title: "Circle in life".to_string(),
            text: "A long new article".to_string(),
            tags: Some(vec!["tag1".to_string(), "tag2".to_string()]),
        },
    ]
}

#[tokio::test]
async fn successfully_return_articles() {
    let app = TestApp::spawn().await;
    let article_payloads = get_payload();

    for article_payload in &article_payloads {
        let response = app
            .create_article(article_payload, &app.test_users[0])
            .await;
        assert_eq!(response.status().as_u16(), StatusCode::CREATED);
    }

    let body = Payload {
        title: Some("Around".to_string()),
        ..Payload::default()
    };

    let response = get_articles_list(&body, &app.address).await;
    let response: SearchType<Article> = response.json().await.unwrap();

    assert_eq!(response.total, 1);
    assert_eq!(response.results[0].title, article_payloads[0].title);
    assert_eq!(response.results[0].text, article_payloads[0].text);

    assert_eq!(
        &response.results[0].tags,
        article_payloads[0].tags.as_ref().unwrap()
    );

    app.clean().await;
}

#[tokio::test]
async fn filter_articles_by_tags() {
    let app = TestApp::spawn().await;

    let article_payloads = get_payload();

    for article_payload in &article_payloads {
        let response = app
            .create_article(article_payload, &app.test_users[0])
            .await;

        assert_eq!(response.status().as_u16(), StatusCode::CREATED);
    }

    let body = json!({ "tag": "tag2" });

    let response = get_articles_list(&body, &app.address).await;
    let response: SearchType<Article> = response.json().await.unwrap();

    assert_eq!(response.total, 1);

    assert_eq!(response.results[0].title, article_payloads[1].title);
    assert_eq!(response.results[0].text, article_payloads[1].text);

    assert_eq!(
        &response.results[0].tags,
        article_payloads[1].tags.as_ref().unwrap()
    );

    app.clean().await;
}

#[tokio::test]
async fn order_articles_by_created_at_asc() {
    let app = TestApp::spawn().await;

    let article_payloads = get_payload();

    for article_payload in &article_payloads {
        let response = app
            .create_article(article_payload, &app.test_users[0])
            .await;

        assert_eq!(response.status().as_u16(), StatusCode::CREATED);

        // For articles to have different created_at
        tokio::time::sleep(Duration::from_secs(1)).await;
    }

    let body = json!({
        "order_by": [
            { "created_at": "ASC" },
            { "username": "ASC" }
        ]
    });

    let response = get_articles_list(&body, &app.address).await;
    let response: SearchType<Article> = response.json().await.unwrap();

    assert_eq!(response.total, 2);

    // Have to keep .rev() because we get articles sorted by last inserted
    for (i, item) in article_payloads.into_iter().enumerate() {
        assert_eq!(response.results[i].title, item.title);
        assert_eq!(response.results[i].text, item.text);
        assert_eq!(&response.results[i].tags, item.tags.as_ref().unwrap());
    }

    app.clean().await;
}

#[tokio::test]
async fn filter_articles_by_username() {
    let app = TestApp::spawn().await;

    let article_payloads = get_payload();

    for (i, article_payload) in article_payloads.iter().enumerate() {
        let response = app
            .create_article(&article_payload, &app.test_users[i])
            .await;

        assert_eq!(response.status().as_u16(), StatusCode::CREATED);
    }

    let body = json!({ "author": &app.test_users[0].username });

    let response = get_articles_list(&body, &app.address).await;
    let response: SearchType<Article> = response.json().await.unwrap();

    assert_eq!(response.total, 1);

    assert_eq!(response.results[0].title, article_payloads[0].title);
    assert_eq!(response.results[0].text, article_payloads[0].text);

    assert_eq!(
        &response.results[0].tags,
        article_payloads[0].tags.as_ref().unwrap()
    );

    app.clean().await;
}

async fn get_articles_list<P: Serialize>(payload: &P, addr: &str) -> Response {
    reqwest::Client::new()
        .post(format!("{}/articles/get-articles", addr))
        .json(payload)
        .send()
        .await
        .unwrap()
}
