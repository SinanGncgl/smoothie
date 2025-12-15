// Audit and logging service
// Provides a high-level API for logging activities, events, errors, and sessions
// Migrated to use Supabase instead of local PostgreSQL

use crate::{
  db::Database, error::Result, logging::METRICS, models::dto::*, repositories::AuditRepository,
};
use chrono::{DateTime, Utc};
use serde_json::json;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Current session state
#[allow(dead_code)]
pub struct SessionState {
  pub session_id: Option<Uuid>,
  pub user_id: Uuid,
  pub started_at: DateTime<Utc>,
}

/// Global audit service for application-wide logging
pub struct AuditService {
  current_session: Arc<RwLock<Option<SessionState>>>,
}

impl AuditService {
  pub fn new() -> Self {
    Self {
      current_session: Arc::new(RwLock::new(None)),
    }
  }

  /// Get the current session ID
  pub async fn get_current_session_id(&self) -> Option<Uuid> {
    let session = self.current_session.read().await;
    session.as_ref().and_then(|s| s.session_id)
  }

  /// Initialize a new session
  pub async fn start_session(
    &self,
    db: &Database,
    user_id: &str,
    device_info: Option<serde_json::Value>,
  ) -> Result<SessionDto> {
    let user_uuid = parse_uuid(user_id)?;
    let repo = AuditRepository::new(db.pool());

    let os_info = get_os_info();
    let app_version = get_app_version();

    let device_id = device_info
      .as_ref()
      .and_then(|d| d.get("device_id").and_then(|v| v.as_str()))
      .map(|s| s.to_string());
    let device_name = device_info
      .as_ref()
      .and_then(|d| d.get("device_name").and_then(|v| v.as_str()))
      .map(|s| s.to_string());
    let device_type = device_info
      .as_ref()
      .and_then(|d| d.get("device_type").and_then(|v| v.as_str()))
      .map(|s| s.to_string());
    let os_name = os_info
      .as_ref()
      .and_then(|o| o.get("name").and_then(|v| v.as_str()))
      .map(|s| s.to_string());
    let os_version_str = os_info
      .as_ref()
      .and_then(|o| o.get("version").and_then(|v| v.as_str()))
      .map(|s| s.to_string());

    let session = repo
      .start_session(
        user_uuid,
        device_id.as_deref(),
        device_name.as_deref(),
        device_type.as_deref(),
        os_name.as_deref(),
        os_version_str.as_deref(),
        app_version.as_deref(),
        device_info,
      )
      .await?;

    // Update internal state
    {
      let mut current = self.current_session.write().await;
      *current = Some(SessionState {
        session_id: Some(session.id),
        user_id: user_uuid,
        started_at: session.started_at,
      });
    }

    // Log system event
    repo
      .log_system_event(
        "session_started",
        "info",
        "AuditService",
        "New session started",
        Some(json!({
          "session_id": session.id.to_string(),
          "user_id": user_id,
        })),
        None,
        os_info,
        app_version.as_deref(),
      )
      .await
      .ok();

    tracing::info!(session_id = %session.id, user_id = %user_id, "Session started");

    Ok(SessionDto::from(session))
  }

  /// End the current session
  pub async fn end_session(&self, db: &Database, reason: &str) -> Result<Option<SessionDto>> {
    let session_id = self.get_current_session_id().await;

    if let Some(sid) = session_id {
      let repo = AuditRepository::new(db.pool());
      let session = repo.end_session(sid, reason).await?;

      // Clear internal state
      {
        let mut current = self.current_session.write().await;
        *current = None;
      }

      // Log system event
      repo
        .log_system_event(
          "session_ended",
          "info",
          "AuditService",
          &format!("Session ended: {}", reason),
          Some(json!({
            "session_id": sid.to_string(),
            "reason": reason,
          })),
          None,
          None,
          None,
        )
        .await
        .ok();

      tracing::info!(session_id = %sid, reason = %reason, "Session ended");

      return Ok(Some(SessionDto::from(session)));
    }

    Ok(None)
  }

