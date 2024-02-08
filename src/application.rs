use axum::{extract::MatchedPath, http::Request, Router};
use sqlx::{postgres::PgPoolOptions, PgPool};
use tokio::net::TcpListener;
use tower_http::trace::{self, TraceLayer};
use tracing::{info_span, Level};
use uuid::Uuid;

use crate::{
    configuration::{Configuration, DBConfig},
    routes::routes,
};

pub struct App {
    router: Router,
    listener: TcpListener,
    port: u16,
}

#[derive(Clone)]
pub struct AppCtx {
    pub db: PgPool,
}

impl App {
    pub async fn build(config: &Configuration) -> Self {
        let pool = connect(&config.db).await;
        let state = AppCtx { db: pool };

        let router = Router::new().merge(routes()).with_state(state).layer(
            TraceLayer::new_for_http()
                .make_span_with(|request: &Request<_>| {
                    let matched_path = request
                        .extensions()
                        .get::<MatchedPath>()
                        .map(MatchedPath::as_str);

                    let request_id = Uuid::new_v4().to_string();

                    info_span!(
                        "http_request",
                        method = ?request.method(),
                        matched_path,
                        ID = request_id
                    )
                })
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

        let listener = TcpListener::bind(format!("{}:{}", &config.app.host, &config.app.port))
            .await
            .unwrap();

        let port = listener.local_addr().unwrap().port();

        Self {
            router,
            listener,
            port,
        }
    }

    pub fn get_port(&self) -> u16 {
        self.port
    }

    pub async fn run(self) {
        axum::serve(self.listener, self.router).await.unwrap()
    }
}

async fn connect(db_config: &DBConfig) -> PgPool {
    PgPoolOptions::new()
        .max_connections(10)
        .connect_with(db_config.with_db())
        .await
        .expect("Failed to connect to DB")
}
