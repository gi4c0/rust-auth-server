use axum::{http::StatusCode, response::IntoResponse};

use crate::{
    extractors::AuthUser,
    utils::{
        jwt::UserData,
        response::{DataResponse, ServerResponse},
    },
};

pub async fn me(user: AuthUser<UserData>) -> ServerResponse {
    Ok((StatusCode::OK, DataResponse::new(user.0)).into_response())
}
