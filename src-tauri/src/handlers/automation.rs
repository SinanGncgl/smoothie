use crate::{error::Result, models::SuccessResponse, services::AutomationService, state::AppState};
use std::sync::Arc;
use tauri::State;

#[tauri::command(rename_all = "camelCase")]
pub async fn create_rule(
  state: State<'_, Arc<AppState>>,
  profile_id: String,
  rule_type: String,
  trigger_config: serde_json::Value,
) -> Result<SuccessResponse<serde_json::Value>> {
  let rule =
    AutomationService::create_rule(&state.db, &profile_id, rule_type, trigger_config).await?;

  state.invalidate_cache(&format!("rules_{}", profile_id));

  Ok(SuccessResponse {
    success: true,
    data: serde_json::to_value(rule)?,
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn get_rules(
  state: State<'_, Arc<AppState>>,
  profile_id: String,
) -> Result<SuccessResponse<Vec<serde_json::Value>>> {
  let rules = AutomationService::get_rules(&state.db, &profile_id).await?;
  let data: Vec<serde_json::Value> = rules
    .into_iter()
    .map(|r| serde_json::to_value(r).unwrap())
    .collect();

  Ok(SuccessResponse {
    success: true,
    data,
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn update_rule(
  state: State<'_, Arc<AppState>>,
  rule_id: String,
  enabled: bool,
) -> Result<SuccessResponse<serde_json::Value>> {
  let rule = AutomationService::toggle_rule(&state.db, &rule_id, enabled).await?;

  Ok(SuccessResponse {
    success: true,
    data: serde_json::to_value(rule)?,
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn delete_rule(
  state: State<'_, Arc<AppState>>,
  rule_id: String,
) -> Result<SuccessResponse<String>> {
  AutomationService::delete_rule(&state.db, &rule_id).await?;

  Ok(SuccessResponse {
    success: true,
    data: "Rule deleted successfully".to_string(),
  })
}

#[tauri::command(rename_all = "camelCase")]
pub async fn evaluate_rules(
  state: State<'_, Arc<AppState>>,
) -> Result<SuccessResponse<Vec<(String, String)>>> {
  let triggered = AutomationService::evaluate_schedule_triggers(&state.db).await?;

  tracing::info!("Evaluated rules, triggered count: {}", triggered.len());

  Ok(SuccessResponse {
    success: true,
    data: triggered,
  })
}
