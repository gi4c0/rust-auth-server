use std::cell::RefCell;

use dotenv::dotenv;
use lib::{
    application::Application,
    configuration::{self, AppConfig, Configuration, DBConfig},
};
use sqlx::{Connection, Executor, PgConnection, PgPool};
use uuid::Uuid;

pub struct TestApp {
    pub address: String,
    pub pool: PgPool,
    connection: RefCell<PgConnection>,
    db_name: String,
}

impl TestApp {
    pub async fn spawn() -> TestApp {
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

        let app = Application::build(&config).await.unwrap();
        let port = app.get_port();

        tokio::spawn(app.run());

        TestApp {
            address: format!("127.0.0.1:{port}"),
            connection: RefCell::new(connection),
            pool,
            db_name,
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
}

async fn create_db(config: &DBConfig) -> (PgConnection, PgPool) {
    let mut connection = PgConnection::connect_with(&config.without_db())
        .await
        .expect("Failed to connect to DB");

    connection
        .execute(format!("CREATE DATABASE \"{}\"", &config.db_name).as_str())
        .await
        .unwrap();

    let pool = PgPool::connect_with(config.with_db())
        .await
        .expect("Failed to create pool");

    sqlx::query("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\";")
        .execute(&pool)
        .await
        .expect("Failed to create UUID extension");

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    (connection, pool)
}
