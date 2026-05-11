use serde::{Deserialize, Serialize};
use toasty::BelongsTo;
use utoipa::ToSchema;
use uuid::Uuid;

use crate::model;

#[derive(toasty::Model)]
pub struct User {
    #[key]
    pub id: Uuid,

    #[unique]
    pub name: String,

    pub default_currency_code: String,
    #[belongs_to(key = default_currency_code, references = code)]
    pub default_currency: BelongsTo<model::currency::Currency>,

    pub password_hash: String,
}

#[derive(Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct Body<T> {
    pub user: T,
}

#[derive(Serialize, Deserialize)]
#[derive(ToSchema)]
pub struct Read {
    pub id: Uuid,
    pub name: String,
    pub default_currency_code: String,
}

impl From<User> for Read {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            name: user.name,
            default_currency_code: user.default_currency_code,
        }
    }
}

impl From<Read> for Body<Read> {
    fn from(user: Read) -> Self {
        Self { user }
    }
}

pub struct Create {
    pub id: Uuid,
    pub name: String,
    pub default_currency_code: String,
    pub password_hash: String,
}