  /// Log a user activity
  pub async fn log_activity(
    &self,
    db: &Database,
    user_id: &str,
    action: &str,
    entity_type: Option<&str>,
    entity_id: Option<&str>,
    entity_name: Option<&str>,
    details: Option<serde_json::Value>,
    status: &str,
    error_message: Option<&str>,
    duration_ms: Option<i32>,
  ) -> Result<ActivityLogDto> {
    let user_uuid = parse_uuid(user_id)?;
    let entity_uuid = entity_id.map(parse_uuid).transpose()?;
    let session_id = self.get_current_session_id().await;

    let repo = AuditRepository::new(db.pool());

    let log = repo
      .log_activity(
        user_uuid,
        session_id,
        action,
        entity_type,
        entity_uuid,
        entity_name,
        details,
        status,
        error_message,
        duration_ms,
      )
      .await?;

    // Update metrics
    if status == "error" {
      METRICS.record_error();
    }

    tracing::debug!(
      action = %action,
      entity_type = ?entity_type,
      status = %status,
      "Activity logged"
    );

    Ok(ActivityLogDto::from(log))
  }

  /// Log a system event
  pub async fn log_system_event(
    &self,
    db: &Database,
    event_type: &str,
    severity: &str,
    source: &str,
    message: &str,
    details: Option<serde_json::Value>,
    stack_trace: Option<&str>,
  ) -> Result<SystemEventDto> {
    let repo = AuditRepository::new(db.pool());
    let os_info = get_os_info();
    let app_version = get_app_version();

    let event = repo
      .log_system_event(
        event_type,
        severity,
        source,
        message,
        details,
        stack_trace,
        os_info,
        app_version.as_deref(),
      )
      .await?;

    if severity == "error" || severity == "critical" {
      METRICS.record_error();
    }

    match severity {
      "critical" | "error" => {
        tracing::error!(
          target: "system_events",
          event_type = %event_type,
          source = %source,
          "{}", message
        );
      }
      "warning" => {
        tracing::warn!(
          target: "system_events",
          event_type = %event_type,
          source = %source,
          "{}", message
        );
      }
      _ => {
        tracing::info!(
          target: "system_events",
          event_type = %event_type,
          source = %source,
          "{}", message
        );
      }
    }

    Ok(SystemEventDto::from(event))
  }

  /// Record a profile activation
  #[allow(clippy::too_many_arguments)]
  pub async fn record_profile_activation(
    &self,
    db: &Database,
    user_id: &str,
    profile_id: &str,
    activation_source: &str,
    previous_profile_id: Option<&str>,
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
    error_message: Option<&str>,
    metadata: Option<serde_json::Value>,
  ) -> Result<ProfileActivationDto> {
    let user_uuid = parse_uuid(user_id)?;
    let profile_uuid = parse_uuid(profile_id)?;
    let prev_profile_uuid = previous_profile_id.map(parse_uuid).transpose()?;
    let session_id = self.get_current_session_id().await;

    let repo = AuditRepository::new(db.pool());

    let activation = repo
      .record_profile_activation(
        user_uuid,
        profile_uuid,
        session_id,
        activation_source,
        prev_profile_uuid,
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
        error_message,
        metadata,
      )
      .await?;

    // Update metrics
    METRICS.record_profile_activated();

    // Also log as activity
    self
      .log_activity(
        db,
        user_id,
        "profile_activated",
        Some("profile"),
        Some(profile_id),
        None,
        Some(json!({
          "activation_source": activation_source,
          "success": success,
          "apps_launched": apps_launched,
          "monitors_applied": monitors_applied,
        })),
        if success { "success" } else { "error" },
        error_message,
        duration_ms,
      )
      .await
      .ok();

    tracing::info!(
      profile_id = %profile_id,
      activation_source = %activation_source,
      success = %success,
      "Profile activation recorded"
    );

    Ok(ProfileActivationDto::from(activation))
  }

  /// Log an error
  pub async fn log_error(
    &self,
    db: &Database,
    user_id: Option<&str>,
    error_code: Option<&str>,
    error_type: &str,
    message: &str,
    stack_trace: Option<&str>,
    context: Option<serde_json::Value>,
    source_file: Option<&str>,
    source_line: Option<i32>,
    source_function: Option<&str>,
    severity: &str,
  ) -> Result<ErrorLogDto> {
    let user_uuid = user_id.map(parse_uuid).transpose()?;
    let session_id = self.get_current_session_id().await;

    let repo = AuditRepository::new(db.pool());

    let error = repo
      .log_error(
        user_uuid,
        session_id,
        error_code,
        error_type,
        message,
        stack_trace,
        context,
        source_file,
        source_line,
        source_function,
        severity,
      )
      .await?;

    METRICS.record_error();

    tracing::error!(
      error_type = %error_type,
      error_code = ?error_code,
      severity = %severity,
      "{}", message
    );

    Ok(ErrorLogDto::from(error))
  }

