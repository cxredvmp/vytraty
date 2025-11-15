use std::env;

use axum::extract::FromRef;
use migration::{Migrator, MigratorTrait};
use sea_orm::Database;

use crate::config::Config;

mod config;
mod error;
mod models;
mod repositories;
mod routes;
mod services;

#[derive(Clone, FromRef)]
struct AppState {
    health_service: services::health::Service,
    user_service: services::user::Service,
    category_service: services::category::Service,
    record_service: services::record::Service,
}

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = Config::from_env();

    let db = Database::connect(config.db_url())
        .await
        .expect("failed to connect to database");
    eprintln!("database connection established");

    Migrator::up(&db, None)
        .await
        .expect("failed to apply pending migrations");
    eprintln!("pending migrations applied");

    let user_repo = repositories::user::Repository::new(db.clone());
    let category_repo = repositories::category::Repository::new(db.clone());
    let record_repo = repositories::record::Repository::new(db.clone());

    let health_service = services::health::Service::new(db.clone());
    let user_service = services::user::Service::new(user_repo.clone());
    let category_service = services::category::Service::new(category_repo);
    let record_service = services::record::Service::new(record_repo, user_repo);

    let state = AppState {
        health_service,
        user_service,
        category_service,
        record_service,
    };
    let router = routes::router().with_state(state);

    let port = env::var("PORT").expect("PORT must be set");
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    eprintln!("listening on http://localhost:{port}");

    axum::serve(listener, router).await.unwrap();
}
