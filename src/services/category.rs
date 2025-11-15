use entity::category as entity;
use sea_orm::ActiveValue::Set;
use uuid::Uuid;

use crate::{error::AppError, models::category as model, repositories::category::Repository};

#[derive(Clone)]
pub struct Service {
    repo: Repository,
}

impl Service {
    pub fn new(repo: Repository) -> Self {
        Self { repo }
    }

    pub async fn create_category(
        self,
        category: model::CategoryCreate,
    ) -> Result<model::Category, AppError> {
        let category = entity::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(category.name),
        };
        self.repo.insert(category).await.map(Into::into)
    }

    pub async fn get_category(self, id: Uuid) -> Result<model::Category, AppError> {
        self.repo.find_by_id(id).await.map(Into::into)
    }

    pub async fn get_categories(self) -> Result<Vec<model::Category>, AppError> {
        self.repo
            .find_all()
            .await
            .map(|entities| entities.into_iter().map(Into::into).collect())
    }

    pub async fn delete_category(self, id: Uuid) -> Result<(), AppError> {
        self.repo.delete_by_id(id).await
    }
}
