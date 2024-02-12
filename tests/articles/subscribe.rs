use axum::http::StatusCode;
use lib::domains::user::UserID;
use serde_json::Value;
use uuid::Uuid;

use crate::helper::TestApp;

#[tokio::test]
async fn fail_to_subscribe_to_myself() {
    let app = TestApp::spawn().await;

    let response = app
        .subscribe(&app.test_users[0], &app.test_users[0].id)
        .await;

    assert_eq!(response.status().as_u16(), 400);

    let response_json: Value = response.json().await.unwrap();
    assert_eq!(
        response_json["message"].as_str().unwrap(),
        "User cannot subscribe to himself"
    );

    app.clean().await;
}

#[tokio::test]
async fn fail_to_subscribe_to_user_that_does_not_exist() {
    let app = TestApp::spawn().await;
    let fake_author_id = UserID(Uuid::new_v4());

    let response = app.subscribe(&app.test_users[0], &fake_author_id).await;

    assert_eq!(response.status().as_u16(), 400);

    let response_json: Value = response.json().await.unwrap();

    assert_eq!(
        response_json["message"].as_str().unwrap(),
        format!("Given author id does not exist: {}", fake_author_id)
    );

    app.clean().await;
}

#[tokio::test]
async fn successfully_subscribed_twice() {
    let app = TestApp::spawn().await;

    for _ in 0..2 {
        let response = app
            .subscribe(&app.test_users[0], &app.test_users[1].id)
            .await;

        assert_eq!(response.status().as_u16(), StatusCode::CREATED);
    }

    app.clean().await;
}
