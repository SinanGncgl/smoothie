#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

mod db;
mod error;
mod handlers;
mod logging;
mod models;
mod repositories;
mod security;
mod services;
mod state;
mod utils;

use db::Database;
use logging::{SmoothieLogger, METRICS};
use services::AUDIT_SERVICE;
use state::AppState;
use std::sync::Arc;

#[tokio::main]
async fn main() {
  // Load environment variables from .env file
  dotenv::dotenv().ok();

  // Initialize logging system
  SmoothieLogger::init();

  tracing::info!("=== Smoothie Desktop Application Starting ===");

  // Initialize database
  let db = Database::new()
    .await
    .expect("Failed to initialize database");
  let db = Arc::new(db);

  // Create app state
  let app_state = AppState::new(db.clone());
  let app_state = Arc::new(app_state);

  // Start a session
  let db_clone = db.clone();
  tokio::spawn(async move {
    if let Err(e) = AUDIT_SERVICE
      .start_session(&db_clone, "00000000-0000-0000-0000-000000000001", None)
      .await
    {
      tracing::warn!("Failed to start session: {}", e);
    }
  });

  // Log application startup
  let db_clone = db.clone();
  tokio::spawn(async move {
    if let Err(e) = AUDIT_SERVICE
      .log_system_event(
        &db_clone,
        "app_started",
        "info",
        "main",
        "Smoothie Desktop Application started",
        None,
        None,
      )
      .await
    {
      tracing::warn!("Failed to log startup event: {}", e);
    }
  });

  tracing::info!("Application state initialized");
  tracing::info!("Smoothie started successfully");

  tauri::Builder::default()
    .plugin(tauri_plugin_process::init())
    .plugin(tauri_plugin_shell::init())
    .manage(app_state.clone())
    .manage((*db).clone())
    .invoke_handler(tauri::generate_handler![
      // Profile handlers
      handlers::profile::create_profile,
      handlers::profile::get_profiles,
      handlers::profile::get_profile,
      handlers::profile::update_profile,
      handlers::profile::delete_profile,
      handlers::profile::activate_profile,
      handlers::profile::duplicate_profile,
      handlers::profile::start_profile,
      handlers::profile::get_favorite_profiles,
      handlers::profile::get_most_used_profiles,
      handlers::profile::set_profile_favorite,
      // Monitor handlers
      handlers::monitor::create_monitor,
      handlers::monitor::get_monitors,
      handlers::monitor::update_monitor,
      handlers::monitor::delete_monitor,
      // App handlers
      handlers::app::create_app,
      handlers::app::get_apps,
      handlers::app::update_app,
      handlers::app::delete_app,
      handlers::app::launch_apps,
      // Browser tab handlers
      handlers::browser::create_browser_tab,
      handlers::browser::get_browser_tabs,
      handlers::browser::update_browser_tab,
      handlers::browser::delete_browser_tab,
      handlers::browser::open_tabs,
      // Automation rule handlers
      handlers::automation::create_rule,
      handlers::automation::get_rules,
      handlers::automation::update_rule,
      handlers::automation::delete_rule,
      handlers::automation::evaluate_rules,
      // Window handlers
      handlers::window::create_window,
      handlers::window::get_windows,
      handlers::window::update_window_position,
      handlers::window::delete_window,
      // User handlers
      handlers::user::get_user_preferences,
      handlers::user::update_user_preferences,
      handlers::user::get_user_settings,
      handlers::user::update_user_settings,
      // System handlers
      handlers::system::get_connected_monitors,
      handlers::system::get_running_apps,
      handlers::system::get_installed_apps,
      handlers::system::get_visible_windows,
      handlers::system::capture_current_layout,
      handlers::system::apply_monitor_layout,
      handlers::system::check_display_permission,
      handlers::system::request_display_permission,
      // Audit and logging handlers
      handlers::audit::start_session,
      handlers::audit::end_session,
      handlers::audit::get_sessions,
      handlers::audit::log_activity,
      handlers::audit::get_activity_logs,
      handlers::audit::log_system_event,
      handlers::audit::get_system_events,
      handlers::audit::record_profile_activation,
      handlers::audit::get_profile_activations,
      handlers::audit::log_error,
      handlers::audit::get_error_logs,
      handlers::audit::resolve_error,
      handlers::audit::record_monitor_change,
      handlers::audit::record_app_launch,
      handlers::audit::record_automation_execution,
      handlers::audit::get_dashboard_stats,
      handlers::audit::get_log_summary,
      handlers::audit::get_app_metrics,
      handlers::audit::cleanup_old_logs,
      handlers::audit::get_monitor_changes,
      handlers::audit::get_app_launches,
      handlers::audit::get_automation_executions,
      // Feedback handlers
      handlers::feedback::submit_feedback,
      handlers::feedback::get_feedback,
      handlers::feedback::update_feedback_status,
      // Subscription handlers
      handlers::subscription::get_subscription,
      handlers::subscription::create_subscription,
      handlers::subscription::delete_subscription,
    ])
    .on_window_event(|_window, event| {
      if let tauri::WindowEvent::Destroyed = event {
        tracing::info!("Window destroyed, cleanup initiated");
        // Log final metrics
        let metrics = METRICS.get_summary();
        tracing::info!("Final metrics: {}", metrics);
      }
    })
    .run(tauri::generate_context!())
    .expect("error while running tauri application");

  tracing::info!("=== Smoothie Desktop Application Shutdown ===");
}
