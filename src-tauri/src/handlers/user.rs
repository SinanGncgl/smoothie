use crate::{
  error::{Result, SmoothieError},
  models::{SuccessResponse, UserSettingsDto},
  services::UserSettingsService,
  state::AppState,
};
use std::sync::Arc;
use tauri::State;
use uuid::Uuid;

#[tauri::command(rename_all = "camelCase")]
pub async fn get_user_settings(
  state: State<'_, Arc<AppState>>,
  user_id: String,
) -> Result<SuccessResponse<UserSettingsDto>> {
  let user_uuid = Uuid::parse_str(&user_id)
    .map_err(|e| SmoothieError::ValidationError(format!("Invalid user ID: {}", e)))?;

  let settings = UserSettingsService::get_settings(&state.db, user_uuid).await?;

  Ok(SuccessResponse {
    success: true,
    data: settings,
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn update_user_settings(
  state: State<'_, Arc<AppState>>,
  user_id: String,
  theme: Option<String>,
  auto_restore: Option<bool>,
  monitor_detection: Option<bool>,
  animations_enabled: Option<bool>,
  cloud_sync: Option<bool>,
  auto_activate_time: Option<String>,
  keyboard_shortcut: Option<String>,
  notifications_enabled: Option<bool>,
) -> Result<SuccessResponse<UserSettingsDto>> {
  let user_uuid = Uuid::parse_str(&user_id)
    .map_err(|e| SmoothieError::ValidationError(format!("Invalid user ID: {}", e)))?;

  let settings = UserSettingsService::update_settings(
    &state.db,
    user_uuid,
    theme,
    auto_restore,
    monitor_detection,
    animations_enabled,
    cloud_sync,
    auto_activate_time,
    keyboard_shortcut,
    notifications_enabled,
  )
  .await?;

  tracing::info!("User settings updated for {}", user_id);

  Ok(SuccessResponse {
    success: true,
    data: settings,
  })
}

// Keep old function names as aliases for backward compatibility
#[tauri::command(rename_all = "camelCase")]
pub async fn get_user_preferences(
  state: State<'_, Arc<AppState>>,
  user_id: String,
) -> Result<SuccessResponse<UserSettingsDto>> {
  get_user_settings(state, user_id).await
}

#[tauri::command(rename_all = "camelCase")]
pub async fn update_user_preferences(
  state: State<'_, Arc<AppState>>,
  user_id: String,
  theme: Option<String>,
  notifications_enabled: Option<bool>,
  auto_restore: Option<bool>,
) -> Result<SuccessResponse<UserSettingsDto>> {
  update_user_settings(
    state,
    user_id,
    theme,
    auto_restore,
    None, // monitor_detection
    None, // animations_enabled
    None, // cloud_sync
    None, // auto_activate_time
    None, // keyboard_shortcut
    notifications_enabled,
  )
  .await
}
