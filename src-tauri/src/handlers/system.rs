use crate::{
    models::SuccessResponse,
    state::AppState,
    error::Result,
};
use std::sync::Arc;
use tauri::State;

#[tauri::command(rename_all = "camelCase")]
pub async fn get_connected_monitors(
    _state: State<'_, Arc<AppState>>,
) -> Result<SuccessResponse<Vec<serde_json::Value>>> {
    let monitors = serde_json::json!([
        {
            "id": "monitor-1",
            "name": "Primary",
            "resolution": "2560x1440",
            "is_primary": true
        }
    ]);

    Ok(SuccessResponse {
        success: true,
        data: vec![monitors],
    })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn get_running_apps(
    _state: State<'_, Arc<AppState>>,
) -> Result<SuccessResponse<Vec<serde_json::Value>>> {
    let apps = serde_json::json!([
        {
            "name": "Visual Studio Code",
            "bundle_id": "com.microsoft.VSCode",
            "pid": 12345
        }
    ]);

    Ok(SuccessResponse {
        success: true,
        data: vec![apps],
    })
}
