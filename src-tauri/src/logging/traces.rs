// Distributed tracing and request tracking

use std::sync::atomic::{AtomicU64, Ordering};

lazy_static::lazy_static! {
    static ref REQUEST_COUNTER: AtomicU64 = AtomicU64::new(0);
}

pub struct RequestTracer;

impl RequestTracer {
    /// Generate unique request ID for tracing
    pub fn generate_request_id() -> String {
        let counter = REQUEST_COUNTER.fetch_add(1, Ordering::SeqCst);
        format!("req-{}-{}", chrono::Utc::now().timestamp_millis(), counter)
    }

    /// Trace command execution with timing
    pub async fn trace_command<F, T>(command_name: &str, f: F) -> Result<T, crate::error::SmoothieError>
    where
        F: std::future::Future<Output = Result<T, crate::error::SmoothieError>>,
    {
        let request_id = Self::generate_request_id();
        let start = std::time::Instant::now();

        tracing::info!(
            request_id = %request_id,
            command = command_name,
            "Command started"
        );

        match f.await {
            Ok(result) => {
                let elapsed = start.elapsed().as_millis();
                tracing::info!(
                    request_id = %request_id,
                    command = command_name,
                    elapsed_ms = elapsed,
                    "Command completed successfully"
                );
                Ok(result)
            }
            Err(e) => {
                let elapsed = start.elapsed().as_millis();
                tracing::error!(
                    request_id = %request_id,
                    command = command_name,
                    elapsed_ms = elapsed,
                    error = %e,
                    "Command failed"
                );
                Err(e)
            }
        }
    }

    /// Create span for nested operations
    pub fn create_span(operation: &str, parent_id: Option<&str>) -> String {
        let span_id = uuid::Uuid::new_v4().to_string();
        tracing::debug!(
            span_id = %span_id,
            operation = operation,
            parent_id = ?parent_id,
            "Span created"
        );
        span_id
    }
}
