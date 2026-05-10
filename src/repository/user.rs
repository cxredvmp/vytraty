use toasty::Db;
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::model::{self, user::User};

#[derive(Clone)]
pub struct Repository {
    db: Db,
}

impl Repository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn create(&mut self, user: model::user::Create) -> Result<User> {
        let res = toasty::create!(User {
            id: user.id,
            name: user.name,
            default_currency_code: user.default_currency_code,
            password_hash: user.password_hash,
        })
        .exec(&mut self.db)
        .await;

        match res {
            Ok(user) => Ok(user),
            Err(err) => {
                let err_msg = err.to_string();
                let mut errs = Vec::new();

                if err_msg.contains("user_name_key") {
                    errs.push(("name", "user already exists"));
                }
                if err_msg.contains("fk_user_currency") {
                    errs.push(("default_currency_code", "currency doesn't exist"));
                }

                Err(if !errs.is_empty() {
                    AppError::validation(errs)
                } else {
                    err.into()
                })
            }
        }
    }

    pub async fn get_by_id(&mut self, id: Uuid) -> Result<User> {
        User::get_by_id(&mut self.db, id).await.map_err(Into::into)
    }

    pub async fn get_by_name(&mut self, name: &str) -> Result<User> {
        User::get_by_name(&mut self.db, name)
            .await
            .map_err(Into::into)
    }

    pub async fn get_default_currency_code(&mut self, id: Uuid) -> Result<String> {
        let user = User::get_by_id(&mut self.db, id).await?;
        Ok(user.default_currency_code)
    }

    pub async fn delete_by_id(&mut self, id: Uuid) -> Result<()> {
        User::delete_by_id(&mut self.db, id)
            .await
            .map_err(Into::into)
    }
}
