// Browser tab repository - database operations for browser tabs

use crate::error::{Result, SmoothieError};
use crate::models::entities::BrowserTabEntity;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub struct BrowserTabRepository<'a> {
  pool: &'a PgPool,
}

impl<'a> BrowserTabRepository<'a> {
  pub fn new(pool: &'a PgPool) -> Self {
    Self { pool }
  }

  /// Find all browser tabs for a profile
  pub async fn find_by_profile_id(&self, profile_id: Uuid) -> Result<Vec<BrowserTabEntity>> {
    sqlx::query_as::<_, BrowserTabEntity>(
      r#"
            SELECT id, profile_id, url, browser, monitor_id, tab_order, favicon, created_at, updated_at
            FROM browser_tabs
            WHERE profile_id = $1
            ORDER BY tab_order
            "#,
    )
    .bind(profile_id)
    .fetch_all(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))
  }

  /// Find a browser tab by ID
  pub async fn find_by_id(&self, id: Uuid) -> Result<Option<BrowserTabEntity>> {
    sqlx::query_as::<_, BrowserTabEntity>(
      r#"
            SELECT id, profile_id, url, browser, monitor_id, tab_order, favicon, created_at, updated_at
            FROM browser_tabs
            WHERE id = $1
            "#,
    )
    .bind(id)
    .fetch_optional(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))
  }

  /// Create a new browser tab
  pub async fn create(
    &self,
    profile_id: Uuid,
    url: &str,
    browser: &str,
    monitor_id: Option<Uuid>,
    tab_order: i32,
    favicon: Option<&str>,
  ) -> Result<BrowserTabEntity> {
    let id = Uuid::new_v4();
    let now = Utc::now();

    sqlx::query(
            r#"
            INSERT INTO browser_tabs (id, profile_id, url, browser, monitor_id, tab_order, favicon, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $8)
            "#,
        )
        .bind(id)
        .bind(profile_id)
        .bind(url)
        .bind(browser)
        .bind(monitor_id)
        .bind(tab_order)
        .bind(favicon)
        .bind(now)
        .execute(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    self
      .find_by_id(id)
      .await?
      .ok_or_else(|| SmoothieError::NotFound("Browser tab not found after creation".into()))
  }

  /// Update a browser tab
  pub async fn update(&self, id: Uuid, url: Option<&str>) -> Result<BrowserTabEntity> {
    let now = Utc::now();
    sqlx::query("UPDATE browser_tabs SET url = COALESCE($1, url), updated_at = $2 WHERE id = $3")
      .bind(url)
      .bind(now)
      .bind(id)
      .execute(self.pool)
      .await
      .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    self
      .find_by_id(id)
      .await?
      .ok_or_else(|| SmoothieError::NotFound("Browser tab not found".into()))
  }

  /// Delete a browser tab
  pub async fn delete(&self, id: Uuid) -> Result<bool> {
    let result = sqlx::query("DELETE FROM browser_tabs WHERE id = $1")
      .bind(id)
      .execute(self.pool)
      .await
      .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(result.rows_affected() > 0)
  }

  /// Count browser tabs for a profile
  pub async fn count_by_profile_id(&self, profile_id: Uuid) -> Result<i64> {
    let (count,) =
      sqlx::query_as::<_, (i64,)>("SELECT COUNT(*) FROM browser_tabs WHERE profile_id = $1")
        .bind(profile_id)
        .fetch_one(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(count)
  }
}
