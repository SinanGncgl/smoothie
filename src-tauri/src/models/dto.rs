// API DTOs (Data Transfer Objects) - for frontend communication
// These types are serialized to JSON and sent to the frontend

use serde::{Deserialize, Serialize};

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

#[derive(Debug, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct LogQueryParams {
  pub limit: Option<i64>,
  pub offset: Option<i64>,
  pub start_date: Option<String>,
  pub end_date: Option<String>,
  pub action: Option<String>,
  pub entity_type: Option<String>,
  pub severity: Option<String>,
}

// ============================================================================
// Response DTOs
// ============================================================================

/// Generic success response (used by handlers)
#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SuccessResponse<T: Serialize> {
  pub success: bool,
  pub data: T,
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
  pub monitor_count: i64,
  pub app_count: i64,
  pub browser_tab_count: i64,
  pub created_at: String,
  pub updated_at: String,
  pub last_used: Option<String>,
  // New fields from v4
  pub last_activated_at: Option<String>,
  pub activation_count: i32,
  pub is_favorite: bool,
  pub color: Option<String>,
  pub icon: Option<String>,
  pub sort_order: i32,
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
  // New fields from v4
  pub brand: Option<String>,
  pub model: Option<String>,
  pub refresh_rate: Option<i32>,
  pub scale_factor: Option<f64>,
  pub is_builtin: Option<bool>,
  pub color_depth: Option<i32>,
  pub created_at: Option<String>,
  pub updated_at: Option<String>,
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
  // New fields from v4
  pub updated_at: Option<String>,
  pub icon_path: Option<String>,
  pub launch_args: Option<String>,
  pub working_directory: Option<String>,
  pub startup_delay_ms: i32,
  pub order_index: i32,
}

#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct BrowserTabDto {
  pub id: String,
  pub profile_id: String,
  pub url: String,
  pub browser: String,
  pub monitor_id: Option<String>,
  pub tab_order: i32,
  pub favicon: Option<String>,
  pub created_at: String,
  pub updated_at: Option<String>,
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

/// User settings DTO - all user preferences
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserSettingsDto {
  pub id: String,
  pub user_id: String,
  pub theme: String,
  pub auto_restore: bool,
  pub monitor_detection: bool,
  pub animations_enabled: bool,
  pub cloud_sync: bool,
  pub auto_activate_time: String,
  pub keyboard_shortcut: String,
  pub notifications_enabled: bool,
  pub created_at: String,
  pub updated_at: String,
  // New fields from v4
  pub default_profile_id: Option<String>,
  pub last_active_profile_id: Option<String>,
  pub onboarding_completed: bool,
  pub onboarding_step: i32,
  pub feature_flags: Option<serde_json::Value>,
  pub keyboard_shortcuts: Option<serde_json::Value>,
  pub ui_preferences: Option<serde_json::Value>,
}

// ============================================================================
// v5 Logging Response DTOs
// ============================================================================

/// Activity log DTO - for user action tracking
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ActivityLogDto {
  pub id: String,
  pub user_id: String,
  pub session_id: Option<String>,
  pub action: String,
  pub entity_type: Option<String>,
  pub entity_id: Option<String>,
  pub entity_name: Option<String>,
  pub details: Option<serde_json::Value>,
  pub ip_address: Option<String>,
  pub user_agent: Option<String>,
  pub status: String,
  pub error_message: Option<String>,
  pub duration_ms: Option<i32>,
  pub created_at: String,
}

/// System event DTO - for application lifecycle events
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemEventDto {
  pub id: String,
  pub event_type: String,
  pub severity: String,
  pub source: String,
  pub message: String,
  pub details: Option<serde_json::Value>,
  pub stack_trace: Option<String>,
  pub os_info: Option<serde_json::Value>,
  pub app_version: Option<String>,
  pub created_at: String,
}