  /// Record a monitor change
  pub async fn record_monitor_change(
    &self,
    db: &Database,
    user_id: Option<&str>,
    change_type: &str,
    monitors_before: Option<serde_json::Value>,
    monitors_after: Option<serde_json::Value>,
    auto_profile_activated: bool,
    activated_profile_id: Option<&str>,
  ) -> Result<MonitorChangeDto> {
    let user_uuid = user_id.map(parse_uuid).transpose()?;
    let profile_uuid = activated_profile_id.map(parse_uuid).transpose()?;
    let session_id = self.get_current_session_id().await;

    let repo = AuditRepository::new(db.pool());

    let change = repo
      .record_monitor_change(
        user_uuid,
        session_id,
        change_type,
        monitors_before,
        monitors_after,
        auto_profile_activated,
        profile_uuid,
      )
      .await?;

    tracing::info!(
      change_type = %change_type,
      auto_activated = %auto_profile_activated,
      "Monitor change recorded"
    );

    Ok(MonitorChangeDto::from(change))
  }

  /// Record an app launch
  #[allow(clippy::too_many_arguments)]
  pub async fn record_app_launch(
    &self,
    db: &Database,
    user_id: &str,
    profile_id: Option<&str>,
    activation_id: Option<&str>,
    app_id: Option<&str>,
    bundle_id: &str,
    app_name: &str,
    exe_path: Option<&str>,
    success: bool,
    error_message: Option<&str>,
    pid: Option<i32>,
    launch_duration_ms: Option<i32>,
    window_positioned: bool,
  ) -> Result<AppLaunchDto> {
    let user_uuid = parse_uuid(user_id)?;
    let profile_uuid = profile_id.map(parse_uuid).transpose()?;
    let activation_uuid = activation_id.map(parse_uuid).transpose()?;
    let app_uuid = app_id.map(parse_uuid).transpose()?;

    let repo = AuditRepository::new(db.pool());

    let launch = repo
      .record_app_launch(
        user_uuid,
        profile_uuid,
        activation_uuid,
        app_uuid,
        bundle_id,
        app_name,
        exe_path,
        success,
        error_message,
        pid,
        launch_duration_ms,
        window_positioned,
      )
      .await?;

    tracing::debug!(
      app_name = %app_name,
      bundle_id = %bundle_id,
      success = %success,
      "App launch recorded"
    );

    Ok(AppLaunchDto::from(launch))
  }

  /// Record an automation execution
  pub async fn record_automation_execution(
    &self,
    db: &Database,
    user_id: &str,
    rule_id: &str,
    profile_id: Option<&str>,
    trigger_type: &str,
    trigger_details: Option<serde_json::Value>,
    success: bool,
    error_message: Option<&str>,
    actions_taken: Option<serde_json::Value>,
    duration_ms: Option<i32>,
  ) -> Result<AutomationExecutionDto> {
    let user_uuid = parse_uuid(user_id)?;
    let rule_uuid = parse_uuid(rule_id)?;
    let profile_uuid = profile_id.map(parse_uuid).transpose()?;

    let repo = AuditRepository::new(db.pool());

    let execution = repo
      .record_automation_execution(
        rule_uuid,
        user_uuid,
        profile_uuid,
        trigger_type,
        trigger_details,
        success,
        error_message,
        actions_taken,
        duration_ms,
      )
      .await?;

    METRICS.record_automation_triggered();

    tracing::info!(
      rule_id = %rule_id,
      trigger_type = %trigger_type,
      success = %success,
      "Automation execution recorded"
    );

    Ok(AutomationExecutionDto::from(execution))
  }

  // ============================================================================
  // Query Methods
  // ============================================================================

