// Profile repository - database operations for profiles

use crate::error::{Result, SmoothieError};
use crate::models::entities::{ProfileEntity, ProfileTagEntity};
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub struct ProfileRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> ProfileRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Find all profiles for a user
    pub async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<ProfileEntity>> {
        sqlx::query_as::<_, ProfileEntity>(
            r#"
            SELECT id, user_id, name, description, type, is_active, 
                   created_at, updated_at, last_used
            FROM profiles 
            WHERE user_id = $1 
            ORDER BY updated_at DESC
            "#,
        )
        .bind(user_id)
        .fetch_all(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))
    }

    /// Find a profile by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<ProfileEntity>> {
        sqlx::query_as::<_, ProfileEntity>(
            r#"
            SELECT id, user_id, name, description, type, is_active,
                   created_at, updated_at, last_used
            FROM profiles 
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))
    }

    /// Create a new profile
    pub async fn create(
        &self,
        user_id: Uuid,
        name: &str,
        description: Option<&str>,
        profile_type: &str,
    ) -> Result<ProfileEntity> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO profiles (id, user_id, name, description, type, is_active, created_at, updated_at)
            VALUES ($1, $2, $3, $4, $5, false, $6, $6)
            "#,
        )
        .bind(id)
        .bind(user_id)
        .bind(name)
        .bind(description)
        .bind(profile_type)
        .bind(now)
        .execute(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| SmoothieError::NotFound("Profile not found after creation".into()))
    }

    /// Update a profile
    pub async fn update(
        &self,
        id: Uuid,
        name: Option<&str>,
        description: Option<&str>,
    ) -> Result<ProfileEntity> {
        let now = Utc::now();

        sqlx::query(
            r#"
            UPDATE profiles 
            SET name = COALESCE($1, name),
                description = COALESCE($2, description),
                updated_at = $3
            WHERE id = $4
            "#,
        )
        .bind(name)
        .bind(description)
        .bind(now)
        .bind(id)
        .execute(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| SmoothieError::NotFound("Profile not found".into()))
    }

    /// Delete a profile
    pub async fn delete(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query("DELETE FROM profiles WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await
            .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    /// Activate a profile (deactivate all others for user)
    pub async fn activate(&self, id: Uuid, user_id: Uuid) -> Result<ProfileEntity> {
        let now = Utc::now();

        // Deactivate all profiles for user
        sqlx::query("UPDATE profiles SET is_active = false WHERE user_id = $1")
            .bind(user_id)
            .execute(self.pool)
            .await
            .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        // Activate the specified profile
        sqlx::query(
            r#"
            UPDATE profiles 
            SET is_active = true, last_used = $1, updated_at = $1 
            WHERE id = $2
            "#,
        )
        .bind(now)
        .bind(id)
        .execute(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| SmoothieError::NotFound("Profile not found".into()))
    }

    /// Get tags for a profile
    pub async fn get_tags(&self, profile_id: Uuid) -> Result<Vec<String>> {
        let tags: Vec<ProfileTagEntity> =
            sqlx::query_as("SELECT profile_id, tag FROM profile_tags WHERE profile_id = $1")
                .bind(profile_id)
                .fetch_all(self.pool)
                .await
                .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        Ok(tags.into_iter().map(|t| t.tag).collect())
    }

    /// Alias for get_tags (used by service layer)
    pub async fn find_tags(&self, profile_id: Uuid) -> Result<Vec<String>> {
        self.get_tags(profile_id).await
    }

    /// Add a tag to a profile
    pub async fn add_tag(&self, profile_id: Uuid, tag: &str) -> Result<()> {
        sqlx::query("INSERT INTO profile_tags (profile_id, tag) VALUES ($1, $2) ON CONFLICT DO NOTHING")
            .bind(profile_id)
            .bind(tag)
            .execute(self.pool)
            .await
            .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        Ok(())
    }

    /// Set tags for a profile (replaces existing)
    pub async fn set_tags(&self, profile_id: Uuid, tags: &[String]) -> Result<()> {
        // Delete existing tags
        sqlx::query("DELETE FROM profile_tags WHERE profile_id = $1")
            .bind(profile_id)
            .execute(self.pool)
            .await
            .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        // Insert new tags
        for tag in tags {
            sqlx::query("INSERT INTO profile_tags (profile_id, tag) VALUES ($1, $2)")
                .bind(profile_id)
                .bind(tag)
                .execute(self.pool)
                .await
                .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;
        }

        Ok(())
    }

    /// Check if a profile exists
    pub async fn exists(&self, id: Uuid) -> Result<bool> {
        let result: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM profiles WHERE id = $1")
            .bind(id)
            .fetch_one(self.pool)
            .await
            .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        Ok(result.0 > 0)
    }
}
