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

  #[error("IO error: {0}")]
  IoError(String),

  #[error("Serialization error: {0}")]
  SerializationError(String),

  #[error("System error: {0}")]
  SystemError(String),
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
