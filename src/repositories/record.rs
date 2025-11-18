use entity::record as entity;
use sea_orm::{DatabaseConnection, QueryFilter, QueryTrait, SqlErr, entity::*};
use uuid::Uuid;

use crate::{errors::AppError, models::record as model};

#[derive(Clone)]
pub struct Repository {
    db: DatabaseConnection,
}

impl Repository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, record: entity::ActiveModel) -> Result<entity::Model, AppError> {
        record
            .insert(&self.db)
            .await
            .map_err(|e| match e.sql_err() {
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
                    AppError::validation(errors)
                }
                _ => e.into(),
            })
    }

    pub async fn find_by_id(&self, id: Uuid) -> Result<entity::Model, AppError> {
        entity::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound)
    }

    pub async fn find_by_filters(
        &self,
        filters: model::RecordFilters,
    ) -> Result<Vec<entity::Model>, AppError> {
        entity::Entity::find()
            .apply_if(filters.user_id, |query, id| {
                query.filter(entity::Column::UserId.eq(id))
            })
            .apply_if(filters.category_id, |query, id| {
                query.filter(entity::Column::CategoryId.eq(id))
            })
            .all(&self.db)
            .await
            .map_err(Into::into)
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<(), AppError> {
        entity::Entity::delete_by_id(id)
            .exec(&self.db)
            .await
            .map_err(Into::into)
            .and_then(|res| match res.rows_affected {
                0 => Err(AppError::NotFound),
                _ => Ok(()),
            })
    }

    pub async fn delete_owned(&self, record_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        entity::Entity::delete_many()
            .filter(entity::Column::Id.eq(record_id))
            .filter(entity::Column::UserId.eq(user_id))
            .exec(&self.db)
            .await
            .map_err(Into::into)
            .and_then(|res| match res.rows_affected {
                0 => Err(AppError::NotFound),
                _ => Ok(()),
            })
    }
}
