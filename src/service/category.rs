use uuid::Uuid;

use crate::{error::Result, model, repository};

#[derive(Clone)]
pub struct Service {
    category_repo: repository::Category,
}

impl Service {
    pub fn new(category_repo: repository::Category) -> Self {
        Self { category_repo }
    }

    pub async fn create_for(
        &mut self,
        category: model::category::CreateRequest,
        user_id: Uuid,
    ) -> Result<model::category::Read> {
        let category = model::category::Create {
            id: Uuid::new_v4(),
            user_id,
            name: category.name,
        };
        self.category_repo
            .create_for(category)
            .await
            .map(Into::into)
    }

    pub async fn get_by_id_for(
        &mut self,
        id: Uuid,
        user_id: Uuid,
    ) -> Result<model::category::Read> {
        self.category_repo
            .get_by_id_for(id, user_id)
            .await
            .map(Into::into)
    }

    pub async fn get_all_for(&mut self, user_id: Uuid) -> Result<Vec<model::category::Read>> {
        self.category_repo
            .get_all_for(user_id)
            .await
            .map(|categories| categories.into_iter().map(Into::into).collect())
    }

    pub async fn delete_by_id_for(&mut self, id: Uuid, user_id: Uuid) -> Result<()> {
        self.category_repo.delete_by_id_for(id, user_id).await
    }
}
