use sqlx::{postgres::PgPoolOptions, Pool, Postgres};

use crate::configuration::DBConfig;

pub async fn connect(db_config: &DBConfig) -> Pool<Postgres> {
    let db_url = db_config.get_connection_string();

    std::env::set_var("DATABASE_URL", &db_url);

    let pool = PgPoolOptions::new()
        .max_connections(10)
        .connect(&db_url)
        .await
        .expect("Failed to connect to DB");

    sqlx::migrate!("./migrations").run(&pool).await.unwrap();

    pool
}
