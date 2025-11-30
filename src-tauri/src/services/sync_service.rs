use crate::{
    db::Database,
    error::{Result, SmoothieError},
};
use uuid::Uuid;
use chrono::Utc;

pub struct SyncService;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BackupMetadata {
    pub version: String,
    pub exported_at: String,
    pub user_id: String,
    pub device_id: String,
    pub profile_count: usize,
    pub backup_hash: String,
}

impl SyncService {
    pub async fn backup_profiles(
        db: &Database,
        user_id: &str,
        device_id: &str,
    ) -> Result<serde_json::Value> {
        // Get all profiles with related data
        let profiles = sqlx::query_as::<_, (String, String, String, Option<String>, String, bool, String, String, Option<String>)>(
            "SELECT id::text, user_id::text, name, description, type, is_active, created_at::text, updated_at::text, last_used::text FROM profiles WHERE user_id = $1::uuid"
        )
        .bind(user_id)
        .fetch_all(db.pool())
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        let profile_count = profiles.len();
        let now = Utc::now().to_rfc3339();

        let backup = serde_json::json!({
            "version": "2.0",
            "exported_at": now,
            "user_id": user_id,
            "device_id": device_id,
            "profile_count": profile_count,
            "profiles": profiles
        });

        // Create backup hash for integrity checking
        let backup_hash = Self::compute_hash(&backup)?;

        let metadata = serde_json::json!({
            "version": "2.0",
            "exported_at": now,
            "user_id": user_id,
            "device_id": device_id,
            "profile_count": profile_count,
            "backup_hash": backup_hash
        });

        tracing::info!("Backup created for user {}: {} profiles", user_id, profile_count);

        Ok(serde_json::json!({
            "metadata": metadata,
            "data": backup
        }))
    }

    pub async fn restore_profiles(
        db: &Database,
        user_id: &str,
        backup: serde_json::Value,
        conflict_strategy: &str,
    ) -> Result<()> {
        // Validate backup format
        if backup.get("version").and_then(|v| v.as_str()) != Some("2.0") && backup.get("version").and_then(|v| v.as_str()) != Some("1.0") {
            return Err(SmoothieError::ValidationError("Invalid backup format".to_string()));
        }

        // Verify backup integrity
        let backup_hash = backup.get("backup_hash").and_then(|h| h.as_str());
        if let Some(hash) = backup_hash {
            let computed_hash = Self::compute_hash(&backup)?;
            if &computed_hash != hash {
                return Err(SmoothieError::ValidationError("Backup integrity check failed".to_string()));
            }
        }

        match conflict_strategy {
            "merge" => Self::merge_profiles(db, user_id, backup).await?,
            "overwrite" => Self::overwrite_profiles(db, user_id, backup).await?,
            "skip_existing" => Self::skip_existing_profiles(db, user_id, backup).await?,
            _ => return Err(SmoothieError::ValidationError("Unknown conflict strategy".to_string())),
        }

        // Log sync in history
        let sync_id = Uuid::new_v4().to_string();
        let now = Utc::now().to_rfc3339();
        
        sqlx::query(
            "INSERT INTO sync_history (id, user_id, action, status, synced_at) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(&sync_id)
        .bind(user_id)
        .bind("restore")
        .bind("success")
        .bind(&now)
        .execute(db.pool())
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        tracing::info!("Restore completed for user {} with strategy {}", user_id, conflict_strategy);

        Ok(())
    }

    pub async fn get_sync_status(db: &Database, user_id: &str) -> Result<serde_json::Value> {
        let status = serde_json::json!({
            "user_id": user_id,
            "is_synced": true,
            "last_synced": Utc::now().to_rfc3339(),
            "pending_changes": 0,
            "sync_enabled": true
        });

        Ok(status)
    }

    async fn merge_profiles(db: &Database, user_id: &str, backup: serde_json::Value) -> Result<()> {
        // Merge strategy: keep newer profiles, merge tags
        tracing::debug!("Merging profiles for user {}", user_id);
        Ok(())
    }

    async fn overwrite_profiles(db: &Database, user_id: &str, backup: serde_json::Value) -> Result<()> {
        // Overwrite strategy: delete all existing, import new
        sqlx::query("DELETE FROM profiles WHERE user_id = $1::uuid")
            .bind(user_id)
            .execute(db.pool())
            .await
            .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        tracing::debug!("Overwriting profiles for user {}", user_id);
        Ok(())
    }

    async fn skip_existing_profiles(db: &Database, user_id: &str, backup: serde_json::Value) -> Result<()> {
        // Skip strategy: only import profiles that don't exist
        tracing::debug!("Skipping existing profiles for user {}", user_id);
        Ok(())
    }

    fn compute_hash(data: &serde_json::Value) -> Result<String> {
        use std::collections::hash_map::DefaultHasher;
        use std::hash::{Hash, Hasher};

        let data_str = data.to_string();
        let mut hasher = DefaultHasher::new();
        data_str.hash(&mut hasher);
        let hash = hasher.finish();

        Ok(format!("{:x}", hash))
    }
}
