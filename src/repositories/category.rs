use entity::category as entity;
use sea_orm::{DatabaseConnection, SqlErr, entity::*};
use uuid::Uuid;

use crate::error::AppError;

pub async fn insert(
    db: &DatabaseConnection,
    category: entity::ActiveModel,
) -> Result<entity::Model, AppError> {
    category.insert(db).await.map_err(|e| match e.sql_err() {
        Some(SqlErr::UniqueConstraintViolation(e)) => {
            let mut errors = Vec::new();
            if e.contains("category_name_key") {
                errors.push(("name", "category already exists"));
            }
            AppError::unprocessable_entity(errors)
        }
        _ => e.into(),
    })
}

pub async fn find_by_id(db: &DatabaseConnection, id: Uuid) -> Result<entity::Model, AppError> {
    entity::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)
}

pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<entity::Model>, AppError> {
    Ok(entity::Entity::find().all(db).await?)
}

pub async fn delete_by_id(db: &DatabaseConnection, id: Uuid) -> Result<(), AppError> {
    let res = entity::Entity::delete_by_id(id).exec(db).await?;
    match res.rows_affected {
        0 => Err(AppError::NotFound),
        _ => Ok(()),
    }
}
