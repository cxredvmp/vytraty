use axum::{Router, extract::State, routing::get};
use chrono::Utc;

use crate::{AppState, models::health::*, services::health::Service};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(get_health))
}

async fn get_health(State(service): State<Service>) -> Health {
    let services = Services {
        db: service.check_db().await,
    };
    Health {
        status: services.health(),
        observed_at: Utc::now(),
        services,
    }
}
