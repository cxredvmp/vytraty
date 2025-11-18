use entity::category as entity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Serialize, Deserialize)]
pub struct CategoryBody<T> {
    pub category: T,
}

#[derive(Serialize, Deserialize)]
pub struct CategoriesBody<T> {
    pub categories: Vec<T>,
}

#[derive(Clone, Serialize)]
pub struct CategoryRead {
    pub id: Uuid,
    pub name: String,
}

impl From<entity::Model> for CategoryRead {
    fn from(value: entity::Model) -> Self {
        Self {
            id: value.id,
            name: value.name,
        }
    }
}

#[derive(Deserialize)]
pub struct CategoryCreate {
    pub name: String,
}

impl CategoryCreate {
    pub fn validate(&self) -> Result<(), AppError> {
        let mut errors = Vec::new();

        if self.name.is_empty() {
            errors.push(("name", "cannot be empty"));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(AppError::unprocessable_entity(errors))
        }
    }
}
