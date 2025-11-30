use crate::{
    db::Database,
    error::{Result, SmoothieError},
    models::{
        dto::{CreateProfileRequest, ProfileDto, ProfileResponse, MonitorDto, AppDto, BrowserTabDto},
    },
    repositories::{ProfileRepository, MonitorRepository, AppRepository, BrowserTabRepository},
};
use uuid::Uuid;

/// Service layer for profile operations
/// Coordinates between handlers and repositories
pub struct ProfileService;

impl ProfileService {
    /// Create a new profile
    pub async fn create_profile(
        db: &Database,
        user_id: &str,
        req: CreateProfileRequest,
    ) -> Result<ProfileDto> {
        let user_uuid = parse_uuid(user_id)?;
        let repo = ProfileRepository::new(db.pool());

        let entity = repo
            .create(
                user_uuid,
                &req.name,
                req.description.as_deref(),
                &req.profile_type,
            )
            .await?;

        // Insert tags if provided
        if let Some(tags) = req.tags {
            for tag in tags {
                repo.add_tag(entity.id, &tag).await?;
            }
        }

        tracing::info!(profile_id = %entity.id, user_id = %user_id, "Profile created");

        // Re-fetch to get updated data with tags
        let updated = repo.find_by_id(entity.id).await?
            .ok_or_else(|| SmoothieError::NotFound("Profile not found".into()))?;
        let tags = repo.find_tags(entity.id).await?;

        Ok(ProfileDto::from_entity(updated, tags))
    }

    /// Get all profiles for a user
    pub async fn get_profiles(db: &Database, user_id: &str) -> Result<Vec<ProfileDto>> {
        let user_uuid = parse_uuid(user_id)?;
        let repo = ProfileRepository::new(db.pool());

        let profiles = repo.find_by_user_id(user_uuid).await?;
        let mut result = Vec::with_capacity(profiles.len());

        for profile in profiles {
            let tags = repo.find_tags(profile.id).await?;
            result.push(ProfileDto::from_entity(profile, tags));
        }

        Ok(result)
    }

    /// Get a specific profile
    pub async fn get_profile(db: &Database, profile_id: &str) -> Result<ProfileDto> {
        let profile_uuid = parse_uuid(profile_id)?;
        let repo = ProfileRepository::new(db.pool());

        let profile = repo.find_by_id(profile_uuid).await?
            .ok_or_else(|| SmoothieError::NotFound("Profile not found".into()))?;
        let tags = repo.find_tags(profile.id).await?;

        Ok(ProfileDto::from_entity(profile, tags))
    }

    /// Get profile with full details (monitors, apps, browser tabs)
    pub async fn get_profile_response(db: &Database, profile_id: &str) -> Result<ProfileResponse> {
        let profile = Self::get_profile(db, profile_id).await?;
        let monitors = MonitorService::get_monitors(db, profile_id).await?;
        let apps = AppService::get_apps(db, profile_id).await?;
        let browser_tabs = BrowserService::get_browser_tabs(db, profile_id).await?;

        Ok(ProfileResponse {
            id: profile.id,
            name: profile.name,
            description: profile.description,
            profile_type: profile.profile_type,
            is_active: profile.is_active,
            tags: profile.tags,
            monitors,
            apps,
            browser_tabs,
            created_at: profile.created_at,
            last_used: profile.last_used,
        })
    }

    /// Update a profile
    pub async fn update_profile(
        db: &Database,
        profile_id: &str,
        name: Option<String>,
        description: Option<String>,
    ) -> Result<ProfileDto> {
        let profile_uuid = parse_uuid(profile_id)?;
        let repo = ProfileRepository::new(db.pool());

        let updated = repo.update(profile_uuid, name.as_deref(), description.as_deref()).await?;
        let tags = repo.find_tags(profile_uuid).await?;

        tracing::info!(profile_id = %profile_id, "Profile updated");

        Ok(ProfileDto::from_entity(updated, tags))
    }

    /// Delete a profile
    pub async fn delete_profile(db: &Database, profile_id: &str) -> Result<()> {
        let profile_uuid = parse_uuid(profile_id)?;
        let repo = ProfileRepository::new(db.pool());

        let deleted = repo.delete(profile_uuid).await?;
        if !deleted {
            return Err(SmoothieError::NotFound("Profile not found".into()));
        }

        tracing::info!(profile_id = %profile_id, "Profile deleted");
        Ok(())
    }

    /// Activate a profile (deactivates all others for the user)
    pub async fn activate_profile(db: &Database, profile_id: &str, user_id: &str) -> Result<ProfileDto> {
        let profile_uuid = parse_uuid(profile_id)?;
        let user_uuid = parse_uuid(user_id)?;
        let repo = ProfileRepository::new(db.pool());

        let activated = repo.activate(profile_uuid, user_uuid).await?;
        let tags = repo.find_tags(profile_uuid).await?;

        tracing::info!(profile_id = %profile_id, user_id = %user_id, "Profile activated");

        Ok(ProfileDto::from_entity(activated, tags))
    }

    /// Duplicate a profile
    pub async fn duplicate_profile(
        db: &Database,
        profile_id: &str,
        user_id: &str,
    ) -> Result<ProfileDto> {
        let source = Self::get_profile(db, profile_id).await?;
        let new_name = format!("{} (Copy)", source.name);

        let new_profile = Self::create_profile(
            db,
            user_id,
            CreateProfileRequest {
                name: new_name,
                description: source.description,
                profile_type: source.profile_type,
                tags: Some(source.tags),
            },
        )
        .await?;

        // Copy monitors
        let monitors = MonitorService::get_monitors(db, profile_id).await?;
        for monitor in monitors {
            MonitorService::create_monitor(
                db,
                &new_profile.id,
                monitor.name,
                monitor.resolution,
                monitor.orientation,
                monitor.is_primary,
                monitor.x,
                monitor.y,
                monitor.width,
                monitor.height,
                monitor.display_index,
            )
            .await?;
        }

        tracing::info!(
            source_id = %profile_id,
            new_id = %new_profile.id,
            "Profile duplicated"
        );

        Self::get_profile(db, &new_profile.id).await
    }
}

// Helper services
pub struct MonitorService;
pub struct AppService;
pub struct BrowserService;

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
}

impl AppService {
    pub async fn get_apps(db: &Database, profile_id: &str) -> Result<Vec<AppDto>> {
        let profile_uuid = parse_uuid(profile_id)?;
        let repo = AppRepository::new(db.pool());

        let apps = repo.find_by_profile_id(profile_uuid).await?;
        Ok(apps.into_iter().map(AppDto::from).collect())
    }
}

impl BrowserService {
    pub async fn get_browser_tabs(db: &Database, profile_id: &str) -> Result<Vec<BrowserTabDto>> {
        let profile_uuid = parse_uuid(profile_id)?;
        let repo = BrowserTabRepository::new(db.pool());

        let tabs = repo.find_by_profile_id(profile_uuid).await?;
        Ok(tabs.into_iter().map(BrowserTabDto::from).collect())
    }
}

/// Parse a string as UUID
fn parse_uuid(s: &str) -> Result<Uuid> {
    Uuid::parse_str(s).map_err(|_| SmoothieError::ValidationError(format!("Invalid UUID: {}", s)))
}