use crate::{
  error::Result,
  models::SuccessResponse,
  services::window_service::{WindowDto, WindowService},
  state::AppState,
};
use std::sync::Arc;
use tauri::State;

#[tauri::command(rename_all = "camelCase")]
pub async fn create_window(
  state: State<'_, Arc<AppState>>,
  profile_id: String,
  app_id: String,
  monitor_id: String,
  x: i32,
  y: i32,
  width: i32,
  height: i32,
  is_maximized: bool,
  window_state: String,
) -> Result<SuccessResponse<WindowDto>> {
  let window = WindowService::create_window(
    &state.db,
    &profile_id,
    &app_id,
    &monitor_id,
    x,
    y,
    width,
    height,
    is_maximized,
    window_state,
  )
  .await?;

  Ok(SuccessResponse {
    success: true,
    data: window,
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn get_windows(
  state: State<'_, Arc<AppState>>,
  profile_id: String,
) -> Result<SuccessResponse<Vec<WindowDto>>> {
  let windows = WindowService::get_windows(&state.db, &profile_id).await?;

  Ok(SuccessResponse {
    success: true,
    data: windows,
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn update_window_position(
  state: State<'_, Arc<AppState>>,
  window_id: String,
  x: i32,
  y: i32,
  width: i32,
  height: i32,
) -> Result<SuccessResponse<WindowDto>> {
  let window =
    WindowService::update_window_position(&state.db, &window_id, x, y, width, height).await?;

  Ok(SuccessResponse {
    success: true,
    data: window,
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn delete_window(
  state: State<'_, Arc<AppState>>,
  window_id: String,
) -> Result<SuccessResponse<String>> {
  WindowService::delete_window(&state.db, &window_id).await?;

  Ok(SuccessResponse {
    success: true,
    data: "Window deleted successfully".to_string(),
  })
}
