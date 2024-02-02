use axum::{extract::MatchedPath, http::Request, Router};
use sqlx::PgPool;
use tokio::net::TcpListener;
use tower_http::trace::{self, TraceLayer};
use tracing::{info_span, Level};
use uuid::Uuid;

use crate::{configuration::Configuration, db, routes::routes};

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
        let pool = db::connect(&config.db).await;
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
