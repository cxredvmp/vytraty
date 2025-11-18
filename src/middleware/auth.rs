use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_extra::headers::{Authorization, HeaderMapExt, authorization::Bearer};

use crate::{config::Config, errors::AppError, models::auth::*, utils::jwt};

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
    Ok(parts
        .headers
        .typed_get::<Authorization<Bearer>>()
        .ok_or(AppError::Auth)?
        .token()
        .to_string())
}
