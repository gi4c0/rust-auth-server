use lib::utils::err::AppError;
use reqwest::header::AUTHORIZATION;
use serde_json::{json, Value};

use crate::helper::TestApp;

#[tokio::test]
async fn login_failed_on_invalid_json_data() {
    let app = TestApp::spawn().await;

    let invalid_username = json!({
        "username": "non_existing_username",
        "password": &app.test_user.password
    });

    let invalid_password = json!({
        "username": &app.test_user.username,
        "password": "invalid_password"
    });

    for body in vec![invalid_username, invalid_password] {
        let response = app.login(&body).await;
        assert_eq!(response.status().as_u16(), 400);

        let json_response: Value = response.json().await.unwrap();

        assert_eq!(
            json_response["message"].as_str().unwrap(),
            AppError::InvalidCredentials.to_string()
        );
    }

    app.clean().await;
}

#[tokio::test]
async fn success_login_returns_header_token() {
    let app = TestApp::spawn().await;

    let body = json!({
        "username": &app.test_user.username,
        "password": &app.test_user.password
    });

    let response = app.login(&body).await;

    assert_eq!(response.status().as_u16(), 200);
    assert!(response.headers().get(AUTHORIZATION).unwrap().len() > 0);

    app.clean().await;
}
