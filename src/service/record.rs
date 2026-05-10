use jiff::Timestamp;
use uuid::Uuid;

use crate::{error::Result, model, repository};

#[derive(Clone)]
pub struct Service {
    record_repo: repository::Record,
    user_repo: repository::User,
}

impl Service {
    pub fn new(record_repo: repository::Record, user_repo: repository::User) -> Self {
        Self {
            record_repo,
            user_repo,
        }
    }

    pub async fn create_for(
        &mut self,
        record: model::record::CreateRequest,
        user_id: Uuid,
    ) -> Result<model::record::Read> {
        let currency_code = match record.currency_code {
            Some(cc) => cc,
            None => self.user_repo.get_default_currency_code(user_id).await?,
        };
        let record = model::record::Create {
            id: Uuid::new_v4(),
            user_id,
            category_id: record.category_id,
            sum: record.sum,
            created_at: Timestamp::now(),
            currency_code,
        };
        self.record_repo.create_for(record).await.map(Into::into)
    }

    pub async fn get_by_id_for(&mut self, id: Uuid, user_id: Uuid) -> Result<model::record::Read> {
        self.record_repo
            .get_by_id_for(id, user_id)
            .await
            .map(Into::into)
    }

    pub async fn get_all_for(
        &mut self,
        filters: model::record::Filters,
        user_id: Uuid,
    ) -> Result<Vec<model::record::Read>> {
        self.record_repo
            .get_all_for(filters, user_id)
            .await
            .map(|entities| entities.into_iter().map(Into::into).collect())
    }

    pub async fn delete_by_id_for(&mut self, id: Uuid, user_id: Uuid) -> Result<()> {
        self.record_repo.delete_by_id_for(id, user_id).await
    }
}
