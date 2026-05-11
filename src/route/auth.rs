use axum::{Json, Router, extract::State, http::StatusCode, routing::post};

use crate::{AppState, error::Result, model, service, utils::jwt};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/signup", post(signup))
        .route("/signin", post(signin))
}

#[utoipa::path(
    post,
    path = "/signup",
    request_body = model::auth::SignUp,
    responses(
        (status = 201, description = "Signed up", body = model::user::Body<model::user::Read>)
    )
)]
async fn signup(
    State(mut service): State<service::Auth>,
    Json(user): Json<model::auth::SignUp>,
) -> Result<(StatusCode, Json<model::user::Body<model::user::Read>>)> {
    user.validate()?;
    Ok((
        StatusCode::CREATED,
        Json(service.sign_up(user).await?.into()),
    ))
}

#[utoipa::path(
    post,
    path = "/signin",
    request_body = model::auth::SignIn,
    responses(
        (status = 200, description = "Signed in", body = model::auth::Token)
    )
)]
async fn signin(
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
