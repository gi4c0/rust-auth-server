use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;

use crate::parsers::format_errors;

#[derive(Debug, Error)]
pub enum ServerError {
    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),

    #[error("Internal server error: {0}")]
    InternalServerError(#[from] anyhow::Error),
}

#[derive(Serialize)]
pub struct ResponseErrorMessage<T> {
    message: String,
    data: Option<T>,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(e) => {
                let json = ResponseErrorMessage {
                    message: "Input validation error".into(),
                    data: Some(format_errors(&e)),
                };
                (StatusCode::BAD_REQUEST, Json(json)).into_response()
            }

            ServerError::AxumJsonRejection(e) => (
                StatusCode::BAD_REQUEST,
                Json(ResponseErrorMessage::<()> {
                    message: e.to_string(),
                    data: None,
                }),
            )
                .into_response(),

            ServerError::InternalServerError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseErrorMessage::<()> {
                    message: "Internal Server Error".to_string(),
                    data: None,
                }),
            )
                .into_response(),
        }
    }
}
