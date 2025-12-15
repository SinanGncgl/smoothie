use crate::services::app_service::LaunchResult;
use crate::services::browser_service::OpenTabResult;
use crate::{
  error::Result,
  models::{CreateProfileRequest, SuccessResponse},
  services::{AppService, BrowserService, MonitorService, ProfileService, SystemService},
  state::AppState,
};
use std::sync::Arc;
use tauri::State;

/// Result of applying monitor layout
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct MonitorLayoutResult {
  pub applied: bool,
  pub monitor_count: usize,
  pub message: String,
}

/// Result of starting a profile
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct StartProfileResult {
  pub profile_id: String,
  pub apps_launched: Vec<LaunchResult>,
  pub tabs_opened: Vec<OpenTabResult>,
  pub monitor_layout: MonitorLayoutResult,
}

#[tauri::command(rename_all = "camelCase")]
pub async fn create_profile(
  state: State<'_, Arc<AppState>>,
  user_id: String,
  req: CreateProfileRequest,
) -> Result<SuccessResponse<serde_json::Value>> {
  let profile_name = req.name.clone();
  let profile = ProfileService::create_profile(&state.db, &user_id, req).await?;
  state.invalidate_cache(&format!("profiles_{}", user_id));

  // Log the creation as a system event
  let _ = crate::services::audit_service::AUDIT_SERVICE
    .log_system_event(
      &state.db,
      "profile_created",
      "info",
      "ProfileHandler",
      &format!("Profile '{}' was created", profile_name),
      Some(serde_json::json!({
        "profile_id": profile.id,
        "profile_name": profile_name
      })),
      None,
    )
    .await;

  Ok(SuccessResponse {
    success: true,
    data: serde_json::to_value(profile)?,
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn get_profiles(
  state: State<'_, Arc<AppState>>,
  user_id: String,
) -> Result<SuccessResponse<Vec<serde_json::Value>>> {
  tracing::info!("get_profiles called with user_id: {}", user_id);
  let profiles = ProfileService::get_profiles(&state.db, &user_id).await?;
  tracing::info!("get_profiles found {} profiles", profiles.len());
  let data: Vec<serde_json::Value> = profiles
    .into_iter()
    .map(|p| serde_json::to_value(p).unwrap())
    .collect();

  Ok(SuccessResponse {
    success: true,
    data,
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn get_profile(
  state: State<'_, Arc<AppState>>,
  profile_id: String,
) -> Result<SuccessResponse<serde_json::Value>> {
  let profile = ProfileService::get_profile_response(&state.db, &profile_id).await?;

  Ok(SuccessResponse {
    success: true,
    data: serde_json::to_value(profile)?,
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn update_profile(
  state: State<'_, Arc<AppState>>,
  profile_id: String,
  name: Option<String>,
  description: Option<String>,
  is_favorite: Option<bool>,
  color: Option<String>,
  icon: Option<String>,
  sort_order: Option<i32>,
) -> Result<SuccessResponse<serde_json::Value>> {
  let profile = ProfileService::update_profile_extended(
    &state.db,
    &profile_id,
    name,
    description,
    is_favorite,
    color,
    icon,
    sort_order,
  )
  .await?;
  state.invalidate_cache(&format!("profile_{}", profile_id));

  Ok(SuccessResponse {
    success: true,
    data: serde_json::to_value(profile)?,
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn delete_profile(
  state: State<'_, Arc<AppState>>,
  profile_id: String,
) -> Result<SuccessResponse<String>> {
  // Get profile name before deletion for logging
  let profile_name = ProfileService::get_profile(&state.db, &profile_id)
    .await
    .ok()
    .map(|p| p.name.clone())
    .unwrap_or_else(|| "Unknown".to_string());

  ProfileService::delete_profile(&state.db, &profile_id).await?;
  state.invalidate_cache(&format!("profile_{}", profile_id));

  // Log the deletion as a system event
  let _ = crate::services::audit_service::AUDIT_SERVICE
    .log_system_event(
      &state.db,
      "profile_deleted",
      "info",
      "ProfileHandler",
      &format!("Profile '{}' was deleted", profile_name),
      Some(serde_json::json!({
        "profile_id": profile_id,
        "profile_name": profile_name
      })),
      None,
    )
    .await;

  Ok(SuccessResponse {
    success: true,
    data: "Profile deleted successfully".to_string(),
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn activate_profile(
  state: State<'_, Arc<AppState>>,
  profile_id: String,
  user_id: String,
) -> Result<SuccessResponse<serde_json::Value>> {
  let profile = ProfileService::activate_profile(&state.db, &profile_id, &user_id).await?;
  state.invalidate_cache(&format!("profiles_{}", user_id));

  // Log the activation as a system event
  let _ = crate::services::audit_service::AUDIT_SERVICE
    .log_system_event(
      &state.db,
      "profile_activated",
      "info",
      "ProfileHandler",
      &format!("Profile '{}' was activated", profile.name),
      Some(serde_json::json!({
        "profile_id": profile_id,
        "profile_name": profile.name
      })),
      None,
    )
    .await;

  Ok(SuccessResponse {
    success: true,
    data: serde_json::to_value(profile)?,
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn duplicate_profile(
  state: State<'_, Arc<AppState>>,
  profile_id: String,
  user_id: String,
) -> Result<SuccessResponse<serde_json::Value>> {
  let profile = ProfileService::duplicate_profile(&state.db, &profile_id, &user_id).await?;
  state.invalidate_cache(&format!("profiles_{}", user_id));

  Ok(SuccessResponse {
    success: true,
    data: serde_json::to_value(profile)?,
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn get_favorite_profiles(
  state: State<'_, Arc<AppState>>,
  user_id: String,
) -> Result<SuccessResponse<Vec<serde_json::Value>>> {
  let profiles = ProfileService::get_favorite_profiles(&state.db, &user_id).await?;
  let data: Vec<serde_json::Value> = profiles
    .into_iter()
    .map(|p| serde_json::to_value(p).unwrap())
    .collect();

  Ok(SuccessResponse {
    success: true,
    data,
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn get_most_used_profiles(
  state: State<'_, Arc<AppState>>,
  user_id: String,
  limit: Option<i64>,
) -> Result<SuccessResponse<Vec<serde_json::Value>>> {
  let profiles =
    ProfileService::get_most_used_profiles(&state.db, &user_id, limit.unwrap_or(5)).await?;
  let data: Vec<serde_json::Value> = profiles
    .into_iter()
    .map(|p| serde_json::to_value(p).unwrap())
    .collect();

  Ok(SuccessResponse {
    success: true,
    data,
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn set_profile_favorite(
  state: State<'_, Arc<AppState>>,
  profile_id: String,
  is_favorite: bool,
) -> Result<SuccessResponse<serde_json::Value>> {
  let profile = ProfileService::set_favorite(&state.db, &profile_id, is_favorite).await?;
  state.invalidate_cache(&format!("profile_{}", profile_id));

  Ok(SuccessResponse {
    success: true,
    data: serde_json::to_value(profile)?,
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn start_profile(
  state: State<'_, Arc<AppState>>,
  profile_id: String,
  user_id: String,
) -> Result<SuccessResponse<StartProfileResult>> {
  tracing::info!("Starting profile: {}", profile_id);

  // Apply monitor layout first (before launching apps)
  let monitor_layout = match MonitorService::get_system_monitors(&state.db, &profile_id).await {
    Ok(monitors) if !monitors.is_empty() => {
      tracing::info!("Applying monitor layout with {} monitors", monitors.len());
      let monitor_count = monitors.len();

      // Try AppleScript method first, then fall back to direct execution
      match SystemService::apply_monitor_layout_applescript(&monitors).await {
        Ok(()) => MonitorLayoutResult {
          applied: true,
          monitor_count,
          message: "Monitor layout applied successfully".to_string(),
        },
        Err(e) => {
          tracing::warn!("AppleScript method failed: {:?}, trying direct method", e);
          match SystemService::apply_monitor_layout(monitors) {
            Ok(()) => MonitorLayoutResult {
              applied: true,
              monitor_count,
              message: "Monitor layout applied successfully".to_string(),
            },
            Err(e) => {
              let error_msg = e.to_string();
              tracing::warn!("Monitor layout application failed: {}", error_msg);
              MonitorLayoutResult {
                applied: false,
                monitor_count,
                message: format!("Failed to apply monitor layout: {}", error_msg),
              }
            }
          }
        }
      }
    }
    Ok(_) => {
      tracing::info!("No monitors configured for this profile");
      MonitorLayoutResult {
        applied: false,
        monitor_count: 0,
        message: "No monitor layout configured for this profile".to_string(),
      }
    }
    Err(e) => {
      tracing::warn!("Failed to get profile monitors: {:?}", e);
      MonitorLayoutResult {
        applied: false,
        monitor_count: 0,
        message: format!("Failed to load monitor layout: {}", e),
      }
    }
  };

  // Launch all launchable apps
  let apps_launched = AppService::launch_profile_apps(&state.db, &profile_id, &user_id).await?;

  // Open all browser tabs
  let tabs_opened = BrowserService::open_profile_tabs(&state.db, &profile_id).await?;

  let result = StartProfileResult {
    profile_id: profile_id.clone(),
    apps_launched,
    tabs_opened,
    monitor_layout,
  };

  tracing::info!(
    "Started profile {}: {} apps launched, {} tabs opened, monitor layout {}",
    profile_id,
    result.apps_launched.len(),
    result.tabs_opened.len(),
    if result.monitor_layout.applied {
      "applied"
    } else {
      "not applied"
    }
  );

  Ok(SuccessResponse {
    success: true,
    data: result,
  })
}
