use axum::{
    Extension, Json, Router,
    extract::{Path, State},
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
    operation_id = "categories_create",
    post,
    path = "/categories",
    security(("bearerAuth" = [])),
    request_body = model::category::CreateRequest,
    responses((status = 201, description = "Created category", body = model::category::Body<model::category::Read>))
)]
async fn create(
    State(mut service): State<service::Category>,
    Extension(user_auth): Extension<model::auth::Auth>,
    Json(category): Json<model::category::CreateRequest>,
) -> Result<(
    StatusCode,
    Json<model::category::Body<model::category::Read>>,
)> {
    category.validate()?;
    let category = service.create_for(category, user_auth.id).await?;
    Ok((
        StatusCode::CREATED,
        Json(model::category::Body { category }),
    ))
}

async fn get_by_id(
    State(mut service): State<service::Category>,
    Extension(user_auth): Extension<model::auth::Auth>,
    Path(id): Path<Uuid>,
) -> Result<Json<model::category::Body<model::category::Read>>> {
    Ok(Json(service.get_by_id_for(id, user_auth.id).await?.into()))
}

#[utoipa::path(
    operation_id = "categories_get_all",
    get,
    path = "/categories",
    security(("bearerAuth" = [])),
    responses((status = 200, description = "Listed available categories", body = model::category::BodyArray<model::category::Read>))
)]
async fn get_all(
    State(mut service): State<service::Category>,
    Extension(user_auth): Extension<model::auth::Auth>,
) -> Result<Json<model::category::BodyArray<model::category::Read>>> {
    Ok(Json(service.get_all_for(user_auth.id).await?.into()))
}

#[utoipa::path(
    operation_id = "categories_delete_by_id",
    delete,
    path = "/categories",
    security(("bearerAuth" = [])),
    responses((status = 204, description = "Deleted category"))
)]
async fn delete_by_id(
    State(mut service): State<service::Category>,
    Path(id): Path<Uuid>,
    Extension(user_auth): Extension<model::auth::Auth>,
) -> Result<StatusCode> {
    service.delete_by_id_for(id, user_auth.id).await?;
    Ok(StatusCode::NO_CONTENT)
}
