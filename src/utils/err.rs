use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::Serialize;
use thiserror::Error;
use validator::ValidationErrors;

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
pub struct ResponseErrorMessage {
    message: String,
}

impl IntoResponse for ServerError {
    fn into_response(self) -> Response {
        match self {
            ServerError::ValidationError(e) => {
                let message = format!("Input validation error: [{e}]");
                (
                    StatusCode::BAD_REQUEST,
                    Json(ResponseErrorMessage { message }),
                )
                    .into_response()
            }

            ServerError::AxumJsonRejection(e) => {
                let message = e.to_string();
                (
                    StatusCode::BAD_REQUEST,
                    Json(ResponseErrorMessage { message }),
                )
                    .into_response()
            }

            ServerError::InternalServerError(_) => (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(ResponseErrorMessage {
                    message: "Internal Server Error".to_string(),
                }),
            )
                .into_response(),
        }
    }
}
