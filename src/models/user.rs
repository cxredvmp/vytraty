use entity::user as entity;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

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
