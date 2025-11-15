use sea_orm::DatabaseConnection;

use crate::models::health as model;

#[derive(Clone)]
pub struct Service {
    db: DatabaseConnection,
}

impl Service {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn check_db(self) -> model::Status {
        match self.db.ping().await {
            Ok(_) => model::Status::Up,
            Err(e) => {
                eprintln!("database ping failed: {e}");
                model::Status::Down
            }
        }
    }
}
