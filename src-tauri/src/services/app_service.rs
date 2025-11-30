// App service - manage application configurations

use crate::{
    db::Database,
    error::{Result, SmoothieError},
    models::dto::AppDto,
    repositories::AppRepository,
};
use uuid::Uuid;

/// Helper to parse UUID from string
fn parse_uuid(s: &str) -> Result<Uuid> {
    Uuid::parse_str(s).map_err(|_| SmoothieError::ValidationError(format!("Invalid UUID: {}", s)))
}

pub struct AppService;

impl AppService {
    pub async fn create_app(
        db: &Database,
        profile_id: &str,
        name: String,
        bundle_id: String,
        exe_path: Option<String>,
        launch_on_activate: bool,
        monitor_preference: Option<i32>,
    ) -> Result<AppDto> {
        let profile_uuid = parse_uuid(profile_id)?;
        let repo = AppRepository::new(db.pool());

        let entity = repo
            .create(
                profile_uuid,
                &name,
                &bundle_id,
                exe_path.as_deref(),
                launch_on_activate,
                monitor_preference,
            )
            .await?;

        Ok(AppDto::from(entity))
    }

    pub async fn get_apps(db: &Database, profile_id: &str) -> Result<Vec<AppDto>> {
        let profile_uuid = parse_uuid(profile_id)?;
        let repo = AppRepository::new(db.pool());

        let apps = repo.find_by_profile_id(profile_uuid).await?;
        Ok(apps.into_iter().map(AppDto::from).collect())
    }

    pub async fn get_launchable_apps(db: &Database, profile_id: &str) -> Result<Vec<AppDto>> {
        let profile_uuid = parse_uuid(profile_id)?;
        let repo = AppRepository::new(db.pool());

        let apps = repo.find_launchable(profile_uuid).await?;
        Ok(apps.into_iter().map(AppDto::from).collect())
    }

    pub async fn update_app(
        db: &Database,
        app_id: &str,
        launch_on_activate: Option<bool>,
    ) -> Result<AppDto> {
        let app_uuid = parse_uuid(app_id)?;
        let repo = AppRepository::new(db.pool());

        let entity = repo.update(app_uuid, launch_on_activate).await?;
        Ok(AppDto::from(entity))
    }

    pub async fn delete_app(db: &Database, app_id: &str) -> Result<()> {
        let app_uuid = parse_uuid(app_id)?;
        let repo = AppRepository::new(db.pool());

        let deleted = repo.delete(app_uuid).await?;
        if !deleted {
            return Err(SmoothieError::NotFound("App not found".into()));
        }

        Ok(())
    }
}
