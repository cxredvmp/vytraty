use entity::user as entity;
use sea_orm::entity::*;
use uuid::Uuid;

use crate::{error::AppError, models::user as model, repositories::user::Repository};

#[derive(Clone)]
pub struct Service {
    repo: Repository,
}

impl Service {
    pub fn new(repo: Repository) -> Self {
        Self { repo }
    }

    pub async fn create_user(self, user: model::UserCreate) -> Result<model::UserRead, AppError> {
        let user = entity::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(user.name),
            default_currency_code: Set(user.default_currency_code),
            password_hash: Set(todo!()),
        };
        self.repo.insert(user).await.map(Into::into)
    }

    pub async fn get_user(self, id: Uuid) -> Result<model::UserRead, AppError> {
        self.repo.find_by_id(id).await.map(Into::into)
    }

    pub async fn get_users(self) -> Result<Vec<model::UserRead>, AppError> {
        self.repo
            .find_all()
            .await
            .map(|entities| entities.into_iter().map(Into::into).collect())
    }

    pub async fn delete_user(self, id: Uuid) -> Result<(), AppError> {
        self.repo.delete_by_id(id).await
    }
}
