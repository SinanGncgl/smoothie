// Database entities - match PostgreSQL schema exactly
// These are internal types used for database operations

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use sqlx::FromRow;
use uuid::Uuid;

/// User entity - maps directly to users table
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct UserEntity {
    pub id: Uuid,
    pub email: String,
    pub password_hash: String,
    pub username: Option<String>,
    pub theme: String,
    pub notifications_enabled: bool,
    pub auto_restore: bool,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

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
}

/// Window entity - maps directly to windows table
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct WindowEntity {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub app_id: Uuid,
    pub monitor_id: Uuid,
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
    pub is_maximized: bool,
    pub state: String,
}

/// BrowserTab entity - maps directly to browser_tabs table
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct BrowserTabEntity {
    pub id: Uuid,
    pub profile_id: Uuid,
    pub url: String,
    pub browser: String,
    pub monitor_id: Uuid,
    pub tab_order: i32,
    pub favicon: Option<String>,
    pub created_at: DateTime<Utc>,
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

/// ProfileTag entity - maps directly to profile_tags table
#[derive(Debug, Clone, FromRow)]
pub struct ProfileTagEntity {
    pub profile_id: Uuid,
    pub tag: String,
}

/// SyncHistory entity - maps directly to sync_history table
#[derive(Debug, Clone, FromRow, Serialize, Deserialize)]
pub struct SyncHistoryEntity {
    pub id: Uuid,
    pub user_id: Uuid,
    pub action: String,
    pub status: String,
    pub synced_at: DateTime<Utc>,
}
