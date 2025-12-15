// Browser service - manage browser tab configurations

use crate::{
  db::Database,
  error::{Result, SmoothieError},
  models::dto::BrowserTabDto,
  repositories::BrowserTabRepository,
};
use std::process::Command;
use uuid::Uuid;

/// Helper to parse UUID from string
fn parse_uuid(s: &str) -> Result<Uuid> {
  Uuid::parse_str(s).map_err(|_| SmoothieError::ValidationError(format!("Invalid UUID: {}", s)))
}

/// Result of opening a browser tab
#[derive(Debug, Clone, serde::Serialize)]
#[serde(rename_all = "camelCase")]
pub struct OpenTabResult {
  pub url: String,
  pub browser: String,
  pub success: bool,
  pub message: String,
}

pub struct BrowserService;

impl BrowserService {
  pub async fn create_browser_tab(
    db: &Database,
    profile_id: &str,
    url: String,
    browser: String,
    monitor_id: Option<String>,
    tab_order: i32,
    favicon: Option<String>,
  ) -> Result<BrowserTabDto> {
    let profile_uuid = parse_uuid(profile_id)?;
    let monitor_uuid = match monitor_id {
      Some(id) if !id.is_empty() && id != "00000000-0000-0000-0000-000000000000" => {
        Some(parse_uuid(&id)?)
      }
      _ => None,
    };
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

  /// Get the bundle ID for a browser name
  fn get_browser_bundle_id(browser: &str) -> &'static str {
    match browser.to_lowercase().as_str() {
      "safari" => "com.apple.Safari",
      "chrome" | "google chrome" => "com.google.Chrome",
      "firefox" | "mozilla firefox" => "org.mozilla.firefox",
      "arc" => "company.thebrowser.Browser",
      "brave" | "brave browser" => "com.brave.Browser",
      "edge" | "microsoft edge" => "com.microsoft.edgemac",
      "opera" => "com.operasoftware.Opera",
      "vivaldi" => "com.vivaldi.Vivaldi",
      _ => "com.apple.Safari", // Default to Safari
    }
  }

  /// Open a URL in a specific browser (macOS)
  pub fn open_url_in_browser(url: &str, browser: &str) -> OpenTabResult {
    tracing::info!("Opening URL {} in {}", url, browser);

    let bundle_id = Self::get_browser_bundle_id(browser);

    // Use 'open' command with browser bundle identifier
    let result = Command::new("open")
      .arg("-b")
      .arg(bundle_id)
      .arg(url)
      .spawn();

    match result {
      Ok(_) => OpenTabResult {
        url: url.to_string(),
        browser: browser.to_string(),
        success: true,
        message: format!("Opened in {}", browser),
      },
      Err(e) => {
        tracing::error!("Failed to open URL {} in {}: {}", url, browser, e);
        // Fallback to default browser
        let fallback = Command::new("open").arg(url).spawn();
        match fallback {
          Ok(_) => OpenTabResult {
            url: url.to_string(),
            browser: "default".to_string(),
            success: true,
            message: "Opened in default browser".to_string(),
          },
          Err(e2) => OpenTabResult {
            url: url.to_string(),
            browser: browser.to_string(),
            success: false,
            message: format!("Failed to open: {}", e2),
          },
        }
      }
    }
  }

  /// Open all browser tabs for a profile
  pub async fn open_profile_tabs(db: &Database, profile_id: &str) -> Result<Vec<OpenTabResult>> {
    let tabs = Self::get_browser_tabs(db, profile_id).await?;
    let mut results = Vec::new();

    for tab in tabs {
      let result = Self::open_url_in_browser(&tab.url, &tab.browser);
      results.push(result);
      // Small delay between opening tabs
      tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
    }

    Ok(results)
  }
}
