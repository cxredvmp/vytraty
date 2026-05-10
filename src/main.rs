use std::env;

use vytraty::{AppState, config::Config, model, repository, route, service};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = Config::from_env();

    let db = toasty::Db::builder()
        .models(model::models())
        .connect(config.db_url())
        .await
        .expect("failed to connect to database");
    eprintln!("database connection established");

    db.push_schema().await.expect("failed to push schema");
    eprintln!("schema pushed");

    let user_repo = repository::User::new(db.clone());
    let category_repo = repository::Category::new(db.clone());
    let record_repo = repository::Record::new(db.clone());

    let health_service = service::Health::new(db.clone());
    let user_service = service::User::new(user_repo.clone());
    let category_service = service::Category::new(category_repo.clone());
    let record_service = service::Record::new(record_repo.clone(), user_repo.clone());
    let auth_service = service::Auth::new(user_repo.clone());

    let state = AppState {
        health_service,
        user_service,
        category_service,
        record_service,
        auth_service,
        config,
    };
    let router = route::router(state.clone()).with_state(state);

    let port = env::var("PORT").expect("PORT must be set");
    let addr = format!("0.0.0.0:{port}");
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    eprintln!("listening on http://localhost:{port}");

    axum::serve(listener, router).await.unwrap();
}
