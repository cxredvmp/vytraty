use std::env;

#[derive(Clone)]
pub struct Config {
    db_url: String,
    setup_db: bool,
    port: String,
    jwt_secret: String,
}

impl Config {
    pub fn new(db_url: String, setup_db: String, port: String, jwt_secret: String) -> Self {
        let setup_db = setup_db.to_lowercase().parse().unwrap_or(false);
        Self {
            db_url,
            setup_db,
            port,
            jwt_secret,
        }
    }

    pub fn from_env() -> Self {
        Self::new(
            get_env("DATABASE_URL"),
            env::var("SETUP_DB").unwrap_or("false".to_string()),
            get_env("PORT"),
            get_env("JWT_SECRET"),
        )
    }

    pub fn db_url(&self) -> &str {
        &self.db_url
    }

    pub fn setup_db(&self) -> bool {
        self.setup_db
    }

    pub fn port(&self) -> &str {
        &self.port
    }

    pub fn jwt_secret(&self) -> &str {
        &self.jwt_secret
    }
}

fn get_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("{key} must be set"))
}