/// Profile activation DTO - detailed activation history
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileActivationDto {
  pub id: String,
  pub user_id: String,
  pub profile_id: String,
  pub profile_name: Option<String>,
  pub session_id: Option<String>,
  pub activation_source: String,
  pub previous_profile_id: Option<String>,
  pub previous_profile_name: Option<String>,
  pub monitors_detected: i32,
  pub monitors_applied: i32,
  pub apps_detected: i32,
  pub apps_launched: i32,
  pub apps_failed: i32,
  pub tabs_detected: i32,
  pub tabs_opened: i32,
  pub windows_restored: i32,
  pub duration_ms: Option<i32>,
  pub success: bool,
  pub error_message: Option<String>,
  pub rollback_performed: bool,
  pub metadata: Option<serde_json::Value>,
  pub started_at: String,
  pub completed_at: Option<String>,
}

/// Error log DTO - for persistent error tracking
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ErrorLogDto {
  pub id: String,
  pub user_id: Option<String>,
  pub session_id: Option<String>,
  pub error_code: Option<String>,
  pub error_type: String,
  pub message: String,
  pub stack_trace: Option<String>,
  pub context: Option<serde_json::Value>,
  pub source_file: Option<String>,
  pub source_line: Option<i32>,
  pub source_function: Option<String>,
  pub severity: String,
  pub is_resolved: bool,
  pub resolved_at: Option<String>,
  pub resolution_notes: Option<String>,
  pub occurrence_count: i32,
  pub first_occurred_at: String,
  pub last_occurred_at: String,
  pub created_at: String,
}

/// Session DTO - for user session tracking
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SessionDto {
  pub id: String,
  pub user_id: String,
  pub device_id: Option<String>,
  pub device_name: Option<String>,
  pub device_type: Option<String>,
  pub os_name: Option<String>,
  pub os_version: Option<String>,
  pub app_version: Option<String>,
  pub ip_address: Option<String>,
  pub started_at: String,
  pub last_activity_at: String,
  pub ended_at: Option<String>,
  pub end_reason: Option<String>,
  pub is_active: bool,
  pub duration_seconds: Option<i64>,
  pub metadata: Option<serde_json::Value>,
}

/// Automation execution DTO
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AutomationExecutionDto {
  pub id: String,
  pub rule_id: String,
  pub rule_name: Option<String>,
  pub user_id: String,
  pub profile_id: Option<String>,
  pub profile_name: Option<String>,
  pub trigger_type: String,
  pub trigger_details: Option<serde_json::Value>,
  pub success: bool,
  pub error_message: Option<String>,
  pub actions_taken: Option<serde_json::Value>,
  pub duration_ms: Option<i32>,
  pub executed_at: String,
}

/// Monitor change DTO
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitorChangeDto {
  pub id: String,
  pub user_id: Option<String>,
  pub session_id: Option<String>,
  pub change_type: String,
  pub monitors_before: Option<serde_json::Value>,
  pub monitors_after: Option<serde_json::Value>,
  pub detected_at: String,
  pub auto_profile_activated: bool,
  pub activated_profile_id: Option<String>,
  pub activated_profile_name: Option<String>,
}

/// App launch DTO
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AppLaunchDto {
  pub id: String,
  pub user_id: String,
  pub profile_id: Option<String>,
  pub profile_name: Option<String>,
  pub activation_id: Option<String>,
  pub app_id: Option<String>,
  pub bundle_id: String,
  pub app_name: String,
  pub exe_path: Option<String>,
  pub success: bool,
  pub error_message: Option<String>,
  pub pid: Option<i32>,
  pub launch_duration_ms: Option<i32>,
  pub window_positioned: bool,
  pub launched_at: String,
}

/// Dashboard statistics DTO
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct DashboardStatsDto {
  pub total_profiles: i64,
  pub total_activations: i64,
  pub total_activations_today: i64,
  pub total_activations_week: i64,
  pub total_errors: i64,
  pub unresolved_errors: i64,
  pub active_session_id: Option<String>,
  pub session_duration_seconds: Option<i64>,
  pub most_used_profile_id: Option<String>,
  pub most_used_profile_name: Option<String>,
  pub most_used_profile_count: i64,
  pub last_activation_at: Option<String>,
  pub uptime_seconds: u64,
}

