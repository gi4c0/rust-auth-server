use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::configuration::DBConfig;

pub async fn connect(db_config: &DBConfig) -> Pool<Postgres> {
    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect_with(db_config.with_db())
        .await
        .expect("Failed to connect to DB");

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    pool
}

pub fn trace_error(context: &'static str) -> impl Fn(sqlx::Error) -> anyhow::Error {
    return move |error: sqlx::Error| -> anyhow::Error {
        tracing::error!("DB Error: {}", error);
        anyhow::anyhow!("DB Error: {context}")
    };
}
