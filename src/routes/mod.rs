use axum::{extract::Request, http::StatusCode, response::IntoResponse};

use crate::utils::{
    err::ServerError,
    jwt::UserData,
    response::{DataResponse, ServerResponse},
};

pub mod auth;

pub async fn authorized_endpoint(request: Request) -> ServerResponse {
    let user_data: &UserData = request
        .extensions()
        .get()
        .ok_or(ServerError::Unauthorized)?;

    Ok((StatusCode::OK, DataResponse::new(user_data.clone())).into_response())
}
