use axum::{
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;
use tower_http::trace::{self, TraceLayer};
use tracing::Level;

use crate::{
    configuration::Configuration,
    db,
    routes::{auth, root},
};

pub struct Application {
    router: Router,
    listener: TcpListener,
    port: u16,
}

impl Application {
    pub async fn build(config: &Configuration) -> Self {
        let pool = db::connect(&config.db).await;

        let router = Router::new()
            .route("/auth/register", post(auth::register))
            .route("/auth/login", post(auth::login))
            .route("/auth/me", get(auth::me))
            .route("/", get(root))
            .with_state(pool)
            .layer(
                TraceLayer::new_for_http()
                    .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
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
