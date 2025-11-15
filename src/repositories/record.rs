use entity::record as entity;
use sea_orm::{DatabaseConnection, QueryFilter, QueryTrait, SqlErr, entity::*};
use uuid::Uuid;

use crate::{error::AppError, models::record as model};

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
                    AppError::unprocessable_entity(errors)
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

    pub async fn filter_by_params(
        &self,
        params: model::RecordFilterParams,
    ) -> Result<Vec<entity::Model>, AppError> {
        entity::Entity::find()
            .apply_if(params.user_id, |query, id| {
                query.filter(entity::Column::UserId.eq(id))
            })
            .apply_if(params.category_id, |query, id| {
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
}
