pub mod auth;
mod root;

use axum::{routing::get, Router};
pub use root::*;

use crate::application::AppCtx;

pub fn routes() -> Router<AppCtx> {
    auth::routes().route("/", get(root))
}
