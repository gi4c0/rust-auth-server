use axum::{extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;

use crate::{
    domains::user::{Email, Username},
    parsers::ValidateJson,
    utils::response::{DataResponse, ServerResponse},
};

use super::loader::insert_new_user;

#[derive(Deserialize, Debug, Serialize, Validate)]
pub struct Payload {
    #[validate(custom = "crate::parsers::user::validate_username")]
    pub username: Username,

    #[validate(length(
        min = 8,
        max = 50,
        message = "Invalid password. Expected length 1 < x < 50"
    ))]
    pub password: String,

    #[validate(custom = "crate::parsers::user::validate_email")]
    pub email: Email,
}

pub async fn register(
    State(pool): State<PgPool>,
    ValidateJson(payload): ValidateJson<Payload>,
) -> ServerResponse {
    let user_id = insert_new_user(&pool, &payload).await?;
    Ok((StatusCode::OK, DataResponse::new(user_id.to_string())).into_response())
}
