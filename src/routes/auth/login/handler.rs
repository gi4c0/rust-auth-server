use anyhow::Context;
use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use validator::Validate;

use crate::{
    domains::user::{Password, Username},
    parsers::ValidateJson,
    utils::{err::ServerError, jwt, password::verify_password, response::ServerResponse},
};

use super::loader::{get_user, UserLoginInfo};

#[derive(Serialize, Deserialize, Validate)]
pub struct Payload {
    #[validate(custom = "crate::parsers::user::validate_username")]
    pub username: Username,

    #[validate(custom = "crate::parsers::user::validate_password")]
    pub password: Password,
}

pub async fn login(
    pool: State<PgPool>,
    ValidateJson(payload): ValidateJson<Payload>,
) -> ServerResponse {
    let UserLoginInfo { password_hash, id } = get_user(&pool, &payload.username)
        .await?
        .ok_or(ServerError::InvalidCredentials)?;

    let is_valid_password = verify_password(payload.password, password_hash).await?;

    if !is_valid_password {
        return Err(ServerError::InvalidCredentials);
    }

    let token = jwt::sign(id)?;
    let mut headers = HeaderMap::new();

    headers.insert(
        header::AUTHORIZATION,
        token.parse().context("Failed to set a JWT header")?,
    );

    Ok((StatusCode::OK, headers).into_response())
}
