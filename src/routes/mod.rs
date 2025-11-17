use axum::{Router, middleware, routing::get};

use crate::{AppState, models::auth as auth_model};

mod auth;
mod categories;
mod health;
mod records;
mod users;

pub fn router(state: AppState) -> Router<AppState> {
    let health = health::router();
    let users = users::router();
    let categories = categories::router();
    let records = records::router();
    let auth = auth::router();

    let public = Router::new()
        .route("/", get(get_root))
        .nest("/auth", auth)
        .nest("/health", health);

    let protected = Router::new()
        .nest("/users", users)
        .nest("/categories", categories)
        .nest("/records", records)
        .layer(middleware::from_extractor_with_state::<
            auth_model::UserAuth,
            AppState,
        >(state.clone()));

    Router::new().merge(public).merge(protected)
}

async fn get_root() -> &'static str {
    "Welcome to the expense tracker application."
}
