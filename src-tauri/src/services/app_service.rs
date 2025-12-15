// App service - manage application configurations

use crate::{
  db::Database,
  error::{Result, SmoothieError},
  models::dto::AppDto,
  repositories::AppRepository,
};
use std::process::Command;
use uuid::Uuid;

/// Helper to parse UUID from string
fn parse_uuid(s: &str) -> Result<Uuid> {
  Uuid::parse_str(s).map_err(|_| SmoothieError::ValidationError(format!("Invalid UUID: {}", s)))
}

pub struct AppService;

/// Result of launching an app
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LaunchResult {
  pub name: String,
  pub success: bool,
  pub message: String,
}

impl AppService {
  pub async fn create_app(
    db: &Database,
    profile_id: &str,
    user_id: &str,
    name: String,
    bundle_id: String,
    exe_path: Option<String>,
    launch_on_activate: bool,
    monitor_preference: Option<i32>,
    startup_delay_ms: Option<i32>,
    order_index: Option<i32>,
  ) -> Result<AppDto> {
    let profile_uuid = parse_uuid(profile_id)?;
    let user_uuid = parse_uuid(user_id)?;
    let repo = AppRepository::new(db.pool());

    let entity = repo
      .create(
        profile_uuid,
        &name,
        &bundle_id,
        exe_path.as_deref(),
        launch_on_activate,
        monitor_preference,
        startup_delay_ms,
        order_index,
      )
      .await?;

    // Log the app creation activity
    let audit_repo = crate::repositories::AuditRepository::new(db.pool());
    let _ = audit_repo.log_activity(
      user_uuid,
      None, // session_id
      "app_created",
      Some("app"),
      Some(entity.id),
      Some(&name),
      Some(serde_json::json!({
        "bundle_id": bundle_id,
        "profile_id": profile_id,
        "launch_on_activate": launch_on_activate
      })),
      "success",
      None,
      None,
    ).await;

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

  /// Launch an application by bundle ID (macOS)
  pub fn launch_app_by_bundle_id(bundle_id: &str, name: &str) -> LaunchResult {
    tracing::info!("Launching app: {} ({})", name, bundle_id);

    // Use 'open' command with bundle identifier on macOS
    let result = Command::new("open").arg("-b").arg(bundle_id).spawn();

    match result {
      Ok(_) => LaunchResult {
        name: name.to_string(),
        success: true,
        message: format!("Launched {}", name),
      },
      Err(e) => {
        tracing::error!("Failed to launch {}: {}", name, e);
        LaunchResult {
          name: name.to_string(),
          success: false,
          message: format!("Failed to launch: {}", e),
        }
      }
    }
  }

  /// Launch all launchable apps for a profile
  pub async fn launch_profile_apps(db: &Database, profile_id: &str, user_id: &str) -> Result<Vec<LaunchResult>> {
    let apps = Self::get_launchable_apps(db, profile_id).await?;
    let mut results = Vec::new();

    let profile_uuid = parse_uuid(profile_id)?;
    let user_uuid = parse_uuid(user_id)?;
    let audit_repo = crate::repositories::AuditRepository::new(db.pool());

    // Get current active profile activation for this user
    let active_activation = audit_repo.get_active_profile_activation(user_uuid).await.ok().flatten();

    for app in apps {
      let app_uuid = parse_uuid(&app.id)?;
      let result = Self::launch_app_by_bundle_id(&app.bundle_id, &app.name);
      
      // Log the app launch
      let _ = audit_repo.record_app_launch(
        user_uuid,
        Some(profile_uuid),
        active_activation.as_ref().map(|a| a.id),
        Some(app_uuid),
        &app.bundle_id,
        &app.name,
        app.exe_path.as_deref(),
        result.success,
        if result.success { None } else { Some(&result.message) },
        None, // pid - could be captured if needed
        None, // launch_duration_ms - could be measured
        false, // window_positioned - will be set when windows are positioned
      ).await;

      results.push(result);
      // Small delay between launches to avoid overwhelming the system
      tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    }

    Ok(results)
  }
}
