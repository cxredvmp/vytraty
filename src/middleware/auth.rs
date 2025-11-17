use axum::{
    extract::{FromRef, FromRequestParts},
    http::{header, request::Parts},
};

use crate::{config::Config, error::AppError, models::auth::*, utils::jwt};

impl<S> FromRequestParts<S> for UserAuth
where
    S: Send + Sync,
    Config: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        let token = token_from_parts(parts)?;
        let claims = jwt::verify(&token, Config::from_ref(state).jwt_secret())?;
        let user_auth = Self { id: claims.sub };
        parts.extensions.insert(user_auth.clone());
        Ok(user_auth)
    }
}

fn token_from_parts(parts: &mut Parts) -> Result<String, AppError> {
    let header = parts
        .headers
        .get(header::AUTHORIZATION)
        .ok_or(AppError::Unauthorized)?
        .to_str()
        .map_err(|e| AppError::Internal(format!("failed to convert header to string: {e}")))?;
    let token = header
        .strip_prefix("Bearer ")
        .ok_or(AppError::Unauthorized)?;
    Ok(token.to_string())
}
