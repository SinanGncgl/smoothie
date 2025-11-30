use crate::{
    models::SuccessResponse,
    services::MonitorService,
    state::AppState,
    error::Result,
};
use std::sync::Arc;
use tauri::State;

#[tauri::command(rename_all = "camelCase")]
pub async fn create_monitor(
    state: State<'_, Arc<AppState>>,
    profile_id: String,
    name: String,
    resolution: String,
    orientation: String,
    is_primary: bool,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
    display_index: i32,
) -> Result<SuccessResponse<serde_json::Value>> {
    let monitor = MonitorService::create_monitor(
        &state.db,
        &profile_id,
        name,
        resolution,
        orientation,
        is_primary,
        x,
        y,
        width,
        height,
        display_index,
    ).await?;

    state.invalidate_cache(&format!("monitors_{}", profile_id));

    Ok(SuccessResponse {
        success: true,
        data: serde_json::to_value(monitor)?,
    })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn get_monitors(
    state: State<'_, Arc<AppState>>,
    profile_id: String,
) -> Result<SuccessResponse<Vec<serde_json::Value>>> {
    let monitors = MonitorService::get_monitors(&state.db, &profile_id).await?;
    let data: Vec<serde_json::Value> = monitors.into_iter().map(|m| serde_json::to_value(m).unwrap()).collect();

    Ok(SuccessResponse {
        success: true,
        data,
    })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn update_monitor(
    state: State<'_, Arc<AppState>>,
    monitor_id: String,
    x: i32,
    y: i32,
    width: i32,
    height: i32,
) -> Result<SuccessResponse<serde_json::Value>> {
    let monitor = MonitorService::update_monitor(&state.db, &monitor_id, x, y, width, height).await?;
    state.invalidate_cache(&format!("monitor_{}", monitor_id));

    Ok(SuccessResponse {
        success: true,
        data: serde_json::to_value(monitor)?,
    })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn delete_monitor(
    state: State<'_, Arc<AppState>>,
    monitor_id: String,
) -> Result<SuccessResponse<String>> {
    MonitorService::delete_monitor(&state.db, &monitor_id).await?;

    Ok(SuccessResponse {
        success: true,
        data: "Monitor deleted successfully".to_string(),
    })
}
