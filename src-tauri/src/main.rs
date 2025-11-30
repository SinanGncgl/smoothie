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
use state::AppState;
use std::sync::Arc;
use logging::{SmoothieLogger, METRICS};

#[tokio::main]
async fn main() {
    // Load environment variables from .env file
    dotenv::dotenv().ok();
    
    // Initialize logging system
    SmoothieLogger::init();
    
    tracing::info!("=== Smoothie Desktop Application Starting ===");

    // Initialize database
    let db = Database::new().await.expect("Failed to initialize database");
    let db = Arc::new(db);

    // Create app state
    let app_state = AppState::new(db.clone());
    let app_state = Arc::new(app_state);

    tracing::info!("Application state initialized");
    tracing::info!("Smoothie started successfully");

    tauri::Builder::default()
        .manage(app_state.clone())
        .invoke_handler(tauri::generate_handler![
            // Profile handlers
            handlers::profile::create_profile,
            handlers::profile::get_profiles,
            handlers::profile::get_profile,
            handlers::profile::update_profile,
            handlers::profile::delete_profile,
            handlers::profile::activate_profile,
            handlers::profile::duplicate_profile,
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
            // Sync handlers
            handlers::sync::backup_profiles,
            handlers::sync::restore_profiles,
            handlers::sync::get_sync_status,
            // User handlers
            handlers::user::get_user_preferences,
            handlers::user::update_user_preferences,
            // System handlers
            handlers::system::get_connected_monitors,
            handlers::system::get_running_apps,
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
