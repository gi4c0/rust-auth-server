use axum::{extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;

use crate::{
    domains::user::{Email, Password, Username},
    parsers::ValidateJson,
    utils::{
        password::hash_password,
        response::{DataResponse, ServerResponse},
    },
};

use super::loader::insert_new_user;

#[derive(Deserialize, Debug, Serialize, Validate)]
pub struct Payload {
    #[validate(custom = "crate::parsers::user::validate_username")]
    pub username: Username,

    #[validate(custom = "crate::parsers::user::validate_password")]
    pub password: Password,

    #[validate(custom = "crate::parsers::user::validate_email")]
    pub email: Email,
}

pub async fn register(
    State(pool): State<PgPool>,
    ValidateJson(payload): ValidateJson<Payload>,
) -> ServerResponse {
    let password_hash = hash_password(payload.password).await?;

    let payload = Payload {
        password: password_hash,
        ..payload
    };

    let user_id = insert_new_user(&pool, &payload).await?;
    Ok((StatusCode::OK, DataResponse::new(user_id.to_string())).into_response())
}