  /// Get activity logs
  pub async fn get_activity_logs(
    &self,
    db: &Database,
    user_id: &str,
    params: LogQueryParams,
  ) -> Result<Vec<ActivityLogDto>> {
    let user_uuid = parse_uuid(user_id)?;
    let repo = AuditRepository::new(db.pool());

    let start_date = params
      .start_date
      .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
      .map(|dt| dt.with_timezone(&Utc));
    let end_date = params
      .end_date
      .and_then(|s| DateTime::parse_from_rfc3339(&s).ok())
      .map(|dt| dt.with_timezone(&Utc));

    let logs = repo
      .get_activity_logs(
        user_uuid,
        params.limit.unwrap_or(50),
        params.offset.unwrap_or(0),
        params.action.as_deref(),
        params.entity_type.as_deref(),
        start_date,
        end_date,
      )
      .await?;

    Ok(logs.into_iter().map(ActivityLogDto::from).collect())
  }

  /// Get system events
  pub async fn get_system_events(
    &self,
    db: &Database,
    params: LogQueryParams,
  ) -> Result<Vec<SystemEventDto>> {
    let repo = AuditRepository::new(db.pool());

    let events = repo
      .get_system_events(
        params.limit.unwrap_or(50),
        params.offset.unwrap_or(0),
        params.severity.as_deref(),
        params.action.as_deref(), // action used as event_type filter
      )
      .await?;

    Ok(events.into_iter().map(SystemEventDto::from).collect())
  }

  /// Get profile activations
  pub async fn get_profile_activations(
    &self,
    db: &Database,
    user_id: &str,
    params: LogQueryParams,
    profile_id: Option<&str>,
  ) -> Result<Vec<ProfileActivationDto>> {
    let user_uuid = parse_uuid(user_id)?;
    let profile_uuid = profile_id.map(parse_uuid).transpose()?;
    let repo = AuditRepository::new(db.pool());

    let activations = repo
      .get_profile_activations(
        user_uuid,
        params.limit.unwrap_or(50),
        params.offset.unwrap_or(0),
        profile_uuid,
      )
      .await?;

    Ok(
      activations
        .into_iter()
        .map(ProfileActivationDto::from)
        .collect(),
    )
  }

  /// Get error logs
  pub async fn get_error_logs(
    &self,
    db: &Database,
    params: LogQueryParams,
    include_resolved: bool,
  ) -> Result<Vec<ErrorLogDto>> {
    let repo = AuditRepository::new(db.pool());

    let errors = repo
      .get_error_logs(
        params.limit.unwrap_or(50),
        params.offset.unwrap_or(0),
        params.severity.as_deref(),
        include_resolved,
      )
      .await?;

    Ok(errors.into_iter().map(ErrorLogDto::from).collect())
  }

  /// Get session history
  pub async fn get_sessions(
    &self,
    db: &Database,
    user_id: &str,
    params: LogQueryParams,
  ) -> Result<Vec<SessionDto>> {
    let user_uuid = parse_uuid(user_id)?;
    let repo = AuditRepository::new(db.pool());

    let sessions = repo
      .get_sessions(
        user_uuid,
        params.limit.unwrap_or(50),
        params.offset.unwrap_or(0),
      )
      .await?;

    Ok(sessions.into_iter().map(SessionDto::from).collect())
  }

