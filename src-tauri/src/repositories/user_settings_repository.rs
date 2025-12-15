//! User settings repository - manages user preferences in database

use crate::error::{Result, SmoothieError};
use crate::models::entities::UserSettingsEntity;
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserSettingsRepository<'a> {
  pool: &'a PgPool,
}

impl<'a> UserSettingsRepository<'a> {
  pub fn new(pool: &'a PgPool) -> Self {
    Self { pool }
  }

  /// Get settings for a user, creating defaults if not exist
  pub async fn get_or_create(&self, user_id: Uuid) -> Result<UserSettingsEntity> {
    // Try to find existing settings
    let existing =
      sqlx::query_as::<_, UserSettingsEntity>(r#"SELECT * FROM user_settings WHERE user_id = ?"#)
        .bind(user_id.to_string())
        .fetch_optional(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    if let Some(settings) = existing {
      return Ok(settings);
    }

    // Create default settings
    let settings = sqlx::query_as::<_, UserSettingsEntity>(
      r#"
      INSERT INTO user_settings (id, user_id)
      VALUES (?, ?)
      RETURNING *
      "#,
    )
    .bind(Uuid::new_v4().to_string())
    .bind(user_id.to_string())
    .fetch_one(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(settings)
  }

  /// Update user settings
  pub async fn update(
    &self,
    user_id: Uuid,
    theme: Option<String>,
    auto_restore: Option<bool>,
    monitor_detection: Option<bool>,
    animations_enabled: Option<bool>,
    cloud_sync: Option<bool>,
    auto_activate_time: Option<String>,
    keyboard_shortcut: Option<String>,
    notifications_enabled: Option<bool>,
  ) -> Result<UserSettingsEntity> {
    let settings = sqlx::query_as::<_, UserSettingsEntity>(
      r#"
      UPDATE user_settings
      SET
        theme = COALESCE(?, theme),
        auto_restore = COALESCE(?, auto_restore),
        monitor_detection = COALESCE(?, monitor_detection),
        animations_enabled = COALESCE(?, animations_enabled),
        cloud_sync = COALESCE(?, cloud_sync),
        auto_activate_time = COALESCE(?, auto_activate_time),
        keyboard_shortcut = COALESCE(?, keyboard_shortcut),
        notifications_enabled = COALESCE(?, notifications_enabled),
        updated_at = CURRENT_TIMESTAMP
      WHERE user_id = ?
      RETURNING *
      "#,
    )
    .bind(theme)
    .bind(auto_restore.map(|b| if b { 1 } else { 0 }))
    .bind(monitor_detection.map(|b| if b { 1 } else { 0 }))
    .bind(animations_enabled.map(|b| if b { 1 } else { 0 }))
    .bind(cloud_sync.map(|b| if b { 1 } else { 0 }))
    .bind(auto_activate_time)
    .bind(keyboard_shortcut)
    .bind(notifications_enabled.map(|b| if b { 1 } else { 0 }))
    .bind(user_id.to_string())
    .fetch_one(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(settings)
  }
}
