use std::collections::HashMap;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::DbErr;
use serde::Serialize;

#[derive(Debug)]
pub enum AppError {
    NotFound,
    UnprocessableEntity(UnprocessableEntityBody),
    Database(DbErr),
    Unauthorized,
    Internal(String),
}

#[derive(Serialize)]
pub struct GenericErrorBody {
    pub error: String,
}

impl GenericErrorBody {
    pub fn from_status(status: StatusCode) -> Self {
        Self {
            error: status.to_string(),
        }
    }
}

#[derive(Debug, Serialize)]
pub struct UnprocessableEntityBody {
    pub errors: HashMap<&'static str, &'static str>,
}

impl AppError {
    pub fn unprocessable_entity(
        errors: impl IntoIterator<Item = (&'static str, &'static str)>,
    ) -> Self {
        Self::UnprocessableEntity(UnprocessableEntityBody {
            errors: errors.into_iter().collect(),
        })
    }

    pub fn status_code(&self) -> StatusCode {
        match self {
            Self::NotFound => StatusCode::NOT_FOUND,
            Self::UnprocessableEntity { .. } => StatusCode::UNPROCESSABLE_ENTITY,
            Self::Database(_) => StatusCode::INTERNAL_SERVER_ERROR,
            Self::Unauthorized => StatusCode::UNAUTHORIZED,
            Self::Internal(_) => StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<DbErr> for AppError {
    fn from(value: DbErr) -> Self {
        AppError::Database(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        let status = self.status_code();

        match &self {
            Self::Database(e) => {
                eprintln!("database error: {:?}", e);
            }
            Self::Internal(e) => {
                eprintln!("internal error: {:?}", e);
            }
            _ => {}
        }

        match self {
            Self::UnprocessableEntity(errors) => (status, Json(errors)).into_response(),
            _ => {
                let body = GenericErrorBody::from_status(status);
                (status, Json(body)).into_response()
            }
        }
    }
}
