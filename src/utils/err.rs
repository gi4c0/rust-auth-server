use axum::{
    extract::rejection::JsonRejection,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use thiserror::Error;
use tracing::error;
use validator::ValidationErrors;

use crate::parsers::format_errors;

use super::response::{AppResult, ErrorResponse};

#[derive(Debug, Error)]
pub enum AppError {
    #[error(transparent)]
    ValidationError(#[from] ValidationErrors),

    #[error(transparent)]
    AxumJsonRejection(#[from] JsonRejection),

    #[error("An error occurred with the DB")]
    DbError(#[from] sqlx::Error),

    #[error("Internal server error: {0}")]
    InternalServerError(#[from] anyhow::Error),

    #[error("User with given username or email already exists")]
    DuplicatedUser,

    #[error("{0}")]
    NotFound(String),

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
            AppError::AxumJsonRejection(_) => StatusCode::BAD_REQUEST,
            AppError::DuplicatedUser => StatusCode::BAD_REQUEST,
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

pub trait DbResultExt<T> {
    fn trace_db(self, msg: &str) -> AppResult<T>;
    fn with_unique_violation(self, provided_err: AppError, context: &str) -> AppResult<T>;
}

impl<T> DbResultExt<T> for Result<T, sqlx::Error> {
    fn trace_db(self, msg: &str) -> AppResult<T> {
        match self {
            Ok(result) => Ok(result),
            Err(e) => {
                tracing::error!("DB Error ({msg}) {e}");
                Err(e.into())
            }
        }
    }

    fn with_unique_violation(self, provided_err: AppError, context: &str) -> AppResult<T> {
        match self {
            Ok(result) => Ok(result),
            Err(db_err) => {
                if let sqlx::Error::Database(e) = &db_err {
                    if e.is_unique_violation() {
                        error!("{context} ({provided_err}) {db_err}");
                        return Err(provided_err);
                    }
                }

                tracing::error!("DB Error ({context}) {db_err}");
                return Err(db_err.into());
            }
        }
    }
}
