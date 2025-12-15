# src/main.rs - Application Entry Point

## Overview
The main entry point for the Smoothie Tauri desktop application. This file initializes the application, sets up the database, logging, and registers all Tauri command handlers.

## Key Responsibilities
- Load environment variables from `.env` file
- Initialize structured logging system
- Set up PostgreSQL database connection
- Create application state management
- Register all Tauri command handlers
- Configure window event handling
- Start the Tauri application runtime

## Dependencies
- `dotenv` - Environment variable loading
- `tracing` - Structured logging
- `tokio` - Async runtime
- `tauri` - Desktop application framework

## Application State
Creates an `AppState` instance containing:
- Database connection pool (`Arc<Database>`)
- In-memory cache for frequently accessed data (`DashMap`)

## Tauri Command Handlers
Registers handlers for all major features:

### Profile Management
- `create_profile`, `get_profiles`, `get_profile`, `update_profile`, `delete_profile`
- `activate_profile`, `duplicate_profile`

### Monitor Management
- `create_monitor`, `get_monitors`, `update_monitor`, `delete_monitor`

### Application Management
- `create_app`, `get_apps`, `update_app`, `delete_app`, `launch_apps`

### Browser Tab Management
- `create_browser_tab`, `get_browser_tabs`, `update_browser_tab`, `delete_browser_tab`, `open_tabs`

### Automation Rules
- `create_rule`, `get_rules`, `update_rule`, `delete_rule`, `evaluate_rules`

### Synchronization
- `backup_profiles`, `restore_profiles`, `get_sync_status`

### User Preferences
- `get_user_preferences`, `update_user_preferences`

### System Detection
- `get_connected_monitors`, `get_running_apps`, `get_visible_windows`, `capture_current_layout`

## Window Event Handling
- Logs application shutdown
- Records final application metrics on window destruction

## Platform-Specific Configuration
- Windows: Uses `windows_subsystem = "windows"` in release builds to hide console window
- macOS: Standard window behavior

## Error Handling
- Database initialization failures cause application panic
- Tauri runtime errors are handled by the framework

## Logging
Uses structured logging throughout initialization:
- Application startup/shutdown events
- Database initialization status
- Final metrics on shutdown

## Security Notes
- All command handlers include input validation
- Database operations use prepared statements
- Sensitive data is encrypted where appropriate</content>
<parameter name="filePath">/Users/sinang/Projects/smoothie/docs/backend/main.md