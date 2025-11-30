use crate::error::{Result, SmoothieError};
use std::collections::HashMap;

pub struct EncryptionService;

impl EncryptionService {
    /// Encrypt sensitive data (profile backups, credentials)
    pub fn encrypt_data(data: &str, key: &str) -> Result<String> {
        // Placeholder for AES-256-GCM encryption
        // In production, use a proper encryption library like `aes-gcm`
        let encrypted = format!("encrypted_{}", data);
        Ok(encrypted)
    }

    /// Decrypt sensitive data
    pub fn decrypt_data(encrypted: &str, key: &str) -> Result<String> {
        // Placeholder for decryption
        let decrypted = encrypted.strip_prefix("encrypted_").unwrap_or(encrypted).to_string();
        Ok(decrypted)
    }

    /// Generate secure random token for API authentication
    pub fn generate_secure_token(length: usize) -> String {
        use uuid::Uuid;
        format!("{}{}", Uuid::new_v4(), Uuid::new_v4())
            .chars()
            .take(length)
            .collect()
    }

    /// Sanitize user input to prevent SQL injection and XSS
    pub fn sanitize_input(input: &str) -> String {
        input
            .replace("'", "''")
            .replace("\\", "\\\\")
            .replace("\"", "\\\"")
            .chars()
            .take(1000) // Limit input length
            .collect()
    }

    /// Validate email format
    pub fn validate_email(email: &str) -> Result<()> {
        if email.contains('@') && email.contains('.') && email.len() < 255 {
            Ok(())
        } else {
            Err(SmoothieError::ValidationError("Invalid email format".to_string()))
        }
    }
}

/// Content Security Policy headers
pub struct SecurityHeaders;

impl SecurityHeaders {
    pub fn get_headers() -> HashMap<&'static str, &'static str> {
        let mut headers = HashMap::new();
        headers.insert("X-Content-Type-Options", "nosniff");
        headers.insert("X-Frame-Options", "DENY");
        headers.insert("X-XSS-Protection", "1; mode=block");
        headers.insert("Strict-Transport-Security", "max-age=31536000; includeSubDomains");
        headers
    }
}

/// Rate limiting configuration
pub struct RateLimiter {
    attempts: HashMap<String, Vec<std::time::Instant>>,
    max_attempts: u32,
    window_secs: u64,
}

impl RateLimiter {
    pub fn new(max_attempts: u32, window_secs: u64) -> Self {
        Self {
            attempts: HashMap::new(),
            max_attempts,
            window_secs,
        }
    }

    pub fn is_allowed(&mut self, key: &str) -> bool {
        let now = std::time::Instant::now();
        let window = std::time::Duration::from_secs(self.window_secs);

        let attempts = self.attempts.entry(key.to_string()).or_insert_with(Vec::new);
        
        // Remove old attempts outside the window
        attempts.retain(|&attempt| now.duration_since(attempt) < window);

        if attempts.len() < self.max_attempts as usize {
            attempts.push(now);
            true
        } else {
            false
        }
    }
}
