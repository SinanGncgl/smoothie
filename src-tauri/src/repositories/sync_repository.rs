// Sync repository - database operations for sync history tracking

use crate::error::{Result, SmoothieError};
use crate::models::entities::SyncHistoryEntity;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub struct SyncRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> SyncRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Get all sync history for a user
    pub async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<SyncHistoryEntity>> {
        sqlx::query_as::<_, SyncHistoryEntity>(
            r#"
            SELECT id, user_id, sync_type, status, started_at, completed_at, 
                   items_synced, error_message
            FROM sync_history 
            WHERE user_id = $1
            ORDER BY started_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))
    }

    /// Get recent sync history with limit
    pub async fn find_recent(&self, user_id: Uuid, limit: i64) -> Result<Vec<SyncHistoryEntity>> {
        sqlx::query_as::<_, SyncHistoryEntity>(
            r#"
            SELECT id, user_id, sync_type, status, started_at, completed_at, 
                   items_synced, error_message
            FROM sync_history 
            WHERE user_id = $1
            ORDER BY started_at DESC
            LIMIT $2
            "#,
        )
        .bind(user_id)
        .bind(limit)
        .fetch_all(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))
    }

    /// Find sync history by status
    pub async fn find_by_status(&self, user_id: Uuid, status: &str) -> Result<Vec<SyncHistoryEntity>> {
        sqlx::query_as::<_, SyncHistoryEntity>(
            r#"
            SELECT id, user_id, sync_type, status, started_at, completed_at, 
                   items_synced, error_message
            FROM sync_history 
            WHERE user_id = $1 AND status = $2
            ORDER BY started_at DESC
            "#,
        )
        .bind(user_id)
        .bind(status)
        .fetch_all(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))
    }

    /// Get a single sync history record by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<SyncHistoryEntity>> {
        sqlx::query_as::<_, SyncHistoryEntity>(
            r#"
            SELECT id, user_id, sync_type, status, started_at, completed_at, 
                   items_synced, error_message
            FROM sync_history 
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))
    }

    /// Start a new sync operation
    pub async fn create(&self, user_id: Uuid, sync_type: &str) -> Result<SyncHistoryEntity> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO sync_history (id, user_id, sync_type, status, started_at)
            VALUES ($1, $2, $3, 'pending', $4)
            "#,
        )
        .bind(id)
        .bind(user_id)
        .bind(sync_type)
        .bind(now)
        .execute(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| SmoothieError::NotFound("Sync record not found after creation".into()))
    }

    /// Update sync status to in_progress
    pub async fn mark_in_progress(&self, id: Uuid) -> Result<()> {
        sqlx::query("UPDATE sync_history SET status = 'in_progress' WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await
            .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Complete a sync operation successfully
    pub async fn complete(&self, id: Uuid, items_synced: i32) -> Result<SyncHistoryEntity> {
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE sync_history 
            SET status = 'completed', completed_at = $1, items_synced = $2
            WHERE id = $3
            "#,
        )
        .bind(now)
        .bind(items_synced)
        .bind(id)
        .execute(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| SmoothieError::NotFound("Sync record not found".into()))
    }

    /// Mark a sync operation as failed
    pub async fn fail(&self, id: Uuid, error_message: &str) -> Result<SyncHistoryEntity> {
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE sync_history 
            SET status = 'failed', completed_at = $1, error_message = $2
            WHERE id = $3
            "#,
        )
        .bind(now)
        .bind(error_message)
        .bind(id)
        .execute(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| SmoothieError::NotFound("Sync record not found".into()))
    }

    /// Delete old sync history records (cleanup)
    pub async fn delete_older_than_days(&self, user_id: Uuid, days: i64) -> Result<u64> {
        let cutoff = Utc::now() - chrono::Duration::days(days);

        let result = sqlx::query(
            "DELETE FROM sync_history WHERE user_id = $1 AND started_at < $2"
        )
        .bind(user_id)
        .bind(cutoff)
        .execute(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected())
    }

    /// Get last successful sync
    pub async fn find_last_successful(&self, user_id: Uuid, sync_type: &str) -> Result<Option<SyncHistoryEntity>> {
        sqlx::query_as::<_, SyncHistoryEntity>(
            r#"
            SELECT id, user_id, sync_type, status, started_at, completed_at, 
                   items_synced, error_message
            FROM sync_history 
            WHERE user_id = $1 AND sync_type = $2 AND status = 'completed'
            ORDER BY completed_at DESC
            LIMIT 1
            "#,
        )
        .bind(user_id)
        .bind(sync_type)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))
    }
}
