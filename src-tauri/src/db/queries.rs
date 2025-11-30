// Optimized database queries with prepared statements

use sqlx::PgPool;
use crate::error::{Result, SmoothieError};
use uuid::Uuid;
use chrono::{DateTime, Utc};

pub struct ProfileQueries;

impl ProfileQueries {
    pub async fn get_profiles_by_user(pool: &PgPool, user_id: &str) -> Result<Vec<(Uuid, Uuid, String, Option<String>, String, bool, DateTime<Utc>, DateTime<Utc>, Option<DateTime<Utc>>)>> {
        let rows = sqlx::query_as(
            r#"
            SELECT id, user_id, name, description, type, is_active, created_at, updated_at, last_used
            FROM profiles
            WHERE user_id = $1::uuid
            ORDER BY updated_at DESC
            "#
        )
        .bind(user_id)
        .fetch_all(pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        Ok(rows)
    }

    pub async fn get_active_profile(pool: &PgPool, user_id: &str) -> Result<Option<(Uuid, Uuid, String, Option<String>, String, bool, DateTime<Utc>, DateTime<Utc>, Option<DateTime<Utc>>)>> {
        let row = sqlx::query_as(
            r#"
            SELECT id, user_id, name, description, type, is_active, created_at, updated_at, last_used
            FROM profiles
            WHERE user_id = $1::uuid AND is_active = true
            "#
        )
        .bind(user_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        Ok(row)
    }

    pub async fn count_profiles(pool: &PgPool, user_id: &str) -> Result<i64> {
        let (count,): (i64,) = sqlx::query_as("SELECT COUNT(*) FROM profiles WHERE user_id = $1::uuid")
            .bind(user_id)
            .fetch_one(pool)
            .await
            .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        Ok(count)
    }

    pub async fn search_profiles(pool: &PgPool, user_id: &str, query: &str) -> Result<Vec<(Uuid, String)>> {
        let search_pattern = format!("%{}%", query);
        let rows = sqlx::query_as::<_, (Uuid, String)>(
            "SELECT id, name FROM profiles WHERE user_id = $1::uuid AND name ILIKE $2 LIMIT 10"
        )
        .bind(user_id)
        .bind(&search_pattern)
        .fetch_all(pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        Ok(rows)
    }
}

pub struct MonitorQueries;

impl MonitorQueries {
    pub async fn get_profile_monitors(pool: &PgPool, profile_id: &str) -> Result<Vec<(Uuid, Uuid, String, String, String, bool, i32, i32, i32, i32, i32)>> {
        let rows = sqlx::query_as(
            "SELECT id, profile_id, name, resolution, orientation, is_primary, x, y, width, height, display_index FROM monitors WHERE profile_id = $1::uuid ORDER BY display_index"
        )
        .bind(profile_id)
        .fetch_all(pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        Ok(rows)
    }

    pub async fn get_primary_monitor(pool: &PgPool, profile_id: &str) -> Result<Option<(Uuid, Uuid, String, String, String, bool, i32, i32, i32, i32, i32)>> {
        let row = sqlx::query_as(
            "SELECT id, profile_id, name, resolution, orientation, is_primary, x, y, width, height, display_index FROM monitors WHERE profile_id = $1::uuid AND is_primary = true"
        )
        .bind(profile_id)
        .fetch_optional(pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        Ok(row)
    }
}

pub struct AppQueries;

impl AppQueries {
    pub async fn get_launchable_apps(pool: &PgPool, profile_id: &str) -> Result<Vec<(Uuid, Uuid, String, String, Option<String>, bool, Option<i32>, DateTime<Utc>)>> {
        let rows = sqlx::query_as(
            "SELECT id, profile_id, name, bundle_id, exe_path, launch_on_activate, monitor_preference, created_at FROM apps WHERE profile_id = $1::uuid AND launch_on_activate = true"
        )
        .bind(profile_id)
        .fetch_all(pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        Ok(rows)
    }
}

pub struct AutomationQueries;

impl AutomationQueries {
    pub async fn get_enabled_rules(pool: &PgPool, profile_id: &str) -> Result<Vec<(Uuid, Uuid, String, serde_json::Value, bool, DateTime<Utc>)>> {
        let rows = sqlx::query_as(
            "SELECT id, profile_id, rule_type, trigger_config, is_enabled, created_at FROM automation_rules WHERE profile_id = $1::uuid AND is_enabled = true"
        )
        .bind(profile_id)
        .fetch_all(pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        Ok(rows)
    }

    pub async fn get_scheduled_rules(pool: &PgPool) -> Result<Vec<(Uuid, Uuid, String, serde_json::Value, bool, DateTime<Utc>)>> {
        let rows = sqlx::query_as(
            "SELECT id, profile_id, rule_type, trigger_config, is_enabled, created_at FROM automation_rules WHERE rule_type = 'schedule' AND is_enabled = true"
        )
        .fetch_all(pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        Ok(rows)
    }
}
