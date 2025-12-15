use crate::{
  error::Result,
  models::SuccessResponse,
  services::{InstalledApp, RunningApp, SystemMonitor, SystemService, SystemWindow},
  state::AppState,
};
use std::sync::Arc;
use tauri::State;

/// Check if the app has screen recording permission (required for display configuration)
#[tauri::command(rename_all = "camelCase")]
pub async fn check_display_permission(
  _state: State<'_, Arc<AppState>>,
) -> Result<SuccessResponse<bool>> {
  let has_permission = SystemService::check_display_permission();
  Ok(SuccessResponse {
    success: true,
    data: has_permission,
  })
}

/// Request screen recording permission from the user
#[tauri::command(rename_all = "camelCase")]
pub async fn request_display_permission(
  _state: State<'_, Arc<AppState>>,
) -> Result<SuccessResponse<bool>> {
  let granted = SystemService::request_display_permission();
  Ok(SuccessResponse {
    success: true,
    data: granted,
  })
}

/// Get all currently connected monitors with their properties
#[tauri::command(rename_all = "camelCase")]
pub async fn get_connected_monitors(
  _state: State<'_, Arc<AppState>>,
) -> Result<SuccessResponse<Vec<SystemMonitor>>> {
  let monitors = SystemService::get_monitors();

  Ok(SuccessResponse {
    success: true,
    data: monitors,
  })
}

/// Get all visible windows with their positions and sizes
#[tauri::command(rename_all = "camelCase")]
pub async fn get_visible_windows(
  _state: State<'_, Arc<AppState>>,
) -> Result<SuccessResponse<Vec<SystemWindow>>> {
  let windows = SystemService::get_windows();

  Ok(SuccessResponse {
    success: true,
    data: windows,
  })
}

/// Get all running applications
#[tauri::command(rename_all = "camelCase")]
pub async fn get_running_apps(
  _state: State<'_, Arc<AppState>>,
) -> Result<SuccessResponse<Vec<RunningApp>>> {
  let apps = SystemService::get_running_apps();

  Ok(SuccessResponse {
    success: true,
    data: apps,
  })
}

/// Get all installed applications on the system
#[tauri::command(rename_all = "camelCase")]
pub async fn get_installed_apps(
  _state: State<'_, Arc<AppState>>,
) -> Result<SuccessResponse<Vec<InstalledApp>>> {
  let apps = SystemService::get_installed_apps();

  Ok(SuccessResponse {
    success: true,
    data: apps,
  })
}

/// Capture the current layout (monitors + windows) for saving to a profile
#[tauri::command(rename_all = "camelCase")]
pub async fn capture_current_layout(
  _state: State<'_, Arc<AppState>>,
) -> Result<SuccessResponse<serde_json::Value>> {
  // Use optimized single-call method to avoid double window detection
  let (monitors, windows, apps) = SystemService::capture_system_layout();

  let layout = serde_json::json!({
      "capturedAt": chrono::Utc::now().to_rfc3339(),
      "monitors": monitors,
      "windows": windows,
      "runningApps": apps,
  });

  Ok(SuccessResponse {
    success: true,
    data: layout,
  })
}

/// Apply a monitor layout configuration to the system
#[tauri::command(rename_all = "camelCase")]
pub async fn apply_monitor_layout(
  _state: State<'_, Arc<AppState>>,
  monitors: Vec<SystemMonitor>,
) -> Result<SuccessResponse<String>> {
  // Log incoming monitor positions for debugging
  tracing::info!(
    "apply_monitor_layout called with {} monitors:",
    monitors.len()
  );
  for m in &monitors {
    tracing::info!(
      "  Monitor {}: {}x{} at ({}, {})",
      m.display_id,
      m.width,
      m.height,
      m.x,
      m.y
    );
  }

  // Skip native CoreGraphics API - it reports success but doesn't actually move monitors
  // Go directly to displayplacer via AppleScript (prompts for admin password, actually works)
  match SystemService::apply_monitor_layout_applescript(&monitors).await {
    Ok(()) => Ok(SuccessResponse {
      success: true,
      data: "Monitor layout applied successfully".to_string(),
    }),
    Err(e) => {
      tracing::warn!("AppleScript method failed: {:?}", e);
      // Fall back to direct execution
      match SystemService::apply_monitor_layout(monitors) {
        Ok(()) => Ok(SuccessResponse {
          success: true,
          data: "Monitor layout applied successfully".to_string(),
        }),
        Err(e) => {
          tracing::error!("apply_monitor_layout command failed: {:?}", e);
          let error_msg = e.to_string();
          if error_msg.contains("Please run this command manually") {
            Ok(SuccessResponse {
              success: true,
              data: format!("MANUAL_COMMAND:{}", error_msg),
            })
          } else {
            Err(e)
          }
        }
      }
    }
  }
}
