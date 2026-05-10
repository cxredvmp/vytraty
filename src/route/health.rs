use axum::{Router, extract::State, routing::get};
use jiff::Timestamp;

use crate::{AppState, model, service};

pub fn router() -> Router<AppState> {
    Router::new().route("/", get(check))
}

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
