use crate::{
    models::{CreateProfileRequest, SuccessResponse},
    services::ProfileService,
    state::AppState,
    error::Result,
};
use std::sync::Arc;
use tauri::State;

#[tauri::command(rename_all = "camelCase")]
pub async fn create_profile(
    state: State<'_, Arc<AppState>>,
    user_id: String,
    req: CreateProfileRequest,
) -> Result<SuccessResponse<serde_json::Value>> {
    let profile = ProfileService::create_profile(&state.db, &user_id, req).await?;
    state.invalidate_cache(&format!("profiles_{}", user_id));

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
    let data: Vec<serde_json::Value> = profiles.into_iter().map(|p| serde_json::to_value(p).unwrap()).collect();

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
) -> Result<SuccessResponse<serde_json::Value>> {
    let profile = ProfileService::update_profile(&state.db, &profile_id, name, description).await?;
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
    ProfileService::delete_profile(&state.db, &profile_id).await?;
    state.invalidate_cache(&format!("profile_{}", profile_id));

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
