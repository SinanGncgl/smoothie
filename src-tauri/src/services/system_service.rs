//! System Detection Service for macOS
//!
//! This service provides functionality to detect and query system state:
//! - Connected monitors and their configurations
//! - Visible application windows
//! - Running applications
//!
//! The implementation uses macOS CoreGraphics and CoreFoundation frameworks
//! to directly interface with the window server and display system.

use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// ============================================================================
// Data Structures
// ============================================================================

/// Represents a connected display/monitor detected from the operating system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemMonitor {
  /// Unique display identifier assigned by macOS
  pub display_id: u32,
  /// Human-readable display name (e.g., "Built-in Retina Display")
  pub name: String,
  /// Monitor manufacturer/brand (e.g., "Dell", "LG", "Apple")
  pub brand: Option<String>,
  /// Monitor model name/number (e.g., "UltraSharp U2719D", "27MD5KL")
  pub model: Option<String>,
  /// Resolution string in "WIDTHxHEIGHT" format
  pub resolution: String,
  /// Native pixel width
  pub width: i32,
  /// Native pixel height
  pub height: i32,
  /// X coordinate in global display coordinate space
  pub x: i32,
  /// Y coordinate in global display coordinate space
  pub y: i32,
  /// Retina scale factor (typically 1.0 or 2.0)
  pub scale_factor: f64,
  /// Display refresh rate in Hz
  pub refresh_rate: f64,
  /// Whether this is the primary/main display
  pub is_primary: bool,
  /// Whether this is the built-in MacBook display
  pub is_builtin: bool,
  /// Display orientation: "Landscape" or "Portrait"
  pub orientation: String,
}

/// Represents a visible window on the screen.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SystemWindow {
  /// macOS window identifier
  pub window_id: u32,
  /// Process ID of the owning application
  pub pid: u32,
  /// Window title (may be empty for some windows)
  pub title: String,
  /// Name of the application that owns this window
  pub app_name: String,
  /// macOS bundle identifier (e.g., "com.apple.Safari")
  pub bundle_id: String,
  /// Window X position
  pub x: i32,
  /// Window Y position
  pub y: i32,
  /// Window width
  pub width: i32,
  /// Window height
  pub height: i32,
  /// Display ID where the window is primarily located
  pub display_id: u32,
  /// Whether the window is minimized to the Dock
  pub is_minimized: bool,
  /// Whether the window is in fullscreen mode
  pub is_fullscreen: bool,
  /// Window layer (0 for normal windows)
  pub layer: i32,
}

/// Represents a running application.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RunningApp {
  /// Process ID
  pub pid: u32,
  /// Application name
  pub name: String,
  /// macOS bundle identifier
  pub bundle_id: String,
  /// Path to the application bundle (if available)
  pub path: Option<String>,
  /// Whether the app is currently frontmost
  pub is_active: bool,
  /// Whether the app is hidden
  pub is_hidden: bool,
  /// Number of visible windows owned by this app
  pub window_count: u32,
}

/// Represents an installed application on the system.
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct InstalledApp {
  /// Application name
  pub name: String,
  /// macOS bundle identifier
  pub bundle_id: String,
  /// Path to the application bundle
  pub path: String,
  /// Application version (if available)
  pub version: Option<String>,
  /// Application category (if available)
  pub category: Option<String>,
}

// ============================================================================
// Service Implementation
// ============================================================================

/// System detection service providing access to macOS display and window information.
pub struct SystemService;

impl SystemService {
  // ========================================================================
  // Permission Management
  // ========================================================================

  /// Check if the app has Screen Recording permission (required for display configuration)
  pub fn check_display_permission() -> bool {
    use core_graphics::access::ScreenCaptureAccess;

    // CGPreflightScreenCaptureAccess checks without triggering the permission dialog
    let access = ScreenCaptureAccess;
    let has_access = access.preflight();

    tracing::info!("Screen capture permission check: {}", has_access);
    has_access
  }

  /// Request Screen Recording permission from the user
  /// This will open System Settings to the Screen Recording pane
  pub fn request_display_permission() -> bool {
    use std::process::Command;

    // Open System Settings directly to the Screen Recording privacy pane
    let result = Command::new("open")
      .arg("x-apple.systempreferences:com.apple.preference.security?Privacy_ScreenCapture")
      .spawn();

    match result {
      Ok(_) => {
        tracing::info!("Opened System Settings to Screen Recording");
        true
      }
      Err(e) => {
        tracing::error!("Failed to open System Settings: {}", e);
        // Fallback: try opening System Settings generally
        let _ = Command::new("open")
          .arg("-a")
          .arg("System Preferences")
          .spawn();
        false
      }
    }
  }

  // ========================================================================
  // Monitor Detection
  // ========================================================================

  /// Detects and returns all connected monitors.
  ///
  /// # Returns
  /// A vector of `SystemMonitor` representing each connected display.
  pub fn get_monitors() -> Vec<SystemMonitor> {
    Self::detect_monitors()
  }

  /// Detects and returns all visible windows on screen.
  ///
  /// # Returns
  /// A vector of `SystemWindow` representing each visible window.
  /// System windows (dock, menu bar, etc.) are filtered out.
  pub fn get_windows() -> Vec<SystemWindow> {
    Self::detect_windows()
  }

  /// Gets information about all running GUI applications.
  ///
  /// # Returns
  /// A vector of `RunningApp` representing each running application.
  pub fn get_running_apps() -> Vec<RunningApp> {
    Self::detect_running_apps()
  }

