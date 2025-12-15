// App repository - database operations for apps

use crate::error::{Result, SmoothieError};
use crate::models::entities::AppEntity;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub struct AppRepository<'a> {
  pool: &'a PgPool,
}

impl<'a> AppRepository<'a> {
  pub fn new(pool: &'a PgPool) -> Self {
    Self { pool }
  }

  /// Find all apps for a profile
  pub async fn find_by_profile_id(&self, profile_id: Uuid) -> Result<Vec<AppEntity>> {
    sqlx::query_as::<_, AppEntity>(
      r#"
            SELECT id, profile_id, name, bundle_id, exe_path, launch_on_activate,
                   monitor_preference, created_at, updated_at, icon_path, launch_args,
                   working_directory, startup_delay_ms, order_index
            FROM apps
            WHERE profile_id = $1
            ORDER BY COALESCE(order_index, 0), name
            "#,
    )
    .bind(profile_id)
    .fetch_all(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))
  }

  /// Find launchable apps for a profile
  pub async fn find_launchable(&self, profile_id: Uuid) -> Result<Vec<AppEntity>> {
    sqlx::query_as::<_, AppEntity>(
      r#"
            SELECT id, profile_id, name, bundle_id, exe_path, launch_on_activate,
                   monitor_preference, created_at, updated_at, icon_path, launch_args,
                   working_directory, startup_delay_ms, order_index
            FROM apps
            WHERE profile_id = $1 AND launch_on_activate = true
            ORDER BY COALESCE(order_index, 0), COALESCE(startup_delay_ms, 0), name
            "#,
    )
    .bind(profile_id)
    .fetch_all(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))
  }

  /// Find an app by ID
  pub async fn find_by_id(&self, id: Uuid) -> Result<Option<AppEntity>> {
    sqlx::query_as::<_, AppEntity>(
      r#"
            SELECT id, profile_id, name, bundle_id, exe_path, launch_on_activate,
                   monitor_preference, created_at, updated_at, icon_path, launch_args,
                   working_directory, startup_delay_ms, order_index
            FROM apps
            WHERE id = $1
            "#,
    )
    .bind(id)
    .fetch_optional(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))
  }

  /// Create a new app
  pub async fn create(
    &self,
    profile_id: Uuid,
    name: &str,
    bundle_id: &str,
    exe_path: Option<&str>,
    launch_on_activate: bool,
    monitor_preference: Option<i32>,
    startup_delay_ms: Option<i32>,
    order_index: Option<i32>,
  ) -> Result<AppEntity> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query(
            r#"
            INSERT INTO apps (id, profile_id, name, bundle_id, exe_path, launch_on_activate,
                              monitor_preference, created_at, updated_at, startup_delay_ms, order_index)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $8, $9, $10)
            "#,
        )
        .bind(id)
        .bind(profile_id)
        .bind(name)
        .bind(bundle_id)
        .bind(exe_path)
        .bind(launch_on_activate)
        .bind(monitor_preference)
        .bind(now)
        .bind(startup_delay_ms.unwrap_or(0))
        .bind(order_index.unwrap_or(0))
        .execute(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    self
      .find_by_id(id)
      .await?
      .ok_or_else(|| SmoothieError::NotFound("App not found after creation".into()))
  }

  /// Update an app
  pub async fn update(&self, id: Uuid, launch_on_activate: Option<bool>) -> Result<AppEntity> {
    let now = Utc::now();
    sqlx::query(
      "UPDATE apps SET launch_on_activate = COALESCE($1, launch_on_activate), updated_at = $2 WHERE id = $3",
    )
    .bind(launch_on_activate)
    .bind(now)
    .bind(id)
    .execute(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    self
      .find_by_id(id)
      .await?
      .ok_or_else(|| SmoothieError::NotFound("App not found".into()))
  }

  /// Delete an app
  pub async fn delete(&self, id: Uuid) -> Result<bool> {
    let result = sqlx::query("DELETE FROM apps WHERE id = $1")
      .bind(id)
      .execute(self.pool)
      .await
      .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(result.rows_affected() > 0)
  }

  /// Count apps for a profile
  pub async fn count_by_profile_id(&self, profile_id: Uuid) -> Result<i64> {
    let (count,) = sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM apps WHERE profile_id = $1")
      .bind(profile_id)
      .fetch_one(self.pool)
      .await
      .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(count)
  }
}
