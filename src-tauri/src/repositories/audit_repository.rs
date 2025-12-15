// Comprehensive audit and logging repository
// Handles all logging operations for activity, system events, errors, sessions, etc.

use crate::error::{Result, SmoothieError};
use crate::models::entities::*;
use chrono::{DateTime, Utc};
use sqlx::PgPool;
use uuid::Uuid;

pub struct AuditRepository<'a> {
  pool: &'a PgPool,
}

impl<'a> AuditRepository<'a> {
  pub fn new(pool: &'a PgPool) -> Self {
    Self { pool }
  }

  // ============================================================================
  // Activity Logs
  // ============================================================================

  /// Log a user activity
  pub async fn log_activity(
    &self,
    user_id: Uuid,
    session_id: Option<Uuid>,
    action: &str,
    entity_type: Option<&str>,
    entity_id: Option<Uuid>,
    entity_name: Option<&str>,
    details: Option<serde_json::Value>,
    status: &str,
    error_message: Option<&str>,
    duration_ms: Option<i32>,
  ) -> Result<ActivityLogEntity> {
    let entity = sqlx::query_as::<_, ActivityLogEntity>(
      r#"
      INSERT INTO activity_logs (
        user_id, session_id, action, entity_type, entity_id, entity_name,
        details, status, error_message, duration_ms
      )
      VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
      RETURNING *
      "#,
    )
    .bind(user_id)
    .bind(session_id)
    .bind(action)
    .bind(entity_type)
    .bind(entity_id)
    .bind(entity_name)
    .bind(details)
    .bind(status)
    .bind(error_message)
    .bind(duration_ms)
    .fetch_one(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(entity)
  }

  /// Get activity logs for a user
  pub async fn get_activity_logs(
    &self,
    user_id: Uuid,
    limit: i64,
    offset: i64,
    action_filter: Option<&str>,
    entity_type_filter: Option<&str>,
    start_date: Option<DateTime<Utc>>,
    end_date: Option<DateTime<Utc>>,
  ) -> Result<Vec<ActivityLogEntity>> {
    let mut query = String::from(
      r#"
      SELECT * FROM activity_logs
      WHERE user_id = $1
      "#,
    );

    let mut param_count = 1;

    if action_filter.is_some() {
      param_count += 1;
      query.push_str(&format!(" AND action = ${}", param_count));
    }
    if entity_type_filter.is_some() {
      param_count += 1;
      query.push_str(&format!(" AND entity_type = ${}", param_count));
    }
    if start_date.is_some() {
      param_count += 1;
      query.push_str(&format!(" AND created_at >= ${}", param_count));
    }
    if end_date.is_some() {
      param_count += 1;
      query.push_str(&format!(" AND created_at <= ${}", param_count));
    }

    query.push_str(&format!(
      " ORDER BY created_at DESC LIMIT ${} OFFSET ${}",
      param_count + 1,
      param_count + 2
    ));

    let mut query_builder = sqlx::query_as::<_, ActivityLogEntity>(&query).bind(user_id);

    if let Some(action) = action_filter {
      query_builder = query_builder.bind(action);
    }
    if let Some(entity_type) = entity_type_filter {
      query_builder = query_builder.bind(entity_type);
    }
    if let Some(start) = start_date {
      query_builder = query_builder.bind(start);
    }
    if let Some(end) = end_date {
      query_builder = query_builder.bind(end);
    }

    let entities = query_builder
      .bind(limit)
      .bind(offset)
      .fetch_all(self.pool)
      .await
      .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(entities)
  }

  /// Count activity logs for a user
  pub async fn count_activity_logs(&self, user_id: Uuid) -> Result<i64> {
    let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM activity_logs WHERE user_id = $1")
      .bind(user_id)
      .fetch_one(self.pool)
      .await
      .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(count)
  }

  // ============================================================================
  // System Events
  // ============================================================================

  /// Log a system event
  pub async fn log_system_event(
    &self,
    event_type: &str,
    severity: &str,
    source: &str,
    message: &str,
    details: Option<serde_json::Value>,
    stack_trace: Option<&str>,
    os_info: Option<serde_json::Value>,
    app_version: Option<&str>,
  ) -> Result<SystemEventEntity> {
    let event_id = Uuid::new_v4();
    let entity = sqlx::query_as::<_, SystemEventEntity>(
      r#"
      INSERT INTO system_events (
        id, event_type, severity, source, message, details,
        stack_trace, os_info, app_version
      )
      VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
      RETURNING *
      "#,
    )
    .bind(event_id)
    .bind(event_type)
    .bind(severity)
    .bind(source)
    .bind(message)
    .bind(details)
    .bind(stack_trace)
    .bind(os_info)
    .bind(app_version)
    .fetch_one(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(entity)
  }

  /// Get system events
  pub async fn get_system_events(
    &self,
    limit: i64,
    offset: i64,
    severity_filter: Option<&str>,
    event_type_filter: Option<&str>,
  ) -> Result<Vec<SystemEventEntity>> {
    let entities = sqlx::query_as::<_, SystemEventEntity>(
      r#"
      SELECT * FROM system_events
      WHERE ($1::text IS NULL OR severity = $1)
        AND ($2::text IS NULL OR event_type = $2)
      ORDER BY created_at DESC
      LIMIT $3 OFFSET $4
      "#,
    )
    .bind(severity_filter)
    .bind(event_type_filter)
    .bind(limit)
    .bind(offset)
    .fetch_all(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(entities)
  }

  // ============================================================================
  // Sessions
  // ============================================================================

  /// Start a new session
  pub async fn start_session(
    &self,
    user_id: Uuid,
    device_id: Option<&str>,
    device_name: Option<&str>,
    device_type: Option<&str>,
    os_name: Option<&str>,
    os_version: Option<&str>,
    app_version: Option<&str>,
    metadata: Option<serde_json::Value>,
  ) -> Result<SessionEntity> {
    let session_id = Uuid::new_v4();
    let entity = sqlx::query_as::<_, SessionEntity>(
      r#"
      INSERT INTO sessions (
        id, user_id, device_id, device_name, device_type, os_name, os_version,
        app_version, metadata, started_at
      )
      VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, CURRENT_TIMESTAMP)
      RETURNING *
      "#,
    )
    .bind(session_id)
    .bind(user_id)
    .bind(device_id)
    .bind(device_name)
    .bind(device_type)
    .bind(os_name)
    .bind(os_version)
    .bind(app_version)
    .bind(metadata)
    .fetch_one(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(entity)
  }

  /// End a session
  pub async fn end_session(&self, session_id: Uuid, reason: &str) -> Result<SessionEntity> {
    let entity = sqlx::query_as::<_, SessionEntity>(
      r#"
      UPDATE sessions
      SET ended_at = CURRENT_TIMESTAMP, end_reason = $2
      WHERE id = $1
      RETURNING *
      "#,
    )
    .bind(session_id)
    .bind(reason)
    .fetch_one(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(entity)
  }

  /// Get active session for user
  pub async fn get_active_session(&self, user_id: Uuid) -> Result<Option<SessionEntity>> {
    let entity = sqlx::query_as::<_, SessionEntity>(
      r#"
      SELECT * FROM sessions
      WHERE user_id = $1 AND ended_at IS NULL
      ORDER BY started_at DESC
      LIMIT 1
      "#,
    )
    .bind(user_id)
    .fetch_optional(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(entity)
  }

  /// Get sessions for user
  pub async fn get_sessions(
    &self,
    user_id: Uuid,
    limit: i64,
    offset: i64,
  ) -> Result<Vec<SessionEntity>> {
    let entities = sqlx::query_as::<_, SessionEntity>(
      r#"
      SELECT * FROM sessions
      WHERE user_id = $1
      ORDER BY started_at DESC
      LIMIT $2 OFFSET $3
      "#,
    )
    .bind(user_id)
    .bind(limit)
    .bind(offset)
    .fetch_all(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(entities)
  }

  // ============================================================================
  // Profile Activations
  // ============================================================================

  /// Record a profile activation
  #[allow(clippy::too_many_arguments)]
  pub async fn record_profile_activation(
    &self,
    user_id: Uuid,
    profile_id: Uuid,
    session_id: Option<Uuid>,
    activation_source: &str,
    previous_profile_id: Option<Uuid>,
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
  ) -> Result<ProfileActivationEntity> {
    let entity = sqlx::query_as::<_, ProfileActivationEntity>(
      r#"
      INSERT INTO profile_activations (
        user_id, profile_id, session_id, activation_source, previous_profile_id,
        monitors_detected, monitors_applied, apps_detected, apps_launched, apps_failed,
        tabs_detected, tabs_opened, windows_restored, duration_ms, success,
        error_message, metadata, completed_at
      )
      VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, CURRENT_TIMESTAMP)
      RETURNING *
      "#,
    )
    .bind(user_id)
    .bind(profile_id)
    .bind(session_id)
    .bind(activation_source)
    .bind(previous_profile_id)
    .bind(monitors_detected)
    .bind(monitors_applied)
    .bind(apps_detected)
    .bind(apps_launched)
    .bind(apps_failed)
    .bind(tabs_detected)
    .bind(tabs_opened)
    .bind(windows_restored)
    .bind(duration_ms)
    .bind(success)
    .bind(error_message)
    .bind(metadata)
    .fetch_one(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    // Update profile activation count and last_activated_at
    sqlx::query(
      r#"
      UPDATE profiles
      SET activation_count = COALESCE(activation_count, 0) + 1,
          last_activated_at = CURRENT_TIMESTAMP,
          last_used = CURRENT_TIMESTAMP
      WHERE id = $1
      "#,
    )
    .bind(profile_id)
    .execute(self.pool)
    .await
    .ok();

    Ok(entity)
  }

  /// Get the active profile activation for a user
  pub async fn get_active_profile_activation(
    &self,
    user_id: Uuid,
  ) -> Result<Option<ProfileActivationEntity>> {
    let entity = sqlx::query_as::<_, ProfileActivationEntity>(
      r#"
      SELECT * FROM profile_activations
      WHERE user_id = $1 AND ended_at IS NULL
      ORDER BY started_at DESC
      LIMIT 1
      "#,
    )
    .bind(user_id)
    .fetch_optional(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(entity)
  }

  /// Get profile activations for a user
  pub async fn get_profile_activations(
    &self,
    user_id: Uuid,
    limit: i64,
    offset: i64,
    profile_id_filter: Option<Uuid>,
  ) -> Result<Vec<ProfileActivationEntity>> {
    let entities = sqlx::query_as::<_, ProfileActivationEntity>(
      r#"
      SELECT * FROM profile_activations
      WHERE user_id = $1
        AND ($2::uuid IS NULL OR profile_id = $2)
      ORDER BY started_at DESC
      LIMIT $3 OFFSET $4
      "#,
    )
    .bind(user_id)
    .bind(profile_id_filter)
    .bind(limit)
    .bind(offset)
    .fetch_all(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(entities)
  }

  /// Get activation count for today
  pub async fn get_activations_today(&self, user_id: Uuid) -> Result<i64> {
    let (count,): (i64,) = sqlx::query_as(
      r#"
      SELECT COUNT(*) FROM profile_activations
      WHERE user_id = $1 AND started_at >= CURRENT_DATE
      "#,
    )
    .bind(user_id)
    .fetch_one(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(count)
  }

  /// Get activation count for this week
  pub async fn get_activations_this_week(&self, user_id: Uuid) -> Result<i64> {
    let (count,): (i64,) = sqlx::query_as(
      r#"
      SELECT COUNT(*) FROM profile_activations
      WHERE user_id = $1 AND started_at >= NOW() - INTERVAL '7 days'
      "#,
    )
    .bind(user_id)
    .fetch_one(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(count)
  }

  // ============================================================================
  // Error Logs
  // ============================================================================

  /// Log an error
  pub async fn log_error(
    &self,
    user_id: Option<Uuid>,
    session_id: Option<Uuid>,
    error_code: Option<&str>,
    error_type: &str,
    message: &str,
    stack_trace: Option<&str>,
    context: Option<serde_json::Value>,
    source_file: Option<&str>,
    source_line: Option<i32>,
    source_function: Option<&str>,
    severity: &str,
  ) -> Result<ErrorLogEntity> {
    // Check if a similar error already exists (deduplication)
    let existing = sqlx::query_as::<_, ErrorLogEntity>(
      r#"
      SELECT * FROM error_logs
      WHERE error_type = $1 AND message = $2 AND is_resolved = false
      LIMIT 1
      "#,
    )
    .bind(error_type)
    .bind(message)
    .fetch_optional(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    if let Some(existing_error) = existing {
      // Update occurrence count
      let updated = sqlx::query_as::<_, ErrorLogEntity>(
        r#"
        UPDATE error_logs
        SET occurrence_count = occurrence_count + 1,
            last_occurred_at = CURRENT_TIMESTAMP
        WHERE id = $1
        RETURNING *
        "#,
      )
      .bind(existing_error.id)
      .fetch_one(self.pool)
      .await
      .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

      return Ok(updated);
    }

    // Create new error log
    let entity = sqlx::query_as::<_, ErrorLogEntity>(
      r#"
      INSERT INTO error_logs (
        user_id, session_id, error_code, error_type, message, stack_trace,
        context, source_file, source_line, source_function, severity
      )
      VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
      RETURNING *
      "#,
    )
    .bind(user_id)
    .bind(session_id)
    .bind(error_code)
    .bind(error_type)
    .bind(message)
    .bind(stack_trace)
    .bind(context)
    .bind(source_file)
    .bind(source_line)
    .bind(source_function)
    .bind(severity)
    .fetch_one(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(entity)
  }

  /// Get error logs
  pub async fn get_error_logs(
    &self,
    limit: i64,
    offset: i64,
    severity_filter: Option<&str>,
    include_resolved: bool,
  ) -> Result<Vec<ErrorLogEntity>> {
    let entities = sqlx::query_as::<_, ErrorLogEntity>(
      r#"
      SELECT * FROM error_logs
      WHERE ($1::text IS NULL OR severity = $1)
        AND ($2 = true OR is_resolved = false)
      ORDER BY last_occurred_at DESC
      LIMIT $3 OFFSET $4
      "#,
    )
    .bind(severity_filter)
    .bind(include_resolved)
    .bind(limit)
    .bind(offset)
    .fetch_all(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(entities)
  }

  /// Mark an error as resolved
  pub async fn resolve_error(
    &self,
    error_id: Uuid,
    resolution_notes: Option<&str>,
  ) -> Result<ErrorLogEntity> {
    let entity = sqlx::query_as::<_, ErrorLogEntity>(
      r#"
      UPDATE error_logs
      SET is_resolved = true, resolved_at = CURRENT_TIMESTAMP, resolution_notes = $2
      WHERE id = $1
      RETURNING *
      "#,
    )
    .bind(error_id)
    .bind(resolution_notes)
    .fetch_one(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(entity)
  }

  /// Count unresolved errors
  pub async fn count_unresolved_errors(&self) -> Result<i64> {
    let (count,): (i64,) =
      sqlx::query_as("SELECT COUNT(*) FROM error_logs WHERE is_resolved = false")
        .fetch_one(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(count)
  }

  // ============================================================================

  // ============================================================================
  // Automation Executions
  // ============================================================================

  /// Record an automation execution
  pub async fn record_automation_execution(
    &self,
    rule_id: Uuid,
    user_id: Uuid,
    profile_id: Option<Uuid>,
    trigger_type: &str,
    trigger_details: Option<serde_json::Value>,
    success: bool,
    error_message: Option<&str>,
    actions_taken: Option<serde_json::Value>,
    duration_ms: Option<i32>,
  ) -> Result<AutomationExecutionEntity> {
    let entity = sqlx::query_as::<_, AutomationExecutionEntity>(
      r#"
      INSERT INTO automation_executions (
        rule_id, user_id, profile_id, trigger_type, trigger_details,
        success, error_message, actions_taken, duration_ms
      )
      VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
      RETURNING *
      "#,
    )
    .bind(rule_id)
    .bind(user_id)
    .bind(profile_id)
    .bind(trigger_type)
    .bind(trigger_details)
    .bind(success)
    .bind(error_message)
    .bind(actions_taken)
    .bind(duration_ms)
    .fetch_one(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    // Update rule trigger count
    sqlx::query(
      r#"
      UPDATE automation_rules
      SET trigger_count = COALESCE(trigger_count, 0) + 1,
          last_triggered_at = CURRENT_TIMESTAMP
      WHERE id = $1
      "#,
    )
    .bind(rule_id)
    .execute(self.pool)
    .await
    .ok();

    Ok(entity)
  }

  /// Get automation executions
  pub async fn get_automation_executions(
    &self,
    user_id: Uuid,
    limit: i64,
    offset: i64,
    rule_id_filter: Option<Uuid>,
  ) -> Result<Vec<AutomationExecutionEntity>> {
    let entities = sqlx::query_as::<_, AutomationExecutionEntity>(
      r#"
      SELECT * FROM automation_executions
      WHERE user_id = $1
        AND ($2::uuid IS NULL OR rule_id = $2)
      ORDER BY executed_at DESC
      LIMIT $3 OFFSET $4
      "#,
    )
    .bind(user_id)
    .bind(rule_id_filter)
    .bind(limit)
    .bind(offset)
    .fetch_all(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(entities)
  }

  // ============================================================================
  // Monitor Changes
  // ============================================================================

  /// Record a monitor change
  pub async fn record_monitor_change(
    &self,
    user_id: Option<Uuid>,
    session_id: Option<Uuid>,
    change_type: &str,
    monitors_before: Option<serde_json::Value>,
    monitors_after: Option<serde_json::Value>,
    auto_profile_activated: bool,
    activated_profile_id: Option<Uuid>,
  ) -> Result<MonitorChangeEntity> {
    let entity = sqlx::query_as::<_, MonitorChangeEntity>(
      r#"
      INSERT INTO monitor_changes (
        user_id, session_id, change_type, monitors_before, monitors_after,
        auto_profile_activated, activated_profile_id
      )
      VALUES ($1, $2, $3, $4, $5, $6, $7)
      RETURNING *
      "#,
    )
    .bind(user_id)
    .bind(session_id)
    .bind(change_type)
    .bind(monitors_before)
    .bind(monitors_after)
    .bind(auto_profile_activated)
    .bind(activated_profile_id)
    .fetch_one(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(entity)
  }

  /// Get monitor change history
  pub async fn get_monitor_changes(
    &self,
    limit: i64,
    offset: i64,
  ) -> Result<Vec<MonitorChangeEntity>> {
    let entities = sqlx::query_as::<_, MonitorChangeEntity>(
      r#"
      SELECT * FROM monitor_changes
      ORDER BY detected_at DESC
      LIMIT $1 OFFSET $2
      "#,
    )
    .bind(limit)
    .bind(offset)
    .fetch_all(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(entities)
  }

  // ============================================================================
  // App Launches
  // ============================================================================

  /// Record an app launch
  #[allow(clippy::too_many_arguments)]
  pub async fn record_app_launch(
    &self,
    user_id: Uuid,
    profile_id: Option<Uuid>,
    activation_id: Option<Uuid>,
    app_id: Option<Uuid>,
    bundle_id: &str,
    app_name: &str,
    exe_path: Option<&str>,
    success: bool,
    error_message: Option<&str>,
    pid: Option<i32>,
    launch_duration_ms: Option<i32>,
    window_positioned: bool,
  ) -> Result<AppLaunchEntity> {
    let entity = sqlx::query_as::<_, AppLaunchEntity>(
      r#"
      INSERT INTO app_launches (
        user_id, profile_id, activation_id, app_id, bundle_id, app_name,
        exe_path, success, error_message, pid, launch_duration_ms, window_positioned
      )
      VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12)
      RETURNING *
      "#,
    )
    .bind(user_id)
    .bind(profile_id)
    .bind(activation_id)
    .bind(app_id)
    .bind(bundle_id)
    .bind(app_name)
    .bind(exe_path)
    .bind(success)
    .bind(error_message)
    .bind(pid)
    .bind(launch_duration_ms)
    .bind(window_positioned)
    .fetch_one(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(entity)
  }

  /// Get app launch history
  pub async fn get_app_launches(
    &self,
    user_id: Uuid,
    limit: i64,
    offset: i64,
    profile_id_filter: Option<Uuid>,
  ) -> Result<Vec<AppLaunchEntity>> {
    let entities = sqlx::query_as::<_, AppLaunchEntity>(
      r#"
      SELECT * FROM app_launches
      WHERE user_id = $1
        AND ($2::uuid IS NULL OR profile_id = $2)
      ORDER BY launched_at DESC
      LIMIT $3 OFFSET $4
      "#,
    )
    .bind(user_id)
    .bind(profile_id_filter)
    .bind(limit)
    .bind(offset)
    .fetch_all(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(entities)
  }

  // ============================================================================
  // Statistics
  // ============================================================================

  /// Get most used profile
  pub async fn get_most_used_profile(&self, user_id: Uuid) -> Result<Option<(Uuid, String, i64)>> {
    let result: Option<(Uuid, String, i64)> = sqlx::query_as(
      r#"
      SELECT p.id, p.name, COUNT(pa.id) as activation_count
      FROM profiles p
      JOIN profile_activations pa ON p.id = pa.profile_id
      WHERE p.user_id = $1
      GROUP BY p.id, p.name
      ORDER BY activation_count DESC
      LIMIT 1
      "#,
    )
    .bind(user_id)
    .fetch_optional(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(result)
  }

  /// Get last activation time
  pub async fn get_last_activation(&self, user_id: Uuid) -> Result<Option<DateTime<Utc>>> {
    let result: Option<(DateTime<Utc>,)> = sqlx::query_as(
      r#"
      SELECT started_at FROM profile_activations
      WHERE user_id = $1
      ORDER BY started_at DESC
      LIMIT 1
      "#,
    )
    .bind(user_id)
    .fetch_optional(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(result.map(|(dt,)| dt))
  }

  /// Get actions count by type
  pub async fn get_actions_by_type(&self, user_id: Uuid) -> Result<serde_json::Value> {
    let results: Vec<(String, i64)> = sqlx::query_as(
      r#"
      SELECT action, COUNT(*) as count
      FROM activity_logs
      WHERE user_id = $1
      GROUP BY action
      ORDER BY count DESC
      "#,
    )
    .bind(user_id)
    .fetch_all(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    let map: std::collections::HashMap<String, i64> = results.into_iter().collect();
    Ok(serde_json::to_value(map).unwrap_or_default())
  }

  /// Get errors count by severity
  pub async fn get_errors_by_severity(&self) -> Result<serde_json::Value> {
    let results: Vec<(String, i64)> = sqlx::query_as(
      r#"
      SELECT severity, COUNT(*) as count
      FROM error_logs
      WHERE is_resolved = false
      GROUP BY severity
      ORDER BY count DESC
      "#,
    )
    .fetch_all(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    let map: std::collections::HashMap<String, i64> = results.into_iter().collect();
    Ok(serde_json::to_value(map).unwrap_or_default())
  }

  /// Get activations count by source
  pub async fn get_activations_by_source(&self, user_id: Uuid) -> Result<serde_json::Value> {
    let results: Vec<(String, i64)> = sqlx::query_as(
      r#"
      SELECT activation_source, COUNT(*) as count
      FROM profile_activations
      WHERE user_id = $1
      GROUP BY activation_source
      ORDER BY count DESC
      "#,
    )
    .bind(user_id)
    .fetch_all(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    let map: std::collections::HashMap<String, i64> = results.into_iter().collect();
    Ok(serde_json::to_value(map).unwrap_or_default())
  }

  // ============================================================================
  // Cleanup
  // ============================================================================

  /// Delete old logs (retention policy)
  pub async fn cleanup_old_logs(&self, days: i64) -> Result<()> {
    let cutoff = format!("NOW() - INTERVAL '{} days'", days);

    // Clean activity logs
    sqlx::query(&format!(
      "DELETE FROM activity_logs WHERE created_at < {}",
      cutoff
    ))
    .execute(self.pool)
    .await
    .ok();

    // Clean system events
    sqlx::query(&format!(
      "DELETE FROM system_events WHERE created_at < {}",
      cutoff
    ))
    .execute(self.pool)
    .await
    .ok();

    // Clean resolved error logs
    sqlx::query(&format!(
      "DELETE FROM error_logs WHERE is_resolved = true AND resolved_at < {}",
      cutoff
    ))
    .execute(self.pool)
    .await
    .ok();

    // Clean old sessions
    sqlx::query(&format!("DELETE FROM sessions WHERE ended_at < {}", cutoff))
      .execute(self.pool)
      .await
      .ok();

    // Clean old app launches
    sqlx::query(&format!(
      "DELETE FROM app_launches WHERE launched_at < {}",
      cutoff
    ))
    .execute(self.pool)
    .await
    .ok();

    // Clean old monitor changes
    sqlx::query(&format!(
      "DELETE FROM monitor_changes WHERE detected_at < {}",
      cutoff
    ))
    .execute(self.pool)
    .await
    .ok();

    // Clean old automation executions
    sqlx::query(&format!(
      "DELETE FROM automation_executions WHERE executed_at < {}",
      cutoff
    ))
    .execute(self.pool)
    .await
    .ok();

    tracing::info!("Cleaned up logs older than {} days", days);
    Ok(())
  }
}
