use crate::{
  db::Database,
  error::{Result, SmoothieError},
  logging::METRICS,
  models::dto::AutomationRuleDto,
  repositories::AutomationRepository,
};
use chrono::{Datelike, Timelike, Utc};
use uuid::Uuid;

/// Helper to parse UUID from string
fn parse_uuid(s: &str) -> Result<Uuid> {
  Uuid::parse_str(s).map_err(|_| SmoothieError::ValidationError(format!("Invalid UUID: {}", s)))
}

pub struct AutomationService;

impl AutomationService {
  pub async fn create_rule(
    db: &Database,
    profile_id: &str,
    rule_type: String,
    trigger_config: serde_json::Value,
  ) -> Result<AutomationRuleDto> {
    let profile_uuid = parse_uuid(profile_id)?;
    let repo = AutomationRepository::new(db.pool());

    let entity = repo
      .create(profile_uuid, &rule_type, trigger_config)
      .await?;

    tracing::info!(rule_id = %entity.id, profile_id = %profile_id, "Automation rule created");

    Ok(AutomationRuleDto::from(entity))
  }

  pub async fn get_rules(db: &Database, profile_id: &str) -> Result<Vec<AutomationRuleDto>> {
    let profile_uuid = parse_uuid(profile_id)?;
    let repo = AutomationRepository::new(db.pool());

    let rules = repo.find_by_profile_id(profile_uuid).await?;
    Ok(rules.into_iter().map(AutomationRuleDto::from).collect())
  }

  pub async fn evaluate_schedule_triggers(db: &Database) -> Result<Vec<(String, String)>> {
    let _now = Utc::now();
    let _hour = _now.hour();
    let _minute = _now.minute();
    let _weekday = _now.weekday().num_days_from_monday() + 1;

    let repo = AutomationRepository::new(db.pool());
    let rules = repo.find_enabled_by_type("schedule").await?;

    let mut triggered = Vec::new();

    for rule in rules {
      // Parse trigger config and evaluate
      // This is a simplified version - full implementation would parse JSON
      triggered.push((rule.id.to_string(), rule.profile_id.to_string()));
      METRICS.record_automation_triggered();
    }

    Ok(triggered)
  }

  pub async fn toggle_rule(
    db: &Database,
    rule_id: &str,
    enabled: bool,
  ) -> Result<AutomationRuleDto> {
    let rule_uuid = parse_uuid(rule_id)?;
    let repo = AutomationRepository::new(db.pool());

    let entity = repo.toggle(rule_uuid, enabled).await?;
    Ok(AutomationRuleDto::from(entity))
  }

  pub async fn delete_rule(db: &Database, rule_id: &str) -> Result<()> {
    let rule_uuid = parse_uuid(rule_id)?;
    let repo = AutomationRepository::new(db.pool());

    let deleted = repo.delete(rule_uuid).await?;
    if !deleted {
      return Err(SmoothieError::NotFound("Rule not found".into()));
    }

    tracing::info!(rule_id = %rule_id, "Automation rule deleted");

    Ok(())
  }
}
