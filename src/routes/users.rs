use axum::{
    Extension, Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{delete, get},
};
use uuid::Uuid;

use crate::{
    AppState,
    errors::AppError,
    models::{auth as auth_model, user::*},
    services::user::Service,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", get(get_all))
        .route("/{id}", get(get_by_id))
        .route("/me", delete(delete_me))
}

async fn get_by_id(
    State(service): State<Service>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserBody<UserRead>>, AppError> {
    let user = service.get_by_id(id).await?;
    Ok(Json(user.into()))
}

async fn get_all(State(service): State<Service>) -> Result<Json<UsersBody<UserRead>>, AppError> {
    let users = service.get_all().await?;
    Ok(Json(users.into()))
}

async fn delete_by_id(
    State(service): State<Service>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    service.delete_by_id(id).await?;
    Ok(StatusCode::NO_CONTENT)
}

async fn delete_me(
    State(service): State<Service>,
    Extension(user_auth): Extension<auth_model::UserAuth>,
) -> Result<StatusCode, AppError> {
    service.delete_by_id(user_auth.id).await?;
    Ok(StatusCode::NO_CONTENT)
}
