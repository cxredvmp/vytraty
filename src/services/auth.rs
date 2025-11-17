use entity::user as user_entity;
use sea_orm::ActiveValue::Set;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::{auth as model, user as user_model},
    repositories::user::Repository as UserRepository,
    utils::password::*,
};

#[derive(Clone)]
pub struct Service {
    user_repo: UserRepository,
}

impl Service {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }

    pub async fn register_user(
        &self,
        user: model::UserRegister,
    ) -> Result<user_model::UserRead, AppError> {
        let user = user_entity::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(user.name),
            default_currency_code: Set(user.default_currency_code),
            password_hash: Set(hash_password(user.password).await?),
        };
        self.user_repo.insert(user).await.map(Into::into)
    }

    pub async fn login_user(
        &self,
        creds: model::UserLogin,
    ) -> Result<user_model::UserRead, AppError> {
        todo!()
    }
}
