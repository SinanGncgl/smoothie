// Subscription handlers - manage user subscriptions
//
// PRODUCTION NOTE: For production deployment, you need to set up Stripe webhooks
// to automatically update subscription status when payments succeed.
// Since this is a desktop app, you'll need a backend service to receive webhooks.
//
// Example webhook handler (to be implemented in your backend service):
//
// POST /webhooks/stripe
// 1. Verify Stripe webhook signature
// 2. Parse event (customer.subscription.created, customer.subscription.updated, etc.)
// 3. Update subscription in database via Tauri command or direct DB access
// 4. Handle failed payments, cancellations, etc.
//
// For development/testing, you can manually create subscriptions using the
// create_subscription command, or use Stripe's dashboard to manage subscriptions.

use crate::{
  error::Result, models::SuccessResponse, repositories::SubscriptionRepository, state::AppState,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tauri::State;
use uuid::Uuid;

/// Subscription response DTO
#[derive(Debug, Clone, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct SubscriptionResponse {
  pub id: String,
  pub user_id: String,
  pub stripe_customer_id: Option<String>,
  pub stripe_subscription_id: Option<String>,
  pub tier: String,
  pub status: Option<String>,
  pub current_period_end: Option<String>,
  pub cancel_at_period_end: Option<bool>,
  pub created_at: String,
  pub updated_at: String,
}

/// Create subscription request for testing
#[derive(Debug, Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateSubscriptionRequest {
  pub user_id: String,
  pub tier: String,
  pub status: Option<String>,
  pub stripe_customer_id: Option<String>,
  pub stripe_subscription_id: Option<String>,
  pub current_period_end: Option<String>, // ISO 8601 date string
}

/// Get subscription for a user
#[tauri::command(rename_all = "camelCase")]
pub async fn get_subscription(
  state: State<'_, Arc<AppState>>,
  user_id: String,
) -> Result<SuccessResponse<Option<SubscriptionResponse>>> {
  let user_uuid = Uuid::parse_str(&user_id).map_err(|_| {
    crate::error::SmoothieError::ValidationError("Invalid user ID format".to_string())
  })?;

  let repo = SubscriptionRepository::new(state.db.pool());
  let subscription = repo.find_by_user_id(user_uuid).await?;

  let response = subscription.map(|sub| SubscriptionResponse {
    id: sub.id.to_string(),
    user_id: sub.user_id.to_string(),
    stripe_customer_id: sub.stripe_customer_id,
    stripe_subscription_id: sub.stripe_subscription_id,
    tier: sub.tier,
    status: sub.status,
    current_period_end: sub.current_period_end.map(|dt| dt.to_rfc3339()),
    cancel_at_period_end: sub.cancel_at_period_end,
    created_at: sub.created_at.to_rfc3339(),
    updated_at: sub.updated_at.to_rfc3339(),
  });

  Ok(SuccessResponse {
    success: true,
    data: response,
  })
}

/// Create or update subscription (for testing purposes)
#[tauri::command(rename_all = "camelCase")]
pub async fn create_subscription(
  state: State<'_, Arc<AppState>>,
  req: CreateSubscriptionRequest,
) -> Result<SuccessResponse<SubscriptionResponse>> {
  let user_uuid = Uuid::parse_str(&req.user_id).map_err(|_| {
    crate::error::SmoothieError::ValidationError("Invalid user ID format".to_string())
  })?;

  let current_period_end = if let Some(date_str) = &req.current_period_end {
    Some(
      chrono::DateTime::parse_from_rfc3339(date_str)
        .map_err(|_| {
          crate::error::SmoothieError::ValidationError("Invalid date format".to_string())
        })?
        .with_timezone(&Utc),
    )
  } else {
    // Default to 1 year from now for testing
    Some(Utc::now() + chrono::Duration::days(365))
  };

  let repo = SubscriptionRepository::new(state.db.pool());
  let subscription = repo
    .upsert_subscription(
      user_uuid,
      req.stripe_customer_id,
      req.stripe_subscription_id,
      req.tier,
      req.status,
      current_period_end,
    )
    .await?;

  let response = SubscriptionResponse {
    id: subscription.id.to_string(),
    user_id: subscription.user_id.to_string(),
    stripe_customer_id: subscription.stripe_customer_id,
    stripe_subscription_id: subscription.stripe_subscription_id,
    tier: subscription.tier,
    status: subscription.status,
    current_period_end: subscription.current_period_end.map(|dt| dt.to_rfc3339()),
    cancel_at_period_end: subscription.cancel_at_period_end,
    created_at: subscription.created_at.to_rfc3339(),
    updated_at: subscription.updated_at.to_rfc3339(),
  };

  Ok(SuccessResponse {
    success: true,
    data: response,
  })
}

/// Delete subscription for a user
#[tauri::command(rename_all = "camelCase")]
pub async fn delete_subscription(
  state: State<'_, Arc<AppState>>,
  user_id: String,
) -> Result<SuccessResponse<serde_json::Value>> {
  let user_uuid = Uuid::parse_str(&user_id).map_err(|_| {
    crate::error::SmoothieError::ValidationError("Invalid user ID format".to_string())
  })?;

  let repo = SubscriptionRepository::new(state.db.pool());
  repo.delete_by_user_id(user_uuid).await?;

  Ok(SuccessResponse {
    success: true,
    data: serde_json::json!({}),
  })
}
