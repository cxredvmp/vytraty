use chrono::Utc;
use entity::record;
use sea_orm::ActiveValue::Set;
use uuid::Uuid;

use crate::{
    error::AppError,
    models::record::*,
    repositories::{record::Repository, user::Repository as UserRepository},
};

#[derive(Clone)]
pub struct Service {
    repo: Repository,
    user_repo: UserRepository,
}

impl Service {
    pub fn new(repo: Repository, user_repo: UserRepository) -> Self {
        Self { repo, user_repo }
    }

    pub async fn create(self, record: RecordCreate) -> Result<RecordRead, AppError> {
        let currency_code = match record.currency_code {
            Some(cc) => cc,
            None => {
                self.user_repo
                    .get_default_currency_code(record.user_id)
                    .await?
            }
        };
        let record = record::ActiveModel {
            id: Set(Uuid::new_v4()),
            user_id: Set(record.user_id),
            category_id: Set(record.category_id),
            sum: Set(record.sum),
            created_at: Set(Utc::now().into()),
            currency_code: Set(currency_code),
        };
        self.repo.insert(record).await.map(Into::into)
    }

    pub async fn get_by_id(self, id: Uuid) -> Result<RecordRead, AppError> {
        self.repo.find_by_id(id).await.map(Into::into)
    }

    pub async fn filter_by_params(
        self,
        params: RecordFilterParams,
    ) -> Result<Vec<RecordRead>, AppError> {
        self.repo
            .filter_by_params(params)
            .await
            .map(|entities| entities.into_iter().map(Into::into).collect())
    }

    pub async fn delete_by_id(self, id: Uuid) -> Result<(), AppError> {
        self.repo.delete_by_id(id).await
    }

    pub async fn delete_owned(self, record_id: Uuid, user_id: Uuid) -> Result<(), AppError> {
        self.repo.delete_owned(record_id, user_id).await
    }
}
