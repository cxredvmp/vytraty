use jiff::Timestamp;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use toasty::BelongsTo;
use uuid::Uuid;

use crate::{
    error::{AppError, Result},
    model,
};

#[derive(toasty::Model)]
pub struct Record {
    #[key]
    pub id: Uuid,

    pub user_id: Uuid,
    #[belongs_to(key = user_id, references = id)]
    pub user: BelongsTo<model::user::User>,

    pub category_id: Uuid,
    #[belongs_to(key = category_id, references = id)]
    pub category: BelongsTo<model::category::Category>,

    pub currency_code: String,
    #[belongs_to(key = currency_code, references = code)]
    pub currency: BelongsTo<model::currency::Currency>,

    pub created_at: Timestamp,
    pub sum: Decimal,
}

#[derive(Serialize, Deserialize)]
pub struct Body<T> {
    pub record: T,
}

#[derive(Serialize, Deserialize)]
pub struct BodyArray<T> {
    pub records: Vec<T>,
}

#[derive(Clone, Serialize, Deserialize)]
pub struct Read {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Uuid,
    pub created_at: Timestamp,
    pub sum: Decimal,
    pub currency_code: String,
}

impl From<Record> for Read {
    fn from(record: Record) -> Self {
        Self {
            id: record.id,
            user_id: record.user_id,
            category_id: record.category_id,
            created_at: record.created_at,
            sum: record.sum,
            currency_code: record.currency_code,
        }
    }
}

impl From<Read> for Body<Read> {
    fn from(record: Read) -> Self {
        Self { record }
    }
}

impl From<Vec<Read>> for BodyArray<Read> {
    fn from(records: Vec<Read>) -> Self {
        Self { records }
    }
}

#[derive(Serialize, Deserialize)]
pub struct CreateRequest {
    pub category_id: Uuid,
    pub sum: Decimal,
    pub currency_code: Option<String>,
}

impl CreateRequest {
    pub fn validate(&self) -> Result<()> {
        let mut errors = Vec::new();

        if self.sum <= Decimal::ZERO {
            errors.push(("sum", "must be positive"));
        }

        if self
            .currency_code
            .as_deref()
            .is_some_and(|cc| cc.is_empty())
        {
            errors.push(("currency_code", "cannot be empty"));
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
    pub category_id: Uuid,
    pub created_at: Timestamp,
    pub sum: Decimal,
    pub currency_code: String,
}

#[derive(Deserialize)]
pub struct Filters {
    pub category_id: Option<Uuid>,
}

impl Filters {
    pub fn validate(&self) -> Result<()> {
        Ok(())
    }
}
