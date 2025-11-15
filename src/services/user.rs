use entity::user as entity;
use sea_orm::{DatabaseConnection, entity::*};
use uuid::Uuid;

use crate::{error::AppError, models::user as model, repositories::user as repository};

pub async fn create_user(
    db: &DatabaseConnection,
    user: model::UserCreate,
) -> Result<model::User, AppError> {
    let user = entity::ActiveModel {
        id: Set(Uuid::new_v4()),
        name: Set(user.name),
        default_currency_code: Set(user.default_currency_code),
        password_hash: Set(todo!()),
    };
    repository::insert(db, user).await.map(Into::into)
}

pub async fn get_user(db: &DatabaseConnection, id: Uuid) -> Result<model::User, AppError> {
    repository::find_by_id(db, id).await.map(Into::into)
}

pub async fn get_users(db: &DatabaseConnection) -> Result<Vec<model::User>, AppError> {
    repository::find_all(db)
        .await
        .map(|entities| entities.into_iter().map(Into::into).collect())
}

pub async fn delete_user(db: &DatabaseConnection, id: Uuid) -> Result<(), AppError> {
    repository::delete_by_id(db, id).await
}
