use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::AppError;

#[derive(Deserialize)]
pub struct UserSignUp {
    pub name: String,
    pub default_currency_code: String,
    pub password: String,
}

impl UserSignUp {
    pub fn validate(&self) -> Result<(), AppError> {
        let mut errors = Vec::new();

        if self.name.is_empty() {
            errors.push(("name", "name is empty"));
        }

        if self.default_currency_code.is_empty() {
            errors.push(("default_currency_code", "default_currency_code is empty"));
        } else if self.default_currency_code.len() != 3 {
            errors.push((
                "default_currency_code",
                "default_currency_code has invalid length",
            ));
        }

        if self.password.is_empty() {
            errors.push(("password", "password is empty"));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(AppError::unprocessable_entity(errors))
        }
    }
}

#[derive(Deserialize)]
pub struct UserSignIn {
    pub name: String,
    pub password: String,
}

impl UserSignIn {
    pub fn validate(&self) -> Result<(), AppError> {
        let mut errors = Vec::new();

        if self.name.is_empty() {
            errors.push(("name", "name is empty"));
        }

        if self.password.is_empty() {
            errors.push(("password", "password is empty"));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(AppError::unprocessable_entity(errors))
        }
    }
}

#[derive(Serialize)]
pub struct Token {
    pub token: String,
    pub schema: String,
}

#[derive(Clone)]
pub struct UserAuth {
    pub id: Uuid,
}
