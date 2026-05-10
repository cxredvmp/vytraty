use toasty::Db;
use uuid::Uuid;

use crate::model;

#[derive(Clone)]
pub struct Service {
    db: Db,
}

impl Service {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn check_db(&mut self) -> model::health::Status {
        match model::user::User::get_by_id(&mut self.db, Uuid::nil()).await {
            Err(err) if err.is_record_not_found() => model::health::Status::Up,
            Err(err) => {
                eprintln!("database ping failed: {err}");
                model::health::Status::Down
            }
            Ok(_) => unreachable!(),
        }
    }
}
