use tokio::sync::OnceCell;
use uuid::Uuid;

use crate::{
    error::{AppError, AuthError, Result},
    model, repository,
    utils::password,
};

static DUMMY_HASH: OnceCell<String> = OnceCell::const_new();
async fn get_dummy_hash() -> Result<&'static String> {
    DUMMY_HASH
        .get_or_try_init(async || -> Result<String> {
            password::hash("password".to_string()).await
        })
        .await
}

#[derive(Clone)]
pub struct Service {
    user_repo: repository::User,
}

impl Service {
    pub fn new(user_repo: repository::User) -> Self {
        Self { user_repo }
    }

    pub async fn sign_up(&mut self, user: model::auth::SignUp) -> Result<model::user::Read> {
        let user = model::user::Create {
            id: Uuid::new_v4(),
            name: user.name,
            default_currency_code: user.default_currency_code,
            password_hash: password::hash(user.password).await?,
        };
        self.user_repo.create(user).await.map(Into::into)
    }

    pub async fn sign_in(&mut self, creds: model::auth::SignIn) -> Result<model::user::Read> {
        let user = match self.user_repo.get_by_name(&creds.name).await {
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
