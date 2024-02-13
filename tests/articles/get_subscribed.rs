use lib::{
    routes::articles::{create_article, Article},
    types::SearchType,
};
use uuid::Uuid;

use crate::helper::TestApp;

#[tokio::test]
async fn should_return_articles_from_subscriptions() {
    let app = TestApp::spawn().await;

    app.create_article(&get_random_article(), &app.test_users[0])
        .await;

    app.subscribe(&app.test_users[0], &app.test_users[1].id)
        .await;

    let user_1_articles = vec![get_random_article(), get_random_article()];

    for article in &user_1_articles {
        app.create_article(article, &app.test_users[1]).await;
    }

    for user_id in vec![Some(app.test_users[1].id.clone()), None] {
        let response = app.get_subscribed(&app.test_users[0], user_id).await;
        let json: SearchType<Article> = response.json().await.unwrap();
        assert_eq!(json.total, 2);
    }

    let response = app
        .get_subscribed(&app.test_users[0], Some(app.test_users[5].id.clone()))
        .await;

    let json: SearchType<Article> = response.json().await.unwrap();
    assert_eq!(json.total, 0);

    app.clean().await;
}

fn get_random_article() -> create_article::Payload {
    create_article::Payload {
        text: Uuid::new_v4().to_string(),
        title: Uuid::new_v4().to_string(),
        tags: None,
    }
}
