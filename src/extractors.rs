use axum::{
    async_trait,
    extract::{FromRequest, FromRequestParts, Request},
    http::{header::AUTHORIZATION, request::Parts},
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::utils::{err::AppError, jwt};

pub struct ValidateJson<T>(pub T);

#[async_trait]
impl<T, S> FromRequest<S> for ValidateJson<T>
where
    S: Sync + Send,
    T: DeserializeOwned + Validate,
{
    type Rejection = AppError;

    async fn from_request(req: Request, state: &S) -> Result<Self, Self::Rejection> {
        let Json(value) = Json::<T>::from_request(req, state).await?;
        value.validate()?;

        Ok(ValidateJson(value))
    }
}

pub struct AuthUser<T>(pub T);

#[async_trait]
impl<S, T> FromRequestParts<S> for AuthUser<T>
where
    S: Sync + Send,
    T: DeserializeOwned,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let token = parts
            .headers
            .get(AUTHORIZATION)
            .ok_or(AppError::Unauthorized)?;

        let token = token.to_str().map_err(|_| AppError::Unauthorized)?;

        let user_info: T = jwt::verify(token).map_err(|e| {
            dbg!(e);
            AppError::Unauthorized
        })?;

        Ok(Self(user_info))
    }
}
