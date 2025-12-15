// Audit and logging handlers - Tauri commands for log management

use crate::{db::Database, error::Result, models::dto::*, services::AUDIT_SERVICE};
use tauri::State;

const DEFAULT_USER_ID: &str = "00000000-0000-0000-0000-000000000001";

// ============================================================================
// Session Management
// ============================================================================

/// Start a new session
#[tauri::command]
pub async fn start_session(
  db: State<'_, Database>,
  device_info: Option<serde_json::Value>,
) -> Result<SessionDto> {
  AUDIT_SERVICE
    .start_session(&db, DEFAULT_USER_ID, device_info)
    .await
}

/// End the current session
#[tauri::command]
pub async fn end_session(db: State<'_, Database>, reason: String) -> Result<Option<SessionDto>> {
  AUDIT_SERVICE.end_session(&db, &reason).await
}

/// Get session history
#[tauri::command]
pub async fn get_sessions(
  db: State<'_, Database>,
  limit: Option<i64>,
  offset: Option<i64>,
) -> Result<Vec<SessionDto>> {
  let params = LogQueryParams {
    limit,
    offset,
    start_date: None,
    end_date: None,
    action: None,
    entity_type: None,
    severity: None,
  };
  AUDIT_SERVICE
    .get_sessions(&db, DEFAULT_USER_ID, params)
    .await
}

// ============================================================================
// Activity Logs
// ============================================================================

/// Log a user activity
#[tauri::command]
pub async fn log_activity(
  db: State<'_, Database>,
  action: String,
  entity_type: Option<String>,
  entity_id: Option<String>,
  entity_name: Option<String>,
  details: Option<serde_json::Value>,
  status: Option<String>,
  error_message: Option<String>,
  duration_ms: Option<i32>,
) -> Result<ActivityLogDto> {
  AUDIT_SERVICE
    .log_activity(
      &db,
      DEFAULT_USER_ID,
      &action,
      entity_type.as_deref(),
      entity_id.as_deref(),
      entity_name.as_deref(),
      details,
      &status.unwrap_or_else(|| "success".to_string()),
      error_message.as_deref(),
      duration_ms,
    )
    .await
}

/// Get activity logs
#[tauri::command]
pub async fn get_activity_logs(
  db: State<'_, Database>,
  limit: Option<i64>,
  offset: Option<i64>,
  action: Option<String>,
  entity_type: Option<String>,
  start_date: Option<String>,
  end_date: Option<String>,
) -> Result<Vec<ActivityLogDto>> {
  let params = LogQueryParams {
    limit,
    offset,
    start_date,
    end_date,
    action,
    entity_type,
    severity: None,
  };
  AUDIT_SERVICE
    .get_activity_logs(&db, DEFAULT_USER_ID, params)
    .await
}

// ============================================================================
// System Events
// ============================================================================

/// Log a system event
#[tauri::command]
pub async fn log_system_event(
  db: State<'_, Database>,
  event_type: String,
  severity: Option<String>,
  source: String,
  message: String,
  details: Option<serde_json::Value>,
  stack_trace: Option<String>,
) -> Result<SystemEventDto> {
  AUDIT_SERVICE
    .log_system_event(
      &db,
      &event_type,
      &severity.unwrap_or_else(|| "info".to_string()),
      &source,
      &message,
      details,
      stack_trace.as_deref(),
    )
    .await
}

/// Get system events
#[tauri::command]
pub async fn get_system_events(
  db: State<'_, Database>,
  limit: Option<i64>,
  offset: Option<i64>,
  severity: Option<String>,
  event_type: Option<String>,
) -> Result<Vec<SystemEventDto>> {
  let params = LogQueryParams {
    limit,
    offset,
    start_date: None,
    end_date: None,
    action: event_type, // Used as event_type filter
    entity_type: None,
    severity,
  };
  AUDIT_SERVICE.get_system_events(&db, params).await
}

// ============================================================================
// Profile Activations
// ============================================================================

