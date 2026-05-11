pub mod config;
pub mod error;
pub mod middleware;
pub mod model;
pub mod repository;
pub mod route;
pub mod service;
pub mod utils;

use axum::extract::FromRef;
use toasty::Db;

pub use crate::config::Config;

pub async fn db(db_url: &str) -> Db {
    let db = toasty::Db::builder()
        .models(model::models())
        .connect(db_url)
        .await
        .expect("failed to connect to database");
    eprintln!("database connection established");

    db.push_schema().await.expect("failed to push schema");
    eprintln!("schema pushed");

    db
}

#[derive(Clone, FromRef)]
pub struct AppState {
    pub health_service: service::Health,
    pub user_service: service::User,
    pub category_service: service::Category,
    pub record_service: service::Record,
    pub auth_service: service::Auth,
    pub config: Config,
}

impl AppState {
    pub fn new(config: Config, db: Db) -> Self {
        let user_repo = repository::User::new(db.clone());
        let category_repo = repository::Category::new(db.clone());
        let record_repo = repository::Record::new(db.clone());

        let health_service = service::Health::new(db.clone());
        let user_service = service::User::new(user_repo.clone());
        let category_service = service::Category::new(category_repo.clone());
        let record_service = service::Record::new(record_repo.clone(), user_repo.clone());
        let auth_service = service::Auth::new(user_repo.clone());

        AppState {
            health_service,
            user_service,
            category_service,
            record_service,
            auth_service,
            config,
        }
    }
}
