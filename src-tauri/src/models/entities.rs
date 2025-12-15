// Database entities - match PostgreSQL schema exactly
// These are internal types used for database operations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// Profile entity - maps directly to profiles table
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ProfileEntity {
  pub id: Uuid,
  pub user_id: Uuid,
  pub name: String,
  pub description: Option<String>,
  #[sqlx(rename = "type")]
  pub profile_type: String,
  pub is_active: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  pub last_used: Option<DateTime<Utc>>,
  // New fields from v4 migration
  pub last_activated_at: Option<DateTime<Utc>>,
  pub activation_count: Option<i32>,
  pub is_favorite: Option<bool>,
  pub color: Option<String>,
  pub icon: Option<String>,
  pub sort_order: Option<i32>,
}

/// Monitor entity - maps directly to monitors table
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MonitorEntity {
  pub id: Uuid,
  pub profile_id: Uuid,
  pub name: String,
  pub resolution: String,
  pub orientation: String,
  pub is_primary: bool,
  pub x: i32,
  pub y: i32,
  pub width: i32,
  pub height: i32,
  pub display_index: i32,
  // New fields from v4 migration
  pub brand: Option<String>,
  pub model: Option<String>,
  pub refresh_rate: Option<i32>,
  pub scale_factor: Option<f64>,
  pub is_builtin: Option<bool>,
  pub color_depth: Option<i32>,
  pub created_at: Option<DateTime<Utc>>,
  pub updated_at: Option<DateTime<Utc>>,
}

/// App entity - maps directly to apps table
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AppEntity {
  pub id: Uuid,
  pub profile_id: Uuid,
  pub name: String,
  pub bundle_id: String,
  pub exe_path: Option<String>,
  pub launch_on_activate: bool,
  pub monitor_preference: Option<i32>,
  pub created_at: DateTime<Utc>,
  // New fields from v4 migration
  pub updated_at: Option<DateTime<Utc>>,
  pub icon_path: Option<String>,
  pub launch_args: Option<String>,
  pub working_directory: Option<String>,
  pub startup_delay_ms: Option<i32>,
  pub order_index: Option<i32>,
}

/// BrowserTab entity - maps directly to browser_tabs table
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct BrowserTabEntity {
  pub id: Uuid,
  pub profile_id: Uuid,
  pub url: String,
  pub browser: String,
  pub monitor_id: Option<Uuid>,
  pub tab_order: i32,
  pub favicon: Option<String>,
  pub created_at: DateTime<Utc>,
  // New fields from v4 migration
  pub updated_at: Option<DateTime<Utc>>,
}

/// AutomationRule entity - maps directly to automation_rules table
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AutomationRuleEntity {
  pub id: Uuid,
  pub profile_id: Uuid,
  pub rule_type: String,
  pub trigger_config: serde_json::Value,
  pub is_enabled: bool,
  pub created_at: DateTime<Utc>,
}

/// UserSettings entity - maps directly to user_settings table
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserSettingsEntity {
  pub id: Uuid,
  pub user_id: Uuid,
  pub theme: String,
  pub auto_restore: bool,
  pub monitor_detection: bool,
  pub animations_enabled: bool,
  pub cloud_sync: bool,
  pub auto_activate_time: String,
  pub keyboard_shortcut: String,
  pub notifications_enabled: bool,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
  // New fields from v4 migration
  pub default_profile_id: Option<Uuid>,
  pub last_active_profile_id: Option<Uuid>,
  pub onboarding_completed: Option<bool>,
  pub onboarding_step: Option<i32>,
  pub feature_flags: Option<serde_json::Value>,
  pub keyboard_shortcuts: Option<serde_json::Value>,
  pub ui_preferences: Option<serde_json::Value>,
}

// ============================================================================
// v5 Migration Entities - Comprehensive Logging and Audit Trail
// ============================================================================

/// Activity log entity - tracks all user actions
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ActivityLogEntity {
  pub id: Uuid,
  pub user_id: Uuid,
  pub session_id: Option<Uuid>,
  pub action: String,
  pub entity_type: Option<String>,
  pub entity_id: Option<Uuid>,
  pub entity_name: Option<String>,
  pub details: Option<serde_json::Value>,
  pub ip_address: Option<String>,
  pub user_agent: Option<String>,
  pub status: String,
  pub error_message: Option<String>,
  pub duration_ms: Option<i32>,
  pub created_at: DateTime<Utc>,
}

/// System event entity - tracks application lifecycle and system events
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SystemEventEntity {
  pub id: Uuid,
  pub event_type: String,
  pub severity: String,
  pub source: String,
  pub message: String,
  pub details: Option<serde_json::Value>,
  pub stack_trace: Option<String>,
  pub os_info: Option<serde_json::Value>,
  pub app_version: Option<String>,
  pub created_at: DateTime<Utc>,
}

