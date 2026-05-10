use argon2::{
    Argon2, PasswordHash,
    password_hash::{Error as PasswordHashError, SaltString, rand_core::OsRng},
};

use crate::error::{AppError, AuthError, Result};

pub async fn hash(password: String) -> Result<String> {
    tokio::task::spawn_blocking(move || -> Result<String> {
        let salt = SaltString::generate(&mut OsRng);
        Ok(PasswordHash::generate(Argon2::default(), password, &salt)
            .map_err(|e| AppError::Internal(format!("failed to hash password: {e}")))?
            .to_string())
    })
    .await
    .map_err(|e| AppError::Internal(format!("failed to join thread: {e}")))
    .flatten()
}

pub async fn verify(password: String, password_hash: String) -> Result<()> {
    tokio::task::spawn_blocking(move || -> Result<()> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| AppError::Internal(format!("invalid password hash: {e}")))?;
        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                PasswordHashError::Password => AppError::Auth(AuthError::InvalidPassword),
                _ => AppError::Internal(format!("failed to verify password: {e}")),
            })
    })
    .await
    .map_err(|e| AppError::Internal(format!("failed to join thread: {e}")))
    .flatten()
}
