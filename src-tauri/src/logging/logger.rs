// Structured logging with tracing

use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;
use std::path::PathBuf;
use std::sync::Mutex;
use lazy_static::lazy_static;

lazy_static! {
    static ref LOG_FILE: Mutex<Option<std::fs::File>> = {
        let log_path = SmoothieLogger::get_log_path();
        if let Some(parent) = log_path.parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        
        match OpenOptions::new()
            .create(true)
            .append(true)
            .open(&log_path)
        {
            Ok(file) => Mutex::new(Some(file)),
            Err(_) => Mutex::new(None),
        }
    };
}

pub struct SmoothieLogger;

impl SmoothieLogger {
    /// Initialize logging system with file and console output
    pub fn init() {
        tracing_subscriber::fmt()
            .with_env_filter(
                tracing_subscriber::EnvFilter::from_default_env()
                    .add_directive(tracing_subscriber::filter::LevelFilter::INFO.into())
            )
            .with_file(true)
            .with_line_number(true)
            .with_thread_ids(true)
            .with_target(true)
            .init();

        tracing::info!("Smoothie logging initialized");
    }

    /// Get log file path in app data directory
    fn get_log_path() -> PathBuf {
        let app_data_dir = if cfg!(target_os = "macos") {
            dirs::home_dir().unwrap().join("Library/Application Support/Smoothie/logs")
        } else if cfg!(target_os = "windows") {
            dirs::data_dir().unwrap().join("Smoothie/logs")
        } else {
            dirs::data_dir().unwrap().join("smoothie/logs")
        };

        app_data_dir.join("smoothie.log")
    }

    /// Write log entry to file
    pub fn log_to_file(level: &str, message: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S%.3f");
        let log_entry = format!("[{}] {}: {}\n", timestamp, level, message);

        if let Ok(mut file) = LOG_FILE.lock() {
            if let Some(ref mut f) = *file {
                let _ = f.write_all(log_entry.as_bytes());
            }
        }
    }

    /// Get recent logs for debugging
    pub fn get_recent_logs(lines: usize) -> Vec<String> {
        let log_path = Self::get_log_path();
        
        match std::fs::read_to_string(&log_path) {
            Ok(content) => {
                content
                    .lines()
                    .rev()
                    .take(lines)
                    .map(|s| s.to_string())
                    .collect()
            }
            Err(_) => vec![],
        }
    }

    /// Clear old log files (older than N days)
    pub fn cleanup_old_logs(days: u64) {
        let log_dir = if cfg!(target_os = "macos") {
            dirs::home_dir().unwrap().join("Library/Application Support/Smoothie/logs")
        } else if cfg!(target_os = "windows") {
            dirs::data_dir().unwrap().join("Smoothie/logs")
        } else {
            dirs::data_dir().unwrap().join("smoothie/logs")
        };

        if let Ok(entries) = std::fs::read_dir(&log_dir) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if let Ok(modified) = metadata.modified() {
                        if let Ok(duration) = std::time::SystemTime::now().duration_since(modified) {
                            if duration.as_secs() > days * 86400 {
                                let _ = std::fs::remove_file(entry.path());
                            }
                        }
                    }
                }
            }
        }

        tracing::info!("Log cleanup completed for logs older than {} days", days);
    }
}

/// Macro for structured logging
#[macro_export]
macro_rules! log_action {
    ($action:expr, $user_id:expr, $resource:expr) => {
        tracing::info!(
            action = $action,
            user_id = $user_id,
            resource = $resource,
            timestamp = chrono::Local::now().to_rfc3339(),
            "User action logged"
        );
    };
}

#[macro_export]
macro_rules! log_error {
    ($context:expr, $error:expr) => {
        tracing::error!(
            context = $context,
            error = %$error,
            timestamp = chrono::Local::now().to_rfc3339(),
            "Error occurred"
        );
    };
}