/// Profile activation entity - detailed history of profile activations
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ProfileActivationEntity {
  pub id: Uuid,
  pub user_id: Uuid,
  pub profile_id: Uuid,
  pub session_id: Option<Uuid>,
  pub activation_source: String,
  pub previous_profile_id: Option<Uuid>,
  pub monitors_detected: Option<i32>,
  pub monitors_applied: Option<i32>,
  pub apps_detected: Option<i32>,
  pub apps_launched: Option<i32>,
  pub apps_failed: Option<i32>,
  pub tabs_detected: Option<i32>,
  pub tabs_opened: Option<i32>,
  pub windows_restored: Option<i32>,
  pub duration_ms: Option<i32>,
  pub success: bool,
  pub error_message: Option<String>,
  pub rollback_performed: Option<bool>,
  pub metadata: Option<serde_json::Value>,
  pub started_at: DateTime<Utc>,
  pub completed_at: Option<DateTime<Utc>>,
}

/// Error log entity - persistent error tracking
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct ErrorLogEntity {
  pub id: Uuid,
  pub user_id: Option<Uuid>,
  pub session_id: Option<Uuid>,
  pub error_code: Option<String>,
  pub error_type: String,
  pub message: String,
  pub stack_trace: Option<String>,
  pub context: Option<serde_json::Value>,
  pub source_file: Option<String>,
  pub source_line: Option<i32>,
  pub source_function: Option<String>,
  pub severity: String,
  pub is_resolved: Option<bool>,
  pub resolved_at: Option<DateTime<Utc>>,
  pub resolution_notes: Option<String>,
  pub occurrence_count: Option<i32>,
  pub first_occurred_at: DateTime<Utc>,
  pub last_occurred_at: DateTime<Utc>,
  pub created_at: DateTime<Utc>,
}

/// Session entity - tracks user sessions
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SessionEntity {
  pub id: Uuid,
  pub user_id: Uuid,
  pub device_id: Option<String>,
  pub device_name: Option<String>,
  pub device_type: Option<String>,
  pub os_name: Option<String>,
  pub os_version: Option<String>,
  pub app_version: Option<String>,
  pub ip_address: Option<String>,
  pub started_at: DateTime<Utc>,
  pub last_activity_at: DateTime<Utc>,
  pub ended_at: Option<DateTime<Utc>>,
  pub end_reason: Option<String>,
  pub is_active: Option<bool>,
  pub metadata: Option<serde_json::Value>,
}

/// Automation execution entity - tracks automation rule executions
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AutomationExecutionEntity {
  pub id: Uuid,
  pub rule_id: Uuid,
  pub user_id: Uuid,
  pub profile_id: Option<Uuid>,
  pub trigger_type: String,
  pub trigger_details: Option<serde_json::Value>,
  pub success: bool,
  pub error_message: Option<String>,
  pub actions_taken: Option<serde_json::Value>,
  pub duration_ms: Option<i32>,
  pub executed_at: DateTime<Utc>,
}

/// Monitor change entity - tracks monitor configuration changes
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct MonitorChangeEntity {
  pub id: Uuid,
  pub user_id: Option<Uuid>,
  pub session_id: Option<Uuid>,
  pub change_type: String,
  pub monitors_before: Option<serde_json::Value>,
  pub monitors_after: Option<serde_json::Value>,
  pub detected_at: DateTime<Utc>,
  pub auto_profile_activated: Option<bool>,
  pub activated_profile_id: Option<Uuid>,
}

/// App launch entity - tracks individual app launches
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct AppLaunchEntity {
  pub id: Uuid,
  pub user_id: Uuid,
  pub profile_id: Option<Uuid>,
  pub activation_id: Option<Uuid>,
  pub app_id: Option<Uuid>,
  pub bundle_id: String,
  pub app_name: String,
  pub exe_path: Option<String>,
  pub success: bool,
  pub error_message: Option<String>,
  pub pid: Option<i32>,
  pub launch_duration_ms: Option<i32>,
  pub window_positioned: Option<bool>,
  pub launched_at: DateTime<Utc>,
}

/// Feedback entity - user feedback and feature requests
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct FeedbackEntity {
  pub id: Uuid,
  pub user_id: Uuid,
  pub feedback_type: String,
  pub title: String,
  pub description: String,
  pub priority: Option<String>,
  pub status: Option<String>,
  pub category: Option<String>,
  pub contact_email: Option<String>,
  pub app_version: Option<String>,
  pub os_info: Option<serde_json::Value>,
  pub metadata: Option<serde_json::Value>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}

/// Subscription entity - maps directly to subscriptions table
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SubscriptionEntity {
  pub id: Uuid,
  pub user_id: Uuid,
  pub stripe_customer_id: Option<String>,
  pub stripe_subscription_id: Option<String>,
  pub tier: String,
  pub status: Option<String>,
  pub current_period_end: Option<DateTime<Utc>>,
  pub cancel_at_period_end: Option<bool>,
  pub created_at: DateTime<Utc>,
  pub updated_at: DateTime<Utc>,
}