/// Record a profile activation
#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub async fn record_profile_activation(
  db: State<'_, Database>,
  profile_id: String,
  activation_source: String,
  previous_profile_id: Option<String>,
  monitors_detected: Option<i32>,
  monitors_applied: Option<i32>,
  apps_detected: Option<i32>,
  apps_launched: Option<i32>,
  apps_failed: Option<i32>,
  tabs_detected: Option<i32>,
  tabs_opened: Option<i32>,
  windows_restored: Option<i32>,
  duration_ms: Option<i32>,
  success: bool,
  error_message: Option<String>,
  metadata: Option<serde_json::Value>,
) -> Result<ProfileActivationDto> {
  AUDIT_SERVICE
    .record_profile_activation(
      &db,
      DEFAULT_USER_ID,
      &profile_id,
      &activation_source,
      previous_profile_id.as_deref(),
      monitors_detected,
      monitors_applied,
      apps_detected,
      apps_launched,
      apps_failed,
      tabs_detected,
      tabs_opened,
      windows_restored,
      duration_ms,
      success,
      error_message.as_deref(),
      metadata,
    )
    .await
}

/// Get profile activations
#[tauri::command]
pub async fn get_profile_activations(
  db: State<'_, Database>,
  limit: Option<i64>,
  offset: Option<i64>,
  profile_id: Option<String>,
) -> Result<Vec<ProfileActivationDto>> {
  let params = LogQueryParams {
    limit,
    offset,
    start_date: None,
    end_date: None,
    action: None,
    entity_type: None,
    severity: None,
  };
  AUDIT_SERVICE
    .get_profile_activations(&db, DEFAULT_USER_ID, params, profile_id.as_deref())
    .await
}

// ============================================================================
// Error Logs
// ============================================================================

/// Log an error
#[tauri::command]
pub async fn log_error(
  db: State<'_, Database>,
  error_code: Option<String>,
  error_type: String,
  message: String,
  stack_trace: Option<String>,
  context: Option<serde_json::Value>,
  source_file: Option<String>,
  source_line: Option<i32>,
  source_function: Option<String>,
  severity: Option<String>,
) -> Result<ErrorLogDto> {
  AUDIT_SERVICE
    .log_error(
      &db,
      Some(DEFAULT_USER_ID),
      error_code.as_deref(),
      &error_type,
      &message,
      stack_trace.as_deref(),
      context,
      source_file.as_deref(),
      source_line,
      source_function.as_deref(),
      &severity.unwrap_or_else(|| "error".to_string()),
    )
    .await
}

/// Get error logs
#[tauri::command]
pub async fn get_error_logs(
  db: State<'_, Database>,
  limit: Option<i64>,
  offset: Option<i64>,
  severity: Option<String>,
  include_resolved: Option<bool>,
) -> Result<Vec<ErrorLogDto>> {
  let params = LogQueryParams {
    limit,
    offset,
    start_date: None,
    end_date: None,
    action: None,
    entity_type: None,
    severity,
  };
  AUDIT_SERVICE
    .get_error_logs(&db, params, include_resolved.unwrap_or(false))
    .await
}

/// Resolve an error
#[tauri::command]
pub async fn resolve_error(
  db: State<'_, Database>,
  error_id: String,
  resolution_notes: Option<String>,
) -> Result<ErrorLogDto> {
  AUDIT_SERVICE
    .resolve_error(&db, &error_id, resolution_notes.as_deref())
    .await
}

// ============================================================================
// Monitor Changes
// ============================================================================

/// Record a monitor change
#[tauri::command]
pub async fn record_monitor_change(
  db: State<'_, Database>,
  change_type: String,
  monitors_before: Option<serde_json::Value>,
  monitors_after: Option<serde_json::Value>,
  auto_profile_activated: Option<bool>,
  activated_profile_id: Option<String>,
) -> Result<MonitorChangeDto> {
  AUDIT_SERVICE
    .record_monitor_change(
      &db,
      Some(DEFAULT_USER_ID),
      &change_type,
      monitors_before,
      monitors_after,
      auto_profile_activated.unwrap_or(false),
      activated_profile_id.as_deref(),
    )
    .await
}

// ============================================================================
// App Launches
// ============================================================================

/// Record an app launch
#[tauri::command]
#[allow(clippy::too_many_arguments)]
pub async fn record_app_launch(
  db: State<'_, Database>,
  profile_id: Option<String>,
  activation_id: Option<String>,
  app_id: Option<String>,
  bundle_id: String,
  app_name: String,
  exe_path: Option<String>,
  success: bool,
  error_message: Option<String>,
  pid: Option<i32>,
  launch_duration_ms: Option<i32>,
  window_positioned: Option<bool>,
) -> Result<AppLaunchDto> {
  AUDIT_SERVICE
    .record_app_launch(
      &db,
      DEFAULT_USER_ID,
      profile_id.as_deref(),
      activation_id.as_deref(),
      app_id.as_deref(),
      &bundle_id,
      &app_name,
      exe_path.as_deref(),
      success,
      error_message.as_deref(),
      pid,
      launch_duration_ms,
      window_positioned.unwrap_or(false),
    )
    .await
}

