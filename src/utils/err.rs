use axum::{
    extract::rejection::{JsonRejection, QueryRejection},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;
use tracing::error;
use validator::ValidationErrors;

use crate::parsers::format_errors;

use super::response::ErrorResponse;

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),

    #[error("Malformed JSON")]
    AxumJsonRejection(#[from] JsonRejection),

    #[error("Failed to parse query: invalid format")]
    AxumQueryRejection(#[from] QueryRejection),

    #[error("An error occurred with the DB")]
    DbError(#[from] sqlx::Error),

    #[error("Internal server error: {0}")]
    InternalServerError(#[from] anyhow::Error),

    #[error("User with given username or email already exists")]
    DuplicatedUser,

    #[error("Article with given title already exists")]
    DuplicatedArticle,

    #[error("{0}")]
    NotFound(String),

    #[error("{0}")]
    BadRequest(String),

    #[error("Invalid login or password")]
    InvalidCredentials,

    #[error("Unauthorized")]
    Unauthorized,
}

impl AppError {
    pub fn status_code(&self) -> StatusCode {
        match self {
            AppError::Unauthorized => StatusCode::UNAUTHORIZED,
            AppError::NotFound(_) => StatusCode::NOT_FOUND,
            AppError::ValidationError(_) => StatusCode::BAD_REQUEST,
            AppError::AxumJsonRejection(_) | AppError::AxumQueryRejection(_) => {
                StatusCode::UNPROCESSABLE_ENTITY
            }
            AppError::DuplicatedUser => StatusCode::BAD_REQUEST,
            AppError::BadRequest(_) => StatusCode::BAD_REQUEST,
            AppError::DuplicatedArticle => StatusCode::BAD_REQUEST,
            AppError::InvalidCredentials => StatusCode::BAD_REQUEST,
            AppError::InternalServerError(_) | AppError::DbError(_) => {
                StatusCode::INTERNAL_SERVER_ERROR
            }
        }
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match self {
            AppError::ValidationError(e) => {
                let json = ErrorResponse::with_data("Input validation error", format_errors(&e));
                return (StatusCode::BAD_REQUEST, Json(json)).into_response();
            }

            AppError::DbError(_) => {
                // Should be already traced
                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse::message("Internal Server Error")),
                )
                    .into_response();
            }

            AppError::InternalServerError(e) => {
                error!("Internal server error: {}", e);

                return (
                    StatusCode::INTERNAL_SERVER_ERROR,
                    Json(ErrorResponse::message("Internal Server Error")),
                )
                    .into_response();
            }

            _ => (),
        };

        (
            self.status_code(),
            Json(ErrorResponse::message(self.to_string())),
        )
            .into_response()
    }
}