  /// Get dashboard statistics
  pub async fn get_dashboard_stats(
    &self,
    db: &Database,
    user_id: &str,
  ) -> Result<DashboardStatsDto> {
    let user_uuid = parse_uuid(user_id)?;
    let repo = AuditRepository::new(db.pool());

    let total_activations = repo.count_activity_logs(user_uuid).await?;
    let total_activations_today = repo.get_activations_today(user_uuid).await?;
    let total_activations_week = repo.get_activations_this_week(user_uuid).await?;
    let unresolved_errors = repo.count_unresolved_errors().await?;
    let most_used = repo.get_most_used_profile(user_uuid).await?;
    let last_activation = repo.get_last_activation(user_uuid).await?;

    let active_session = repo.get_active_session(user_uuid).await?;
    let session_duration = active_session
      .as_ref()
      .map(|s| (Utc::now() - s.started_at).num_minutes());

    // Get profile count
    let (total_profiles,): (i64,) =
      sqlx::query_as("SELECT COUNT(*) FROM profiles WHERE user_id = $1")
        .bind(user_uuid)
        .fetch_one(db.pool())
        .await
        .unwrap_or((0,));

    // Get total error count
    let (total_errors,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM error_logs")
      .fetch_one(db.pool())
      .await
      .unwrap_or((0,));

    Ok(DashboardStatsDto {
      total_profiles,
      total_activations,
      total_activations_today,
      total_activations_week,
      total_errors,
      unresolved_errors,
      active_session_id: active_session.as_ref().map(|s| s.id.to_string()),
      session_duration_seconds: session_duration,
      most_used_profile_id: most_used.as_ref().map(|(id, _, _)| id.to_string()),
      most_used_profile_name: most_used.as_ref().map(|(_, name, _)| name.clone()),
      most_used_profile_count: most_used.map(|(_, _, count)| count).unwrap_or(0),
      last_activation_at: last_activation.map(|dt| dt.to_rfc3339()),
      uptime_seconds: METRICS.get_uptime_secs(),
    })
  }

  /// Get log summary for analytics
  pub async fn get_log_summary(&self, db: &Database, user_id: &str) -> Result<LogSummaryDto> {
    let user_uuid = parse_uuid(user_id)?;
    let repo = AuditRepository::new(db.pool());

    let total_activity_logs = repo.count_activity_logs(user_uuid).await?;

    let (total_system_events,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM system_events")
      .fetch_one(db.pool())
      .await
      .unwrap_or((0,));

    let (total_profile_activations,): (i64,) =
      sqlx::query_as("SELECT COUNT(*) FROM profile_activations WHERE user_id = $1")
        .bind(user_uuid)
        .fetch_one(db.pool())
        .await
        .unwrap_or((0,));

    let (total_error_logs,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM error_logs")
      .fetch_one(db.pool())
      .await
      .unwrap_or((0,));

    let (total_sessions,): (i64,) =
      sqlx::query_as("SELECT COUNT(*) FROM sessions WHERE user_id = $1")
        .bind(user_uuid)
        .fetch_one(db.pool())
        .await
        .unwrap_or((0,));

    let actions_by_type = repo.get_actions_by_type(user_uuid).await?;
    let errors_by_severity = repo.get_errors_by_severity().await?;
    let activations_by_source = repo.get_activations_by_source(user_uuid).await?;

    Ok(LogSummaryDto {
      total_activity_logs,
      total_system_events,
      total_profile_activations,
      total_error_logs,
      total_sessions,
      actions_by_type,
      errors_by_severity,
      activations_by_source,
    })
  }

  /// Resolve an error
  pub async fn resolve_error(
    &self,
    db: &Database,
    error_id: &str,
    resolution_notes: Option<&str>,
  ) -> Result<ErrorLogDto> {
    let error_uuid = parse_uuid(error_id)?;
    let repo = AuditRepository::new(db.pool());

    let error = repo.resolve_error(error_uuid, resolution_notes).await?;

    tracing::info!(error_id = %error_id, "Error resolved");

    Ok(ErrorLogDto::from(error))
  }

  /// Cleanup old logs
  pub async fn cleanup_old_logs(&self, db: &Database, days: i64) -> Result<()> {
    let repo = AuditRepository::new(db.pool());
    repo.cleanup_old_logs(days).await?;

    tracing::info!(days = %days, "Old logs cleaned up");
    Ok(())
  }
}

impl Default for AuditService {
  fn default() -> Self {
    Self::new()
  }
}

// ============================================================================
// Helper Functions
// ============================================================================

fn parse_uuid(s: &str) -> Result<Uuid> {
  Uuid::parse_str(s)
    .map_err(|_| crate::error::SmoothieError::ValidationError(format!("Invalid UUID: {}", s)))
}

fn get_os_info() -> Option<serde_json::Value> {
  Some(json!({
    "name": std::env::consts::OS,
    "arch": std::env::consts::ARCH,
    "family": std::env::consts::FAMILY,
  }))
}

fn get_app_version() -> Option<String> {
  option_env!("CARGO_PKG_VERSION").map(|v| v.to_string())
}

// Global instance for easy access
lazy_static::lazy_static! {
  pub static ref AUDIT_SERVICE: AuditService = AuditService::new();
}