// ============================================================================
// Automation Executions
// ============================================================================

/// Record an automation execution
#[tauri::command]
pub async fn record_automation_execution(
  db: State<'_, Database>,
  rule_id: String,
  profile_id: Option<String>,
  trigger_type: String,
  trigger_details: Option<serde_json::Value>,
  success: bool,
  error_message: Option<String>,
  actions_taken: Option<serde_json::Value>,
  duration_ms: Option<i32>,
) -> Result<AutomationExecutionDto> {
  AUDIT_SERVICE
    .record_automation_execution(
      &db,
      DEFAULT_USER_ID,
      &rule_id,
      profile_id.as_deref(),
      &trigger_type,
      trigger_details,
      success,
      error_message.as_deref(),
      actions_taken,
      duration_ms,
    )
    .await
}

// ============================================================================
// Dashboard & Analytics
// ============================================================================

/// Get dashboard statistics
#[tauri::command]
pub async fn get_dashboard_stats(db: State<'_, Database>) -> Result<DashboardStatsDto> {
  AUDIT_SERVICE
    .get_dashboard_stats(&db, DEFAULT_USER_ID)
    .await
}

/// Get log summary for analytics
#[tauri::command]
pub async fn get_log_summary(db: State<'_, Database>) -> Result<LogSummaryDto> {
  AUDIT_SERVICE.get_log_summary(&db, DEFAULT_USER_ID).await
}

/// Get application metrics
#[tauri::command]
pub async fn get_app_metrics() -> Result<serde_json::Value> {
  Ok(crate::logging::METRICS.get_summary())
}

// ============================================================================
// Maintenance
// ============================================================================

/// Cleanup old logs (retention policy)
#[tauri::command]
pub async fn cleanup_old_logs(db: State<'_, Database>, days: Option<i64>) -> Result<()> {
  AUDIT_SERVICE
    .cleanup_old_logs(&db, days.unwrap_or(30))
    .await
}

/// Get monitor change history
#[tauri::command]
pub async fn get_monitor_changes(
  db: State<'_, Database>,
  limit: Option<i64>,
  offset: Option<i64>,
) -> Result<Vec<MonitorChangeDto>> {
  use crate::repositories::AuditRepository;
  let repo = AuditRepository::new(db.pool());
  let changes = repo
    .get_monitor_changes(limit.unwrap_or(50), offset.unwrap_or(0))
    .await?;
  Ok(changes.into_iter().map(MonitorChangeDto::from).collect())
}

/// Get app launch history
#[tauri::command]
pub async fn get_app_launches(
  db: State<'_, Database>,
  profile_id: Option<String>,
  limit: Option<i64>,
  offset: Option<i64>,
) -> Result<Vec<AppLaunchDto>> {
  use crate::repositories::AuditRepository;
  use uuid::Uuid;

  let user_uuid = Uuid::parse_str(DEFAULT_USER_ID).unwrap();
  let profile_uuid = profile_id
    .map(|id| Uuid::parse_str(&id))
    .transpose()
    .map_err(|_| crate::error::SmoothieError::ValidationError("Invalid profile ID".into()))?;

  let repo = AuditRepository::new(db.pool());
  let launches = repo
    .get_app_launches(
      user_uuid,
      limit.unwrap_or(50),
      offset.unwrap_or(0),
      profile_uuid,
    )
    .await?;
  Ok(launches.into_iter().map(AppLaunchDto::from).collect())
}

/// Get automation execution history
#[tauri::command]
pub async fn get_automation_executions(
  db: State<'_, Database>,
  rule_id: Option<String>,
  limit: Option<i64>,
  offset: Option<i64>,
) -> Result<Vec<AutomationExecutionDto>> {
  use crate::repositories::AuditRepository;
  use uuid::Uuid;

  let user_uuid = Uuid::parse_str(DEFAULT_USER_ID).unwrap();
  let rule_uuid = rule_id
    .map(|id| Uuid::parse_str(&id))
    .transpose()
    .map_err(|_| crate::error::SmoothieError::ValidationError("Invalid rule ID".into()))?;

  let repo = AuditRepository::new(db.pool());
  let executions = repo
    .get_automation_executions(
      user_uuid,
      limit.unwrap_or(50),
      offset.unwrap_or(0),
      rule_uuid,
    )
    .await?;
  Ok(
    executions
      .into_iter()
      .map(AutomationExecutionDto::from)
      .collect(),
  )
}
