use toasty::Db;
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::model::{self, record::Record};

#[derive(Clone)]
pub struct Repository {
    db: Db,
}

impl Repository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn create_for(&mut self, record: model::record::Create) -> Result<Record> {
        let res = toasty::create!(Record {
            id: record.id,
            user_id: record.user_id,
            category_id: record.category_id,
            created_at: record.created_at,
            sum: record.sum,
            currency_code: record.currency_code,
        })
        .exec(&mut self.db)
        .await;

        match res {
            Ok(record) => Ok(record),
            Err(e) => {
                let err_msg = e.to_string();
                let mut errs = Vec::new();

                if err_msg.contains("fk_record_user") {
                    errs.push(("user", "user doesn't exist"))
                }
                if err_msg.contains("fk_record_category") {
                    errs.push(("category", "category doesn't exist"))
                }
                if err_msg.contains("fk_record_currency") {
                    errs.push(("currency_code", "currency code doesn't exist"))
                }

                if !errs.is_empty() {
                    Err(AppError::validation(errs))
                } else {
                    Err(e.into())
                }
            }
        }
    }

    pub async fn get_by_id_for(&mut self, id: Uuid, user_id: Uuid) -> Result<Record> {
        Record::filter(Record::fields().id().eq(id))
            .filter(Record::fields().user_id().eq(user_id))
            .get(&mut self.db)
            .await
            .map_err(Into::into)
    }

    pub async fn get_all_for(
        &mut self,
        filters: model::record::Filters,
        user_id: Uuid,
    ) -> Result<Vec<Record>> {
        let mut query = Record::filter(Record::fields().user_id().eq(user_id));
        if let Some(category_id) = filters.category_id {
            query = query.filter(Record::fields().category_id().eq(category_id));
        }
        query.exec(&mut self.db).await.map_err(Into::into)
    }

    pub async fn delete_by_id_for(&mut self, record_id: Uuid, user_id: Uuid) -> Result<()> {
        Record::all()
            .filter(Record::fields().id().eq(record_id))
            .filter(Record::fields().user_id().eq(user_id))
            .delete()
            .exec(&mut self.db)
            .await
            .map_err(Into::into)
    }
}
