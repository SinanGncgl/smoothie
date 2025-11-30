// Automation repository - database operations for automation rules

use crate::error::{Result, SmoothieError};
use crate::models::entities::AutomationRuleEntity;
use chrono::Utc;
use sqlx::PgPool;
use uuid::Uuid;

pub struct AutomationRepository<'a> {
    pool: &'a PgPool,
}

impl<'a> AutomationRepository<'a> {
    pub fn new(pool: &'a PgPool) -> Self {
        Self { pool }
    }

    /// Find all automation rules for a profile
    pub async fn find_by_profile_id(&self, profile_id: Uuid) -> Result<Vec<AutomationRuleEntity>> {
        sqlx::query_as::<_, AutomationRuleEntity>(
            r#"
            SELECT id, profile_id, rule_type, trigger_config, is_enabled, created_at
            FROM automation_rules 
            WHERE profile_id = $1
            "#,
        )
        .bind(profile_id)
        .fetch_all(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))
    }

    /// Find enabled rules by type
    pub async fn find_enabled_by_type(&self, rule_type: &str) -> Result<Vec<AutomationRuleEntity>> {
        sqlx::query_as::<_, AutomationRuleEntity>(
            r#"
            SELECT id, profile_id, rule_type, trigger_config, is_enabled, created_at
            FROM automation_rules 
            WHERE rule_type = $1 AND is_enabled = true
            "#,
        )
        .bind(rule_type)
        .fetch_all(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))
    }

    /// Find an automation rule by ID
    pub async fn find_by_id(&self, id: Uuid) -> Result<Option<AutomationRuleEntity>> {
        sqlx::query_as::<_, AutomationRuleEntity>(
            r#"
            SELECT id, profile_id, rule_type, trigger_config, is_enabled, created_at
            FROM automation_rules 
            WHERE id = $1
            "#,
        )
        .bind(id)
        .fetch_optional(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))
    }

    /// Create a new automation rule
    pub async fn create(
        &self,
        profile_id: Uuid,
        rule_type: &str,
        trigger_config: serde_json::Value,
    ) -> Result<AutomationRuleEntity> {
        let id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO automation_rules (id, profile_id, rule_type, trigger_config, is_enabled, created_at)
            VALUES ($1, $2, $3, $4, true, $5)
            "#,
        )
        .bind(id)
        .bind(profile_id)
        .bind(rule_type)
        .bind(&trigger_config)
        .bind(now)
        .execute(self.pool)
        .await
        .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| SmoothieError::NotFound("Automation rule not found after creation".into()))
    }

    /// Toggle a rule's enabled state
    pub async fn toggle(&self, id: Uuid, enabled: bool) -> Result<AutomationRuleEntity> {
        sqlx::query("UPDATE automation_rules SET is_enabled = $1 WHERE id = $2")
            .bind(enabled)
            .bind(id)
            .execute(self.pool)
            .await
            .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        self.find_by_id(id)
            .await?
            .ok_or_else(|| SmoothieError::NotFound("Automation rule not found".into()))
    }

    /// Delete an automation rule
    pub async fn delete(&self, id: Uuid) -> Result<bool> {
        let result = sqlx::query("DELETE FROM automation_rules WHERE id = $1")
            .bind(id)
            .execute(self.pool)
            .await
            .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected() > 0)
    }

    /// Delete all automation rules for a profile
    pub async fn delete_by_profile_id(&self, profile_id: Uuid) -> Result<u64> {
        let result = sqlx::query("DELETE FROM automation_rules WHERE profile_id = $1")
            .bind(profile_id)
            .execute(self.pool)
            .await
            .map_err(|e| SmoothieError::DatabaseError(e.to_string()))?;

        Ok(result.rows_affected())
    }
}
