use chrono::Utc;
use entity::record;
use sea_orm::{ActiveValue::Set, DatabaseConnection};
use uuid::Uuid;

use crate::{error::AppError, models::record::*, repositories, repositories::record as repository};

pub async fn create_record(
    db: &DatabaseConnection,
    record: RecordCreate,
) -> Result<Record, AppError> {
    let currency_code = match record.currency_code {
        Some(cc) => cc,
        None => repositories::user::get_default_currency_code(db, record.user_id).await?,
    };
    let record = record::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(record.user_id),
        category_id: Set(record.category_id),
        sum: Set(record.sum),
        created_at: Set(Utc::now().into()),
        currency_code: Set(currency_code),
    };
    repository::insert(db, record).await.map(Into::into)
}

pub async fn get_record(db: &DatabaseConnection, id: Uuid) -> Result<Record, AppError> {
    repository::find_by_id(db, id).await.map(Into::into)
}

pub async fn filter_records(
    db: &DatabaseConnection,
    params: RecordFilterParams,
) -> Result<Vec<Record>, AppError> {
    repository::filter_by_params(db, params)
        .await
        .map(|entities| entities.into_iter().map(Into::into).collect())
}

pub async fn delete_record(db: &DatabaseConnection, id: Uuid) -> Result<(), AppError> {
    repository::delete_by_id(db, id).await
}
