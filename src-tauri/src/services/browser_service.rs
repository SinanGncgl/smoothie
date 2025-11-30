// Browser service - manage browser tab configurations

use crate::{
    db::Database,
    error::{Result, SmoothieError},
    models::dto::BrowserTabDto,
    repositories::BrowserTabRepository,
};
use uuid::Uuid;

/// Helper to parse UUID from string
fn parse_uuid(s: &str) -> Result<Uuid> {
    Uuid::parse_str(s).map_err(|_| SmoothieError::ValidationError(format!("Invalid UUID: {}", s)))
}

pub struct BrowserService;

impl BrowserService {
    pub async fn create_browser_tab(
        db: &Database,
        profile_id: &str,
        url: String,
        browser: String,
        monitor_id: String,
        tab_order: i32,
        favicon: Option<String>,
    ) -> Result<BrowserTabDto> {
        let profile_uuid = parse_uuid(profile_id)?;
        let monitor_uuid = parse_uuid(&monitor_id)?;
        let repo = BrowserTabRepository::new(db.pool());

        let entity = repo
            .create(
                profile_uuid,
                &url,
                &browser,
                monitor_uuid,
                tab_order,
                favicon.as_deref(),
            )
            .await?;

        Ok(BrowserTabDto::from(entity))
    }

    pub async fn get_browser_tabs(db: &Database, profile_id: &str) -> Result<Vec<BrowserTabDto>> {
        let profile_uuid = parse_uuid(profile_id)?;
        let repo = BrowserTabRepository::new(db.pool());

        let tabs = repo.find_by_profile_id(profile_uuid).await?;
        Ok(tabs.into_iter().map(BrowserTabDto::from).collect())
    }

    pub async fn update_browser_tab(
        db: &Database,
        tab_id: &str,
        url: Option<String>,
    ) -> Result<BrowserTabDto> {
        let tab_uuid = parse_uuid(tab_id)?;
        let repo = BrowserTabRepository::new(db.pool());

        let entity = repo.update(tab_uuid, url.as_deref()).await?;
        Ok(BrowserTabDto::from(entity))
    }

    pub async fn delete_browser_tab(db: &Database, tab_id: &str) -> Result<()> {
        let tab_uuid = parse_uuid(tab_id)?;
        let repo = BrowserTabRepository::new(db.pool());

        let deleted = repo.delete(tab_uuid).await?;
        if !deleted {
            return Err(SmoothieError::NotFound("Browser tab not found".into()));
        }

        Ok(())
    }
}
