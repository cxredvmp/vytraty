use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use uuid::Uuid;

use crate::{AppState, error::AppError, models::user::*, services::user::Service};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_user).get(get_users))
        .route("/{id}", get(get_user).delete(delete_user))
}

async fn create_user(
    State(service): State<Service>,
    Json(UserBody { user }): Json<UserBody<UserCreate>>,
) -> Result<(StatusCode, Json<UserBody<UserRead>>), AppError> {
    user.validate()?;
    let user = service.create_user(user).await?;
    Ok((StatusCode::CREATED, Json(UserBody { user })))
}

async fn get_user(
    State(service): State<Service>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserBody<UserRead>>, AppError> {
    let user = service.get_user(id).await?;
    Ok(Json(UserBody { user }))
}

async fn get_users(State(service): State<Service>) -> Result<Json<UsersBody<UserRead>>, AppError> {
    let users = service.get_users().await?;
    Ok(Json(UsersBody { users }))
}

async fn delete_user(
    State(service): State<Service>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    service.delete_user(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
