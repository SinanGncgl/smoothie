// API DTOs (Data Transfer Objects) - for frontend communication
// These types are serialized to JSON and sent to the frontend

use serde::{Deserialize, Serialize};
use uuid::Uuid;

// ============================================================================
// Request DTOs
// ============================================================================

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateProfileRequest {
    pub name: String,
    pub description: Option<String>,
    pub profile_type: String,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateProfileRequest {
    pub name: Option<String>,
    pub description: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateMonitorRequest {
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

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAppRequest {
    pub profile_id: Uuid,
    pub name: String,
    pub bundle_id: String,
    pub exe_path: Option<String>,
    pub launch_on_activate: bool,
    pub monitor_preference: Option<i32>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateBrowserTabRequest {
    pub profile_id: Uuid,
    pub url: String,
    pub browser: String,
    pub monitor_id: Uuid,
    pub tab_order: i32,
    pub favicon: Option<String>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateAutomationRuleRequest {
    pub profile_id: Uuid,
    pub rule_type: String,
    pub trigger_config: serde_json::Value,
}

// ============================================================================
// Response DTOs
// ============================================================================

/// Generic API response wrapper
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiResponse<T: Serialize> {
    pub success: bool,
    pub data: T,
}

impl<T: Serialize> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self { success: true, data }
    }
}

/// Generic success response (used by handlers)
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponse<T: Serialize> {
    pub success: bool,
    pub data: T,
}

impl<T: Serialize> SuccessResponse<T> {
    pub fn new(data: T) -> Self {
        Self { success: true, data }
    }
}

/// Profile response with related data
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileDto {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: Option<String>,
    pub profile_type: String,
    pub is_active: bool,
    pub tags: Vec<String>,
    pub created_at: String,
    pub updated_at: String,
    pub last_used: Option<String>,
}

/// Full profile with all related entities
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileDetailDto {
    pub id: String,
    pub user_id: String,
    pub name: String,
    pub description: Option<String>,
    pub profile_type: String,
    pub is_active: bool,
    pub tags: Vec<String>,
    pub monitors: Vec<MonitorDto>,
    pub apps: Vec<AppDto>,
    pub browser_tabs: Vec<BrowserTabDto>,
    pub created_at: String,
    pub updated_at: String,
    pub last_used: Option<String>,
}

/// ProfileResponse is an alias for ProfileDetailDto (for backward compatibility)
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileResponse {
    pub id: String,
    pub name: String,
    pub description: Option<String>,
    pub profile_type: String,
    pub is_active: bool,
    pub tags: Vec<String>,
    pub monitors: Vec<MonitorDto>,
    pub apps: Vec<AppDto>,
    pub browser_tabs: Vec<BrowserTabDto>,
    pub created_at: String,
    pub last_used: Option<String>,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitorDto {
    pub id: String,
    pub profile_id: String,
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

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppDto {
    pub id: String,
    pub profile_id: String,
    pub name: String,
    pub bundle_id: String,
    pub exe_path: Option<String>,
    pub launch_on_activate: bool,
    pub monitor_preference: Option<i32>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserTabDto {
    pub id: String,
    pub profile_id: String,
    pub url: String,
    pub browser: String,
    pub monitor_id: String,
    pub tab_order: i32,
    pub favicon: Option<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutomationRuleDto {
    pub id: String,
    pub profile_id: String,
    pub rule_type: String,
    pub trigger_config: serde_json::Value,
    pub is_enabled: bool,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserPreferencesDto {
    pub id: String,
    pub email: String,
    pub username: Option<String>,
    pub theme: String,
    pub notifications_enabled: bool,
    pub auto_restore: bool,
}

// ============================================================================
// Entity to DTO conversions
// ============================================================================

use super::entities::*;

impl From<ProfileEntity> for ProfileDto {
    fn from(entity: ProfileEntity) -> Self {
        Self {
            id: entity.id.to_string(),
            user_id: entity.user_id.to_string(),
            name: entity.name,
            description: entity.description,
            profile_type: entity.profile_type,
            is_active: entity.is_active,
            tags: vec![], // Tags loaded separately
            created_at: entity.created_at.to_rfc3339(),
            updated_at: entity.updated_at.to_rfc3339(),
            last_used: entity.last_used.map(|dt| dt.to_rfc3339()),
        }
    }
}

impl ProfileDto {
    /// Create from entity with tags
    pub fn from_entity(entity: ProfileEntity, tags: Vec<String>) -> Self {
        Self {
            id: entity.id.to_string(),
            user_id: entity.user_id.to_string(),
            name: entity.name,
            description: entity.description,
            profile_type: entity.profile_type,
            is_active: entity.is_active,
            tags,
            created_at: entity.created_at.to_rfc3339(),
            updated_at: entity.updated_at.to_rfc3339(),
            last_used: entity.last_used.map(|dt| dt.to_rfc3339()),
        }
    }

    pub fn with_tags(mut self, tags: Vec<String>) -> Self {
        self.tags = tags;
        self
    }
}

impl From<MonitorEntity> for MonitorDto {
    fn from(entity: MonitorEntity) -> Self {
        Self {
            id: entity.id.to_string(),
            profile_id: entity.profile_id.to_string(),
            name: entity.name,
            resolution: entity.resolution,
            orientation: entity.orientation,
            is_primary: entity.is_primary,
            x: entity.x,
            y: entity.y,
            width: entity.width,
            height: entity.height,
            display_index: entity.display_index,
        }
    }
}

impl From<AppEntity> for AppDto {
    fn from(entity: AppEntity) -> Self {
        Self {
            id: entity.id.to_string(),
            profile_id: entity.profile_id.to_string(),
            name: entity.name,
            bundle_id: entity.bundle_id,
            exe_path: entity.exe_path,
            launch_on_activate: entity.launch_on_activate,
            monitor_preference: entity.monitor_preference,
            created_at: entity.created_at.to_rfc3339(),
        }
    }
}

impl From<BrowserTabEntity> for BrowserTabDto {
    fn from(entity: BrowserTabEntity) -> Self {
        Self {
            id: entity.id.to_string(),
            profile_id: entity.profile_id.to_string(),
            url: entity.url,
            browser: entity.browser,
            monitor_id: entity.monitor_id.to_string(),
            tab_order: entity.tab_order,
            favicon: entity.favicon,
            created_at: entity.created_at.to_rfc3339(),
        }
    }
}

impl From<AutomationRuleEntity> for AutomationRuleDto {
    fn from(entity: AutomationRuleEntity) -> Self {
        Self {
            id: entity.id.to_string(),
            profile_id: entity.profile_id.to_string(),
            rule_type: entity.rule_type,
            trigger_config: entity.trigger_config,
            is_enabled: entity.is_enabled,
            created_at: entity.created_at.to_rfc3339(),
        }
    }
}

impl From<UserEntity> for UserPreferencesDto {
    fn from(entity: UserEntity) -> Self {
        Self {
            id: entity.id.to_string(),
            email: entity.email,
            username: entity.username,
            theme: entity.theme,
            notifications_enabled: entity.notifications_enabled,
            auto_restore: entity.auto_restore,
        }
    }
}
