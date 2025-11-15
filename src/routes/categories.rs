use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use uuid::Uuid;

use crate::{AppState, error::AppError, models::category::*, services};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_category).get(get_categories))
        .route("/{id}", get(get_category).delete(delete_category))
}

async fn create_category(
    State(AppState { db }): State<AppState>,
    Json(CategoryBody { category }): Json<CategoryBody<CategoryCreate>>,
) -> Result<(StatusCode, Json<CategoryBody<Category>>), AppError> {
    category.validate()?;
    let category = services::category::create_category(&db, category).await?;
    Ok((StatusCode::CREATED, Json(CategoryBody { category })))
}

async fn get_category(
    State(AppState { db }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<CategoryBody<Category>>, AppError> {
    let category = services::category::get_category(&db, id).await?;
    Ok(Json(CategoryBody { category }))
}

async fn get_categories(
    State(AppState { db }): State<AppState>,
) -> Result<Json<CategoriesBody<Category>>, AppError> {
    let categories = services::category::get_categories(&db).await?;
    Ok(Json(CategoriesBody { categories }))
}

async fn delete_category(
    State(AppState { db }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    services::category::delete_category(&db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
