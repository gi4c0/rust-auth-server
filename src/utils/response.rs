use std::fmt::Display;

use axum::{response::Response, Json};
use serde::{Deserialize, Serialize};

use super::err::ServerError;

#[derive(Serialize)]
pub struct ErrorResponse<T> {
    message: String,
    data: Option<T>,
}

impl ErrorResponse<()> {
    pub fn message<Message: Display>(err_message: Message) -> ErrorResponse<()> {
        ErrorResponse {
            message: err_message.to_string(),
            data: None,
        }
    }
}

impl<T> ErrorResponse<T> {
    pub fn with_data<Message: Display>(message: Message, data: T) -> Self {
        ErrorResponse {
            message: message.to_string(),
            data: Some(data),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DataResponse<T> {
    pub data: T,
}

impl<T: Serialize> DataResponse<T> {
    pub fn new(data: T) -> Json<Self> {
        Json(DataResponse { data })
    }
}

pub type ServerResult<T> = Result<T, ServerError>;
pub type ServerResponse = Result<Response, ServerError>;
