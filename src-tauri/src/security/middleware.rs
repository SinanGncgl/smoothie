// Middleware for request validation and security checks

use crate::error::{Result, SmoothieError};

pub struct RequestValidator;

impl RequestValidator {
    /// Validate user owns the resource
    pub fn validate_ownership(user_id: &str, resource_owner_id: &str) -> Result<()> {
        if user_id == resource_owner_id {
            Ok(())
        } else {
            Err(SmoothieError::AuthorizationError("Access denied: insufficient permissions".to_string()))
        }
    }

    /// Validate API key format
    pub fn validate_api_key(api_key: &str) -> Result<()> {
        if api_key.len() >= 32 && api_key.chars().all(|c| c.is_alphanumeric() || c == '-') {
            Ok(())
        } else {
            Err(SmoothieError::AuthenticationError("Invalid API key format".to_string()))
        }
    }

    /// Validate request payload size
    pub fn validate_payload_size(size: usize, max_size: usize) -> Result<()> {
        if size <= max_size {
            Ok(())
        } else {
            Err(SmoothieError::InvalidInput("Payload size exceeds maximum".to_string()))
        }
    }
}

pub struct AuditLog;

impl AuditLog {
    pub fn log_action(action: &str, user_id: &str, resource_id: &str, details: &str) {
        tracing::info!(
            action = action,
            user_id = user_id,
            resource_id = resource_id,
            details = details,
            "Audit log entry"
        );
    }

    pub fn log_security_event(event: &str, user_id: &str, severity: &str) {
        tracing::warn!(
            event = event,
            user_id = user_id,
            severity = severity,
            "Security event"
        );
    }
}
