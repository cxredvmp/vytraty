use axum::{
    Extension, Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
};
use uuid::Uuid;

use crate::{
    AppState,
    errors::AppError,
    models::{auth as auth_model, record::*},
    services::record::Service,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create).get(find_by_filters))
        .route("/{id}", get(get_by_id).delete(delete_owned))
}

async fn create(
    State(service): State<Service>,
    Json(RecordBody { record }): Json<RecordBody<RecordCreate>>,
) -> Result<(StatusCode, Json<RecordBody<RecordRead>>), AppError> {
    record.validate()?;
    let record = service.create(record).await?;
    Ok((StatusCode::CREATED, Json(RecordBody { record })))
}

async fn get_by_id(
    State(service): State<Service>,
    Path(id): Path<Uuid>,
) -> Result<Json<RecordBody<RecordRead>>, AppError> {
    let record = service.get_by_id(id).await?;
    Ok(Json(RecordBody { record }))
}

async fn find_by_filters(
    State(service): State<Service>,
    Query(filters): Query<RecordFilters>,
) -> Result<Json<RecordsBody<RecordRead>>, AppError> {
    filters.validate()?;
    let records = service.find_by_filters(filters).await?;
    Ok(Json(RecordsBody { records }))
}

async fn delete_owned(
    State(service): State<Service>,
    Path(record_id): Path<Uuid>,
    Extension(user_auth): Extension<auth_model::UserAuth>,
) -> Result<StatusCode, AppError> {
    service.delete_owned(record_id, user_auth.id).await?;
    Ok(StatusCode::NO_CONTENT)
}
