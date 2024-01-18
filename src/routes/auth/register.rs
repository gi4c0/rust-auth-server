use axum::{
    extract::State,
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;

use crate::{
    domains::user::{Email, Username},
    parsers::{
        user::{validate_email, validate_username},
        ValidateJson,
    },
};

#[derive(Deserialize, Debug, Serialize, Validate)]
pub struct Payload {
    #[validate(custom = "validate_username")]
    username: Username,

    #[validate(length(
        min = 8,
        max = 50,
        message = "Invalid password. Expected length 1 < x < 50"
    ))]
    password: String,

    #[validate(custom = "validate_email")]
    email: Email,
}

pub async fn register(
    // ValidateJson(payload): ValidateJson<Payload>,
    State(_pool): State<PgPool>,
    ValidateJson(payload): ValidateJson<Payload>,
) -> Response {
    (StatusCode::OK, Json(payload)).into_response()
}