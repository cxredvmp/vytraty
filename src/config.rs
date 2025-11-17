use std::env;

#[derive(Clone)]
pub struct Config {
    db_url: String,
    jwt_secret: String,
}

impl Config {
    pub fn from_env() -> Self {
        Self {
            db_url: get_env("DATABASE_URL"),
            jwt_secret: get_env("JWT_SECRET"),
        }
    }

    pub fn db_url(&self) -> &str {
        &self.db_url
    }

    pub fn jwt_secret(&self) -> &str {
        &self.jwt_secret
    }
}

fn get_env(key: &str) -> String {
    env::var(key).unwrap_or_else(|_| panic!("{key} must be set"))
}
