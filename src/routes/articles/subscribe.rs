use axum::{extract::State, http::StatusCode, response::IntoResponse};
use serde::{Deserialize, Serialize};
use sqlx::PgPool;
use tracing::instrument;
use validator::Validate;

use crate::{
    application::AppCtx,
    db::DbResultExt,
    domains::user::UserID,
    extractors::{AuthUser, ValidateJson},
    utils::{
        err::AppError,
        jwt::UserData,
        response::{AppResponse, AppResult},
    },
};

#[derive(Serialize, Deserialize, Debug, Validate)]
pub struct Payload {
    pub user_id: UserID,
}

#[instrument(skip(ctx))]
pub async fn subscribe(
    ctx: State<AppCtx>,
    AuthUser(user): AuthUser<UserData>,
    ValidateJson(payload): ValidateJson<Payload>,
) -> AppResponse {
    if user.user_id == payload.user_id {
        return Err(AppError::BadRequest(
            "User cannot subscribe to himself".to_string(),
        ));
    }

    add_subscription(&ctx.db, &user.user_id, &payload.user_id).await?;

    Ok((StatusCode::CREATED).into_response())
}

#[instrument(skip(pool))]
async fn add_subscription(
    pool: &PgPool,
    user_id: &UserID,
    target_user_id: &UserID,
) -> AppResult<()> {
    sqlx::query!(
        r#"
            INSERT INTO public.subscriptions (subscriber_id, author_id) VALUES (
                $1, $2
            )
            ON CONFLICT DO NOTHING
        "#,
        user_id.as_ref(),
        target_user_id.as_ref()
    )
    .execute(pool)
    .await
    .trace_db("Failed to create subscription")
    .on_constraint("fk_author_id", |_| {
        AppError::BadRequest(format!(
            "Given author id does not exist: {}",
            &target_user_id
        ))
    })?;

    Ok(())
}
