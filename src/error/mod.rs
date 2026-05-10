mod auth;

use axum::{
    Json,
    http::StatusCode,
    response::{IntoResponse, Response},
};
use serde::Serialize;
use toasty::Error as DbError;

pub use auth::Error as AuthError;
pub type Result<T> = std::result::Result<T, AppError>;

pub enum AppError {
    Auth(AuthError),
    NotFound,
    Validation(Vec<ValidationError>),
    Internal(String),
    Database(DbError),
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
    fn from(err: &AppError) -> Self {
        match err {
            AppError::Auth(_) => Self::UNAUTHORIZED,
            AppError::NotFound => Self::NOT_FOUND,
            AppError::Validation { .. } => Self::UNPROCESSABLE_ENTITY,
            AppError::Internal(_) => Self::INTERNAL_SERVER_ERROR,
            AppError::Database(_) => Self::INTERNAL_SERVER_ERROR,
        }
    }
}

impl From<AppError> for ErrorBody {
    fn from(err: AppError) -> Self {
        match err {
            AppError::Auth(e) => Self {
                message: e.to_string(),
                errors: None,
            },
            AppError::Validation(es) => Self {
                message: "validation failed".to_string(),
                errors: Some(es),
            },
            _ => Self {
                message: StatusCode::from(&err).to_string(),
                errors: None,
            },
        }
    }
}

impl AppError {
    pub fn validation(errs: impl IntoIterator<Item = (&'static str, &'static str)>) -> Self {
        Self::Validation(
            errs.into_iter()
                .map(|e| ValidationError {
                    field: e.0,
                    message: e.1,
                })
                .collect(),
        )
    }
}

impl From<DbError> for AppError {
    fn from(err: DbError) -> Self {
        match err {
            value if value.is_record_not_found() => AppError::NotFound,
            _ => AppError::Database(err),
        }
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
