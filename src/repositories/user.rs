use entity::user as entity;
use sea_orm::{DatabaseConnection, SqlErr, entity::*, query::*};
use uuid::Uuid;

use crate::error::AppError;

pub async fn insert(
    db: &DatabaseConnection,
    user: entity::ActiveModel,
) -> Result<entity::Model, AppError> {
    user.insert(db).await.map_err(|e| match e.sql_err() {
        Some(SqlErr::UniqueConstraintViolation(e)) => {
            let mut errors = Vec::new();
            if e.contains("user_name_key") {
                errors.push(("name", "user already exists"));
            }
            AppError::unprocessable_entity(errors)
        }
        Some(SqlErr::ForeignKeyConstraintViolation(e)) => {
            let mut errors = Vec::new();
            if e.contains("fk_user_currency") {
                errors.push(("default_currency_code", "currency doesn't exist"));
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

pub async fn find_by_name(db: &DatabaseConnection, name: &str) -> Result<entity::Model, AppError> {
    entity::Entity::find()
        .filter(entity::Column::Name.eq(name))
        .one(db)
        .await?
        .ok_or(AppError::NotFound)
}

pub async fn find_all(db: &DatabaseConnection) -> Result<Vec<entity::Model>, AppError> {
    Ok(entity::Entity::find().all(db).await?)
}

pub async fn get_default_currency_code(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<String, AppError> {
    let user = entity::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::NotFound)?;
    Ok(user.default_currency_code)
}

pub async fn delete_by_id(db: &DatabaseConnection, id: Uuid) -> Result<(), AppError> {
    let res = entity::Entity::delete_by_id(id).exec(db).await?;
    match res.rows_affected {
        0 => Err(AppError::NotFound),
        _ => Ok(()),
    }
}
