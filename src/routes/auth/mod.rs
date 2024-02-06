mod login;
mod me;
mod register;

use axum::{
    routing::{get, post},
    Router,
};

use login::login;
use me::me;
use register::register;

use crate::application::AppCtx;

pub fn routes() -> axum::Router<AppCtx> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/me", get(me))
}
