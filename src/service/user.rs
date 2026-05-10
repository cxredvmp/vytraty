use uuid::Uuid;

use crate::{error::Result, model, repository};

#[derive(Clone)]
pub struct Service {
    user_repo: repository::User,
}

impl Service {
    pub fn new(user_repo: repository::User) -> Self {
        Self { user_repo }
    }

    pub async fn get_by_id(&mut self, id: Uuid) -> Result<model::user::Read> {
        self.user_repo.get_by_id(id).await.map(Into::into)
    }

    pub async fn delete_by_id(&mut self, id: Uuid) -> Result<()> {
        self.user_repo.delete_by_id(id).await
    }
}
