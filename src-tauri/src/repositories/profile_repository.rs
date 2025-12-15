// Profile repository - database operations for profiles

use crate::error::{Result, SmoothieError};
use crate::models::entities::ProfileEntity;
use chrono::Utc;
use sqlx::PgPool;
use tracing::{error, info, instrument, warn};
use uuid::Uuid;

pub struct ProfileRepository<'a> {
  pool: &'a PgPool,
}

impl<'a> ProfileRepository<'a> {
  pub fn new(pool: &'a PgPool) -> Self {
    Self { pool }
  }

  /// Find all profiles for a user
  #[instrument(skip(self), fields(user_id = %user_id))]
  pub async fn find_by_user_id(&self, user_id: Uuid) -> Result<Vec<ProfileEntity>> {
    info!("Finding profiles for user");
    let start = std::time::Instant::now();

    let result = sqlx::query_as::<_, ProfileEntity>(
      r#"
            SELECT id, user_id, name, description, type, is_active,
                   created_at, updated_at, last_used, last_activated_at,
                   activation_count, is_favorite, color, icon, sort_order
            FROM profiles
            WHERE user_id = $1
            ORDER BY COALESCE(sort_order, 0), updated_at DESC
            "#,
    )
    .bind(user_id)
    .fetch_all(self.pool)
    .await;

    let duration = start.elapsed();
    match &result {
      Ok(profiles) => {
        info!(
          user_id = %user_id,
          count = profiles.len(),
          duration_ms = duration.as_millis(),
          "Successfully found profiles for user"
        );
      }
      Err(e) => {
        error!(
          user_id = %user_id,
          error = %e,
          duration_ms = duration.as_millis(),
          "Failed to find profiles for user"
        );
      }
    }

    result.map_err(|e| SmoothieError::DatabaseError(e.to_string()))
  }

  /// Find a profile by ID
  #[instrument(skip(self), fields(profile_id = %id))]
  pub async fn find_by_id(&self, id: Uuid) -> Result<Option<ProfileEntity>> {
    info!("Finding profile by ID");
    let start = std::time::Instant::now();

    let result = sqlx::query_as::<_, ProfileEntity>(
      r#"
            SELECT id, user_id, name, description, type, is_active,
                   created_at, updated_at, last_used, last_activated_at,
                   activation_count, is_favorite, color, icon, sort_order
            FROM profiles
            WHERE id = $1
            "#,
    )
    .bind(id)
    .fetch_optional(self.pool)
    .await;

    let duration = start.elapsed();
    match &result {
      Ok(Some(profile)) => {
        info!(
          profile_id = %id,
          profile_name = %profile.name,
          user_id = %profile.user_id,
          duration_ms = duration.as_millis(),
          "Successfully found profile"
        );
      }
      Ok(None) => {
        warn!(
          profile_id = %id,
          duration_ms = duration.as_millis(),
          "Profile not found"
        );
      }
      Err(e) => {
        error!(
          profile_id = %id,
          error = %e,
          duration_ms = duration.as_millis(),
          "Failed to find profile"
        );
      }
    }

    result.map_err(|e| SmoothieError::DatabaseError(e.to_string()))
  }

  /// Find favorite profiles for a user
  #[instrument(skip(self), fields(user_id = %user_id))]
  pub async fn find_favorites(&self, user_id: Uuid) -> Result<Vec<ProfileEntity>> {
    info!("Finding favorite profiles for user");
    let start = std::time::Instant::now();

    let result = sqlx::query_as::<_, ProfileEntity>(
      r#"
            SELECT id, user_id, name, description, type, is_active,
                   created_at, updated_at, last_used, last_activated_at,
                   activation_count, is_favorite, color, icon, sort_order
            FROM profiles
            WHERE user_id = $1 AND is_favorite = true
            ORDER BY COALESCE(sort_order, 0), updated_at DESC
            "#,
    )
    .bind(user_id)
    .fetch_all(self.pool)
    .await;

    let duration = start.elapsed();
    match &result {
      Ok(profiles) => {
        info!(
          user_id = %user_id,
          count = profiles.len(),
          duration_ms = duration.as_millis(),
          "Successfully found favorite profiles for user"
        );
      }
      Err(e) => {
        error!(
          user_id = %user_id,
          error = %e,
          duration_ms = duration.as_millis(),
          "Failed to find favorite profiles for user"
        );
      }
    }

    result.map_err(|e| SmoothieError::DatabaseError(e.to_string()))
  }

