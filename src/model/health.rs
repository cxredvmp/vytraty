use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use jiff::Timestamp;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct Health {
    pub status: Status,
    pub observed_at: Timestamp,
    pub services: Services,
}

impl IntoResponse for Health {
    fn into_response(self) -> Response {
        let status = match self.status {
            Status::Up => StatusCode::OK,
            Status::Down => StatusCode::SERVICE_UNAVAILABLE,
        };
        let body = Json(self);
        (status, body).into_response()
    }
}

#[derive(Serialize, Deserialize, PartialEq, Eq, Clone, Copy)]
#[serde(rename_all = "lowercase")]
pub enum Status {
    Up,
    Down,
}

#[derive(Serialize, Deserialize)]
pub struct Services {
    pub db: Status,
}

impl Services {
    pub fn health(&self) -> Status {
        let services = [self.db];
        if services.into_iter().all(|s| s == Status::Up) {
            Status::Up
        } else {
            Status::Down
        }
    }
}
