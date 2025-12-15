use crate::services::app_service::LaunchResult;
use crate::{error::Result, models::SuccessResponse, services::AppService, state::AppState};
use std::sync::Arc;
use tauri::State;

#[tauri::command(rename_all = "camelCase")]
pub async fn create_app(
  state: State<'_, Arc<AppState>>,
  profile_id: String,
  user_id: String,
  name: String,
  bundle_id: String,
  exe_path: Option<String>,
  launch_on_activate: bool,
  monitor_preference: Option<i32>,
  startup_delay_ms: Option<i32>,
  order_index: Option<i32>,
) -> Result<SuccessResponse<serde_json::Value>> {
  let app = AppService::create_app(
    &state.db,
    &profile_id,
    &user_id,
    name,
    bundle_id,
    exe_path,
    launch_on_activate,
    monitor_preference,
    startup_delay_ms,
    order_index,
  )
  .await?;

  state.invalidate_cache(&format!("apps_{}", profile_id));

  Ok(SuccessResponse {
    success: true,
    data: serde_json::to_value(app)?,
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn get_apps(
  state: State<'_, Arc<AppState>>,
  profile_id: String,
) -> Result<SuccessResponse<Vec<serde_json::Value>>> {
  let apps = AppService::get_apps(&state.db, &profile_id).await?;
  let data: Vec<serde_json::Value> = apps
    .into_iter()
    .map(|a| serde_json::to_value(a).unwrap())
    .collect();

  Ok(SuccessResponse {
    success: true,
    data,
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn update_app(
  state: State<'_, Arc<AppState>>,
  app_id: String,
  launch_on_activate: Option<bool>,
) -> Result<SuccessResponse<serde_json::Value>> {
  let app = AppService::update_app(&state.db, &app_id, launch_on_activate).await?;

  Ok(SuccessResponse {
    success: true,
    data: serde_json::to_value(app)?,
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn delete_app(
  state: State<'_, Arc<AppState>>,
  app_id: String,
) -> Result<SuccessResponse<String>> {
  AppService::delete_app(&state.db, &app_id).await?;

  Ok(SuccessResponse {
    success: true,
    data: "App deleted successfully".to_string(),
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn launch_apps(
  state: State<'_, Arc<AppState>>,
  profile_id: String,
) -> Result<SuccessResponse<Vec<LaunchResult>>> {
  let results = AppService::launch_profile_apps(
    &state.db,
    &profile_id,
    "00000000-0000-0000-0000-000000000001",
  )
  .await?;

  tracing::info!("Launched {} apps for profile {}", results.len(), profile_id);

  Ok(SuccessResponse {
    success: true,
    data: results,
  })
}
