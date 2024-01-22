use crate::helper::TestApp;

use serde_json::json;

#[tokio::test]
async fn return_400_for_invalid_data() {
    let app = TestApp::spawn().await;

    let invalid_username = json!({
        "username": "u",
        "password": "pass12359823",
        "email": "user@mail.com"
    });

    let response = reqwest::Client::new()
        .post(format!("http://{}/auth/register", &app.address))
        .json(&invalid_username)
        .send()
        .await;

    dbg!(&response);

    assert_eq!(response.unwrap().status().as_u16(), 400);

    app.clean().await;
}