  /// Find most used profiles for a user
  #[instrument(skip(self), fields(user_id = %user_id, limit = %limit))]
  pub async fn find_most_used(&self, user_id: Uuid, limit: i64) -> Result<Vec<ProfileEntity>> {
    info!("Finding most used profiles for user");
    let start = std::time::Instant::now();

    let result = sqlx::query_as::<_, ProfileEntity>(
      r#"
            SELECT id, user_id, name, description, type, is_active,
                   created_at, updated_at, last_used, last_activated_at,
                   activation_count, is_favorite, color, icon, sort_order
            FROM profiles
            WHERE user_id = $1
            ORDER BY COALESCE(activation_count, 0) DESC
            LIMIT $2
            "#,
    )
    .bind(user_id)
    .bind(limit)
    .fetch_all(self.pool)
    .await;

    let duration = start.elapsed();
    match &result {
      Ok(profiles) => {
        info!(
          user_id = %user_id,
          limit = %limit,
          count = profiles.len(),
          duration_ms = duration.as_millis(),
          "Successfully found most used profiles for user"
        );
      }
      Err(e) => {
        error!(
          user_id = %user_id,
          limit = %limit,
          error = %e,
          duration_ms = duration.as_millis(),
          "Failed to find most used profiles for user"
        );
      }
    }

    result.map_err(|e| SmoothieError::DatabaseError(e.to_string()))
  }

