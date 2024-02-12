use anyhow::Context;
use axum::{
    extract::State,
    http::{header, HeaderMap, StatusCode},
    response::IntoResponse,
};
use serde::{Deserialize, Serialize};
use tracing::instrument;
use validator::Validate;

use crate::{
    application::AppCtx,
    domains::user::{Password, Username},
    extractors::ValidateJson,
    utils::{err::AppError, jwt, password::verify_password, response::AppResponse},
};

use super::loader::{get_user, UserLoginInfo};

#[derive(Serialize, Deserialize, Validate, Debug)]
pub struct Payload {
    #[validate(custom = "crate::parsers::user::validate_username")]
    pub username: Username,

    #[validate(custom = "crate::parsers::user::validate_password")]
    pub password: Password,
}

#[instrument(skip(ctx))]
pub async fn login(
    State(ctx): State<AppCtx>,
    ValidateJson(payload): ValidateJson<Payload>,
) -> AppResponse {
    let UserLoginInfo { password_hash, id } = get_user(&ctx.db, &payload.username)
        .await?
        .ok_or(AppError::InvalidCredentials)?;

    let is_valid_password = verify_password(payload.password, password_hash).await?;

    if !is_valid_password {
        return Err(AppError::InvalidCredentials);
    }

    let token = jwt::sign(id)?;
    let mut headers = HeaderMap::new();

    headers.insert(
        header::AUTHORIZATION,
        token.parse().context("Failed to set a JWT header")?,
    );

    Ok((StatusCode::OK, headers).into_response())
}
