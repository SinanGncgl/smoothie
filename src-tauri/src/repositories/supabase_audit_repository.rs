// Supabase-based audit repository
// Migrated from local PostgreSQL to Supabase REST API

use crate::db::supabase::SupabaseClient;
use crate::error::{Result, SmoothieError};
use crate::models::entities::*;
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Clone)]
pub struct SupabaseAuditRepository {
    client: SupabaseClient,
}

impl SupabaseAuditRepository {
    pub fn new(client: SupabaseClient) -> Self {
        Self { client }
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
        let event_data = serde_json::json!({
            "user_id": user_id,
            "session_id": session_id,
            "action": action,
            "entity_type": entity_type,
            "entity_id": entity_id,
            "entity_name": entity_name,
            "details": details,
            "status": status,
            "error_message": error_message,
            "duration_ms": duration_ms,
            "event_type": "activity",
            "created_at": Utc::now().to_rfc3339()
        });

        let result: AuditEventEntity = self.client
            .post("audit_events", &event_data)
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        // Convert audit event to activity log format for compatibility
        Ok(ActivityLogEntity {
            id: result.id,
            user_id: result.user_id,
            session_id: result.session_id,
            action: result.action,
            entity_type: result.entity_type,
            entity_id: result.entity_id,
            entity_name: result.entity_name,
            details: result.details,
            status: result.status.unwrap_or_else(|| "unknown".to_string()),
            error_message: result.error_message,
            duration_ms: result.duration_ms,
            created_at: result.created_at,
        })
    }

    /// Get activity logs for a user
    pub async fn get_activity_logs(
        &self,
        user_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<ActivityLogEntity>> {
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);
        let query = format!("user_id=eq.{}&event_type=eq.activity&order=created_at.desc&limit={}&offset={}", user_id, limit, offset);

        let events: Vec<AuditEventEntity> = self.client
            .get("audit_events", Some(&query))
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        // Convert audit events to activity logs
        let activity_logs = events.into_iter().map(|event| ActivityLogEntity {
            id: event.id,
            user_id: event.user_id,
            session_id: event.session_id,
            action: event.action,
            entity_type: event.entity_type,
            entity_id: event.entity_id,
            entity_name: event.entity_name,
            details: event.details,
            status: event.status.unwrap_or_else(|| "unknown".to_string()),
            error_message: event.error_message,
            duration_ms: event.duration_ms,
            created_at: event.created_at,
        }).collect();

        Ok(activity_logs)
    }

    /// Count activity logs for a user
    pub async fn count_activity_logs(&self, user_id: Uuid) -> Result<i64> {
        let query = format!("user_id=eq.{}&event_type=eq.activity", user_id);
        let events: Vec<AuditEventEntity> = self.client
            .get("audit_events", Some(&query))
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        Ok(events.len() as i64)
    }

    // ============================================================================
    // System Events
    // ============================================================================

    /// Log a system event
    pub async fn log_system_event(
        &self,
        user_id: Option<Uuid>,
        event_type: &str,
        severity: &str,
        component: &str,
        message: &str,
        details: Option<serde_json::Value>,
        error_code: Option<&str>,
    ) -> Result<SystemEventEntity> {
        let event_data = serde_json::json!({
            "user_id": user_id,
            "action": event_type,
            "entity_type": "system",
            "details": details,
            "status": severity,
            "error_message": error_code,
            "event_type": "system",
            "created_at": Utc::now().to_rfc3339()
        });

        let result: AuditEventEntity = self.client
            .post("audit_events", &event_data)
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        // Convert to system event format
        Ok(SystemEventEntity {
            id: result.id,
            user_id: result.user_id,
            event_type: result.action,
            severity: result.status.unwrap_or_else(|| "info".to_string()),
            component: component.to_string(),
            message: message.to_string(),
            details: result.details,
            error_code: result.error_message,
            created_at: result.created_at,
        })
    }

    /// Get system events
    pub async fn get_system_events(
        &self,
        limit: Option<i64>,
        offset: Option<i64>,
        severity: Option<&str>,
    ) -> Result<Vec<SystemEventEntity>> {
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);
        let mut query = format!("event_type=eq.system&order=created_at.desc&limit={}&offset={}", limit, offset);

        if let Some(sev) = severity {
            query.push_str(&format!("&status=eq.{}", sev));
        }

