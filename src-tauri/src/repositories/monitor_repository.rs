// Monitor repository - database operations for monitors

use crate::error::{Result, SmoothieError};
use crate::models::entities::MonitorEntity;
use sqlx::PgPool;
use uuid::Uuid;

pub struct MonitorRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> MonitorRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Find all monitors for a profile
    pub async fn find_by_profile_id(&self, profile_id: Uuid) -> Result<Vec<MonitorEntity>> {
        sqlx::query_as::<_, MonitorEntity>(
            r#"
            SELECT id, profile_id, name, resolution, orientation, is_primary, 
                   x, y, width, height, display_index
            FROM monitors 
            WHERE profile_id = $1 
            ORDER BY display_index
            "#,
        )
        .bind(profile_id)
        .fetch_all(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))
    }

    /// Find a monitor by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<MonitorEntity>> {
        sqlx::query_as::<_, MonitorEntity>(
            r#"
            SELECT id, profile_id, name, resolution, orientation, is_primary,
                   x, y, width, height, display_index
            FROM monitors 
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))
    }

    /// Create a new monitor
    pub async fn create(
        &self,
        profile_id: Uuid,
        name: &str,
        resolution: &str,
        orientation: &str,
        is_primary: bool,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        display_index: i32,
    ) -> Result<MonitorEntity> {
        let id = Uuid::new_v4();

        sqlx::query(
            r#"
            INSERT INTO monitors (id, profile_id, name, resolution, orientation, is_primary, x, y, width, height, display_index)
            VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
            "#,
        )
        .bind(id)
        .bind(profile_id)
        .bind(name)
        .bind(resolution)
        .bind(orientation)
        .bind(is_primary)
        .bind(x)
        .bind(y)
        .bind(width)
        .bind(height)
        .bind(display_index)
        .execute(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| SmoothieError::NotFound("Monitor not found after creation".into()))
    }

    /// Update monitor position
    pub async fn update_position(
        &self,
        id: Uuid,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<MonitorEntity> {
        sqlx::query(
            "UPDATE monitors SET x = $1, y = $2, width = $3, height = $4 WHERE id = $5",
        )
        .bind(x)
        .bind(y)
        .bind(width)
        .bind(height)
        .bind(id)
        .execute(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| SmoothieError::NotFound("Monitor not found".into()))
    }

    /// Delete a monitor
    pub async fn delete(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query("DELETE FROM monitors WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await
            .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    /// Delete all monitors for a profile
    pub async fn delete_by_profile_id(&self, profile_id: Uuid) -> Result<u64> {
        let result = sqlx::query("DELETE FROM monitors WHERE profile_id = $1")
            .bind(profile_id)
            .execute(self.pool)
            .await
            .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected())
    }
}
