use axum::{
    async_trait,
    extract::{FromRequest, Request},
    Json,
};
use serde::de::DeserializeOwned;
use validator::Validate;

use crate::utils::err::AppError;

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
