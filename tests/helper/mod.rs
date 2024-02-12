mod app;
mod users;

pub use app::*;
pub use users::*;

use lib::configuration::DBConfig;
use sqlx::{Connection, Executor, PgConnection, PgPool};

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
