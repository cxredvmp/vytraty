use std::net::SocketAddr;

use reqwest::Client;
use testcontainers_modules::{
    postgres::Postgres,
    testcontainers::{ContainerAsync, runners::AsyncRunner},
};
use toasty::Db;

use uuid::Uuid;
use vytraty::{AppState, Config, db, model, route};

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
        let mut db = db::connect_db(&db_url).await;
        db::setup_db(&mut db).await;

        let config = Config::new(
            db_url.clone(),
            "false".to_string(),
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
#[ignore]
async fn end_to_end() {
    let mut ctx = text_ctx().await;

    test_health(&mut ctx).await;

    let (signin, token) = test_auth(&mut ctx).await;

    let category_id = test_create_category(&mut ctx, &token).await;

    let record_id = test_create_record(&mut ctx, &token, category_id.clone()).await;

    test_get_categories(&mut ctx, &token, category_id).await;
    test_get_records(&mut ctx, &token, record_id).await;

    test_delete_record(&mut ctx, &token, record_id).await;
    test_get_record_not_found(&mut ctx, &token, record_id).await;

    test_delete_category(&mut ctx, &token, category_id).await;
    test_get_category_not_found(&mut ctx, &token, category_id).await;

    test_delete_me(&mut ctx, &token).await;
    test_signin_deleted(&mut ctx, &signin).await;
}

async fn text_ctx() -> Ctx {
    Ctx::new().await
}

async fn test_health(ctx: &mut Ctx) {
    let response = ctx
        .client
        .get(format!("{}/health", ctx.url))
        .send()
        .await
        .unwrap();

    if let status = response.status()
        && !status.is_success()
    {
        let body = response.text().await.unwrap();
        panic!("failed to check health: status: {}, body: {}", status, body)
    }

    let health = response.json::<model::health::Health>().await.unwrap();

    assert!(matches!(health.status, model::health::Status::Up));
}

async fn test_auth(ctx: &mut Ctx) -> (model::auth::SignIn, model::auth::Token) {
    let name = "test_user".to_string();
    let password = "test_password".to_string();
    let default_currency_code = "UAH".to_string();

    let signup = model::auth::SignUp {
        name: name.clone(),
        default_currency_code: default_currency_code.clone(),
        password: password.clone(),
    };

    let response = ctx
        .client
        .post(format!("{}/signup", ctx.url))
        .json(&signup)
        .send()
        .await
        .unwrap();

    if let status = response.status()
        && !status.is_success()
    {
        let body = response.text().await.unwrap();
        panic!("failed to sign up: status: {}, body: {}", status, body)
    }

    let model::user::Body { user } = response
        .json::<model::user::Body<model::user::Read>>()
        .await
        .unwrap();

    assert_eq!(user.name, name);
    assert_eq!(user.default_currency_code, default_currency_code);

    let signin = model::auth::SignIn {
        name: name.clone(),
        password: password.clone(),
    };

    let response = ctx
        .client
        .post(format!("{}/signin", ctx.url))
        .json(&signin)
        .send()
        .await
        .unwrap();

    if let status = response.status()
        && !status.is_success()
    {
        let body = response.text().await.unwrap();
        panic!("failed to sign in: status: {}, body: {}", status, body)
    }

    let token = response.json::<model::auth::Token>().await.unwrap();
    (signin, token)
}

async fn test_create_category(ctx: &mut Ctx, token: &model::auth::Token) -> Uuid {
    let create = model::category::CreateRequest {
        name: "Video games".to_string(),
    };

    let response = ctx
        .client
        .post(format!("{}/categories", ctx.url))
        .bearer_auth(&token.token)
        .json(&create)
        .send()
        .await
        .unwrap();

    if let status = response.status()
        && !status.is_success()
    {
        let body = response.text().await.unwrap();
        panic!(
            "failed to create category: status: {}, body: {}",
            status, body
        )
    }

    let model::category::Body { category } = response
        .json::<model::category::Body<model::category::Read>>()
        .await
        .unwrap();

    assert_eq!(category.name, create.name);

    category.id
}

async fn test_create_record(ctx: &mut Ctx, token: &model::auth::Token, category_id: Uuid) -> Uuid {
    let create = model::record::CreateRequest {
        category_id,
        sum: 10.into(),
        currency_code: Some("USD".to_string()),
    };

    let response = ctx
        .client
        .post(format!("{}/records", ctx.url))
        .bearer_auth(&token.token)
        .json(&create)
        .send()
        .await
        .unwrap();

    if let status = response.status()
        && !status.is_success()
    {
        let body = response.text().await.unwrap();
        panic!(
            "failed to create record: status: {}, body: {}",
            status, body
        )
    }

    let model::record::Body { record } = response
        .json::<model::record::Body<model::record::Read>>()
        .await
        .unwrap();

    assert_eq!(record.category_id, create.category_id);
    assert_eq!(record.sum, create.sum);
    assert_eq!(record.currency_code, create.currency_code.unwrap());

    record.id
}

async fn test_get_categories(ctx: &mut Ctx, token: &model::auth::Token, category_id: Uuid) {
    let response = ctx
        .client
        .get(format!("{}/categories", ctx.url))
        .bearer_auth(&token.token)
        .send()
        .await
        .unwrap();

    if let status = response.status()
        && !status.is_success()
    {
        let body = response.text().await.unwrap();
        panic!(
            "failed to get categories: status: {}, body: {}",
            status, body
        )
    }

    let model::category::BodyArray { categories } = response
        .json::<model::category::BodyArray<model::category::Read>>()
        .await
        .unwrap();

    assert!(
        categories.iter().any(|c| c.id == category_id),
        "expected created category {} to be in the list of categories",
        category_id
    );
}

async fn test_get_records(ctx: &mut Ctx, token: &model::auth::Token, record_id: Uuid) {
    let response = ctx
        .client
        .get(format!("{}/records", ctx.url))
        .bearer_auth(&token.token)
        .send()
        .await
        .unwrap();

    if let status = response.status()
        && !status.is_success()
    {
        let body = response.text().await.unwrap();
        panic!("failed to get records: status: {}, body: {}", status, body)
    }

    let model::record::BodyArray { records } = response
        .json::<model::record::BodyArray<model::record::Read>>()
        .await
        .unwrap();

    assert!(
        records.iter().any(|r| r.id == record_id),
        "expected created record {} to be in the list of records",
        record_id
    );
}

async fn test_delete_record(ctx: &mut Ctx, token: &model::auth::Token, record_id: Uuid) {
    let response = ctx
        .client
        .delete(format!("{}/records/{}", ctx.url, record_id))
        .bearer_auth(&token.token)
        .send()
        .await
        .unwrap();

    if let status = response.status()
        && !status.is_success()
    {
        let body = response.text().await.unwrap();
        panic!(
            "failed to delete record: status: {}, body: {}",
            status, body
        )
    }
}

async fn test_get_record_not_found(ctx: &mut Ctx, token: &model::auth::Token, record_id: Uuid) {
    let response = ctx
        .client
        .get(format!("{}/records/{}", ctx.url, record_id))
        .bearer_auth(&token.token)
        .send()
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        reqwest::StatusCode::NOT_FOUND,
        "expected NOT FOUND for deleted record, got: {}",
        response.status()
    );
}

