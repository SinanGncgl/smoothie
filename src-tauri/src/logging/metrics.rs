// Application metrics and performance monitoring

use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Arc;
use lazy_static::lazy_static;
use chrono::Utc;

lazy_static! {
    pub static ref METRICS: Arc<AppMetrics> = Arc::new(AppMetrics::new());
}

pub struct AppMetrics {
    pub total_profiles_created: AtomicU64,
    pub total_profiles_deleted: AtomicU64,
    pub total_profiles_activated: AtomicU64,
    pub total_windows_managed: AtomicU64,
    pub total_automations_triggered: AtomicU64,
    pub total_syncs: AtomicU64,
    pub total_errors: AtomicU64,
    pub total_requests: AtomicU64,
    pub startup_time: std::time::Instant,
}

impl AppMetrics {
    pub fn new() -> Self {
        Self {
            total_profiles_created: AtomicU64::new(0),
            total_profiles_deleted: AtomicU64::new(0),
            total_profiles_activated: AtomicU64::new(0),
            total_windows_managed: AtomicU64::new(0),
            total_automations_triggered: AtomicU64::new(0),
            total_syncs: AtomicU64::new(0),
            total_errors: AtomicU64::new(0),
            total_requests: AtomicU64::new(0),
            startup_time: std::time::Instant::now(),
        }
    }

    pub fn record_profile_created(&self) {
        self.total_profiles_created.fetch_add(1, Ordering::SeqCst);
        tracing::debug!("Metric: Profile created (total: {})", self.total_profiles_created.load(Ordering::SeqCst));
    }

    pub fn record_profile_deleted(&self) {
        self.total_profiles_deleted.fetch_add(1, Ordering::SeqCst);
    }

    pub fn record_profile_activated(&self) {
        self.total_profiles_activated.fetch_add(1, Ordering::SeqCst);
    }

    pub fn record_window_managed(&self) {
        self.total_windows_managed.fetch_add(1, Ordering::SeqCst);
    }

    pub fn record_automation_triggered(&self) {
        self.total_automations_triggered.fetch_add(1, Ordering::SeqCst);
    }

    pub fn record_sync(&self) {
        self.total_syncs.fetch_add(1, Ordering::SeqCst);
    }

    pub fn record_error(&self) {
        self.total_errors.fetch_add(1, Ordering::SeqCst);
    }

    pub fn record_request(&self) {
        self.total_requests.fetch_add(1, Ordering::SeqCst);
    }

    pub fn get_uptime_secs(&self) -> u64 {
        self.startup_time.elapsed().as_secs()
    }

    pub fn get_summary(&self) -> serde_json::Value {
        serde_json::json!({
            "uptime_seconds": self.get_uptime_secs(),
            "total_profiles_created": self.total_profiles_created.load(Ordering::SeqCst),
            "total_profiles_deleted": self.total_profiles_deleted.load(Ordering::SeqCst),
            "total_profiles_activated": self.total_profiles_activated.load(Ordering::SeqCst),
            "total_windows_managed": self.total_windows_managed.load(Ordering::SeqCst),
            "total_automations_triggered": self.total_automations_triggered.load(Ordering::SeqCst),
            "total_syncs": self.total_syncs.load(Ordering::SeqCst),
            "total_errors": self.total_errors.load(Ordering::SeqCst),
            "total_requests": self.total_requests.load(Ordering::SeqCst),
            "timestamp": Utc::now().to_rfc3339()
        })
    }
}
