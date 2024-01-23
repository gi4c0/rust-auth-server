use std::time::{Duration, SystemTime, UNIX_EPOCH};

use anyhow::Context;
use jsonwebtoken::{encode, Algorithm, EncodingKey, Header};
use serde::{Deserialize, Serialize};

use crate::domains::user::UserID;

#[derive(Serialize, Deserialize)]
struct Claims {
    exp: u64,
    data: ClaimsData,
}

#[derive(Serialize, Deserialize)]
struct ClaimsData {
    user_id: UserID,
}

pub fn sign(user_id: UserID) -> anyhow::Result<String> {
    let now = SystemTime::now();
    let exp = now
        .duration_since(UNIX_EPOCH)
        .context("Failed to count time from unix epoch")?;

    let exp = exp + Duration::from_secs(60 * 60 * 24 * 2); // 48 hours

    let claims = Claims {
        exp: exp.as_secs(),
        data: ClaimsData { user_id },
    };

    let header = Header {
        alg: Algorithm::HS512,
        ..Default::default()
    };

    let secret = std::env::var("JWT_SECRET")?;

    encode(
        &header,
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .context("Failed to sign jwt")
}
