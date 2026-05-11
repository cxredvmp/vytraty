use serde::{Deserialize, Serialize};
use uuid::Uuid;

use crate::error::{AppError, Result};

#[derive(Serialize, Deserialize)]
pub struct SignUp {
    pub name: String,
    pub default_currency_code: String,
    pub password: String,
}

impl SignUp {
    pub fn validate(&self) -> Result<()> {
        let mut errors = Vec::new();

        if self.name.is_empty() {
            errors.push(("name", "cannot be empty"));
        }

        if self.default_currency_code.is_empty() {
            errors.push(("default_currency_code", "cannot be empty"));
        } else if self.default_currency_code.len() != 3 {
            errors.push(("default_currency_code", "must be 3 chars long"));
        }

        if self.password.is_empty() {
            errors.push(("password", "cannot be empty"));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(AppError::validation(errors))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct SignIn {
    pub name: String,
    pub password: String,
}

impl SignIn {
    pub fn validate(&self) -> Result<()> {
        let mut errors = Vec::new();

        if self.name.is_empty() {
            errors.push(("name", "cannot be empty"));
        }

        if self.password.is_empty() {
            errors.push(("password", "cannot be empty"));
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(AppError::validation(errors))
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct Token {
    pub token: String,
    pub schema: String,
}

#[derive(Clone)]
pub struct Auth {
    pub id: Uuid,
}
