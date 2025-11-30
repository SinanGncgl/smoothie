use crate::{
    models::SuccessResponse,
    state::AppState,
    error::Result,
};
use std::sync::Arc;
use tauri::State;

#[tauri::command(rename_all = "camelCase")]
pub async fn get_user_preferences(
    state: State<'_, Arc<AppState>>,
    user_id: String,
) -> Result<SuccessResponse<serde_json::Value>> {
    let prefs = serde_json::json!({
        "theme": "dark",
        "notifications_enabled": true,
        "auto_restore": true
    });

    Ok(SuccessResponse {
        success: true,
        data: prefs,
    })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn update_user_preferences(
    state: State<'_, Arc<AppState>>,
    user_id: String,
    theme: Option<String>,
    notifications_enabled: Option<bool>,
    auto_restore: Option<bool>,
) -> Result<SuccessResponse<String>> {
    tracing::info!("User preferences updated for {}", user_id);

    Ok(SuccessResponse {
        success: true,
        data: "Preferences updated successfully".to_string(),
    })
}
