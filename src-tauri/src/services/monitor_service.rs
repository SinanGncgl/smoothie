// Monitor service - manage monitor configurations

use crate::{
    db::Database,
    error::{Result, SmoothieError},
    models::dto::MonitorDto,
    repositories::MonitorRepository,
};
use uuid::Uuid;

/// Helper to parse UUID from string
fn parse_uuid(s: &str) -> Result<Uuid> {
    Uuid::parse_str(s).map_err(|_| SmoothieError::ValidationError(format!("Invalid UUID: {}", s)))
}

pub struct MonitorService;

impl MonitorService {
    pub async fn create_monitor(
        db: &Database,
        profile_id: &str,
        name: String,
        resolution: String,
        orientation: String,
        is_primary: bool,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
        display_index: i32,
    ) -> Result<MonitorDto> {
        let profile_uuid = parse_uuid(profile_id)?;
        let repo = MonitorRepository::new(db.pool());

        let entity = repo
            .create(
                profile_uuid,
                &name,
                &resolution,
                &orientation,
                is_primary,
                x,
                y,
                width,
                height,
                display_index,
            )
            .await?;

        Ok(MonitorDto::from(entity))
    }

    pub async fn get_monitors(db: &Database, profile_id: &str) -> Result<Vec<MonitorDto>> {
        let profile_uuid = parse_uuid(profile_id)?;
        let repo = MonitorRepository::new(db.pool());

        let monitors = repo.find_by_profile_id(profile_uuid).await?;
        Ok(monitors.into_iter().map(MonitorDto::from).collect())
    }

    pub async fn update_monitor(
        db: &Database,
        monitor_id: &str,
        x: i32,
        y: i32,
        width: i32,
        height: i32,
    ) -> Result<MonitorDto> {
        let monitor_uuid = parse_uuid(monitor_id)?;
        let repo = MonitorRepository::new(db.pool());

        let entity = repo.update_position(monitor_uuid, x, y, width, height).await?;
        Ok(MonitorDto::from(entity))
    }

    pub async fn delete_monitor(db: &Database, monitor_id: &str) -> Result<()> {
        let monitor_uuid = parse_uuid(monitor_id)?;
        let repo = MonitorRepository::new(db.pool());

        let deleted = repo.delete(monitor_uuid).await?;
        if !deleted {
            return Err(SmoothieError::NotFound("Monitor not found".into()));
        }

        Ok(())
    }
}
