use axum::{Json, Router, extract::State, http::StatusCode, routing::post};

use crate::{AppState, error::Result, model, service, utils::jwt};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/signup", post(sign_up))
        .route("/signin", post(sign_in))
}

async fn sign_up(
    State(mut service): State<service::Auth>,
    Json(user): Json<model::auth::SignUp>,
) -> Result<(StatusCode, Json<model::user::Body<model::user::Read>>)> {
    user.validate()?;
    Ok((
        StatusCode::CREATED,
        Json(service.sign_up(user).await?.into()),
    ))
}

async fn sign_in(
    State(mut state): State<AppState>,
    Json(creds): Json<model::auth::SignIn>,
) -> Result<Json<model::auth::Token>> {
    creds.validate()?;
    let user = state.auth_service.sign_in(creds).await?;
    let token = jwt::sign(user.id, state.config.jwt_secret())?;
    Ok(Json(model::auth::Token {
        token,
        schema: "Bearer".to_string(),
    }))
}
