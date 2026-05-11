use axum::{Extension, Json, Router, extract::State, http::StatusCode, routing::get};

use crate::{AppState, error::Result, model, service};

pub fn router() -> Router<AppState> {
    Router::new().route("/me", get(get_me).delete(delete_me))
}

#[utoipa::path(
    get,
    path = "/me",
    security(("bearerAuth" = [])),
    responses((status = 200, description = "Current user profile", body = model::user::Body<model::user::Read>))
)]
async fn get_me(
    State(mut service): State<service::User>,
    Extension(user_auth): Extension<model::auth::Auth>,
) -> Result<Json<model::user::Body<model::user::Read>>> {
    let user = service.get_by_id(user_auth.id).await?;
    Ok(Json(user.into()))
}

#[utoipa::path(
    delete,
    path = "/me",
    security(("bearerAuth" = [])),
    responses((status = 204, description = "Deleted current user"))
)]
async fn delete_me(
    State(mut service): State<service::User>,
    Extension(user_auth): Extension<model::auth::Auth>,
) -> Result<StatusCode> {
    service.delete_by_id(user_auth.id).await?;
    Ok(StatusCode::NO_CONTENT)
}
