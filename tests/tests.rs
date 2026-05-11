use std::net::SocketAddr;

use reqwest::Client;
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{ContainerAsync, runners::AsyncRunner},
};
use toasty::Db;

use vytraty::{AppState, Config, db, route};

struct Ctx {
    _db_node: ContainerAsync<Postgres>,
    _db: Db,
    client: Client,
    url: String,
}

impl Ctx {
    async fn new() -> Self {
        let db_node = Postgres::default().start().await.unwrap();

        let host = db_node.get_host().await.unwrap();
        let host_port = db_node.get_host_port_ipv4(5432).await.unwrap();

        let db_url = format!("postgres://postgres:postgres@{host}:{host_port}/postgres",);
        let db = db::db(&db_url).await;

        let config = Config::new(
            db_url.clone(),
            "0".to_string(),
            "test_jwt_secret".to_string(),
        );
        let state = AppState::new(config.clone(), db.clone());
        let router = route::router(state.clone()).with_state(state);

        let addr = format!("0.0.0.0:{}", config.port());
        let listener = tokio::net::TcpListener::bind(addr).await.unwrap();
        let addr: SocketAddr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            axum::serve(listener, router).await.unwrap();
        });

        Self {
            _db_node: db_node,
            _db: db,
            client: Client::new(),
            url: format!("http://{}", addr),
        }
    }
}

#[tokio::test]
async fn test_ctx() {
    let _ctx = Ctx::new().await;
}

#[tokio::test]
async fn test_health() {
    use vytraty::model::health::{Health, Status};

    let ctx = Ctx::new().await;

    let response = ctx
        .client
        .get(format!("{}/health", ctx.url))
        .send()
        .await
        .unwrap();

    assert!(response.status().is_success());

    let health = response.json::<Health>().await.unwrap();

    assert!(matches!(health.status, Status::Up));
}
