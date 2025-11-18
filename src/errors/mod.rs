use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use sea_orm::DbErr;
use serde::Serialize;

pub enum AppError {
    Auth,
    NotFound,
    Validation(Vec<ValidationError>),
    Internal(String),
    Database(DbErr),
}

#[derive(Serialize)]
pub struct ErrorBody {
    pub message: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub errors: Option<Vec<ValidationError>>,
}

#[derive(Serialize)]
pub struct ValidationError {
    pub field: &'static str,
    pub message: &'static str,
}

impl From<&AppError> for StatusCode {
    fn from(value: &AppError) -> Self {
        match value {
            AppError::Auth => Self::UNAUTHORIZED,
            AppError::NotFound => Self::NOT_FOUND,
            AppError::Validation { .. } => Self::UNPROCESSABLE_ENTITY,
            AppError::Internal(_) => Self::INTERNAL_SERVER_ERROR,
            AppError::Database(_) => Self::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<AppError> for ErrorBody {
    fn from(value: AppError) -> Self {
        match value {
            AppError::Validation(es) => Self {
                message: "validation failed".to_string(),
                errors: Some(es),
            },
            _ => Self {
                message: StatusCode::from(&value).to_string(),
                errors: None,
            },
        }
    }
}

impl AppError {
    pub fn validation(errors: impl IntoIterator<Item = (&'static str, &'static str)>) -> Self {
        Self::Validation(
            errors
                .into_iter()
                .map(|e| ValidationError {
                    field: e.0,
                    message: e.1,
                })
                .collect(),
        )
    }
}

impl From<DbErr> for AppError {
    fn from(value: DbErr) -> Self {
        AppError::Database(value)
    }
}

impl IntoResponse for AppError {
    fn into_response(self) -> Response {
        match &self {
            Self::Internal(e) => {
                eprintln!("internal error: {e}");
            }
            Self::Database(e) => {
                eprintln!("database error: {e}");
            }
            _ => {}
        }

        let status_code = StatusCode::from(&self);
        let body = ErrorBody::from(self);
        (status_code, Json(body)).into_response()
    }
}