async fn test_delete_category(ctx: &mut Ctx, token: &model::auth::Token, category_id: Uuid) {
    let response = ctx
        .client
        .delete(format!("{}/categories/{}", ctx.url, category_id))
        .bearer_auth(&token.token)
        .send()
        .await
        .unwrap();

    if let status = response.status()
        && !status.is_success()
    {
        let body = response.text().await.unwrap();
        panic!(
            "failed to delete category: status: {}, body: {}",
            status, body
        )
    }
}

async fn test_get_category_not_found(ctx: &mut Ctx, token: &model::auth::Token, category_id: Uuid) {
    let response = ctx
        .client
        .get(format!("{}/categories/{}", ctx.url, category_id))
        .bearer_auth(&token.token)
        .send()
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        reqwest::StatusCode::NOT_FOUND,
        "expected NOT FOUND for deleted category, got: {}",
        response.status()
    );
}

async fn test_delete_me(ctx: &mut Ctx, token: &model::auth::Token) {
    let response = ctx
        .client
        .delete(format!("{}/me", ctx.url))
        .bearer_auth(&token.token)
        .send()
        .await
        .unwrap();

    if let status = response.status()
        && !status.is_success()
    {
        let body = response.text().await.unwrap();
        panic!("failed to delete user: status: {}, body: {}", status, body)
    }
}

async fn test_signin_deleted(ctx: &mut Ctx, signin: &model::auth::SignIn) {
    let response = ctx
        .client
        .post(format!("{}/signin", ctx.url))
        .json(signin)
        .send()
        .await
        .unwrap();

    assert_eq!(
        response.status(),
        reqwest::StatusCode::UNAUTHORIZED,
        "expected UNAUTHORIZED for deleted user, got: {}",
        response.status()
    );
}
