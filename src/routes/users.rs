use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use uuid::Uuid;

use crate::{AppState, error::AppError, models::user::*, services};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_user).get(get_users))
        .route("/{id}", get(get_user).delete(delete_user))
}

async fn create_user(
    State(AppState { db }): State<AppState>,
    Json(UserBody { user }): Json<UserBody<UserCreate>>,
) -> Result<(StatusCode, Json<UserBody<User>>), AppError> {
    user.validate()?;
    let user = services::user::create_user(&db, user).await?;
    Ok((StatusCode::CREATED, Json(UserBody { user })))
}

async fn get_user(
    State(AppState { db }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<UserBody<User>>, AppError> {
    let user = services::user::get_user(&db, id).await?;
    Ok(Json(UserBody { user }))
}

async fn get_users(
    State(AppState { db }): State<AppState>,
) -> Result<Json<UsersBody<User>>, AppError> {
    let users = services::user::get_users(&db).await?;
    Ok(Json(UsersBody { users }))
}

async fn delete_user(
    State(AppState { db }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    services::user::delete_user(&db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