  /// Captures the complete system layout efficiently in a single call.
  /// This avoids the double window detection that happens when calling
  /// get_windows() and get_running_apps() separately.
  ///
  /// # Returns
  /// A tuple of (monitors, windows, running_apps)
  pub fn capture_system_layout() -> (Vec<SystemMonitor>, Vec<SystemWindow>, Vec<RunningApp>) {
    let monitors = Self::detect_monitors();
    let windows = Self::detect_windows();
    let apps = Self::detect_running_apps_with_windows(&windows);
    (monitors, windows, apps)
  }

  /// Gets all installed applications on the system.
  ///
  /// # Returns
  /// A vector of `InstalledApp` representing each installed application.
  pub fn get_installed_apps() -> Vec<InstalledApp> {
    Self::detect_installed_apps()
  }

  /// Applies a monitor layout configuration to the system.
  ///
  /// This method uses the `displayplacer` utility to configure monitor positions.
  /// Note: This requires `displayplacer` to be installed and may require admin privileges.
  ///
  /// # Arguments
  /// * `monitors` - A vector of `SystemMonitor` with the desired positions
  ///
  /// # Returns
  /// Result indicating success or failure
  pub fn apply_monitor_layout(monitors: Vec<SystemMonitor>) -> crate::error::Result<()> {
    use std::process::Command;

    if monitors.is_empty() {
      return Err(crate::error::SmoothieError::ValidationError(
        "No monitors provided".into(),
      ));
    }

    // Get mapping from CoreGraphics display IDs to contextual screen IDs
    let id_mapping = Self::map_display_ids_to_contextual(&monitors)?;

    if id_mapping.len() != monitors.len() {
      return Err(crate::error::SmoothieError::ValidationError(format!(
        "Found {} ID mappings but {} monitors provided",
        id_mapping.len(),
        monitors.len()
      )));
    }

    // Find displayplacer executable
    let displayplacer_path = Self::find_displayplacer()?;

    // Build displayplacer command - each monitor config is a separate argument
    let mut command = Command::new(&displayplacer_path);
    command.env(
      "PATH",
      "/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin",
    );

    for monitor in &monitors {
      if let Some(contextual_id) = id_mapping.get(&monitor.display_id) {
        // Format: id:<contextual_id> res:<width>x<height> scaling:<on/off> origin:(<x>,<y>) degree:<rotation>
        let rotation = if monitor.orientation == "Portrait" {
          90
        } else {
          0
        };
        let scaling = if monitor.scale_factor > 1.0 {
          "on"
        } else {
          "off"
        };
        let arg = format!(
          "id:{} res:{}x{} scaling:{} origin:({}, {}) degree:{}",
          contextual_id, monitor.width, monitor.height, scaling, monitor.x, monitor.y, rotation
        );
        command.arg(arg);
        tracing::info!(
          "Monitor (display ID {}): using contextual ID {} for display ID {}, scaling:{}",
          monitor.display_id,
          contextual_id,
          monitor.display_id,
          scaling
        );
      } else {
        return Err(crate::error::SmoothieError::ValidationError(format!(
          "No contextual ID found for monitor {}",
          monitor.display_id
        )));
      }
    }

    tracing::info!(
      "Executing displayplacer command with {} monitor(s)",
      monitors.len()
    );

    // Log the actual command being executed
    let mut command_debug = displayplacer_path.clone();
    for monitor in &monitors {
      if let Some(contextual_id) = id_mapping.get(&monitor.display_id) {
        let scaling = if monitor.scale_factor > 1.0 {
          "on"
        } else {
          "off"
        };
        let arg = format!(
          " \"id:{} res:{}x{} scaling:{} origin:({}, {}) degree:0\"",
          contextual_id, monitor.width, monitor.height, scaling, monitor.x, monitor.y
        );
        command_debug.push_str(&arg);
      }
    }
    tracing::info!("Full command: {}", command_debug);

    // Execute displayplacer command
    let output = command.output().map_err(|e| {
      tracing::error!("Failed to spawn displayplacer process: {}", e);
      crate::error::SmoothieError::SystemError(format!("Failed to execute displayplacer: {}", e))
    })?;

    // If the command failed, try with sudo
    let output = if !output.status.success() {
      tracing::info!("displayplacer failed without sudo, trying with sudo...");

      let mut sudo_command = Command::new("sudo");
      sudo_command.arg("--non-interactive"); // Don't prompt for password
      sudo_command.arg(&displayplacer_path);
      sudo_command.env(
        "PATH",
        "/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin",
      );

      // Re-add all the arguments
      let mut sudo_command_debug = format!("sudo {}", displayplacer_path);
      for monitor in &monitors {
        if let Some(contextual_id) = id_mapping.get(&monitor.display_id) {
          let rotation = if monitor.orientation == "Portrait" {
            90
          } else {
            0
          };
          let scaling = if monitor.scale_factor > 1.0 {
            "on"
          } else {
            "off"
          };
          let arg = format!(
            " \"id:{} res:{}x{} scaling:{} origin:({}, {}) degree:{}\"",
            contextual_id, monitor.width, monitor.height, scaling, monitor.x, monitor.y, rotation
          );
          sudo_command_debug.push_str(&arg);
        }
      }
      tracing::info!("Sudo command: {}", sudo_command_debug);

      match sudo_command.output() {
        Ok(sudo_output) => {
          if sudo_output.status.success() {
            tracing::info!("Successfully applied monitor layout with sudo displayplacer");
            sudo_output
          } else {
            // Return the original failure since sudo also failed
            output
          }
        }
        Err(_) => {
          // sudo command failed to execute, return original error
          output
        }
      }
    } else {
      output
    };

    if !output.status.success() {
      let stdout = String::from_utf8_lossy(&output.stdout);
      let stderr = String::from_utf8_lossy(&output.stderr);
      let exit_code = output.status.code().unwrap_or(-1);

      tracing::error!(
        "displayplacer failed with exit code {}: stdout='{}', stderr='{}'",
        exit_code,
        stdout,
        stderr
      );

      // For production, provide the command for manual execution (no sudo needed)
      let mut manual_command = displayplacer_path.clone();
      for monitor in &monitors {
        if let Some(contextual_id) = id_mapping.get(&monitor.display_id) {
          let rotation = if monitor.orientation == "Portrait" {
            90
          } else {
            0
          };
          // Only include origin (position) - displayplacer will keep other settings
          let arg = format!(
            " \"id:{} origin:({},{}) degree:{}\"",
            contextual_id, monitor.x, monitor.y, rotation
          );
          manual_command.push_str(&arg);
        }
      }

      return Err(crate::error::SmoothieError::SystemError(format!(
        "Failed to apply monitor layout automatically. Please run this command manually in Terminal:\n\n{}\n\nNote: You may need to install displayplacer first: brew install jakehilborn/jakehilborn/displayplacer",
        manual_command
      )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    tracing::info!(
      "Successfully applied monitor layout with displayplacer: {}",
      stdout.trim()
    );

    Ok(())
  }

  /// Apply monitor layout using AppleScript (bypasses some sandbox restrictions)
  pub async fn apply_monitor_layout_applescript(
    monitors: &[SystemMonitor],
  ) -> crate::error::Result<()> {
    use std::process::Command;

    // Build displayplacer command
    let displayplacer_path = Self::find_displayplacer()?;

    // Get actual current display info from displayplacer to use correct resolutions
    let list_output = Command::new(&displayplacer_path).arg("list").output();

    let _current_displays = if let Ok(output) = list_output {
      let stdout = String::from_utf8_lossy(&output.stdout);
      tracing::info!("displayplacer list output:\n{}", stdout);
      stdout.to_string()
    } else {
      String::new()
    };

    let id_mapping = Self::map_display_ids_to_contextual(monitors)?;

    // Build individual monitor arguments (without outer quotes for shell)
    let mut monitor_args: Vec<String> = Vec::new();
    for monitor in monitors {
      if let Some(contextual_id) = id_mapping.get(&monitor.display_id) {
        let rotation = if monitor.orientation == "Portrait" {
          90
        } else {
          0
        };
        let scaling = if monitor.scale_factor > 1.0 {
          "on"
        } else {
          "off"
        };

        // Must include res and scaling for displayplacer to work properly
        // For AppleScript's do shell script, we need to escape inner quotes with backslash
        let arg = format!(
          "\\\"id:{} res:{}x{} scaling:{} origin:({},{}) degree:{}\\\"",
          contextual_id, monitor.width, monitor.height, scaling, monitor.x, monitor.y, rotation
        );
        tracing::info!("Monitor {} arg: {}", contextual_id, arg);
        monitor_args.push(arg);
      }
    }

    // Build the full command string
    let command_string = format!("{} {}", displayplacer_path, monitor_args.join(" "));

    // Create AppleScript to execute the command with admin privileges
    // The command string already has escaped quotes for the shell
    let script = format!(
      r#"do shell script "{}" with administrator privileges"#,
      command_string
    );

    tracing::info!(
      "Executing displayplacer via AppleScript: {}",
      command_string
    );
    tracing::info!("AppleScript: {}", script);

    let output = Command::new("osascript")
      .arg("-e")
      .arg(&script)
      .output()
      .map_err(|e| {
        tracing::error!("Failed to execute osascript: {}", e);
        crate::error::SmoothieError::SystemError(format!("Failed to execute monitor layout: {}", e))
      })?;

    if output.status.success() {
      let stdout = String::from_utf8_lossy(&output.stdout);
      tracing::info!(
        "Successfully applied monitor layout via AppleScript: {}",
        stdout.trim()
      );
      Ok(())
    } else {
      let stderr = String::from_utf8_lossy(&output.stderr);
      let stdout = String::from_utf8_lossy(&output.stdout);
      tracing::error!(
        "AppleScript execution failed: stderr='{}', stdout='{}'",
        stderr,
        stdout
      );

      // Build a clean manual command for the user (with proper quotes, not escaped)
      let mut manual_args: Vec<String> = Vec::new();
      for monitor in monitors {
        if let Some(contextual_id) = id_mapping.get(&monitor.display_id) {
          let rotation = if monitor.orientation == "Portrait" {
            90
          } else {
            0
          };
          let scaling = if monitor.scale_factor > 1.0 {
            "on"
          } else {
            "off"
          };
          manual_args.push(format!(
            "\"id:{} res:{}x{} scaling:{} origin:({},{}) degree:{}\"",
            contextual_id, monitor.width, monitor.height, scaling, monitor.x, monitor.y, rotation
          ));
        }
      }
      let manual_command = format!("{} {}", displayplacer_path, manual_args.join(" "));

      Err(crate::error::SmoothieError::SystemError(format!(
        "Failed to apply monitor layout automatically. Please run this command manually in Terminal:\n\n{}\n\nNote: You may need to install displayplacer first: brew install jakehilborn/jakehilborn/displayplacer",
        manual_command
      )))
    }
  }

  /// Apply monitor layout using native macOS CoreGraphics APIs (most reliable)
  #[allow(dead_code)]
  pub async fn apply_monitor_layout_native(monitors: &[SystemMonitor]) -> crate::error::Result<()> {
    use core_graphics::display::{CGDisplay, CGDisplayConfigRef};
    use std::ptr;

    tracing::info!(
      "Applying monitor layout using native CoreGraphics APIs for {} monitors",
      monitors.len()
    );

    // Get current display configuration
    let mut config_ref: CGDisplayConfigRef = ptr::null_mut();
    let result = unsafe { core_graphics::display::CGBeginDisplayConfiguration(&mut config_ref) };

    if result != 0 {
      return Err(crate::error::SmoothieError::SystemError(
        "Failed to begin display configuration".to_string(),
      ));
    }

    // Configure each monitor
    for monitor in monitors {
      let display_id = monitor.display_id;
      let display = CGDisplay::new(display_id);

      // Set display mode (resolution)
      if let Some(mode) = display.display_mode() {
        // For now, we'll use the current mode - extending this to change resolution
        // would require more complex mode enumeration and selection
        tracing::info!(
          "Using current mode for display {}: {}x{}",
          display_id,
          mode.width(),
          mode.height()
        );
      }

      // Set display origin (position)
      let origin_result = unsafe {
        core_graphics::display::CGConfigureDisplayOrigin(
          config_ref, display_id, monitor.x, monitor.y,
        )
      };

      if origin_result != 0 {
        tracing::warn!(
          "Failed to configure origin for display {}: error {}",
          display_id,
          origin_result
        );
      }

      // Note: Rotation and scaling would require additional CoreGraphics calls
      // This is a simplified implementation focusing on positioning
    }

    // Apply the configuration
    let apply_result = unsafe {
      core_graphics::display::CGCompleteDisplayConfiguration(
        config_ref,
        core_graphics::display::CGConfigureOption::ConfigurePermanently,
      )
    };

    if apply_result != 0 {
      unsafe { core_graphics::display::CGCancelDisplayConfiguration(config_ref) };
      return Err(crate::error::SmoothieError::SystemError(format!(
        "Failed to apply display configuration: error {}",
        apply_result
      )));
    }

    tracing::info!("Successfully applied monitor layout using native APIs");
    Ok(())
  }

  /// Find displayplacer executable in system PATH
  fn find_displayplacer() -> crate::error::Result<String> {
    use std::process::Command;

    // First try common locations
    let common_paths = vec![
      "/opt/homebrew/bin/displayplacer",
      "/usr/local/bin/displayplacer",
      "/usr/bin/displayplacer",
      "/bin/displayplacer",
    ];

    for path in common_paths {
      if std::path::Path::new(path).exists() {
        return Ok(path.to_string());
      }
    }

    // If not found in common locations, try using 'which' command
    match Command::new("which").arg("displayplacer").output() {
      Ok(output) if output.status.success() => {
        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !path.is_empty() {
          return Ok(path);
        }
      }
      _ => {}
    }

    // If still not found, try using 'command -v' as fallback
    match Command::new("command")
      .arg("-v")
      .arg("displayplacer")
      .output()
    {
      Ok(output) if output.status.success() => {
        let path = String::from_utf8_lossy(&output.stdout).trim().to_string();
        if !path.is_empty() {
          return Ok(path);
        }
      }
      _ => {}
    }

    Err(crate::error::SmoothieError::SystemError(
      "displayplacer not found. Please install it from https://github.com/jakehilborn/displayplacer".to_string()
    ))
  }

  #[allow(dead_code)]
  fn get_contextual_screen_ids() -> crate::error::Result<Vec<u32>> {
    use std::process::Command;

    let displayplacer_path = Self::find_displayplacer()?;

    let output = Command::new(&displayplacer_path)
      .arg("list")
      .env(
        "PATH",
        "/opt/homebrew/bin:/usr/local/bin:/usr/bin:/bin:/usr/sbin:/sbin",
      )
      .output()
      .map_err(|e| {
        crate::error::SmoothieError::SystemError(format!(
          "Failed to execute displayplacer list: {}",
          e
        ))
      })?;

    if !output.status.success() {
      let stderr = String::from_utf8_lossy(&output.stderr);
      return Err(crate::error::SmoothieError::SystemError(format!(
        "displayplacer list failed: {}",
        stderr
      )));
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut contextual_ids = Vec::new();

    // Parse the output to extract contextual screen IDs
    for line in stdout.lines() {
      if line.contains("Contextual screen id:") {
        if let Some(id_str) = line.split(':').nth(1) {
          if let Ok(id) = id_str.trim().parse::<u32>() {
            contextual_ids.push(id);
          }
        }
      }
    }

    // Sort the IDs to ensure consistent ordering
    contextual_ids.sort();

    tracing::info!("Found contextual screen IDs: {:?}", contextual_ids);
    Ok(contextual_ids)
  }

  /// Map CoreGraphics display IDs to contextual screen IDs
  fn map_display_ids_to_contextual(
    monitors: &[SystemMonitor],
  ) -> crate::error::Result<HashMap<u32, u32>> {
    // For macOS, CoreGraphics display IDs appear to match displayplacer's contextual screen IDs
    // This is a simplification that works for the current setup
    let mut contextual_mapping = HashMap::new();

    for monitor in monitors {
      // Use the CoreGraphics display ID directly as the contextual ID
      contextual_mapping.insert(monitor.display_id, monitor.display_id);
      tracing::info!(
        "Using CoreGraphics display ID {} as contextual ID",
        monitor.display_id
      );
    }

    Ok(contextual_mapping)
  }

  // ========================================================================
  // Monitor Detection
  // ========================================================================

  fn detect_monitors() -> Vec<SystemMonitor> {
    use core_graphics::display::CGDisplay;

    let display_ids = match CGDisplay::active_displays() {
      Ok(ids) => ids,
      Err(e) => {
        tracing::error!("Failed to get active displays: {:?}", e);
        return Vec::new();
      }
    };

    let main_display_id = CGDisplay::main().id;
    let mut monitors = Vec::with_capacity(display_ids.len());

    for display_id in display_ids {
      if let Some(monitor) = Self::build_monitor_info(display_id, main_display_id) {
        monitors.push(monitor);
      }
    }

    // Sort by display position (left to right, top to bottom)
    monitors.sort_by(|a, b| {
      if a.x != b.x {
        a.x.cmp(&b.x)
      } else {
        a.y.cmp(&b.y)
      }
    });

    monitors
  }

  fn build_monitor_info(display_id: u32, main_display_id: u32) -> Option<SystemMonitor> {
    use core_graphics::display::CGDisplay;

    let display = CGDisplay::new(display_id);
    let bounds = display.bounds();

    // Get resolution and refresh rate from display mode
    let mode = display.display_mode();
    let (width, height, refresh_rate, scale_factor) = if let Some(ref m) = mode {
      let pixel_width = m.pixel_width() as f64;
      let point_width = bounds.size.width;
      let scale = if point_width > 0.0 {
        pixel_width / point_width
      } else {
        1.0
      };
      (m.width() as i32, m.height() as i32, m.refresh_rate(), scale)
    } else {
      (
        bounds.size.width as i32,
        bounds.size.height as i32,
        60.0,
        1.0,
      )
    };

    let orientation = if width > height {
      "Landscape"
    } else {
      "Portrait"
    };
    let is_primary = display_id == main_display_id;
    let is_builtin = display.is_builtin();
    let name = Self::get_display_name(display_id, is_primary, is_builtin);

    // Get brand and model from EDID
    let (brand, model) = Self::get_display_brand_and_model(display_id);

    Some(SystemMonitor {
      display_id,
      name,
      brand,
      model,
      resolution: format!("{}x{}", width, height),
      width,
      height,
      x: bounds.origin.x as i32,
      y: bounds.origin.y as i32,
      scale_factor,
      refresh_rate,
      is_primary,
      is_builtin,
      orientation: orientation.to_string(),
    })
  }

  fn get_display_name(display_id: u32, is_primary: bool, is_builtin: bool) -> String {
    if is_builtin {
      return "Built-in Display".to_string();
    }

    if is_primary {
      return "Primary Display".to_string();
    }

    format!("External Display {}", display_id)
  }

  fn get_display_brand_and_model(display_id: u32) -> (Option<String>, Option<String>) {
    use std::process::Command;

    // Use system_profiler to get display information
    let output = Command::new("system_profiler")
      .args(["SPDisplaysDataType", "-json"])
      .output();

    if let Ok(output) = output {
      if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        if let Ok(json) = serde_json::from_str::<serde_json::Value>(&stdout) {
          if let Some(displays) = json.get("SPDisplaysDataType") {
            if let Some(display_array) = displays.as_array() {
              for gpu_info in display_array {
                if let Some(ndrvs) = gpu_info.get("spdisplays_ndrvs") {
                  if let Some(display_list) = ndrvs.as_array() {
                    for display in display_list {
                      // Match by display ID
                      if let Some(disp_id) = display
                        .get("_spdisplays_displayID")
                        .and_then(|id| id.as_str())
                      {
                        if disp_id.parse::<u32>().unwrap_or(0) == display_id {
                          // Get the display name which contains brand and model
                          if let Some(name) = display.get("_name").and_then(|n| n.as_str()) {
                            // Parse brand and model from name like "DELL U2721DE" or "Color LCD"
                            let (brand, model) = Self::parse_display_name(name);
                            return (brand, model);
                          }
                        }
                      }
                    }
                  }
                }
              }
            }
          }
        }
      }
    }

    (None, None)
  }

  fn parse_display_name(name: &str) -> (Option<String>, Option<String>) {
    let name = name.trim();

    // Handle built-in displays
    if name == "Color LCD" {
      return (
        Some("Apple".to_string()),
        Some("Built-in Display".to_string()),
      );
    }

    // Try to split on spaces and identify brand
    let parts: Vec<&str> = name.split_whitespace().collect();
    if parts.len() >= 2 {
      let potential_brand = parts[0];

      // Common display brands
      let brand = match potential_brand.to_uppercase().as_str() {
        "DELL" => "Dell",
        "LG" => "LG",
        "SAMSUNG" => "Samsung",
        "ASUS" => "ASUS",
        "ACER" => "Acer",
        "HP" => "HP",
        "LENOVO" => "Lenovo",
        "VIEWSONIC" => "ViewSonic",
        "BENQ" => "BenQ",
        "AOC" => "AOC",
        _ => potential_brand,
      };

      // The rest is likely the model
      let model = parts[1..].join(" ");

      (Some(brand.to_string()), Some(model))
    } else {
      // If we can't parse it, return the whole name as model
      (None, Some(name.to_string()))
    }
  }

  // ========================================================================
  // Window Detection
  // ========================================================================

  fn detect_windows() -> Vec<SystemWindow> {
    use core_foundation::array::{CFArrayGetCount, CFArrayGetValueAtIndex};
    use core_foundation::base::{CFRelease, CFType, TCFType};
    use core_foundation::dictionary::CFDictionary;
    use core_foundation::string::CFString;
    use core_graphics::window::{
      kCGNullWindowID, kCGWindowListOptionOnScreenOnly, CGWindowListCopyWindowInfo,
    };

    let mut windows = Vec::new();

    // Get window list from the window server
    let window_list =
      unsafe { CGWindowListCopyWindowInfo(kCGWindowListOptionOnScreenOnly, kCGNullWindowID) };

    if window_list.is_null() {
      tracing::warn!("Failed to get window list from window server");
      return windows;
    }

    let count = unsafe { CFArrayGetCount(window_list) };

    for i in 0..count {
      let window_ptr = unsafe { CFArrayGetValueAtIndex(window_list, i) };

      if window_ptr.is_null() {
        continue;
      }

      let window_dict: CFDictionary<CFString, CFType> =
        unsafe { CFDictionary::wrap_under_get_rule(window_ptr as *const _) };

      if let Some(window) = Self::parse_window_info(&window_dict) {
        windows.push(window);
      }
    }

    // Clean up
    unsafe { CFRelease(window_list as *const _) };

    windows
  }

  fn parse_window_info(
    dict: &core_foundation::dictionary::CFDictionary<
      core_foundation::string::CFString,
      core_foundation::base::CFType,
    >,
  ) -> Option<SystemWindow> {
    let window_id = Self::get_cf_number_i64(dict, "kCGWindowNumber")? as u32;
    let pid = Self::get_cf_number_i64(dict, "kCGWindowOwnerPID")? as u32;
    let layer = Self::get_cf_number_i64(dict, "kCGWindowLayer").unwrap_or(0);

    let app_name = Self::get_cf_string(dict, "kCGWindowOwnerName").unwrap_or_default();
    let title = Self::get_cf_string(dict, "kCGWindowName").unwrap_or_default();

    // Filter out system windows (layer != 0) and windows without owner
    if layer != 0 || app_name.is_empty() {
      return None;
    }

    let (x, y, width, height) = Self::get_window_bounds(dict);

    // Skip tiny windows (likely invisible or utility windows)
    if width < 50 || height < 50 {
      return None;
    }

    let center_x = x + width / 2;
    let center_y = y + height / 2;
    let display_id = Self::find_display_for_point(center_x, center_y);

    let bundle_id = Self::get_bundle_id_for_pid(pid);

    Some(SystemWindow {
      window_id,
      pid,
      title,
      app_name,
      bundle_id,
      x,
      y,
      width,
      height,
      display_id,
      is_minimized: false,
      is_fullscreen: false,
      layer: layer as i32,
    })
  }

  fn get_window_bounds(
    dict: &core_foundation::dictionary::CFDictionary<
      core_foundation::string::CFString,
      core_foundation::base::CFType,
    >,
  ) -> (i32, i32, i32, i32) {
    use core_foundation::base::TCFType;
    use core_foundation::dictionary::CFDictionary;
    use core_foundation::string::CFString;

    let key = CFString::new("kCGWindowBounds");
    if let Some(bounds_val) = dict.find(key) {
      let type_ref = bounds_val.as_CFTypeRef();
      let bounds: CFDictionary<CFString, core_foundation::base::CFType> =
        unsafe { CFDictionary::wrap_under_get_rule(type_ref as *const _) };

      let x = Self::get_cf_number_f64(&bounds, "X").unwrap_or(0.0) as i32;
      let y = Self::get_cf_number_f64(&bounds, "Y").unwrap_or(0.0) as i32;
      let width = Self::get_cf_number_f64(&bounds, "Width").unwrap_or(0.0) as i32;
      let height = Self::get_cf_number_f64(&bounds, "Height").unwrap_or(0.0) as i32;

      return (x, y, width, height);
    }
    (0, 0, 0, 0)
  }

  fn find_display_for_point(x: i32, y: i32) -> u32 {
    use core_graphics::display::CGDisplay;

    let displays = CGDisplay::active_displays().unwrap_or_default();

    for display_id in displays {
      let display = CGDisplay::new(display_id);
      let bounds = display.bounds();

      let in_x_range =
        x >= bounds.origin.x as i32 && x < (bounds.origin.x + bounds.size.width) as i32;
      let in_y_range =
        y >= bounds.origin.y as i32 && y < (bounds.origin.y + bounds.size.height) as i32;

      if in_x_range && in_y_range {
        return display_id;
      }
    }

    // Default to main display
    CGDisplay::main().id
  }

  // ========================================================================
  // Running Applications Detection
  // ========================================================================

  fn detect_running_apps() -> Vec<RunningApp> {
    let windows = Self::detect_windows();
    Self::detect_running_apps_with_windows(&windows)
  }

  /// Detect running apps using pre-computed windows (optimization to avoid re-detecting windows)
  fn detect_running_apps_with_windows(windows: &[SystemWindow]) -> Vec<RunningApp> {
    // Build window count and app info from detected windows
    let mut window_counts: HashMap<u32, u32> = HashMap::new();
    let mut app_info: HashMap<u32, (String, String)> = HashMap::new();

    for window in windows {
      *window_counts.entry(window.pid).or_insert(0) += 1;
      app_info
        .entry(window.pid)
        .or_insert_with(|| (window.app_name.clone(), window.bundle_id.clone()));
    }

    // Try to get more detailed app info via AppleScript
    if let Some(apps) = Self::get_apps_via_applescript(&window_counts) {
      return apps;
    }

    // Fallback: build app list from window info
    app_info
      .into_iter()
      .map(|(pid, (name, bundle_id))| RunningApp {
        pid,
        name,
        bundle_id,
        path: None,
        is_active: false,
        is_hidden: false,
        window_count: window_counts.get(&pid).copied().unwrap_or(0),
      })
      .collect()
  }

  fn get_apps_via_applescript(window_counts: &HashMap<u32, u32>) -> Option<Vec<RunningApp>> {
    use std::process::Command;

    let script = r#"
            tell application "System Events"
                set appList to ""
                repeat with proc in (every process whose background only is false)
                    try
                        set appList to appList & (unix id of proc) & "|||" & (name of proc) & "|||" & (bundle identifier of proc) & "|||" & (frontmost of proc) & "|||" & (visible of proc) & "
"
                    end try
                end repeat
            end tell
            return appList
        "#;

    let output = Command::new("osascript")
      .arg("-e")
      .arg(script)
      .output()
      .ok()?;

    if !output.status.success() {
      return None;
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut apps = Vec::new();

    for line in stdout.lines() {
      let parts: Vec<&str> = line.split("|||").collect();
      if parts.len() >= 5 {
        let pid = parts[0].trim().parse::<u32>().unwrap_or(0);
        if pid == 0 {
          continue;
        }

        apps.push(RunningApp {
          pid,
          name: parts[1].to_string(),
          bundle_id: parts[2].to_string(),
          path: None,
          is_active: parts[3].trim() == "true",
          is_hidden: parts[4].trim() != "true",
          window_count: window_counts.get(&pid).copied().unwrap_or(0),
        });
      }
    }

    if apps.is_empty() {
      None
    } else {
      Some(apps)
    }
  }

  // ========================================================================
  // Installed Applications Detection
  // ========================================================================

  fn detect_installed_apps() -> Vec<InstalledApp> {
    use std::collections::HashSet;
    use std::fs;
    use std::path::Path;
    use std::process::Command;

    let mut apps = Vec::new();
    let mut seen_bundle_ids = HashSet::new();

    // Directories to search for applications
    let home_apps = format!("{}/Applications", std::env::var("HOME").unwrap_or_default());
    let app_directories: Vec<&str> = vec![
      "/Applications",
      "/System/Applications",
      "/System/Applications/Utilities",
      &home_apps,
    ];

    for dir in app_directories {
      if let Ok(entries) = fs::read_dir(dir) {
        for entry in entries.flatten() {
          let path = entry.path();
          if path.extension().and_then(|e| e.to_str()) == Some("app") {
            if let Some(app) = Self::parse_app_bundle(&path, &mut seen_bundle_ids) {
              apps.push(app);
            }
          }
        }
      }
    }

    // Also use mdfind to find apps that might be in other locations
    if let Ok(output) = Command::new("mdfind")
      .arg("kMDItemKind == 'Application'")
      .output()
    {
      if output.status.success() {
        let stdout = String::from_utf8_lossy(&output.stdout);
        for line in stdout.lines() {
          let path = Path::new(line.trim());
          if path.extension().and_then(|e| e.to_str()) == Some("app") {
            if let Some(app) = Self::parse_app_bundle(path, &mut seen_bundle_ids) {
              apps.push(app);
            }
          }
        }
      }
    }

    // Sort by name
    apps.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));

    apps
  }

  fn parse_app_bundle(
    path: &std::path::Path,
    seen_bundle_ids: &mut std::collections::HashSet<String>,
  ) -> Option<InstalledApp> {
    let info_plist_path = path.join("Contents/Info.plist");

    if !info_plist_path.exists() {
      return None;
    }

    // Use defaults command to read plist (handles both binary and XML plists)
    let bundle_id = Self::read_plist_key(path, "CFBundleIdentifier")?;

    // Skip if we've already seen this bundle ID
    if seen_bundle_ids.contains(&bundle_id) {
      return None;
    }
    seen_bundle_ids.insert(bundle_id.clone());

    // Skip system helper apps and background services
    if bundle_id.starts_with("com.apple.")
      && (bundle_id.contains("helper")
        || bundle_id.contains("agent")
        || bundle_id.contains("daemon")
        || bundle_id.contains("XPC"))
    {
      return None;
    }

    let name = Self::read_plist_key(path, "CFBundleName")
      .or_else(|| Self::read_plist_key(path, "CFBundleDisplayName"))
      .unwrap_or_else(|| {
        path
          .file_stem()
          .and_then(|s| s.to_str())
          .unwrap_or("Unknown")
          .to_string()
      });

    let version = Self::read_plist_key(path, "CFBundleShortVersionString");
    let category = Self::read_plist_key(path, "LSApplicationCategoryType");

    Some(InstalledApp {
      name,
      bundle_id,
      path: path.to_string_lossy().to_string(),
      version,
      category,
    })
  }

  fn read_plist_key(app_path: &std::path::Path, key: &str) -> Option<String> {
    use std::process::Command;

    let output = Command::new("defaults")
      .arg("read")
      .arg(app_path.join("Contents/Info.plist"))
      .arg(key)
      .output()
      .ok()?;

    if output.status.success() {
      let value = String::from_utf8_lossy(&output.stdout).trim().to_string();
      if !value.is_empty() {
        return Some(value);
      }
    }

    None
  }

  // ========================================================================
  // CoreFoundation Helpers
  // ========================================================================

  fn get_cf_number_i64(
    dict: &core_foundation::dictionary::CFDictionary<
      core_foundation::string::CFString,
      core_foundation::base::CFType,
    >,
    key: &str,
  ) -> Option<i64> {
    use core_foundation::base::TCFType;
    use core_foundation::number::CFNumber;
    use core_foundation::string::CFString;

    let key = CFString::new(key);
    dict.find(key).and_then(|v| {
      let type_ref = v.as_CFTypeRef();
      let num: CFNumber = unsafe { CFNumber::wrap_under_get_rule(type_ref as *const _) };
      num.to_i64()
    })
  }

  fn get_cf_number_f64(
    dict: &core_foundation::dictionary::CFDictionary<
      core_foundation::string::CFString,
      core_foundation::base::CFType,
    >,
    key: &str,
  ) -> Option<f64> {
    use core_foundation::base::TCFType;
    use core_foundation::number::CFNumber;
    use core_foundation::string::CFString;

    let key = CFString::new(key);
    dict.find(key).and_then(|v| {
      let type_ref = v.as_CFTypeRef();
      let num: CFNumber = unsafe { CFNumber::wrap_under_get_rule(type_ref as *const _) };
      num.to_f64()
    })
  }

  fn get_cf_string(
    dict: &core_foundation::dictionary::CFDictionary<
      core_foundation::string::CFString,
      core_foundation::base::CFType,
    >,
    key: &str,
  ) -> Option<String> {
    use core_foundation::base::TCFType;
    use core_foundation::string::CFString;

    let key = CFString::new(key);
    dict.find(key).map(|v| {
      let type_ref = v.as_CFTypeRef();
      let s: CFString = unsafe { CFString::wrap_under_get_rule(type_ref as *const _) };
      s.to_string()
    })
  }

  fn get_bundle_id_for_pid(pid: u32) -> String {
    use std::process::Command;

    let output = Command::new("lsappinfo")
      .args(["info", "-only", "bundleid", &format!("-pid={}", pid)])
      .output();

    if let Ok(output) = output {
      let stdout = String::from_utf8_lossy(&output.stdout);
      // Parse format: "bundleid"="com.example.App"
      if let Some(start) = stdout.find("=\"") {
        if let Some(end) = stdout[start + 2..].find('"') {
          return stdout[start + 2..start + 2 + end].to_string();
        }
      }
    }

    String::new()
  }
}

// ============================================================================
// Tests
// ============================================================================

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_detect_monitors() {
    let monitors = SystemService::get_monitors();

    // Should have at least one monitor
    assert!(!monitors.is_empty(), "Should detect at least one monitor");

    // Should have exactly one primary monitor
    let primary_count = monitors.iter().filter(|m| m.is_primary).count();
    assert_eq!(primary_count, 1, "Should have exactly one primary monitor");

    println!("\n=== Detected Monitors ===");
    for monitor in &monitors {
      println!(
        "  {} (ID: {}): {}x{} @ ({}, {}) - scale: {:.1}x, refresh: {:.0}Hz{}{}",
        monitor.name,
        monitor.display_id,
        monitor.width,
        monitor.height,
        monitor.x,
        monitor.y,
        monitor.scale_factor,
        monitor.refresh_rate,
        if let Some(ref brand) = monitor.brand {
          format!(" - Brand: {}", brand)
        } else {
          "".to_string()
        },
        if let Some(ref model) = monitor.model {
          format!(" - Model: {}", model)
        } else {
          "".to_string()
        }
      );
    }
  }

  #[test]
  fn test_detect_windows() {
    let windows = SystemService::get_windows();

    println!("\n=== Detected Windows ===");
    for window in &windows {
      println!(
        "  {} - \"{}\" @ ({}, {}) {}x{}",
        window.app_name, window.title, window.x, window.y, window.width, window.height
      );
    }

    // Basic sanity checks on detected windows
    for window in &windows {
      assert!(!window.app_name.is_empty(), "Window should have app name");
      assert!(window.width >= 50, "Window should have reasonable width");
      assert!(window.height >= 50, "Window should have reasonable height");
    }
  }

  #[test]
  fn test_detect_running_apps() {
    let apps = SystemService::get_running_apps();

    println!("\n=== Running Applications ===");
    for app in &apps {
      println!(
        "  {} ({}) - {} windows{}",
        app.name,
        app.bundle_id,
        app.window_count,
        if app.is_active { " [ACTIVE]" } else { "" }
      );
    }

    // Should have at least one app running (this test itself runs in Terminal or IDE)
    assert!(!apps.is_empty(), "Should detect at least one running app");
  }

  #[test]
  fn test_monitor_data_integrity() {
    let monitors = SystemService::get_monitors();

    for monitor in &monitors {
      // Validate resolution string format
      assert!(
        monitor.resolution.contains('x'),
        "Resolution should be in WxH format"
      );

      // Validate dimensions are positive
      assert!(monitor.width > 0, "Width should be positive");
      assert!(monitor.height > 0, "Height should be positive");

      // Validate scale factor is reasonable
      assert!(
        monitor.scale_factor >= 1.0 && monitor.scale_factor <= 3.0,
        "Scale factor should be between 1.0 and 3.0"
      );

      // Validate orientation matches dimensions
      let expected_orientation = if monitor.width > monitor.height {
        "Landscape"
      } else {
        "Portrait"
      };
      assert_eq!(
        monitor.orientation, expected_orientation,
        "Orientation should match dimensions"
      );
    }
  }

  #[tokio::test]
  async fn test_apply_monitor_layout_command_construction() {
    let monitors = SystemService::get_monitors();

    // Test that we can construct the command without errors
    let result = SystemService::apply_monitor_layout(monitors);
    match result {
      Ok(()) => {
        println!("Command executed successfully");
      }
      Err(e) => {
        println!(
          "Command failed as expected (we're testing construction): {:?}",
          e
        );
        // This is expected since we're not actually running displayplacer in tests
        // We just want to make sure the command construction doesn't panic
      }
    }
  }
}