        let events: Vec<AuditEventEntity> = self.client
            .get("audit_events", Some(&query))
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        // Convert to system events
        let system_events = events.into_iter().map(|event| SystemEventEntity {
            id: event.id,
            user_id: event.user_id,
            event_type: event.action,
            severity: event.status.unwrap_or_else(|| "info".to_string()),
            component: "system".to_string(), // Default component
            message: event.entity_name.unwrap_or_else(|| "System event".to_string()),
            details: event.details,
            error_code: event.error_message,
            created_at: event.created_at,
        }).collect();

        Ok(system_events)
    }

    // ============================================================================
    // Sessions
    // ============================================================================

    /// Start a new session
    pub async fn start_session(
        &self,
        user_id: Uuid,
        device_info: Option<serde_json::Value>,
    ) -> Result<SessionEntity> {
        let session_data = serde_json::json!({
            "user_id": user_id,
            "device_info": device_info,
            "is_active": true,
            "started_at": Utc::now().to_rfc3339()
        });

        let result: SessionEntity = self.client
            .post("active_sessions", &session_data)
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        Ok(result)
    }

    /// End a session
    pub async fn end_session(&self, session_id: Uuid, reason: &str) -> Result<SessionEntity> {
        let update_data = serde_json::json!({
            "is_active": false,
            "ended_at": Utc::now().to_rfc3339(),
            "end_reason": reason
        });

        let result: SessionEntity = self.client
            .put("active_sessions", &session_id.to_string(), &update_data)
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        Ok(result)
    }

    /// Get active session for a user
    pub async fn get_active_session(&self, user_id: Uuid) -> Result<Option<SessionEntity>> {
        let query = format!("user_id=eq.{}&is_active=eq.true", user_id);
        let mut sessions: Vec<SessionEntity> = self.client
            .get("active_sessions", Some(&query))
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        Ok(sessions.pop())
    }

    /// Get sessions for a user
    pub async fn get_sessions(
        &self,
        user_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<SessionEntity>> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);
        let query = format!("user_id=eq.{}&order=started_at.desc&limit={}&offset={}", user_id, limit, offset);

        let sessions: Vec<SessionEntity> = self.client
            .get("active_sessions", Some(&query))
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        Ok(sessions)
    }

    // ============================================================================
    // Profile Activations
    // ============================================================================

    /// Record profile activation
    pub async fn record_profile_activation(
        &self,
        user_id: Uuid,
        profile_id: Uuid,
        session_id: Option<Uuid>,
        activation_method: &str,
        previous_profile_id: Option<Uuid>,
    ) -> Result<ProfileActivationEntity> {
        let activation_data = serde_json::json!({
            "user_id": user_id,
            "profile_id": profile_id,
            "session_id": session_id,
            "activation_method": activation_method,
            "previous_profile_id": previous_profile_id,
            "activated_at": Utc::now().to_rfc3339()
        });

        let result: ProfileActivationEntity = self.client
            .post("profile_activations", &activation_data)
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        Ok(result)
    }

    /// Get active profile activation
    pub async fn get_active_profile_activation(&self, user_id: Uuid) -> Result<Option<ProfileActivationEntity>> {
        let query = format!("user_id=eq.{}&order=activated_at.desc&limit=1", user_id);
        let mut activations: Vec<ProfileActivationEntity> = self.client
            .get("profile_activations", Some(&query))
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        Ok(activations.pop())
    }

    /// Get profile activations
    pub async fn get_profile_activations(
        &self,
        user_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<ProfileActivationEntity>> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);
        let query = format!("user_id=eq.{}&order=activated_at.desc&limit={}&offset={}", user_id, limit, offset);

        let activations: Vec<ProfileActivationEntity> = self.client
            .get("profile_activations", Some(&query))
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        Ok(activations)
    }

    // ============================================================================
    // Error Logs
    // ============================================================================

    /// Log an error
    pub async fn log_error(
        &self,
        user_id: Option<Uuid>,
        error_type: &str,
        message: &str,
        stack_trace: Option<&str>,
        component: &str,
        severity: &str,
        context: Option<serde_json::Value>,
    ) -> Result<ErrorLogEntity> {
        let error_data = serde_json::json!({
            "user_id": user_id,
            "error_type": error_type,
            "message": message,
            "stack_trace": stack_trace,
            "component": component,
            "severity": severity,
            "context": context,
            "is_resolved": false,
            "created_at": Utc::now().to_rfc3339()
        });

        let result: ErrorLogEntity = self.client
            .post("error_logs", &error_data)
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        Ok(result)
    }

    /// Get error logs
    pub async fn get_error_logs(
        &self,
        resolved: Option<bool>,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<ErrorLogEntity>> {
        let limit = limit.unwrap_or(100);
        let offset = offset.unwrap_or(0);
        let mut query = format!("order=created_at.desc&limit={}&offset={}", limit, offset);

        if let Some(resolved_val) = resolved {
            query.push_str(&format!("&is_resolved=eq.{}", resolved_val));
        }

        let errors: Vec<ErrorLogEntity> = self.client
            .get("error_logs", Some(&query))
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        Ok(errors)
    }

    /// Resolve an error
    pub async fn resolve_error(&self, error_id: Uuid, resolution: &str) -> Result<ErrorLogEntity> {
        let update_data = serde_json::json!({
            "is_resolved": true,
            "resolution": resolution,
            "resolved_at": Utc::now().to_rfc3339()
        });

        let result: ErrorLogEntity = self.client
            .put("error_logs", &error_id.to_string(), &update_data)
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        Ok(result)
    }

    // ============================================================================
    // Automation Executions
    // ============================================================================

    /// Record automation execution
    pub async fn record_automation_execution(
        &self,
        user_id: Uuid,
        rule_id: Uuid,
        session_id: Option<Uuid>,
        trigger_type: &str,
        success: bool,
        execution_time_ms: Option<i32>,
        error_message: Option<&str>,
        result_data: Option<serde_json::Value>,
    ) -> Result<AutomationExecutionEntity> {
        let execution_data = serde_json::json!({
            "user_id": user_id,
            "rule_id": rule_id,
            "session_id": session_id,
            "trigger_type": trigger_type,
            "success": success,
            "execution_time_ms": execution_time_ms,
            "error_message": error_message,
            "result_data": result_data,
            "executed_at": Utc::now().to_rfc3339()
        });

        let result: AutomationExecutionEntity = self.client
            .post("automation_executions", &execution_data)
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        Ok(result)
    }

    /// Get automation executions
    pub async fn get_automation_executions(
        &self,
        user_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<AutomationExecutionEntity>> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);
        let query = format!("user_id=eq.{}&order=executed_at.desc&limit={}&offset={}", user_id, limit, offset);

        let executions: Vec<AutomationExecutionEntity> = self.client
            .get("automation_executions", Some(&query))
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        Ok(executions)
    }

    // ============================================================================
    // Monitor Changes
    // ============================================================================

    /// Record monitor change
    pub async fn record_monitor_change(
        &self,
        user_id: Uuid,
        session_id: Option<Uuid>,
        change_type: &str,
        monitor_id: Uuid,
        old_config: Option<serde_json::Value>,
        new_config: Option<serde_json::Value>,
    ) -> Result<MonitorChangeEntity> {
        let change_data = serde_json::json!({
            "user_id": user_id,
            "session_id": session_id,
            "change_type": change_type,
            "monitor_id": monitor_id,
            "old_config": old_config,
            "new_config": new_config,
            "changed_at": Utc::now().to_rfc3339()
        });

        let result: MonitorChangeEntity = self.client
            .post("monitor_changes", &change_data)
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        Ok(result)
    }

    /// Get monitor changes
    pub async fn get_monitor_changes(
        &self,
        user_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<MonitorChangeEntity>> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);
        let query = format!("user_id=eq.{}&order=changed_at.desc&limit={}&offset={}", user_id, limit, offset);

        let changes: Vec<MonitorChangeEntity> = self.client
            .get("monitor_changes", Some(&query))
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        Ok(changes)
    }

    // ============================================================================
    // App Launches
    // ============================================================================

    /// Record app launch
    pub async fn record_app_launch(
        &self,
        user_id: Uuid,
        session_id: Option<Uuid>,
        app_id: Uuid,
        launch_method: &str,
        success: bool,
        error_message: Option<&str>,
    ) -> Result<AppLaunchEntity> {
        let launch_data = serde_json::json!({
            "user_id": user_id,
            "session_id": session_id,
            "app_id": app_id,
            "launch_method": launch_method,
            "success": success,
            "error_message": error_message,
            "launched_at": Utc::now().to_rfc3339()
        });

        let result: AppLaunchEntity = self.client
            .post("app_launches", &launch_data)
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        Ok(result)
    }

    /// Get app launches
    pub async fn get_app_launches(
        &self,
        user_id: Uuid,
        limit: Option<i64>,
        offset: Option<i64>,
    ) -> Result<Vec<AppLaunchEntity>> {
        let limit = limit.unwrap_or(50);
        let offset = offset.unwrap_or(0);
        let query = format!("user_id=eq.{}&order=launched_at.desc&limit={}&offset={}", user_id, limit, offset);

        let launches: Vec<AppLaunchEntity> = self.client
            .get("app_launches", Some(&query))
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        Ok(launches)
    }

    // ============================================================================
    // Dashboard Stats
    // ============================================================================

    /// Get dashboard statistics
    pub async fn get_dashboard_stats(&self, user_id: Uuid) -> Result<DashboardStats> {
        // Get today's activity count
        let today = Utc::now().date_naive();
        let today_str = today.format("%Y-%m-%d").to_string();
        let today_query = format!("user_id=eq.{}&created_at=gte.{}-00:00:00&created_at=lte.{}-23:59:59", user_id, today_str, today_str);
        let today_events: Vec<AuditEventEntity> = self.client
            .get("audit_events", Some(&today_query))
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        // Get this week's activity count
        let week_ago = Utc::now() - chrono::Duration::days(7);
        let week_query = format!("user_id=eq.{}&created_at=gte.{}", user_id, week_ago.to_rfc3339());
        let week_events: Vec<AuditEventEntity> = self.client
            .get("audit_events", Some(&week_query))
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        // Get active session
        let active_session = self.get_active_session(user_id).await?;

        // Get unresolved errors count
        let errors_query = "is_resolved=eq.false";
        let unresolved_errors: Vec<ErrorLogEntity> = self.client
            .get("error_logs", Some(errors_query))
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        Ok(DashboardStats {
            total_activities_today: today_events.len() as i64,
            total_activities_this_week: week_events.len() as i64,
            active_session_duration: active_session.map(|s| {
                Utc::now().signed_duration_since(s.started_at).num_minutes()
            }).unwrap_or(0),
            unresolved_errors_count: unresolved_errors.len() as i64,
        })
    }

    /// Get log summary
    pub async fn get_log_summary(&self, user_id: Uuid, days: i32) -> Result<LogSummary> {
        let since = Utc::now() - chrono::Duration::days(days as i64);
        let query = format!("user_id=eq.{}&created_at=gte.{}", user_id, since.to_rfc3339());

        let events: Vec<AuditEventEntity> = self.client
            .get("audit_events", Some(&query))
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        let mut activity_count = 0;
        let mut system_count = 0;
        let mut error_count = 0;

        for event in events {
            match event.event_type.as_deref() {
                Some("activity") => activity_count += 1,
                Some("system") => system_count += 1,
                Some("error") => error_count += 1,
                _ => {}
            }
        }

        Ok(LogSummary {
            total_logs: (activity_count + system_count + error_count) as i64,
            activity_logs: activity_count as i64,
            system_logs: system_count as i64,
            error_logs: error_count as i64,
            period_days: days,
        })
    }

    /// Get app metrics
    pub async fn get_app_metrics(&self, user_id: Uuid, days: i32) -> Result<AppMetrics> {
        let since = Utc::now() - chrono::Duration::days(days as i64);
        let query = format!("user_id=eq.{}&launched_at=gte.{}", user_id, since.to_rfc3339());

        let launches: Vec<AppLaunchEntity> = self.client
            .get("app_launches", Some(&query))
            .await
            .map_err(|e| SmoothieError::Database(e.to_string()))?;

        let total_launches = launches.len() as i64;
        let successful_launches = launches.iter().filter(|l| l.success).count() as i64;
        let failed_launches = total_launches - successful_launches;

        Ok(AppMetrics {
            total_app_launches: total_launches,
            successful_launches,
            failed_launches,
            average_launch_time: None, // Would need execution_time_ms from automation executions
            period_days: days,
        })
    }

    /// Cleanup old logs (auto-cleanup function in Supabase handles this)
    pub async fn cleanup_old_logs(&self, _days_to_keep: i64) -> Result<i64> {
        // This is handled by the auto-cleanup function in Supabase
        // We could trigger it here if needed
        Ok(0)
    }
}