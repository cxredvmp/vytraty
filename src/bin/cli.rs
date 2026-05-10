use toasty_cli::{Config as ToastyConfig, ToastyCli};

use vytraty::config::Config;

#[tokio::main]
async fn main() {
    dotenvy::dotenv().ok();
    let config = Config::from_env();
    let toasty_config = ToastyConfig::load().expect("failed to load toasty config");

    let db = toasty::Db::builder()
        .models(vytraty::model::models())
        .connect(config.db_url())
        .await
        .expect("failed to connect to database");

    let cli = ToastyCli::with_config(db, toasty_config);
    cli.parse_and_run().await.expect("failed to parse and run");
}
