use axum::{response::IntoResponse, routing::get, Router};
use tokio::net::TcpListener;

#[tokio::main]
async fn main() {
    let app = Router::new().route("/", get(handler));

    let listener = TcpListener::bind("0.0.0.0:3000").await.unwrap();
    dbg!("Listen on port 3000");

    axum::serve(listener, app).await.unwrap();
}

async fn handler() -> impl IntoResponse {
    "Hello world"
}
