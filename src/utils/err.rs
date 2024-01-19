use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;
use validator::ValidationErrors;

use crate::parsers::format_errors;

use super::response::ErrorResponse;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),

    #[error("Internal server error: {0}")]
    InternalServerError(#[from] anyhow::Error),

    #[error("User with given username or email already exists")]
    DuplicatedUser,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(e) => {
                let json = ErrorResponse::with_data("Input validation error", format_errors(&e));
                (StatusCode::BAD_REQUEST, Json(json)).into_response()
            }

            ServerError::DuplicatedUser => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::message(
                    ServerError::DuplicatedUser.to_string(),
                )),
            )
                .into_response(),

            ServerError::AxumJsonRejection(e) => (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse::message(e.to_string())),
            )
                .into_response(),

            ServerError::InternalServerError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ErrorResponse::message("Internal Server Error")),
            )
                .into_response(),
        }
    }
}
