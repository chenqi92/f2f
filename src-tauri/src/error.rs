use serde::{Deserialize, Serialize};
use thiserror::Error;

/// Application error types
#[derive(Error, Debug)]
pub enum AppError {
    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Resource limit exceeded: {0}")]
    ResourceLimit(String),

    #[error("Tool error: {0}")]
    ToolError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Operation cancelled")]
    Cancelled,

    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Serialization error: {0}")]
    SerializationError(#[from] serde_json::Error),

    #[error("Internal error: {0}")]
    Internal(String),
}

impl From<rusqlite::Error> for AppError {
    fn from(err: rusqlite::Error) -> Self {
        AppError::DatabaseError(err.to_string())
    }
}

/// Error response sent to frontend
#[derive(Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub kind: ErrorKind,
    pub message: String,
    pub suggestion: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(rename_all = "SCREAMING_SNAKE_CASE")]
pub enum ErrorKind {
    ValidationError,
    ResourceLimit,
    ToolError,
    IoError,
    Canceled,
    DatabaseError,
    Internal,
}

impl From<AppError> for ErrorResponse {
    fn from(err: AppError) -> Self {
        let (kind, message, suggestion) = match err {
            AppError::ValidationError(msg) => (
                ErrorKind::ValidationError,
                msg,
                Some("请检查输入参数是否正确".to_string()),
            ),
            AppError::ResourceLimit(msg) => (
                ErrorKind::ResourceLimit,
                msg,
                Some("请尝试降低质量设置或分批处理".to_string()),
            ),
            AppError::ToolError(msg) => (
                ErrorKind::ToolError,
                msg,
                Some("请检查工具是否正确安装".to_string()),
            ),
            AppError::IoError(e) => (
                ErrorKind::IoError,
                e.to_string(),
                Some("请检查文件路径和权限".to_string()),
            ),
            AppError::Cancelled => (
                ErrorKind::Canceled,
                "操作已取消".to_string(),
                Some("可从断点继续运行".to_string()),
            ),
            AppError::DatabaseError(msg) => (
                ErrorKind::DatabaseError,
                msg,
                None,
            ),
            AppError::SerializationError(e) => (
                ErrorKind::Internal,
                e.to_string(),
                None,
            ),
            AppError::Internal(msg) => (
                ErrorKind::Internal,
                msg,
                None,
            ),
        };

        ErrorResponse {
            kind,
            message,
            suggestion,
        }
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
