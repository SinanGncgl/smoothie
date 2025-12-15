// Subscription repository - database operations for subscriptions

use crate::error::{Result, SmoothieError};
use crate::models::entities::SubscriptionEntity;
use chrono::Utc;
use sqlx::PgPool;
use tracing::{error, info, instrument};
use uuid::Uuid;

pub struct SubscriptionRepository<'a> {
  pool: &'a PgPool,
}

impl<'a> SubscriptionRepository<'a> {
  pub fn new(pool: &'a PgPool) -> Self {
    Self { pool }
  }

  /// Find subscription for a user
  #[instrument(skip(self), fields(user_id = %user_id))]
  pub async fn find_by_user_id(&self, user_id: Uuid) -> Result<Option<SubscriptionEntity>> {
    info!("Finding subscription for user");
    let start = std::time::Instant::now();

    let result = sqlx::query_as::<_, SubscriptionEntity>(
      r#"
            SELECT id, user_id, stripe_customer_id, stripe_subscription_id,
                   tier, status, current_period_end, cancel_at_period_end,
                   created_at, updated_at
            FROM subscriptions
            WHERE user_id = $1
            LIMIT 1
            "#,
    )
    .bind(user_id)
    .fetch_optional(self.pool)
    .await;

    let duration = start.elapsed();
    match &result {
      Ok(Some(subscription)) => {
        info!(
          user_id = %user_id,
          subscription_id = %subscription.id,
          tier = %subscription.tier,
          status = ?subscription.status,
          duration_ms = duration.as_millis(),
          "Successfully found subscription for user"
        );
      }
      Ok(None) => {
        info!(
          user_id = %user_id,
          duration_ms = duration.as_millis(),
          "No subscription found for user"
        );
      }
      Err(e) => {
        error!(
          user_id = %user_id,
          error = %e,
          duration_ms = duration.as_millis(),
          "Failed to find subscription for user"
        );
      }
    }

    result.map_err(|e| {
      error!("Database error finding subscription: {}", e);
      SmoothieError::DatabaseError(e.to_string())
    })
  }

  /// Create or update subscription for a user
  #[instrument(skip(self), fields(user_id = %user_id, tier = %tier))]
  pub async fn upsert_subscription(
    &self,
    user_id: Uuid,
    stripe_customer_id: Option<String>,
    stripe_subscription_id: Option<String>,
    tier: String,
    status: Option<String>,
    current_period_end: Option<chrono::DateTime<Utc>>,
  ) -> Result<SubscriptionEntity> {
    info!("Upserting subscription for user");
    let start = std::time::Instant::now();

    let result = sqlx::query_as::<_, SubscriptionEntity>(
      r#"
            INSERT INTO subscriptions (
              user_id, stripe_customer_id, stripe_subscription_id,
              tier, status, current_period_end, created_at, updated_at
            ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8)
            ON CONFLICT (user_id)
            DO UPDATE SET
              stripe_customer_id = EXCLUDED.stripe_customer_id,
              stripe_subscription_id = EXCLUDED.stripe_subscription_id,
              tier = EXCLUDED.tier,
              status = EXCLUDED.status,
              current_period_end = EXCLUDED.current_period_end,
              updated_at = EXCLUDED.updated_at
            RETURNING id, user_id, stripe_customer_id, stripe_subscription_id,
                      tier, status, current_period_end, cancel_at_period_end,
                      created_at, updated_at
            "#,
    )
    .bind(user_id)
    .bind(stripe_customer_id)
    .bind(stripe_subscription_id)
    .bind(&tier)
    .bind(&status)
    .bind(current_period_end)
    .bind(Utc::now())
    .bind(Utc::now())
    .fetch_one(self.pool)
    .await;

    let duration = start.elapsed();
    match &result {
      Ok(subscription) => {
        info!(
          user_id = %user_id,
          subscription_id = %subscription.id,
          tier = %subscription.tier,
          status = ?subscription.status,
          duration_ms = duration.as_millis(),
          "Successfully upserted subscription for user"
        );
      }
      Err(e) => {
        error!(
          user_id = %user_id,
          tier = %tier,
          error = %e,
          duration_ms = duration.as_millis(),
          "Failed to upsert subscription for user"
        );
      }
    }

    result.map_err(|e| {
      error!("Database error upserting subscription: {}", e);
      SmoothieError::DatabaseError(e.to_string())
    })
  }

  /// Delete subscription for a user
  #[instrument(skip(self), fields(user_id = %user_id))]
  pub async fn delete_by_user_id(&self, user_id: Uuid) -> Result<()> {
    info!("Deleting subscription for user");
    let start = std::time::Instant::now();

    let result = sqlx::query("DELETE FROM subscriptions WHERE user_id = $1")
      .bind(user_id)
      .execute(self.pool)
      .await;

    let duration = start.elapsed();
    match &result {
      Ok(rows_affected) => {
        info!(
          user_id = %user_id,
          rows_affected = rows_affected.rows_affected(),
          duration_ms = duration.as_millis(),
          "Successfully deleted subscription for user"
        );
      }
      Err(e) => {
        error!(
          user_id = %user_id,
          error = %e,
          duration_ms = duration.as_millis(),
          "Failed to delete subscription for user"
        );
      }
    }

    result.map(|_| ()).map_err(|e| {
      error!("Database error deleting subscription: {}", e);
      SmoothieError::DatabaseError(e.to_string())
    })
  }
}
