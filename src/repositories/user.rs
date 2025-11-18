use entity::user as entity;
use sea_orm::{DatabaseConnection, SqlErr, entity::*, query::*};
use uuid::Uuid;

use crate::errors::AppError;

#[derive(Clone)]
pub struct Repository {
    db: DatabaseConnection,
}

impl Repository {
    pub fn new(db: DatabaseConnection) -> Self {
        Self { db }
    }

    pub async fn insert(&self, user: entity::ActiveModel) -> Result<entity::Model, AppError> {
        user.insert(&self.db).await.map_err(|e| match e.sql_err() {
            Some(SqlErr::UniqueConstraintViolation(e)) => {
                let mut errors = Vec::new();
                if e.contains("user_name_key") {
                    errors.push(("name", "user already exists"));
                }
                AppError::validation(errors)
            }
            Some(SqlErr::ForeignKeyConstraintViolation(e)) => {
                let mut errors = Vec::new();
                if e.contains("fk_user_currency") {
                    errors.push(("default_currency_code", "currency doesn't exist"));
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

    pub async fn find_by_name(&self, name: &str) -> Result<entity::Model, AppError> {
        entity::Entity::find()
            .filter(entity::Column::Name.eq(name))
            .one(&self.db)
            .await?
            .ok_or(AppError::NotFound)
    }

    pub async fn find_all(&self) -> Result<Vec<entity::Model>, AppError> {
        Ok(entity::Entity::find().all(&self.db).await?)
    }

    pub async fn get_default_currency_code(&self, id: Uuid) -> Result<String, AppError> {
        let user = entity::Entity::find_by_id(id)
            .one(&self.db)
            .await?
            .ok_or_else(|| AppError::NotFound)?;
        Ok(user.default_currency_code)
    }

    pub async fn delete_by_id(&self, id: Uuid) -> Result<(), AppError> {
        let res = entity::Entity::delete_by_id(id).exec(&self.db).await?;
        match res.rows_affected {
            0 => Err(AppError::NotFound),
            _ => Ok(()),
        }
    }
}
