use axum::{
    extract::{FromRef, FromRequestParts},
    http::request::Parts,
};
use axum_extra::headers::{Authorization, HeaderMapExt, authorization::Bearer};

use crate::{
    config::Config,
    error::{AppError, AuthError, Result},
    model,
    utils::jwt,
};

impl<S> FromRequestParts<S> for model::auth::Auth
where
    S: Send + Sync,
    Config: FromRef<S>,
{
    type Rejection = AppError;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self> {
        let token = token_from_parts(parts)?;
        let claims = jwt::verify(&token, Config::from_ref(state).jwt_secret())?;
        let user_auth = Self { id: claims.sub };
        parts.extensions.insert(user_auth.clone());
        Ok(user_auth)
    }
}

fn token_from_parts(parts: &mut Parts) -> Result<String> {
    Ok(parts
        .headers
        .typed_get::<Authorization<Bearer>>()
        .ok_or(AppError::Auth(AuthError::MissingCredentials))?
        .token()
        .to_string())
}
