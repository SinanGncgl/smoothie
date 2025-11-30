// Error handling for Smoothie

use serde::Serialize;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum SmoothieError {
    #[error("Database error: {0}")]
    DatabaseError(String),

    #[error("Not found: {0}")]
    NotFound(String),

    #[error("Validation error: {0}")]
    ValidationError(String),

    #[error("Authentication error: {0}")]
    AuthenticationError(String),

    #[error("Authorization error: {0}")]
    AuthorizationError(String),

    #[error("Internal server error: {0}")]
    InternalServerError(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("IO error: {0}")]
    IoError(String),

    #[error("Serialization error: {0}")]
    SerializationError(String),
}

// Implement Serialize manually for Tauri error handling
impl Serialize for SmoothieError {
    fn serialize<S>(&self, serializer: S) -> std::result::Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        serializer.serialize_str(&self.to_string())
    }
}

impl SmoothieError {
    pub fn error_code(&self) -> &str {
        match self {
            SmoothieError::DatabaseError(_) => "DB_ERROR",
            SmoothieError::NotFound(_) => "NOT_FOUND",
            SmoothieError::ValidationError(_) => "VALIDATION_ERROR",
            SmoothieError::AuthenticationError(_) => "AUTH_ERROR",
            SmoothieError::AuthorizationError(_) => "UNAUTHORIZED",
            SmoothieError::InternalServerError(_) => "INTERNAL_ERROR",
            SmoothieError::InvalidInput(_) => "INVALID_INPUT",
            SmoothieError::IoError(_) => "IO_ERROR",
            SmoothieError::SerializationError(_) => "SERIALIZATION_ERROR",
        }
    }

    pub fn to_response(&self) -> ErrorResponseData {
        ErrorResponseData {
            success: false,
            error: self.to_string(),
            code: self.error_code().to_string(),
        }
    }
}

#[derive(Serialize)]
pub struct ErrorResponseData {
    pub success: bool,
    pub error: String,
    pub code: String,
}

pub type Result<T> = std::result::Result<T, SmoothieError>;

impl From<std::io::Error> for SmoothieError {
    fn from(err: std::io::Error) -> Self {
        SmoothieError::IoError(err.to_string())
    }
}

impl From<serde_json::Error> for SmoothieError {
    fn from(err: serde_json::Error) -> Self {
        SmoothieError::SerializationError(err.to_string())
    }
}
