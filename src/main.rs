use vytraty::{AppState, config::Config, db, route};

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = Config::from_env();
    let db = db(config.db_url()).await;
    let state = AppState::new(config.clone(), db);
    let router = route::router(state.clone()).with_state(state);

    let addr = format!("0.0.0.0:{}", config.port());
    let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
    eprintln!("listening on http://localhost:{}", config.port());
    axum::serve(listener, router).await.unwrap();
}
