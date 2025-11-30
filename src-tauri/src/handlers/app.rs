use crate::{
    models::SuccessResponse,
    services::AppService,
    state::AppState,
    error::Result,
};
use std::sync::Arc;
use tauri::State;

#[tauri::command(rename_all = "camelCase")]
pub async fn create_app(
    state: State<'_, Arc<AppState>>,
    profile_id: String,
    name: String,
    bundle_id: String,
    exe_path: Option<String>,
    launch_on_activate: bool,
    monitor_preference: Option<i32>,
) -> Result<SuccessResponse<serde_json::Value>> {
    let app = AppService::create_app(
        &state.db,
        &profile_id,
        name,
        bundle_id,
        exe_path,
        launch_on_activate,
        monitor_preference,
    ).await?;

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
    let data: Vec<serde_json::Value> = apps.into_iter().map(|a| serde_json::to_value(a).unwrap()).collect();

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
) -> Result<SuccessResponse<Vec<String>>> {
    let apps = AppService::get_launchable_apps(&state.db, &profile_id).await?;
    let app_names: Vec<String> = apps.into_iter().map(|a| a.name).collect();

    tracing::info!("Launching {} apps for profile {}", app_names.len(), profile_id);

    Ok(SuccessResponse {
        success: true,
        data: app_names,
    })
}
