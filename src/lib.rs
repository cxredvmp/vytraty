pub mod config;
pub mod error;
pub mod middleware;
pub mod model;
pub mod repository;
pub mod route;
pub mod service;
pub mod utils;

use axum::extract::FromRef;

use crate::config::Config;

#[derive(Clone, FromRef)]
pub struct AppState {
    pub health_service: service::Health,
    pub user_service: service::User,
    pub category_service: service::Category,
    pub record_service: service::Record,
    pub auth_service: service::Auth,
    pub config: Config,
}
