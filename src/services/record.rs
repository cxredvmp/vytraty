use chrono::Utc;
use entity::record;
use sea_orm::{
    ActiveModelTrait, ActiveValue::Set, ColumnTrait, DatabaseConnection, EntityTrait, QueryFilter,
    SqlErr,
};
use uuid::Uuid;

use crate::{error::AppError, models::record::*};

pub async fn create_record(
    db: &DatabaseConnection,
    record: RecordCreate,
) -> Result<Record, AppError> {
    let currency_code = match record.currency_code {
        Some(cc) => cc,
        None => super::user::get_default_currency_code(db, record.user_id).await?,
    };
    let record = record::ActiveModel {
        id: Set(Uuid::new_v4()),
        user_id: Set(record.user_id),
        category_id: Set(record.category_id),
        sum: Set(record.sum),
        created_at: Set(Utc::now().into()),
        currency_code: Set(currency_code),
    };
    let record = record
        .insert(db)
        .await
        .map_err(|e| match e.sql_err() {
            Some(SqlErr::ForeignKeyConstraintViolation(e)) => {
                let mut errors = Vec::new();
                if e.contains("fk_record_user") {
                    errors.push(("user", "user doesn't exist"))
                }
                if e.contains("fk_record_category") {
                    errors.push(("category", "category doesn't exist"))
                }
                if e.contains("fk_record_currency") {
                    errors.push(("currency_code", "currency doesn't exist"))
                }
                AppError::unprocessable_entity(errors)
            }
            _ => e.into(),
        })?
        .into();
    Ok(record)
}

pub async fn get_record(db: &DatabaseConnection, id: Uuid) -> Result<Record, AppError> {
    let record = record::Entity::find_by_id(id)
        .one(db)
        .await?
        .ok_or(AppError::NotFound)?
        .into();
    Ok(record)
}

pub async fn filter_records(
    db: &DatabaseConnection,
    params: RecordFilterParams,
) -> Result<Vec<Record>, AppError> {
    let mut query = record::Entity::find();
    if let Some(user_id) = params.user_id {
        query = query.filter(record::Column::UserId.eq(user_id));
    }
    if let Some(category_id) = params.category_id {
        query = query.filter(record::Column::CategoryId.eq(category_id));
    }
    let records = query.all(db).await?.into_iter().map(Into::into).collect();
    Ok(records)
}

pub async fn delete_record(db: &DatabaseConnection, id: Uuid) -> Result<(), AppError> {
    let res = record::Entity::delete_by_id(id).exec(db).await?;
    match res.rows_affected {
        0 => Err(AppError::NotFound),
        _ => Ok(()),
    }
}
