use entity::user as user_entity;
use sea_orm::ActiveValue::Set;
use tokio::sync::OnceCell;
use uuid::Uuid;

use crate::{
    errors::{AppError, AuthError},
    models::{auth as model, user as user_model},
    repositories::user::Repository as UserRepository,
    utils::password,
};

static DUMMY_HASH: OnceCell<String> = OnceCell::const_new();
async fn get_dummy_hash() -> Result<&'static String, AppError> {
    DUMMY_HASH
        .get_or_try_init(async || -> Result<String, AppError> {
            password::hash("password".to_string()).await
        })
        .await
}

#[derive(Clone)]
pub struct Service {
    user_repo: UserRepository,
}

impl Service {
    pub fn new(user_repo: UserRepository) -> Self {
        Self { user_repo }
    }

    pub async fn sign_up(&self, user: model::UserSignUp) -> Result<user_model::UserRead, AppError> {
        let user = user_entity::ActiveModel {
            id: Set(Uuid::new_v4()),
            name: Set(user.name),
            default_currency_code: Set(user.default_currency_code),
            password_hash: Set(password::hash(user.password).await?),
        };
        self.user_repo.insert(user).await.map(Into::into)
    }

    pub async fn sign_in(
        &self,
        creds: model::UserSignIn,
    ) -> Result<user_model::UserRead, AppError> {
        let user = match self.user_repo.find_by_name(&creds.name).await {
            Ok(user) => Ok(Some(user)),
            Err(AppError::NotFound) => Ok(None),
            Err(e) => Err(e),
        }?;
        let password_hash = match &user {
            Some(user) => user.password_hash.clone(),
            None => get_dummy_hash().await?.to_string(),
        };
        password::verify(creds.password, password_hash)
            .await
            .and_then(|_| match user {
                Some(user) => Ok(user.into()),
                None => Err(AppError::Auth(AuthError::InvalidPassword)),
            })
    }
}
