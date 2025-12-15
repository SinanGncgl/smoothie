use crate::services::browser_service::OpenTabResult;
use crate::{error::Result, models::SuccessResponse, services::BrowserService, state::AppState};
use std::sync::Arc;
use tauri::State;

#[tauri::command(rename_all = "camelCase")]
pub async fn create_browser_tab(
  state: State<'_, Arc<AppState>>,
  profile_id: String,
  url: String,
  browser: String,
  monitor_id: Option<String>,
  tab_order: i32,
  favicon: Option<String>,
) -> Result<SuccessResponse<serde_json::Value>> {
  let tab = BrowserService::create_browser_tab(
    &state.db,
    &profile_id,
    url,
    browser,
    monitor_id,
    tab_order,
    favicon,
  )
  .await?;

  state.invalidate_cache(&format!("browser_tabs_{}", profile_id));

  Ok(SuccessResponse {
    success: true,
    data: serde_json::to_value(tab)?,
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn get_browser_tabs(
  state: State<'_, Arc<AppState>>,
  profile_id: String,
) -> Result<SuccessResponse<Vec<serde_json::Value>>> {
  let tabs = BrowserService::get_browser_tabs(&state.db, &profile_id).await?;
  let data: Vec<serde_json::Value> = tabs
    .into_iter()
    .map(|t| serde_json::to_value(t).unwrap())
    .collect();

  Ok(SuccessResponse {
    success: true,
    data,
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn update_browser_tab(
  state: State<'_, Arc<AppState>>,
  tab_id: String,
  url: Option<String>,
) -> Result<SuccessResponse<serde_json::Value>> {
  let tab = BrowserService::update_browser_tab(&state.db, &tab_id, url).await?;

  Ok(SuccessResponse {
    success: true,
    data: serde_json::to_value(tab)?,
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn delete_browser_tab(
  state: State<'_, Arc<AppState>>,
  tab_id: String,
) -> Result<SuccessResponse<String>> {
  BrowserService::delete_browser_tab(&state.db, &tab_id).await?;

  Ok(SuccessResponse {
    success: true,
    data: "Browser tab deleted successfully".to_string(),
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn open_tabs(
  state: State<'_, Arc<AppState>>,
  profile_id: String,
) -> Result<SuccessResponse<Vec<OpenTabResult>>> {
  let results = BrowserService::open_profile_tabs(&state.db, &profile_id).await?;

  tracing::info!(
    "Opened {} browser tabs for profile {}",
    results.len(),
    profile_id
  );

  Ok(SuccessResponse {
    success: true,
    data: results,
  })
}
