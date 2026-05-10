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

async fn create(
    State(mut service): State<service::Category>,
    Extension(user_auth): Extension<model::auth::Auth>,
    Json(model::category::Body { category }): Json<
        model::category::Body<model::category::CreateRequest>,
    >,
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

async fn get_all(
    State(mut service): State<service::Category>,
    Extension(user_auth): Extension<model::auth::Auth>,
) -> Result<Json<model::category::BodyArray<model::category::Read>>> {
    Ok(Json(service.get_all_for(user_auth.id).await?.into()))
}

async fn delete_by_id(
    State(mut service): State<service::Category>,
    Path(id): Path<Uuid>,
    Extension(user_auth): Extension<model::auth::Auth>,
) -> Result<StatusCode> {
    service.delete_by_id_for(id, user_auth.id).await?;
    Ok(StatusCode::NO_CONTENT)
}
