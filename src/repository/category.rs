use toasty::Db;
use uuid::Uuid;

use crate::error::{AppError, Result};
use crate::model::{self, category::Category};

#[derive(Clone)]
pub struct Repository {
    db: Db,
}

impl Repository {
    pub fn new(db: Db) -> Self {
        Self { db }
    }

    pub async fn create_for(&mut self, category: model::category::Create) -> Result<Category> {
        let res = toasty::create!(Category {
            id: category.id,
            user_id: category.user_id,
            name: category.name,
        })
        .exec(&mut self.db)
        .await;

        match res {
            Ok(category) => Ok(category),
            Err(err) => {
                let err_msg = err.to_string();
                let mut errs = Vec::new();

                if err_msg.contains("category_name_key") {
                    errs.push(("name", "category already exists"));
                }

                if !errs.is_empty() {
                    Err(AppError::validation(errs))
                } else {
                    Err(err.into())
                }
            }
        }
    }

    pub async fn get_by_id_for(&mut self, id: Uuid, user_id: Uuid) -> Result<Category> {
        Category::filter(Category::fields().user_id().eq(user_id))
            .filter(Category::fields().id().eq(id))
            .get(&mut self.db)
            .await
            .map_err(Into::into)
    }

    pub async fn get_all_for(&mut self, user_id: Uuid) -> Result<Vec<Category>> {
        Category::filter(
            Category::fields()
                .user_id()
                .eq(user_id)
                .or(Category::fields().user_id().is_none()),
        )
        .exec(&mut self.db)
        .await
        .map_err(Into::into)
    }

    pub async fn delete_by_id_for(&mut self, id: Uuid, user_id: Uuid) -> Result<()> {
        Category::filter(Category::fields().user_id().eq(user_id))
            .filter(Category::fields().id().eq(id))
            .delete()
            .exec(&mut self.db)
            .await
            .map_err(Into::into)
    }
}
