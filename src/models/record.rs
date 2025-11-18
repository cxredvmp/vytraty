use chrono::{DateTime, Utc};
use entity::record as entity;
use rust_decimal::Decimal;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Serialize, Deserialize)]
pub struct RecordBody<T> {
    pub record: T,
}

#[derive(Serialize)]
pub struct RecordsBody<T> {
    pub records: Vec<T>,
}

#[derive(Clone, Serialize)]
pub struct RecordRead {
    pub id: Uuid,
    pub user_id: Uuid,
    pub category_id: Uuid,
    pub created_at: DateTime<Utc>,
    pub sum: Decimal,
}

impl From<entity::Model> for RecordRead {
    fn from(value: entity::Model) -> Self {
        Self {
            id: value.id,
            user_id: value.user_id,
            category_id: value.category_id,
            created_at: value.created_at.to_utc(),
            sum: value.sum,
        }
    }
}

#[derive(Deserialize)]
pub struct RecordCreate {
    pub user_id: Uuid,
    pub category_id: Uuid,
    pub sum: Decimal,
    pub currency_code: Option<String>,
}

impl RecordCreate {
    pub fn validate(&self) -> Result<(), AppError> {
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
            Err(AppError::unprocessable_entity(errors))
        }
    }
}

#[derive(Deserialize)]
pub struct RecordFilterParams {
    pub user_id: Option<Uuid>,
    pub category_id: Option<Uuid>,
}

impl RecordFilterParams {
    pub fn validate(&self) -> Result<(), AppError> {
        if self.user_id.is_none() && self.category_id.is_none() {
            Err(AppError::unprocessable_entity([(
                "params",
                "at least one is required",
            )]))
        } else {
            Ok(())
        }
    }
}
