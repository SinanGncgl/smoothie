// Feedback and Feature Request handlers

use crate::{
  db::Database,
  error::Result,
  models::{CreateFeedbackRequest, FeedbackDto, SuccessResponse},
};
use tauri::State;
use uuid::Uuid;

const DEFAULT_USER_ID: &str = "00000000-0000-0000-0000-000000000001";

/// Submit new feedback or feature request
#[tauri::command(rename_all = "camelCase")]
pub async fn submit_feedback(
  db: State<'_, Database>,
  req: CreateFeedbackRequest,
) -> Result<SuccessResponse<FeedbackDto>> {
  let user_id = Uuid::parse_str(DEFAULT_USER_ID)
    .map_err(|e| crate::error::SmoothieError::ValidationError(e.to_string()))?;

  // Get app version and OS info
  let app_version = option_env!("CARGO_PKG_VERSION").map(|v| v.to_string());
  let os_info = Some(serde_json::json!({
    "os": std::env::consts::OS,
    "arch": std::env::consts::ARCH,
  }));

  let entity = sqlx::query_as::<_, crate::models::entities::FeedbackEntity>(
    r#"
    INSERT INTO feedback (user_id, feedback_type, title, description, priority, category, contact_email, app_version, os_info)
    VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9)
    RETURNING *
    "#,
  )
  .bind(user_id)
  .bind(&req.feedback_type)
  .bind(&req.title)
  .bind(&req.description)
  .bind(req.priority.as_deref().unwrap_or("medium"))
  .bind(&req.category)
  .bind(&req.contact_email)
  .bind(&app_version)
  .bind(&os_info)
  .fetch_one(db.pool())
  .await
  .map_err(|e| crate::error::SmoothieError::DatabaseError(e.to_string()))?;

  // Log this as a system event
  let _ = crate::services::audit_service::AUDIT_SERVICE
    .log_system_event(
      &db,
      "feedback_submitted",
      "info",
      "FeedbackHandler",
      &format!("{}: {}", req.feedback_type, req.title),
      Some(serde_json::json!({
        "feedback_id": entity.id.to_string(),
        "feedback_type": req.feedback_type,
        "title": req.title,
      })),
      None,
    )
    .await;

  Ok(SuccessResponse {
    success: true,
    data: FeedbackDto::from(entity),
  })
}

/// Get all feedback for the current user
#[tauri::command(rename_all = "camelCase")]
pub async fn get_feedback(
  db: State<'_, Database>,
  status: Option<String>,
  feedback_type: Option<String>,
  limit: Option<i64>,
) -> Result<SuccessResponse<Vec<FeedbackDto>>> {
  let user_id = Uuid::parse_str(DEFAULT_USER_ID)
    .map_err(|e| crate::error::SmoothieError::ValidationError(e.to_string()))?;

  let entities = sqlx::query_as::<_, crate::models::entities::FeedbackEntity>(
    r#"
    SELECT * FROM feedback
    WHERE user_id = $1
      AND ($2::text IS NULL OR status = $2)
      AND ($3::text IS NULL OR feedback_type = $3)
    ORDER BY created_at DESC
    LIMIT $4
    "#,
  )
  .bind(user_id)
  .bind(&status)
  .bind(&feedback_type)
  .bind(limit.unwrap_or(50))
  .fetch_all(db.pool())
  .await
  .map_err(|e| crate::error::SmoothieError::DatabaseError(e.to_string()))?;

  let data: Vec<FeedbackDto> = entities.into_iter().map(FeedbackDto::from).collect();

  Ok(SuccessResponse {
    success: true,
    data,
  })
}

/// Update feedback status (for internal use / admin)
#[tauri::command(rename_all = "camelCase")]
pub async fn update_feedback_status(
  db: State<'_, Database>,
  feedback_id: String,
  status: String,
) -> Result<SuccessResponse<FeedbackDto>> {
  let id = Uuid::parse_str(&feedback_id)
    .map_err(|e| crate::error::SmoothieError::ValidationError(e.to_string()))?;

  let entity = sqlx::query_as::<_, crate::models::entities::FeedbackEntity>(
    r#"
    UPDATE feedback
    SET status = $2, updated_at = CURRENT_TIMESTAMP
    WHERE id = $1
    RETURNING *
    "#,
  )
  .bind(id)
  .bind(&status)
  .fetch_one(db.pool())
  .await
  .map_err(|e| crate::error::SmoothieError::DatabaseError(e.to_string()))?;

  Ok(SuccessResponse {
    success: true,
    data: FeedbackDto::from(entity),
  })
}
