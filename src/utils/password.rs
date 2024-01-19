use argon2::{
    password_hash::{rand_core::OsRng, SaltString},
    Argon2, PasswordHasher,
};
use tokio::task::spawn_blocking;

pub async fn hash_password(password: String) -> anyhow::Result<String> {
    spawn_blocking(move || {
        let salt = SaltString::generate(&mut OsRng);
        Argon2::default()
            .hash_password(password.as_bytes(), &salt)
            .map_err(|e| anyhow::anyhow!("Failed to hash password: {}", e))
            .map(|p| p.to_string())
    })
    .await?
}
