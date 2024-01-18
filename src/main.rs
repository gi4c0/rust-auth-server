use axum::{routing::post, Router};
use tokio::net::TcpListener;

use crate::routes::auth::register;

mod configuration;
mod db;
mod domains;
mod parsers;
mod routes;
mod utils;

#[tokio::main]
async fn main() {
    let config = configuration::parse_config();
    let pool = db::connect(&config.db).await;

    let app = Router::new()
        .route("/auth/register", post(register))
        .with_state(pool);

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    dbg!("Listen on port 3000");

    axum::serve(listener, app).await.unwrap();
}
