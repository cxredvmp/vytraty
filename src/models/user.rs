use entity::user as entity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Serialize, Deserialize)]
pub struct UserBody<T> {
    pub user: T,
}

#[derive(Serialize, Deserialize)]
pub struct UsersBody<T> {
    pub users: Vec<T>,
}

#[derive(Clone, Serialize)]
pub struct UserRead {
    pub id: Uuid,
    pub name: String,
    pub default_currency_code: String,
}

impl From<entity::Model> for UserRead {
    fn from(value: entity::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
            default_currency_code: value.default_currency_code,
        }
    }
}

#[derive(Deserialize)]
pub struct UserCreate {
    pub name: String,
    pub default_currency_code: String,
}

impl UserCreate {
    pub fn validate(&self) -> Result<(), AppError> {
        let mut errors = Vec::new();

        if self.name.is_empty() {
            errors.push(("name", "name is empty"));
        }

        if self.default_currency_code.is_empty() {
            errors.push(("default_currency_code", "default_currency_code is empty"));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(AppError::unprocessable_entity(errors))
        }
    }
}
