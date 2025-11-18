use axum::{Json, Router, extract::State, http::StatusCode, routing::post};

use crate::{
    AppState,
    error::AppError,
    models::{auth::*, user as user_model},
    services::auth::Service,
    utils::jwt,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/signup", post(sign_up))
        .route("/signin", post(sign_in))
}

async fn sign_up(
    State(service): State<Service>,
    Json(user): Json<UserSignUp>,
) -> Result<(StatusCode, Json<user_model::UserBody<user_model::UserRead>>), AppError> {
    user.validate()?;
    Ok((
        StatusCode::CREATED,
        Json(service.sign_up(user).await?.into()),
    ))
}

async fn sign_in(
    State(state): State<AppState>,
    Json(creds): Json<UserSignIn>,
) -> Result<Json<Token>, AppError> {
    creds.validate()?;
    let user = state.auth_service.sign_in(creds).await?;
    let token = jwt::sign(user.id, state.config.jwt_secret())?;
    Ok(Json(Token {
        token,
        schema: "Bearer".to_string(),
    }))
}
