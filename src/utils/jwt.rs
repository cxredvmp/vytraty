use chrono::{Duration, Utc};
use jsonwebtoken::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Serialize, Deserialize)]
pub struct Claims {
    pub sub: Uuid,
    pub exp: usize,
}

pub fn sign(user_id: Uuid, secret: &str) -> Result<String, AppError> {
    let claims = Claims {
        sub: user_id,
        exp: (Utc::now() + Duration::hours(1))
            .timestamp()
            .try_into()
            .unwrap(),
    };
    encode(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret(secret.as_bytes()),
    )
    .map_err(|e| AppError::Internal(format!("failed to encode token: {e}")))
}

pub fn verify(token: &str, secret: &str) -> Result<Claims, AppError> {
    decode::<Claims>(
        token,
        &DecodingKey::from_secret(secret.as_bytes()),
        &Validation::default(),
    )
    .map_err(|_| AppError::Unauthorized)
    .map(|data| data.claims)
}
