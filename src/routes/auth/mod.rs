mod login;
mod me;
pub mod register;

use axum::{
    routing::{get, post},
    Router,
};
pub use login::*;
pub use me::*;
pub use register::register;

use crate::application::AppCtx;

pub fn routes() -> axum::Router<AppCtx> {
    Router::new()
        .route("/auth/register", post(register))
        .route("/auth/login", post(login))
        .route("/auth/me", get(me))
}
