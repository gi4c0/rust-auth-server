use crate::helper::TestApp;

use claims::assert_ok;
use lib::utils::{err::ServerError, response::DataResponse};
use reqwest::Response;
use serde_json::{json, Value};
use uuid::Uuid;

#[tokio::test]
async fn return_400_for_invalid_data() {
    let app = TestApp::spawn().await;

    let invalid_username = json!({
        "username": "u",
        "password": "pass12359823",
        "email": "user@mail.com"
    });

    check_endpoint(invalid_username, "username", &app.address).await;

    let invalid_password = json!({
        "username": "username",
        "password": "pa",
        "email": "user@mail.com"
    });

    check_endpoint(invalid_password, "password", &app.address).await;

    let invalid_email = json!({
        "username": "username",
        "password": "pass12359823",
        "email": "user_mail.com"
    });

    check_endpoint(invalid_email, "email", &app.address).await;

    app.clean().await;
}

#[tokio::test]
async fn return_200_on_valid_data() {
    let app = TestApp::spawn().await;

    let body = json!({
        "username": "username",
        "password": "pass12359823",
        "email": "user@mail.com"
    });

    let response = make_request(&body, &app.address).await;
    assert_eq!(response.status().as_u16(), 200);

    let response: DataResponse<String> = response.json().await.unwrap();
    assert_ok!(Uuid::parse_str(&response.data));

    app.clean().await;
}

#[tokio::test]
async fn return_400_on_existing_user() {
    let app = TestApp::spawn().await;

    let body = json!({
        "username": "username",
        "password": "pass12359823",
        "email": "user@mail.com"
    });

    let response = make_request(&body, &app.address).await;
    assert_eq!(response.status().as_u16(), 200);

    let duplicated_email_body = json!({
        "username": "another_username",
        "password": "pass12359823",
        "email": "user@mail.com"
    });

    let duplicated_username_body = json!({
        "username": "username",
        "password": "pass12359823",
        "email": "another_email@mail.com"
    });

    for body in vec![duplicated_username_body, duplicated_email_body] {
        let response = make_request(&body, &app.address).await;
        assert_eq!(response.status().as_u16(), 400);

        let json: Value = response.json().await.unwrap();
        assert_eq!(
            json["message"].as_str().unwrap(),
            &ServerError::DuplicatedUser.to_string()
        );
    }

    app.clean().await;
}

async fn check_endpoint(body: Value, error_part: &str, address: &str) {
    let response = make_request(&body, address).await;
    assert_eq!(response.status().as_u16(), 400);

    let response_json: Value = response.json().await.unwrap();

    assert!(response_json["data"].as_array().unwrap()[0]
        .as_str()
        .unwrap()
        .contains(error_part));
}

async fn make_request(body: &Value, address: &str) -> Response {
    reqwest::Client::new()
        .post(format!("http://{}/auth/register", address))
        .json(body)
        .send()
        .await
        .unwrap()
}