/// Log summary for analytics
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogSummaryDto {
  pub total_activity_logs: i64,
  pub total_system_events: i64,
  pub total_profile_activations: i64,
  pub total_error_logs: i64,
  pub total_sessions: i64,
  pub actions_by_type: serde_json::Value,
  pub errors_by_severity: serde_json::Value,
  pub activations_by_source: serde_json::Value,
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
      tags: vec![],     // Tags loaded separately
      monitor_count: 0, // Counts loaded separately
      app_count: 0,
      browser_tab_count: 0,
      created_at: entity.created_at.to_rfc3339(),
      updated_at: entity.updated_at.to_rfc3339(),
      last_used: entity.last_used.map(|dt| dt.to_rfc3339()),
      last_activated_at: entity.last_activated_at.map(|dt| dt.to_rfc3339()),
      activation_count: entity.activation_count.unwrap_or(0),
      is_favorite: entity.is_favorite.unwrap_or(false),
      color: entity.color,
      icon: entity.icon,
      sort_order: entity.sort_order.unwrap_or(0),
    }
  }
}

impl ProfileDto {
  /// Create from entity with tags and counts
  pub fn from_entity_with_counts(
    entity: ProfileEntity,
    tags: Vec<String>,
    monitor_count: i64,
    app_count: i64,
    browser_tab_count: i64,
  ) -> Self {
    Self {
      id: entity.id.to_string(),
      user_id: entity.user_id.to_string(),
      name: entity.name,
      description: entity.description,
      profile_type: entity.profile_type,
      is_active: entity.is_active,
      tags,
      monitor_count,
      app_count,
      browser_tab_count,
      created_at: entity.created_at.to_rfc3339(),
      updated_at: entity.updated_at.to_rfc3339(),
      last_used: entity.last_used.map(|dt| dt.to_rfc3339()),
      last_activated_at: entity.last_activated_at.map(|dt| dt.to_rfc3339()),
      activation_count: entity.activation_count.unwrap_or(0),
      is_favorite: entity.is_favorite.unwrap_or(false),
      color: entity.color,
      icon: entity.icon,
      sort_order: entity.sort_order.unwrap_or(0),
    }
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
      brand: entity.brand,
      model: entity.model,
      refresh_rate: entity.refresh_rate,
      scale_factor: entity.scale_factor,
      is_builtin: entity.is_builtin,
      color_depth: entity.color_depth,
      created_at: entity.created_at.map(|dt| dt.to_rfc3339()),
      updated_at: entity.updated_at.map(|dt| dt.to_rfc3339()),
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
      updated_at: entity.updated_at.map(|dt| dt.to_rfc3339()),
      icon_path: entity.icon_path,
      launch_args: entity.launch_args,
      working_directory: entity.working_directory,
      startup_delay_ms: entity.startup_delay_ms.unwrap_or(0),
      order_index: entity.order_index.unwrap_or(0),
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
      monitor_id: entity.monitor_id.map(|id| id.to_string()),
      tab_order: entity.tab_order,
      favicon: entity.favicon,
      created_at: entity.created_at.to_rfc3339(),
      updated_at: entity.updated_at.map(|dt| dt.to_rfc3339()),
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

impl From<UserSettingsEntity> for UserSettingsDto {
  fn from(entity: UserSettingsEntity) -> Self {
    Self {
      id: entity.id.to_string(),
      user_id: entity.user_id.to_string(),
      theme: entity.theme,
      auto_restore: entity.auto_restore,
      monitor_detection: entity.monitor_detection,
      animations_enabled: entity.animations_enabled,
      cloud_sync: entity.cloud_sync,
      auto_activate_time: entity.auto_activate_time,
      keyboard_shortcut: entity.keyboard_shortcut,
      notifications_enabled: entity.notifications_enabled,
      created_at: entity.created_at.to_rfc3339(),
      updated_at: entity.updated_at.to_rfc3339(),
      default_profile_id: entity.default_profile_id.map(|id| id.to_string()),
      last_active_profile_id: entity.last_active_profile_id.map(|id| id.to_string()),
      onboarding_completed: entity.onboarding_completed.unwrap_or(false),
      onboarding_step: entity.onboarding_step.unwrap_or(0),
      feature_flags: entity.feature_flags,
      keyboard_shortcuts: entity.keyboard_shortcuts,
      ui_preferences: entity.ui_preferences,
    }
  }
}

// ============================================================================
// v5 Entity to DTO conversions
// ============================================================================

impl From<ActivityLogEntity> for ActivityLogDto {
  fn from(entity: ActivityLogEntity) -> Self {
    Self {
      id: entity.id.to_string(),
      user_id: entity.user_id.to_string(),
      session_id: entity.session_id.map(|id| id.to_string()),
      action: entity.action,
      entity_type: entity.entity_type,
      entity_id: entity.entity_id.map(|id| id.to_string()),
      entity_name: entity.entity_name,
      details: entity.details,
      ip_address: entity.ip_address,
      user_agent: entity.user_agent,
      status: entity.status,
      error_message: entity.error_message,
      duration_ms: entity.duration_ms,
      created_at: entity.created_at.to_rfc3339(),
    }
  }
}

impl From<SystemEventEntity> for SystemEventDto {
  fn from(entity: SystemEventEntity) -> Self {
    Self {
      id: entity.id.to_string(),
      event_type: entity.event_type,
      severity: entity.severity,
      source: entity.source,
      message: entity.message,
      details: entity.details,
      stack_trace: entity.stack_trace,
      os_info: entity.os_info,
      app_version: entity.app_version,
      created_at: entity.created_at.to_rfc3339(),
    }
  }
}

impl From<ProfileActivationEntity> for ProfileActivationDto {
  fn from(entity: ProfileActivationEntity) -> Self {
    Self {
      id: entity.id.to_string(),
      user_id: entity.user_id.to_string(),
      profile_id: entity.profile_id.to_string(),
      profile_name: None, // Set by service layer
      session_id: entity.session_id.map(|id| id.to_string()),
      activation_source: entity.activation_source,
      previous_profile_id: entity.previous_profile_id.map(|id| id.to_string()),
      previous_profile_name: None, // Set by service layer
      monitors_detected: entity.monitors_detected.unwrap_or(0),
      monitors_applied: entity.monitors_applied.unwrap_or(0),
      apps_detected: entity.apps_detected.unwrap_or(0),
      apps_launched: entity.apps_launched.unwrap_or(0),
      apps_failed: entity.apps_failed.unwrap_or(0),
      tabs_detected: entity.tabs_detected.unwrap_or(0),
      tabs_opened: entity.tabs_opened.unwrap_or(0),
      windows_restored: entity.windows_restored.unwrap_or(0),
      duration_ms: entity.duration_ms,
      success: entity.success,
      error_message: entity.error_message,
      rollback_performed: entity.rollback_performed.unwrap_or(false),
      metadata: entity.metadata,
      started_at: entity.started_at.to_rfc3339(),
      completed_at: entity.completed_at.map(|dt| dt.to_rfc3339()),
    }
  }
}

impl From<ErrorLogEntity> for ErrorLogDto {
  fn from(entity: ErrorLogEntity) -> Self {
    Self {
      id: entity.id.to_string(),
      user_id: entity.user_id.map(|id| id.to_string()),
      session_id: entity.session_id.map(|id| id.to_string()),
      error_code: entity.error_code,
      error_type: entity.error_type,
      message: entity.message,
      stack_trace: entity.stack_trace,
      context: entity.context,
      source_file: entity.source_file,
      source_line: entity.source_line,
      source_function: entity.source_function,
      severity: entity.severity,
      is_resolved: entity.is_resolved.unwrap_or(false),
      resolved_at: entity.resolved_at.map(|dt| dt.to_rfc3339()),
      resolution_notes: entity.resolution_notes,
      occurrence_count: entity.occurrence_count.unwrap_or(1),
      first_occurred_at: entity.first_occurred_at.to_rfc3339(),
      last_occurred_at: entity.last_occurred_at.to_rfc3339(),
      created_at: entity.created_at.to_rfc3339(),
    }
  }
}

impl From<SessionEntity> for SessionDto {
  fn from(entity: SessionEntity) -> Self {
    let duration_seconds = entity
      .ended_at
      .map(|ended| (ended - entity.started_at).num_seconds());

    Self {
      id: entity.id.to_string(),
      user_id: entity.user_id.to_string(),
      device_id: entity.device_id,
      device_name: entity.device_name,
      device_type: entity.device_type,
      os_name: entity.os_name,
      os_version: entity.os_version,
      app_version: entity.app_version,
      ip_address: entity.ip_address,
      started_at: entity.started_at.to_rfc3339(),
      last_activity_at: entity.last_activity_at.to_rfc3339(),
      ended_at: entity.ended_at.map(|dt| dt.to_rfc3339()),
      end_reason: entity.end_reason,
      is_active: entity.is_active.unwrap_or(false),
      duration_seconds,
      metadata: entity.metadata,
    }
  }
}

impl From<AutomationExecutionEntity> for AutomationExecutionDto {
  fn from(entity: AutomationExecutionEntity) -> Self {
    Self {
      id: entity.id.to_string(),
      rule_id: entity.rule_id.to_string(),
      rule_name: None, // Set by service layer
      user_id: entity.user_id.to_string(),
      profile_id: entity.profile_id.map(|id| id.to_string()),
      profile_name: None, // Set by service layer
      trigger_type: entity.trigger_type,
      trigger_details: entity.trigger_details,
      success: entity.success,
      error_message: entity.error_message,
      actions_taken: entity.actions_taken,
      duration_ms: entity.duration_ms,
      executed_at: entity.executed_at.to_rfc3339(),
    }
  }
}

impl From<MonitorChangeEntity> for MonitorChangeDto {
  fn from(entity: MonitorChangeEntity) -> Self {
    Self {
      id: entity.id.to_string(),
      user_id: entity.user_id.map(|id| id.to_string()),
      session_id: entity.session_id.map(|id| id.to_string()),
      change_type: entity.change_type,
      monitors_before: entity.monitors_before,
      monitors_after: entity.monitors_after,
      detected_at: entity.detected_at.to_rfc3339(),
      auto_profile_activated: entity.auto_profile_activated.unwrap_or(false),
      activated_profile_id: entity.activated_profile_id.map(|id| id.to_string()),
      activated_profile_name: None, // Set by service layer
    }
  }
}

impl From<AppLaunchEntity> for AppLaunchDto {
  fn from(entity: AppLaunchEntity) -> Self {
    Self {
      id: entity.id.to_string(),
      user_id: entity.user_id.to_string(),
      profile_id: entity.profile_id.map(|id| id.to_string()),
      profile_name: None, // Set by service layer
      activation_id: entity.activation_id.map(|id| id.to_string()),
      app_id: entity.app_id.map(|id| id.to_string()),
      bundle_id: entity.bundle_id,
      app_name: entity.app_name,
      exe_path: entity.exe_path,
      success: entity.success,
      error_message: entity.error_message,
      pid: entity.pid,
      launch_duration_ms: entity.launch_duration_ms,
      window_positioned: entity.window_positioned.unwrap_or(false),
      launched_at: entity.launched_at.to_rfc3339(),
    }
  }
}

// ============================================================================
// Feedback DTOs
// ============================================================================

/// Feedback DTO - for user feedback and feature requests
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct FeedbackDto {
  pub id: String,
  pub user_id: String,
  pub feedback_type: String,
  pub title: String,
  pub description: String,
  pub priority: String,
  pub status: String,
  pub category: Option<String>,
  pub contact_email: Option<String>,
  pub app_version: Option<String>,
  pub os_info: Option<serde_json::Value>,
  pub metadata: Option<serde_json::Value>,
  pub created_at: String,
  pub updated_at: String,
}

/// Create feedback request
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateFeedbackRequest {
  pub feedback_type: String,
  pub title: String,
  pub description: String,
  pub priority: Option<String>,
  pub category: Option<String>,
  pub contact_email: Option<String>,
}

impl From<FeedbackEntity> for FeedbackDto {
  fn from(entity: FeedbackEntity) -> Self {
    Self {
      id: entity.id.to_string(),
      user_id: entity.user_id.to_string(),
      feedback_type: entity.feedback_type,
      title: entity.title,
      description: entity.description,
      priority: entity.priority.unwrap_or_else(|| "medium".to_string()),
      status: entity.status.unwrap_or_else(|| "open".to_string()),
      category: entity.category,
      contact_email: entity.contact_email,
      app_version: entity.app_version,
      os_info: entity.os_info,
      metadata: entity.metadata,
      created_at: entity.created_at.to_rfc3339(),
      updated_at: entity.updated_at.to_rfc3339(),
    }
  }
}
