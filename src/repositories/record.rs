use entity::record as entity;
use sea_orm::{DatabaseConnection, QueryFilter, QueryTrait, SqlErr, entity::*};
use uuid::Uuid;

use crate::{error::AppError, models::record as model};

pub async fn insert(
    db: &DatabaseConnection,
    record: entity::ActiveModel,
) -> Result<entity::Model, AppError> {
    record.insert(db).await.map_err(|e| match e.sql_err() {
        Some(SqlErr::ForeignKeyConstraintViolation(e)) => {
            let mut errors = Vec::new();
            if e.contains("fk_record_user") {
                errors.push(("user", "user doesn't exist"))
            }
            if e.contains("fk_record_category") {
                errors.push(("category", "category doesn't exist"))
            }
            if e.contains("fk_record_currency") {
                errors.push(("currency_code", "currency code doesn't exist"))
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

pub async fn filter_by_params(
    db: &DatabaseConnection,
    params: model::RecordFilterParams,
) -> Result<Vec<entity::Model>, AppError> {
    entity::Entity::find()
        .apply_if(params.user_id, |query, id| {
            query.filter(entity::Column::UserId.eq(id))
        })
        .apply_if(params.category_id, |query, id| {
            query.filter(entity::Column::CategoryId.eq(id))
        })
        .all(db)
        .await
        .map_err(Into::into)
}

pub async fn delete_by_id(db: &DatabaseConnection, id: Uuid) -> Result<(), AppError> {
    entity::Entity::delete_by_id(id)
        .exec(db)
        .await
        .map_err(Into::into)
        .and_then(|res| match res.rows_affected {
            0 => Err(AppError::NotFound),
            _ => Ok(()),
        })
}
