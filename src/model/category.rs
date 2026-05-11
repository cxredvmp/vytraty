use serde::{Deserialize, Serialize};
use toasty::BelongsTo;
use uuid::Uuid;

use crate::{
    error::{AppError, Result},
    model,
};

#[derive(toasty::Model)]
pub struct Category {
    #[key]
    pub id: Uuid,

    pub user_id: Option<Uuid>,
    #[belongs_to(key = user_id, references = id)]
    pub user: BelongsTo<Option<model::user::User>>,

    pub name: String,
}

#[derive(Serialize, Deserialize)]
pub struct Body<T> {
    pub category: T,
}

#[derive(Serialize, Deserialize)]
pub struct BodyArray<T> {
    pub categories: Vec<T>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Read {
    pub id: Uuid,
    pub name: String,
}

impl From<Category> for Read {
    fn from(category: Category) -> Self {
        Self {
            id: category.id,
            name: category.name,
        }
    }
}

impl From<Read> for Body<Read> {
    fn from(category: Read) -> Self {
        Self { category }
    }
}

impl From<Vec<Read>> for BodyArray<Read> {
    fn from(categories: Vec<Read>) -> Self {
        Self { categories }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateRequest {
    pub name: String,
}

impl CreateRequest {
    pub fn validate(&self) -> Result<()> {
        let mut errors = Vec::new();

        if self.name.is_empty() {
            errors.push(("name", "cannot be empty"));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(AppError::validation(errors))
        }
    }
}

pub struct Create {
    pub id: Uuid,
    pub user_id: Uuid,
    pub name: String,
}
