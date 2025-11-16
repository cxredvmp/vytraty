use serde::Deserialize;

use crate::error::AppError;

#[derive(Deserialize)]
pub struct UserRegister {
    pub name: String,
    pub default_currency_code: String,
    pub password: String,
}

impl UserRegister {
    pub fn validate(&self) -> Result<(), AppError> {
        let mut errors = Vec::new();

        if self.name.is_empty() {
            errors.push(("name", "name is empty"));
        }

        if self.default_currency_code.is_empty() {
            errors.push(("default_currency_code", "default_currency_code is empty"));
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
pub struct UserLogin {
    pub name: String,
    pub password: String,
}

impl UserLogin {
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
