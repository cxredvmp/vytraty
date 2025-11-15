use entity::category as entity;
use sea_orm::{ActiveValue::Set, DatabaseConnection};
use uuid::Uuid;

use crate::{error::AppError, models::category as model, repositories::category as repository};

pub async fn create_category(
    db: &DatabaseConnection,
    category: model::CategoryCreate,
) -> Result<model::Category, AppError> {
    let category = entity::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(category.name),
    };
    repository::insert(db, category).await.map(Into::into)
}

pub async fn get_category(db: &DatabaseConnection, id: Uuid) -> Result<model::Category, AppError> {
    repository::find_by_id(db, id).await.map(Into::into)
}

pub async fn get_categories(db: &DatabaseConnection) -> Result<Vec<model::Category>, AppError> {
    repository::find_all(db)
        .await
        .map(|entities| entities.into_iter().map(Into::into).collect())
}

pub async fn delete_category(db: &DatabaseConnection, id: Uuid) -> Result<(), AppError> {
    repository::delete_by_id(db, id).await
}
