pub mod articles;
pub mod auth;

use axum::Router;

use crate::application::AppCtx;

pub fn routes() -> Router<AppCtx> {
    auth::routes().merge(articles::routes())
}
