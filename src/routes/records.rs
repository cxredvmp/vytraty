use axum::{
    Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
};
use uuid::Uuid;

use crate::{AppState, error::AppError, models::record::*, services};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create_record).get(filter_records))
        .route("/{id}", get(get_record).delete(delete_record))
}

async fn create_record(
    State(AppState { db }): State<AppState>,
    Json(RecordBody { record }): Json<RecordBody<RecordCreate>>,
) -> Result<(StatusCode, Json<RecordBody<Record>>), AppError> {
    record.validate()?;
    let record = services::record::create_record(&db, record).await?;
    Ok((StatusCode::CREATED, Json(RecordBody { record })))
}

async fn get_record(
    State(AppState { db }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<Json<RecordBody<Record>>, AppError> {
    let record = services::record::get_record(&db, id).await?;
    Ok(Json(RecordBody { record }))
}

async fn filter_records(
    State(AppState { db }): State<AppState>,
    Query(params): Query<RecordFilterParams>,
) -> Result<Json<RecordsBody<Record>>, AppError> {
    params.validate()?;
    let records = services::record::filter_records(&db, params).await?;
    Ok(Json(RecordsBody { records }))
}

async fn delete_record(
    State(AppState { db }): State<AppState>,
    Path(id): Path<Uuid>,
) -> Result<StatusCode, AppError> {
    services::record::delete_record(&db, id).await?;
    Ok(StatusCode::NO_CONTENT)
}
