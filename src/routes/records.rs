use axum::{
    Extension, Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
};
use uuid::Uuid;

use crate::{
    AppState,
    error::AppError,
    models::{auth as auth_model, record::*},
    services::record::Service,
};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_record).get(filter_records))
        .route("/{id}", get(get_record).delete(delete_user_record))
}

async fn create_record(
    State(service): State<Service>,
    Json(RecordBody { record }): Json<RecordBody<RecordCreate>>,
) -> Result<(StatusCode, Json<RecordBody<RecordRead>>), AppError> {
    record.validate()?;
    let record = service.create(record).await?;
    Ok((StatusCode::CREATED, Json(RecordBody { record })))
}

async fn get_record(
    State(service): State<Service>,
    Path(id): Path<Uuid>,
) -> Result<Json<RecordBody<RecordRead>>, AppError> {
    let record = service.get_by_id(id).await?;
    Ok(Json(RecordBody { record }))
}

async fn filter_records(
    State(service): State<Service>,
    Query(params): Query<RecordFilterParams>,
) -> Result<Json<RecordsBody<RecordRead>>, AppError> {
    params.validate()?;
    let records = service.filter_by_params(params).await?;
    Ok(Json(RecordsBody { records }))
}

async fn delete_user_record(
    State(service): State<Service>,
    Path(record_id): Path<Uuid>,
    Extension(user_auth): Extension<auth_model::UserAuth>,
) -> Result<StatusCode, AppError> {
    service.delete_owned(record_id, user_auth.id).await?;
    Ok(StatusCode::NO_CONTENT)
}
