//! User settings service - business logic for user preferences

use crate::db::Database;
use crate::error::{Result, SmoothieError};
use crate::models::dto::UserSettingsDto;
use crate::repositories::UserSettingsRepository;
use sqlx::PgPool;
use uuid::Uuid;

pub struct UserSettingsService;

impl UserSettingsService {
  /// Ensure a user exists in the local database (creates if not exists)
  async fn ensure_user_exists(pool: &PgPool, user_id: Uuid) -> Result<()> {
    sqlx::query(
      r#"
      INSERT INTO users (id, created_at, updated_at)
      VALUES ($1, NOW(), NOW())
      ON CONFLICT (id) DO NOTHING
      "#,
    )
    .bind(user_id)
    .execute(pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    tracing::debug!(user_id = %user_id, "User ensured in local database");
    Ok(())
  }

  /// Get user settings (creates defaults if not exist)
  pub async fn get_settings(db: &Database, user_id: Uuid) -> Result<UserSettingsDto> {
    // Ensure the user exists in the local database
    Self::ensure_user_exists(db.pool(), user_id).await?;

    let repo = UserSettingsRepository::new(db.pool());
    let settings = repo.get_or_create(user_id).await?;

    Ok(UserSettingsDto::from(settings))
  }

  /// Update user settings
  pub async fn update_settings(
    db: &Database,
    user_id: Uuid,
    theme: Option<String>,
    auto_restore: Option<bool>,
    monitor_detection: Option<bool>,
    animations_enabled: Option<bool>,
    cloud_sync: Option<bool>,
    auto_activate_time: Option<String>,
    keyboard_shortcut: Option<String>,
    notifications_enabled: Option<bool>,
  ) -> Result<UserSettingsDto> {
    // Ensure the user exists in the local database
    Self::ensure_user_exists(db.pool(), user_id).await?;

    let repo = UserSettingsRepository::new(db.pool());

    // Ensure settings exist first
    let _ = repo.get_or_create(user_id).await?;

    // Then update
    let settings = repo
      .update(
        user_id,
        theme,
        auto_restore,
        monitor_detection,
        animations_enabled,
        cloud_sync,
        auto_activate_time,
        keyboard_shortcut,
        notifications_enabled,
      )
      .await?;

    Ok(UserSettingsDto::from(settings))
  }
}
