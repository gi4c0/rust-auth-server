use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::configuration::DBConfig;

pub async fn connect(db_config: &DBConfig) -> Pool<Postgres> {
    PgPoolOptions::new()
        .max_connections(10)
        .connect_with(db_config.with_db())
        .await
        .expect("Failed to connect to DB")
}
