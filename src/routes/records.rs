use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
};
use uuid::Uuid;

use crate::{AppState, error::AppError, models::record::*, services::record::Service};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_record).get(filter_records))
        .route("/{id}", get(get_record).delete(delete_record))
}

async fn create_record(
    State(service): State<Service>,
    Json(RecordBody { record }): Json<RecordBody<RecordCreate>>,
) -> Result<(StatusCode, Json<RecordBody<Record>>), AppError> {
    record.validate()?;
    let record = service.create_record(record).await?;
    Ok((StatusCode::CREATED, Json(RecordBody { record })))
}

async fn get_record(
    State(service): State<Service>,
    Path(id): Path<Uuid>,
) -> Result<Json<RecordBody<Record>>, AppError> {
    let record = service.get_record(id).await?;
    Ok(Json(RecordBody { record }))
}

async fn filter_records(
    State(service): State<Service>,
    Query(params): Query<RecordFilterParams>,
) -> Result<Json<RecordsBody<Record>>, AppError> {
    params.validate()?;
    let records = service.filter_records(params).await?;
    Ok(Json(RecordsBody { records }))
}

async fn delete_record(
    State(service): State<Service>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    service.delete_record(id).await?;
    Ok(StatusCode::NO_CONTENT)
}
