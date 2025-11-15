use entity::user;
use sea_orm::{ActiveModelTrait, ActiveValue::Set, DatabaseConnection, EntityTrait, SqlErr};
use uuid::Uuid;

use crate::{error::AppError, models::user::*};

pub async fn create_user(db: &DatabaseConnection, user: UserCreate) -> Result<User, AppError> {
    let user = user::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(user.name),
        default_currency_code: Set(user.default_currency_code),
    };
    let user = user
        .insert(db)
        .await
        .map_err(|e| match e.sql_err() {
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
        })?
        .into();
    Ok(user)
}

pub async fn get_user(db: &DatabaseConnection, id: Uuid) -> Result<User, AppError> {
    let user = user::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?
        .into();
    Ok(user)
}

pub async fn get_users(db: &DatabaseConnection) -> Result<Vec<User>, AppError> {
    let users = user::Entity::find()
        .all(db)
        .await?
        .into_iter()
        .map(Into::into)
        .collect();
    Ok(users)
}

pub async fn get_default_currency_code(
    db: &DatabaseConnection,
    id: Uuid,
) -> Result<String, AppError> {
    let user = user::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or_else(|| AppError::unprocessable_entity([("user_id", "user doesn't exist")]))?;
    Ok(user.default_currency_code)
}

pub async fn delete_user(db: &DatabaseConnection, id: Uuid) -> Result<(), AppError> {
    let res = user::Entity::delete_by_id(id).exec(db).await?;
    match res.rows_affected {
        0 => Err(AppError::NotFound),
        _ => Ok(()),
    }
}