  /// Create a new profile
  #[instrument(skip(self), fields(user_id = %user_id, name = %name, profile_type = %profile_type))]
  pub async fn create(
    &self,
    user_id: Uuid,
    name: &str,
    description: Option<&str>,
    profile_type: &str,
  ) -> Result<ProfileEntity> {
    info!("Creating new profile");
    let id = Uuid::new_v4();
    let now = Utc::now();
    let start = std::time::Instant::now();

    let insert_result = sqlx::query(
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
    .await;

    let duration = start.elapsed();
    match &insert_result {
      Ok(_) => {
        info!(
          profile_id = %id,
          user_id = %user_id,
          name = %name,
          profile_type = %profile_type,
          duration_ms = duration.as_millis(),
          "Profile inserted successfully"
        );
      }
      Err(e) => {
        error!(
          user_id = %user_id,
          name = %name,
          profile_type = %profile_type,
          error = %e,
          duration_ms = duration.as_millis(),
          "Failed to insert profile"
        );
        return Err(SmoothieError::DatabaseError(e.to_string()));
      }
    }

    // Fetch the created profile
    let fetch_start = std::time::Instant::now();
    let fetch_result = self.find_by_id(id).await;
    let fetch_duration = fetch_start.elapsed();

    match fetch_result {
      Ok(Some(profile)) => {
        info!(
          profile_id = %id,
          fetch_duration_ms = fetch_duration.as_millis(),
          "Profile created and retrieved successfully"
        );
        Ok(profile)
      }
      Ok(None) => {
        error!(
          profile_id = %id,
          "Profile not found after creation"
        );
        Err(SmoothieError::NotFound(
          "Profile not found after creation".into(),
        ))
      }
      Err(e) => {
        error!(
          profile_id = %id,
          error = %e,
          "Failed to retrieve created profile"
        );
        Err(e)
      }
    }
  }

  /// Update a profile
  #[instrument(skip(self), fields(profile_id = %id))]
  pub async fn update(
    &self,
    id: Uuid,
    name: Option<&str>,
    description: Option<&str>,
  ) -> Result<ProfileEntity> {
    info!("Updating profile");
    let now = Utc::now();
    let start = std::time::Instant::now();

    let update_result = sqlx::query(
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
    .await;

    let duration = start.elapsed();
    match &update_result {
      Ok(result) => {
        info!(
          profile_id = %id,
          rows_affected = result.rows_affected(),
          duration_ms = duration.as_millis(),
          "Profile updated successfully"
        );
      }
      Err(e) => {
        error!(
          profile_id = %id,
          error = %e,
          duration_ms = duration.as_millis(),
          "Failed to update profile"
        );
        return Err(SmoothieError::DatabaseError(e.to_string()));
      }
    }

    // Fetch the updated profile
    let fetch_start = std::time::Instant::now();
    let fetch_result = self.find_by_id(id).await;
    let fetch_duration = fetch_start.elapsed();

    match fetch_result {
      Ok(Some(profile)) => {
        info!(
          profile_id = %id,
          fetch_duration_ms = fetch_duration.as_millis(),
          "Profile updated and retrieved successfully"
        );
        Ok(profile)
      }
      Ok(None) => {
        error!(
          profile_id = %id,
          "Profile not found after update"
        );
        Err(SmoothieError::NotFound("Profile not found".into()))
      }
      Err(e) => {
        error!(
          profile_id = %id,
          error = %e,
          "Failed to retrieve updated profile"
        );
        Err(e)
      }
    }
  }

  /// Update profile with extended fields
  pub async fn update_extended(
    &self,
    id: Uuid,
    name: Option<&str>,
    description: Option<&str>,
    is_favorite: Option<bool>,
    color: Option<&str>,
    icon: Option<&str>,
    sort_order: Option<i32>,
  ) -> Result<ProfileEntity> {
    let now = Utc::now();

    sqlx::query(
      r#"
            UPDATE profiles
            SET name = COALESCE($1, name),
                description = COALESCE($2, description),
                is_favorite = COALESCE($3, is_favorite),
                color = COALESCE($4, color),
                icon = COALESCE($5, icon),
                sort_order = COALESCE($6, sort_order),
                updated_at = $7
            WHERE id = $8
            "#,
    )
    .bind(name)
    .bind(description)
    .bind(is_favorite)
    .bind(color)
    .bind(icon)
    .bind(sort_order)
    .bind(now)
    .bind(id)
    .execute(self.pool)
    .await
    .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    self
      .find_by_id(id)
      .await?
      .ok_or_else(|| SmoothieError::NotFound("Profile not found".into()))
  }

  /// Set favorite status
  #[instrument(skip(self), fields(profile_id = %id, is_favorite = %is_favorite))]
  pub async fn set_favorite(&self, id: Uuid, is_favorite: bool) -> Result<ProfileEntity> {
    info!("Setting profile favorite status");
    let now = Utc::now();
    let start = std::time::Instant::now();

    let result = sqlx::query("UPDATE profiles SET is_favorite = $1, updated_at = $2 WHERE id = $3")
      .bind(is_favorite)
      .bind(now)
      .bind(id)
      .execute(self.pool)
      .await;

    let duration = start.elapsed();
    match &result {
      Ok(res) => {
        info!(
          profile_id = %id,
          is_favorite = %is_favorite,
          rows_affected = res.rows_affected(),
          duration_ms = duration.as_millis(),
          "Profile favorite status updated successfully"
        );
      }
      Err(e) => {
        error!(
          profile_id = %id,
          is_favorite = %is_favorite,
          error = %e,
          duration_ms = duration.as_millis(),
          "Failed to update profile favorite status"
        );
        return Err(SmoothieError::DatabaseError(e.to_string()));
      }
    }

    // Fetch the updated profile
    let fetch_start = std::time::Instant::now();
    let fetch_result = self.find_by_id(id).await;
    let fetch_duration = fetch_start.elapsed();

    match fetch_result {
      Ok(Some(profile)) => {
        info!(
          profile_id = %id,
          fetch_duration_ms = fetch_duration.as_millis(),
          "Updated profile retrieved successfully"
        );
        Ok(profile)
      }
      Ok(None) => {
        error!(
          profile_id = %id,
          "Profile not found after favorite status update"
        );
        Err(SmoothieError::NotFound("Profile not found".into()))
      }
      Err(e) => {
        error!(
          profile_id = %id,
          error = %e,
          "Failed to retrieve profile after favorite status update"
        );
        Err(e)
      }
    }
  }

  /// Delete a profile
  #[instrument(skip(self), fields(profile_id = %id))]
  pub async fn delete(&self, id: Uuid) -> Result<bool> {
    info!("Deleting profile");
    let start = std::time::Instant::now();

    let result = sqlx::query("DELETE FROM profiles WHERE id = $1")
      .bind(id)
      .execute(self.pool)
      .await
      .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    let duration = start.elapsed();
    let deleted = result.rows_affected() > 0;

    if deleted {
      info!(
        profile_id = %id,
        rows_affected = result.rows_affected(),
        duration_ms = duration.as_millis(),
        "Profile deleted successfully"
      );
    } else {
      warn!(
        profile_id = %id,
        duration_ms = duration.as_millis(),
        "Profile not found for deletion"
      );
    }

    Ok(deleted)
  }

  /// Activate a profile (deactivate all others for user)
  #[instrument(skip(self), fields(profile_id = %id, user_id = %user_id))]
  pub async fn activate(&self, id: Uuid, user_id: Uuid) -> Result<ProfileEntity> {
    info!("Activating profile");
    let now = Utc::now();
    let start = std::time::Instant::now();

    // Deactivate all profiles for user
    let deactivate_result = sqlx::query("UPDATE profiles SET is_active = false WHERE user_id = $1")
      .bind(user_id)
      .execute(self.pool)
      .await;

    match &deactivate_result {
      Ok(result) => {
        info!(
          user_id = %user_id,
          deactivated_count = result.rows_affected(),
          "Deactivated other profiles for user"
        );
      }
      Err(e) => {
        error!(
          user_id = %user_id,
          error = %e,
          "Failed to deactivate other profiles"
        );
        return Err(SmoothieError::DatabaseError(e.to_string()));
      }
    }

    // Activate the specified profile and update stats
    let activate_result = sqlx::query(
      r#"
            UPDATE profiles
            SET is_active = true,
                last_used = $1,
                last_activated_at = $1,
                activation_count = COALESCE(activation_count, 0) + 1,
                updated_at = $1
            WHERE id = $2
            "#,
    )
    .bind(now)
    .bind(id)
    .execute(self.pool)
    .await;

    let duration = start.elapsed();
    match &activate_result {
      Ok(result) => {
        info!(
          profile_id = %id,
          user_id = %user_id,
          rows_affected = result.rows_affected(),
          duration_ms = duration.as_millis(),
          "Profile activated successfully"
        );
      }
      Err(e) => {
        error!(
          profile_id = %id,
          user_id = %user_id,
          error = %e,
          duration_ms = duration.as_millis(),
          "Failed to activate profile"
        );
        return Err(SmoothieError::DatabaseError(e.to_string()));
      }
    }

    // Fetch the activated profile
    let fetch_start = std::time::Instant::now();
    let fetch_result = self.find_by_id(id).await;
    let fetch_duration = fetch_start.elapsed();

    match fetch_result {
      Ok(Some(profile)) => {
        info!(
          profile_id = %id,
          fetch_duration_ms = fetch_duration.as_millis(),
          "Activated profile retrieved successfully"
        );
        Ok(profile)
      }
      Ok(None) => {
        error!(
          profile_id = %id,
          "Activated profile not found"
        );
        Err(SmoothieError::NotFound("Profile not found".into()))
      }
      Err(e) => {
        error!(
          profile_id = %id,
          error = %e,
          "Failed to retrieve activated profile"
        );
        Err(e)
      }
    }
  }

  /// Get tags for a profile
  pub async fn get_tags(&self, profile_id: Uuid) -> Result<Vec<String>> {
    let tags: Vec<(String,)> = sqlx::query_as("SELECT tag FROM profile_tags WHERE profile_id = $1")
      .bind(profile_id)
      .fetch_all(self.pool)
      .await
      .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(tags.into_iter().map(|(tag,)| tag).collect())
  }

  /// Alias for get_tags (used by service layer)
  pub async fn find_tags(&self, profile_id: Uuid) -> Result<Vec<String>> {
    self.get_tags(profile_id).await
  }

  /// Add a tag to a profile
  pub async fn add_tag(&self, profile_id: Uuid, tag: &str) -> Result<()> {
    sqlx::query("INSERT OR IGNORE INTO profile_tags (profile_id, tag) VALUES ($1, $2)")
      .bind(profile_id)
      .bind(tag)
      .execute(self.pool)
      .await
      .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

    Ok(())
  }
}
