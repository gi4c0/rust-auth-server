use std::cell::RefCell;

use dotenv::dotenv;
use lib::{
    application::App,
    configuration::{self, AppConfig, Configuration, DBConfig},
    domains::user::UserID,
    routes::articles::create_article,
};
use reqwest::{
    header::{HeaderValue, AUTHORIZATION},
    Client, Response,
};
use serde_json::{json, Value};
use sqlx::{Executor, PgConnection, PgPool};
use tokio::sync::OnceCell;
use tracing::Level;
use uuid::Uuid;

use super::{create_db, create_test_user, TestUser};

pub struct TestApp {
    pub address: String,
    pub pool: PgPool,
    pub test_users: Vec<TestUser>,
    connection: RefCell<PgConnection>,
    db_name: String,
}

async fn init_tracing() {
    if std::env::var("TRACE").is_ok() {
        tracing_subscriber::fmt()
            .with_target(false)
            .with_max_level(Level::DEBUG)
            .pretty()
            .init();
    }
}

static TRACING: OnceCell<()> = OnceCell::const_new();

impl TestApp {
    pub async fn spawn() -> TestApp {
        TRACING.get_or_init(init_tracing).await;

        dotenv().unwrap();
        let config = configuration::parse_config();
        let db_name = Uuid::new_v4().to_string();

        let config = Configuration {
            app: AppConfig {
                port: 0,
                host: config.app.host,
            },
            db: DBConfig {
                db_name: db_name.clone(),
                ..config.db
            },
        };

        let (connection, pool) = create_db(&config.db).await;

        let app = App::build(&config).await;
        let port = app.get_port();

        tokio::spawn(app.run());
        let mut test_users = vec![];

        for _ in 0..10 {
            test_users.push(create_test_user(&pool).await);
        }

        TestApp {
            address: format!("http://127.0.0.1:{port}"),
            connection: RefCell::new(connection),
            pool,
            db_name,
            test_users,
        }
    }

    pub async fn clean(self) {
        let mut connection = self.connection.borrow_mut();

        connection
            .execute(
                format!(
                    r#"SELECT pg_terminate_backend(pid) FROM pg_stat_activity WHERE datname = '{}';"#,
                    &self.db_name
                )
                .as_str(),
            )
            .await
            .expect("Failed to disconnect users from database before drop");

        connection
            .execute(format!(r#"DROP DATABASE IF EXISTS "{}";"#, &self.db_name).as_str())
            .await
            .expect("Failed to drop database");
    }

    pub async fn login(&self, body: &Value) -> Response {
        reqwest::Client::new()
            .post(format!("{}/auth/login", &self.address))
            .json(body)
            .send()
            .await
            .unwrap()
    }

    pub async fn subscribe(&self, test_user: &TestUser, target_user_id: &UserID) -> Response {
        let json = json!({
            "user_id": target_user_id
        });

        let jwt = self.get_jwt(test_user).await;

        Client::new()
            .post(format!("{}/articles/subscribe", &self.address))
            .json(&json)
            .header(AUTHORIZATION, jwt)
            .send()
            .await
            .unwrap()
    }

    async fn get_jwt(&self, test_user: &TestUser) -> HeaderValue {
        let body = json!({
            "username": &test_user.username,
            "password": &test_user.password
        });

        let response = self.login(&body).await;

        response.headers().get(AUTHORIZATION).unwrap().to_owned()
    }

    pub async fn create_article(
        &self,
        payload: &create_article::Payload,
        user: &TestUser,
    ) -> Response {
        let jwt = self.get_jwt(user).await;

        Client::new()
            .post(format!("{}/articles", &self.address))
            .json(payload)
            .header(AUTHORIZATION, jwt)
            .send()
            .await
            .unwrap()
    }

    pub async fn get_subscribed(
        &self,
        user: &TestUser,
        filter_by_user_id: Option<UserID>,
    ) -> Response {
        let jwt = self.get_jwt(user).await;

        Client::new()
            .get(format!("{}/articles/get-subscribed", &self.address))
            .query(&[("user_id", filter_by_user_id)])
            .header(AUTHORIZATION, jwt)
            .send()
            .await
            .unwrap()
    }
}
