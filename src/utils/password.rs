use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHash, PasswordHasher, PasswordVerifier,
};
use tokio::task::spawn_blocking;

use crate::domains::user::Password;

pub async fn hash_password(password: Password) -> anyhow::Result<Password> {
    spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);

        Argon2::default()
            .hash_password(password.as_ref().as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))
            .map(|p| Password(p.to_string()))
    })
    .await?
}

pub async fn verify_password(raw_password: Password, hash: Password) -> anyhow::Result<bool> {
    spawn_blocking(move || {
        let parsed_hash = PasswordHash::new(hash.as_ref())
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))?;

        let result = Argon2::default()
            .verify_password(raw_password.0.as_bytes(), &parsed_hash)
            .is_ok();

        Ok(result)
    })
    .await?
}
