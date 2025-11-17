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
        .route("/register", post(register_user))
        .route("/login", post(login_user))
}

async fn register_user(
    State(service): State<Service>,
    Json(user): Json<UserRegister>,
) -> Result<(StatusCode, Json<user_model::UserBody<user_model::UserRead>>), AppError> {
    user.validate()?;
    Ok((
        StatusCode::CREATED,
        Json(service.register_user(user).await?.into()),
    ))
}

async fn login_user(
    State(state): State<AppState>,
    Json(creds): Json<UserLogin>,
) -> Result<Json<Token>, AppError> {
    creds.validate()?;
    let user = state.auth_service.login_user(creds).await?;
    let token = jwt::sign(user.id, state.config.jwt_secret())?;
    Ok(Json(Token {
        token,
        schema: "Bearer".to_string(),
    }))
}
