use axum::{Router, extract::State, routing::get};
use jiff::Timestamp;

use crate::{AppState, model, service};

pub fn router() -> Router<AppState> {
    Router::new().route("/health", get(check))
}

#[utoipa::path(
    get,
    path = "/health",
    responses(
        (status = 200, description = "Health check successful", body = model::health::Health),
        (status = 503, description = "One or more services are down", body = model::health::Health)
    )
)]
async fn check(State(mut service): State<service::Health>) -> model::health::Health {
    let services = model::health::Services {
        db: service.check_db().await,
    };
    model::health::Health {
        status: services.health(),
        observed_at: Timestamp::now(),
        services,
    }
}
