use crate::{
    models::SuccessResponse,
    services::SyncService,
    state::AppState,
    error::Result,
};
use std::sync::Arc;
use tauri::State;

#[tauri::command(rename_all = "camelCase")]
pub async fn backup_profiles(
    state: State<'_, Arc<AppState>>,
    user_id: String,
    device_id: String,
) -> Result<SuccessResponse<serde_json::Value>> {
    let backup = SyncService::backup_profiles(&state.db, &user_id, &device_id).await?;

    Ok(SuccessResponse {
        success: true,
        data: backup,
    })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn restore_profiles(
    state: State<'_, Arc<AppState>>,
    user_id: String,
    backup: serde_json::Value,
    conflict_strategy: String,
) -> Result<SuccessResponse<String>> {
    SyncService::restore_profiles(&state.db, &user_id, backup, &conflict_strategy).await?;

    Ok(SuccessResponse {
        success: true,
        data: format!("Profiles restored with {} strategy", conflict_strategy),
    })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn get_sync_status(
    state: State<'_, Arc<AppState>>,
    user_id: String,
) -> Result<SuccessResponse<serde_json::Value>> {
    let status = SyncService::get_sync_status(&state.db, &user_id).await?;

    Ok(SuccessResponse {
        success: true,
        data: status,
    })
}
