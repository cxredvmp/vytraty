use axum::{
    Json, Router,
    extract::{Path, State},
    http::StatusCode,
    routing::{get, post},
};
use uuid::Uuid;

use crate::{AppState, error::AppError, models::category::*, services::category::Service};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_category).get(get_categories))
        .route("/{id}", get(get_category).delete(delete_category))
}

async fn create_category(
    State(service): State<Service>,
    Json(CategoryBody { category }): Json<CategoryBody<CategoryCreate>>,
) -> Result<(StatusCode, Json<CategoryBody<CategoryRead>>), AppError> {
    category.validate()?;
    let category = service.create(category).await?;
    Ok((StatusCode::CREATED, Json(CategoryBody { category })))
}

async fn get_category(
    State(service): State<Service>,
    Path(id): Path<Uuid>,
) -> Result<Json<CategoryBody<CategoryRead>>, AppError> {
    let category = service.get_by_id(id).await?;
    Ok(Json(CategoryBody { category }))
}

async fn get_categories(
    State(service): State<Service>,
) -> Result<Json<CategoriesBody<CategoryRead>>, AppError> {
    let categories = service.get_all().await?;
    Ok(Json(CategoriesBody { categories }))
}

async fn delete_category(
    State(service): State<Service>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    service.delete_by_id(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
