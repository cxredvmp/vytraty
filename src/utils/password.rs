use argon2::{
    Argon2, PasswordHash,
    password_hash::{SaltString, rand_core::OsRng},
};

use crate::errors::AppError;

pub async fn hash(password: String) -> Result<String, AppError> {
    tokio::task::spawn_blocking(move || -> Result<String, AppError> {
        let salt = SaltString::generate(&mut OsRng);
        Ok(PasswordHash::generate(Argon2::default(), password, &salt)
            .map_err(|e| AppError::Internal(format!("failed to hash password: {e}")))?
            .to_string())
    })
    .await
    .map_err(|e| AppError::Internal(format!("failed to join thread: {e}")))
    .flatten()
}

pub async fn verify(password: String, password_hash: String) -> Result<(), AppError> {
    tokio::task::spawn_blocking(move || -> Result<(), AppError> {
        let hash = PasswordHash::new(&password_hash)
            .map_err(|e| AppError::Internal(format!("invalid password hash: {e}")))?;
        hash.verify_password(&[&Argon2::default()], password)
            .map_err(|e| match e {
                argon2::password_hash::Error::Password => AppError::Auth,
                _ => AppError::Internal(format!("failed to verify password: {e}")),
            })
    })
    .await
    .map_err(|e| AppError::Internal(format!("failed to join thread: {e}")))
    .flatten()
}
