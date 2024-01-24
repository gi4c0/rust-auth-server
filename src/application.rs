use axum::{
    middleware,
    routing::{get, post},
    Router,
};
use tokio::net::TcpListener;

use crate::{
    configuration::Configuration,
    db, middlewares,
    routes::{
        self,
        auth::{login::login, register},
    },
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
            .route("/auth/register", post(register))
            .route("/auth/login", post(login))
            .route(
                "/me",
                get(routes::authorized_endpoint).layer(middleware::from_fn(middlewares::auth)),
            )
            .with_state(pool);

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
