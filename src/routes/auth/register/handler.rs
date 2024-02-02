use axum::{extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use validator::Validate;

use crate::{
    application::AppCtx,
    domains::user::{Email, Password, Username},
    extractors::ValidateJson,
    utils::{
        password::hash_password,
        response::{DataResponse, AppResponse},
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
    State(ctx): State<AppCtx>,
    ValidateJson(payload): ValidateJson<Payload>,
) -> AppResponse {
    let password_hash = hash_password(payload.password).await?;

    let payload = Payload {
        password: password_hash,
        ..payload
    };

    let user_id = insert_new_user(&ctx.db, &payload).await?;
    Ok((StatusCode::OK, DataResponse::new(user_id.to_string())).into_response())
}
