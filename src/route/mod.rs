mod auth;
mod categories;
mod doc;
mod health;
mod me;
mod records;

use axum::{Router, http::StatusCode, middleware, routing::get};

use crate::{AppState, model};

pub fn router(state: AppState) -> Router<AppState> {
    let health = health::router();
    let me = me::router();
    let categories = categories::router();
    let records = records::router();
    let auth = auth::router();
    let doc = doc::router();

    let public = Router::new()
        .route("/", get(get_root))
        .merge(auth)
        .merge(health)
        .merge(doc)
        .with_state(state.clone());

    let protected = Router::new()
        .merge(me)
        .nest("/categories", categories)
        .nest("/records", records)
        .layer(middleware::from_extractor_with_state::<
            model::auth::Auth,
            AppState,
        >(state.clone()));

    Router::new()
        .merge(public)
        .merge(protected)
        .fallback(|| async { StatusCode::NOT_FOUND })
}

async fn get_root() -> &'static str {
    "Welcome to Vytraty - an expense tracker web application."
}
