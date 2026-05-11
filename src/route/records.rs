use axum::{
    Extension, Json, Router,
    extract::{Path, Query, State},
    http::StatusCode,
    routing::{get, post},
};
use uuid::Uuid;

use crate::{AppState, error::Result, model, service};

pub fn router() -> Router<AppState> {
    Router::new()
        .route("/", post(create).get(get_all))
        .route("/{id}", get(get_by_id).delete(delete_by_id))
}

#[utoipa::path(
    operation_id = "records_create",
    post,
    path = "/records",
    security(("bearerAuth" = [])),
    request_body = model::record::CreateRequest,
    responses((status = 201, description = "Created record", body = model::record::Body<model::record::Read>))
)]
async fn create(
    State(mut service): State<service::Record>,
    Extension(user_auth): Extension<model::auth::Auth>,
    Json(record): Json<model::record::CreateRequest>,
) -> Result<(StatusCode, Json<model::record::Body<model::record::Read>>)> {
    record.validate()?;
    let record = service.create_for(record, user_auth.id).await?;
    Ok((StatusCode::CREATED, Json(model::record::Body { record })))
}

async fn get_by_id(
    State(mut service): State<service::Record>,
    Extension(user_auth): Extension<model::auth::Auth>,
    Path(id): Path<Uuid>,
) -> Result<Json<model::record::Body<model::record::Read>>> {
    let record = service.get_by_id_for(id, user_auth.id).await?;
    Ok(Json(model::record::Body { record }))
}

#[utoipa::path(
    operation_id = "records_get_all",
    get,
    path = "/records",
    security(("bearerAuth" = [])),
    responses((status = 200, description = "Listed records", body = model::record::BodyArray<model::record::Read>))
)]
async fn get_all(
    State(mut service): State<service::Record>,
    Extension(user_auth): Extension<model::auth::Auth>,
    Query(filters): Query<model::record::Filters>,
) -> Result<Json<model::record::BodyArray<model::record::Read>>> {
    filters.validate()?;
    let records = service.get_all_for(filters, user_auth.id).await?;
    Ok(Json(model::record::BodyArray { records }))
}

#[utoipa::path(
    operation_id = "records_delete_by_id",
    delete,
    path = "/records",
    security(("bearerAuth" = [])),
    responses((status = 204, description = "Deleted record"))
)]
async fn delete_by_id(
    State(mut service): State<service::Record>,
    Extension(user_auth): Extension<model::auth::Auth>,
    Path(record_id): Path<Uuid>,
) -> Result<StatusCode> {
    service.delete_by_id_for(record_id, user_auth.id).await?;
    Ok(StatusCode::NO_CONTENT)
}
