// Database migrations for Smoothie schema
// PostgreSQL version - v1

use sqlx::PgPool;
use tracing::info;

pub async fn run(pool: &PgPool) -> anyhow::Result<()> {
  info!("Starting database migrations");
  let start = std::time::Instant::now();

  run_migration_v1(pool).await?;

  let duration = start.elapsed();
  info!(
    "All database migrations completed successfully in {}ms",
    duration.as_millis()
  );
  Ok(())
}

/// Migration v1: Complete Smoothie schema for PostgreSQL
async fn run_migration_v1(pool: &PgPool) -> anyhow::Result<()> {
  info!("Running migration v1: Complete schema setup");
  let start = std::time::Instant::now();

  // Enable UUID extension
  sqlx::query("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"")
    .execute(pool)
    .await?;
  info!("UUID extension enabled");

  // ============================================================================
  // Core Tables
  // ============================================================================

  // Users table
  sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS users (
      id TEXT PRIMARY KEY,
      email TEXT UNIQUE,
      password_hash TEXT,
      username TEXT UNIQUE,
      created_at TIMESTAMP NOT NULL DEFAULT NOW(),
      updated_at TIMESTAMP NOT NULL DEFAULT NOW()
    )
    "#,
  )
  .execute(pool)
  .await?;
  info!("Users table created");

  // User Settings table
  sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS user_settings (
      id TEXT PRIMARY KEY,
      user_id TEXT NOT NULL UNIQUE REFERENCES users(id) ON DELETE CASCADE,
      theme TEXT NOT NULL DEFAULT 'dark',
      auto_restore BOOLEAN NOT NULL DEFAULT true,
      monitor_detection BOOLEAN NOT NULL DEFAULT true,
      animations_enabled BOOLEAN NOT NULL DEFAULT true,
      cloud_sync BOOLEAN NOT NULL DEFAULT false,
      auto_activate_time TEXT NOT NULL DEFAULT 'never',
      keyboard_shortcut TEXT NOT NULL DEFAULT 'Cmd+Shift+1',
      notifications_enabled BOOLEAN NOT NULL DEFAULT true,
      default_profile_id TEXT,
      last_active_profile_id TEXT,
      onboarding_completed BOOLEAN DEFAULT false,
      onboarding_step INTEGER DEFAULT 0,
      feature_flags TEXT DEFAULT '{}',
      keyboard_shortcuts TEXT DEFAULT '{}',
      ui_preferences TEXT DEFAULT '{}',
      created_at TIMESTAMP NOT NULL DEFAULT NOW(),
      updated_at TIMESTAMP NOT NULL DEFAULT NOW()
    )
    "#,
  )
  .execute(pool)
  .await?;
  info!("User settings table created");

  // Profiles table
  sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS profiles (
      id TEXT PRIMARY KEY,
      user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
      name TEXT NOT NULL,
      description TEXT,
      type TEXT NOT NULL,
      is_active BOOLEAN DEFAULT false,
      last_used TIMESTAMP,
      last_activated_at TIMESTAMP,
      activation_count INTEGER DEFAULT 0,
      is_favorite BOOLEAN DEFAULT false,
      color TEXT,
      icon TEXT,
      sort_order INTEGER DEFAULT 0,
      created_at TIMESTAMP NOT NULL DEFAULT NOW(),
      updated_at TIMESTAMP NOT NULL DEFAULT NOW()
    )
    "#,
  )
  .execute(pool)
  .await?;
  info!("Profiles table created");

  // Profile Tags table
  sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS profile_tags (
      profile_id TEXT NOT NULL REFERENCES profiles(id) ON DELETE CASCADE,
      tag TEXT NOT NULL,
      PRIMARY KEY (profile_id, tag)
    )
    "#,
  )
  .execute(pool)
  .await?;
  info!("Profile tags table created");

  // Monitors table
  sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS monitors (
      id TEXT PRIMARY KEY,
      profile_id TEXT NOT NULL REFERENCES profiles(id) ON DELETE CASCADE,
      name TEXT NOT NULL,
      resolution TEXT NOT NULL,
      orientation TEXT NOT NULL,
      is_primary BOOLEAN DEFAULT false,
      x INTEGER NOT NULL,
      y INTEGER NOT NULL,
      width INTEGER NOT NULL,
      height INTEGER NOT NULL,
      display_index INTEGER NOT NULL,
      brand TEXT,
      model TEXT,
      refresh_rate INTEGER,
      scale_factor REAL DEFAULT 1.0,
      is_builtin BOOLEAN DEFAULT false,
      color_depth INTEGER,
      created_at TIMESTAMP NOT NULL DEFAULT NOW(),
      updated_at TIMESTAMP NOT NULL DEFAULT NOW()
    )
    "#,
  )
  .execute(pool)
  .await?;
  info!("Monitors table created");

  // Apps table
  sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS apps (
      id TEXT PRIMARY KEY,
      profile_id TEXT NOT NULL REFERENCES profiles(id) ON DELETE CASCADE,
      name TEXT NOT NULL,
      bundle_id TEXT NOT NULL,
      exe_path TEXT,
      launch_on_activate BOOLEAN DEFAULT true,
      monitor_preference INTEGER,
      icon_path TEXT,
      launch_args TEXT,
      working_directory TEXT,
      startup_delay_ms INTEGER DEFAULT 0,
      order_index INTEGER DEFAULT 0,
      created_at TIMESTAMP NOT NULL DEFAULT NOW(),
      updated_at TIMESTAMP NOT NULL DEFAULT NOW()
    )
    "#,
  )
  .execute(pool)
  .await?;
  info!("Apps table created");

  // Windows table
  sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS windows (
      id TEXT PRIMARY KEY,
      profile_id TEXT NOT NULL REFERENCES profiles(id) ON DELETE CASCADE,
      app_id TEXT NOT NULL REFERENCES apps(id) ON DELETE CASCADE,
      monitor_id TEXT NOT NULL REFERENCES monitors(id) ON DELETE CASCADE,
      x INTEGER NOT NULL,
      y INTEGER NOT NULL,
      width INTEGER NOT NULL,
      height INTEGER NOT NULL,
      is_maximized BOOLEAN DEFAULT false,
      state TEXT NOT NULL,
      created_at TIMESTAMP NOT NULL DEFAULT NOW(),
      updated_at TIMESTAMP NOT NULL DEFAULT NOW()
    )
    "#,
  )
  .execute(pool)
  .await?;
  info!("Windows table created");

  // Browser Tabs table
  sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS browser_tabs (
      id TEXT PRIMARY KEY,
      profile_id TEXT NOT NULL REFERENCES profiles(id) ON DELETE CASCADE,
      url TEXT NOT NULL,
      browser TEXT NOT NULL,
      monitor_id TEXT REFERENCES monitors(id) ON DELETE CASCADE,
      tab_order INTEGER NOT NULL,
      favicon TEXT,
      created_at TIMESTAMP NOT NULL DEFAULT NOW(),
      updated_at TIMESTAMP NOT NULL DEFAULT NOW()
    )
    "#,
  )
  .execute(pool)
  .await?;
  info!("Browser tabs table created");

  // Automation Rules table
  sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS automation_rules (
      id TEXT PRIMARY KEY,
      profile_id TEXT NOT NULL REFERENCES profiles(id) ON DELETE CASCADE,
      name TEXT,
      description TEXT,
      rule_type TEXT NOT NULL,
      trigger_config TEXT NOT NULL,
      is_enabled BOOLEAN DEFAULT true,
      trigger_count INTEGER DEFAULT 0,
      last_triggered_at TIMESTAMP,
      created_at TIMESTAMP NOT NULL DEFAULT NOW(),
      updated_at TIMESTAMP NOT NULL DEFAULT NOW()
    )
    "#,
  )
  .execute(pool)
  .await?;
  info!("Automation rules table created");

  // ============================================================================
  // Logging & Audit Tables
  // ============================================================================

  // Sessions table
  sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS sessions (
      id TEXT PRIMARY KEY,
      user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
      device_id TEXT,
      device_name TEXT,
      device_type TEXT,
      os_name TEXT,
      os_version TEXT,
      app_version TEXT,
      ip_address TEXT,
      is_active BOOLEAN DEFAULT true,
      end_reason TEXT,
      metadata TEXT,
      started_at TIMESTAMP NOT NULL DEFAULT NOW(),
      last_activity_at TIMESTAMP NOT NULL DEFAULT NOW(),
      ended_at TIMESTAMP
    )
    "#,
  )
  .execute(pool)
  .await?;
  info!("Sessions table created");

  // Activity Logs table
  sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS activity_logs (
      id TEXT PRIMARY KEY,
      user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
      session_id TEXT REFERENCES sessions(id) ON DELETE SET NULL,
      action TEXT NOT NULL,
      entity_type TEXT,
      entity_id TEXT,
      entity_name TEXT,
      details TEXT,
      ip_address TEXT,
      user_agent TEXT,
      status TEXT NOT NULL DEFAULT 'success',
      error_message TEXT,
      duration_ms INTEGER,
      created_at TIMESTAMP NOT NULL DEFAULT NOW()
    )
    "#,
  )
  .execute(pool)
  .await?;
  info!("Activity logs table created");

  // System Events table
  sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS system_events (
      id TEXT PRIMARY KEY,
      event_type TEXT NOT NULL,
      severity TEXT NOT NULL DEFAULT 'info',
      source TEXT NOT NULL,
      message TEXT NOT NULL,
      details TEXT,
      stack_trace TEXT,
      os_info TEXT,
      app_version TEXT,
      created_at TIMESTAMP NOT NULL DEFAULT NOW()
    )
    "#,
  )
  .execute(pool)
  .await?;
  info!("System events table created");

  // Profile Activations table
  sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS profile_activations (
      id TEXT PRIMARY KEY,
      user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
      profile_id TEXT NOT NULL REFERENCES profiles(id) ON DELETE CASCADE,
      session_id TEXT REFERENCES sessions(id) ON DELETE SET NULL,
      activation_source TEXT NOT NULL,
      previous_profile_id TEXT REFERENCES profiles(id) ON DELETE SET NULL,
      monitors_detected INTEGER DEFAULT 0,
      monitors_applied INTEGER DEFAULT 0,
      apps_detected INTEGER DEFAULT 0,
      apps_launched INTEGER DEFAULT 0,
      apps_failed INTEGER DEFAULT 0,
      tabs_detected INTEGER DEFAULT 0,
      tabs_opened INTEGER DEFAULT 0,
      windows_restored INTEGER DEFAULT 0,
      duration_ms INTEGER,
      success BOOLEAN NOT NULL DEFAULT true,
      error_message TEXT,
      rollback_performed BOOLEAN DEFAULT false,
      metadata TEXT,
      started_at TIMESTAMP NOT NULL DEFAULT NOW(),
      completed_at TIMESTAMP
    )
    "#,
  )
  .execute(pool)
  .await?;
  info!("Profile activations table created");

  // Error Logs table
  sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS error_logs (
      id TEXT PRIMARY KEY,
      user_id TEXT REFERENCES users(id) ON DELETE SET NULL,
      session_id TEXT REFERENCES sessions(id) ON DELETE SET NULL,
      error_code TEXT,
      error_type TEXT NOT NULL,
      message TEXT NOT NULL,
      stack_trace TEXT,
      context TEXT,
      source_file TEXT,
      source_line INTEGER,
      source_function TEXT,
      severity TEXT NOT NULL DEFAULT 'error',
      is_resolved BOOLEAN DEFAULT false,
      resolved_at TIMESTAMP,
      resolution_notes TEXT,
      occurrence_count INTEGER DEFAULT 1,
      first_occurred_at TIMESTAMP NOT NULL DEFAULT NOW(),
      last_occurred_at TIMESTAMP NOT NULL DEFAULT NOW(),
      created_at TIMESTAMP NOT NULL DEFAULT NOW()
    )
    "#,
  )
  .execute(pool)
  .await?;
  info!("Error logs table created");

  // Automation Executions table
  sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS automation_executions (
      id TEXT PRIMARY KEY,
      rule_id TEXT NOT NULL REFERENCES automation_rules(id) ON DELETE CASCADE,
      user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
      profile_id TEXT REFERENCES profiles(id) ON DELETE SET NULL,
      trigger_type TEXT NOT NULL,
      trigger_details TEXT,
      success BOOLEAN NOT NULL DEFAULT true,
      error_message TEXT,
      actions_taken TEXT,
      duration_ms INTEGER,
      executed_at TIMESTAMP NOT NULL DEFAULT NOW()
    )
    "#,
  )
  .execute(pool)
  .await?;
  info!("Automation executions table created");

  // Monitor Changes table
  sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS monitor_changes (
      id TEXT PRIMARY KEY,
      user_id TEXT REFERENCES users(id) ON DELETE SET NULL,
      session_id TEXT REFERENCES sessions(id) ON DELETE SET NULL,
      change_type TEXT NOT NULL,
      monitors_before TEXT,
      monitors_after TEXT,
      auto_profile_activated BOOLEAN DEFAULT false,
      activated_profile_id TEXT REFERENCES profiles(id) ON DELETE SET NULL,
      detected_at TIMESTAMP NOT NULL DEFAULT NOW()
    )
    "#,
  )
  .execute(pool)
  .await?;
  info!("Monitor changes table created");

  // App Launches table
  sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS app_launches (
      id TEXT PRIMARY KEY,
      user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
      profile_id TEXT REFERENCES profiles(id) ON DELETE SET NULL,
      activation_id TEXT REFERENCES profile_activations(id) ON DELETE SET NULL,
      app_id TEXT REFERENCES apps(id) ON DELETE SET NULL,
      bundle_id TEXT NOT NULL,
      app_name TEXT NOT NULL,
      exe_path TEXT,
      success BOOLEAN NOT NULL DEFAULT true,
      error_message TEXT,
      pid INTEGER,
      launch_duration_ms INTEGER,
      window_positioned BOOLEAN DEFAULT false,
      launched_at TIMESTAMP NOT NULL DEFAULT NOW()
    )
    "#,
  )
  .execute(pool)
  .await?;
  info!("App launches table created");

  // Sync History table
  sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS sync_history (
      id TEXT PRIMARY KEY,
      user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
      profile_id TEXT REFERENCES profiles(id) ON DELETE CASCADE,
      action TEXT NOT NULL,
      status TEXT NOT NULL,
      sync_type TEXT DEFAULT 'full',
      items_synced INTEGER DEFAULT 0,
      conflicts_resolved INTEGER DEFAULT 0,
      error_message TEXT,
      duration_ms INTEGER,
      metadata TEXT,
      synced_at TIMESTAMP NOT NULL DEFAULT NOW()
    )
    "#,
  )
  .execute(pool)
  .await?;
  info!("Sync history table created");

  // Feedback / Feature Requests table
  sqlx::query(
    r#"
    CREATE TABLE IF NOT EXISTS feedback (
      id TEXT PRIMARY KEY,
      user_id TEXT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
      feedback_type TEXT NOT NULL,
      title TEXT NOT NULL,
      description TEXT NOT NULL,
      priority TEXT DEFAULT 'medium',
      status TEXT DEFAULT 'open',
      category TEXT,
      contact_email TEXT,
      app_version TEXT,
      os_info TEXT,
      metadata TEXT,
      created_at TIMESTAMP NOT NULL DEFAULT NOW(),
      updated_at TIMESTAMP NOT NULL DEFAULT NOW()
    )
    "#,
  )
  .execute(pool)
  .await?;
  info!("Feedback table created");

  // ============================================================================
  // Indexes
  // ============================================================================
  info!("Creating indexes");

  // Core table indexes
  sqlx::query("CREATE INDEX IF NOT EXISTS idx_user_settings_user_id ON user_settings(user_id)")
    .execute(pool)
    .await?;
  sqlx::query("CREATE INDEX IF NOT EXISTS idx_profiles_user_id ON profiles(user_id)")
    .execute(pool)
    .await?;
  sqlx::query(
    "CREATE INDEX IF NOT EXISTS idx_profiles_sort_order ON profiles(user_id, sort_order)",
  )
  .execute(pool)
  .await?;
  sqlx::query("CREATE INDEX IF NOT EXISTS idx_monitors_profile_id ON monitors(profile_id)")
    .execute(pool)
    .await?;
  sqlx::query("CREATE INDEX IF NOT EXISTS idx_apps_profile_id ON apps(profile_id)")
    .execute(pool)
    .await?;
  sqlx::query("CREATE INDEX IF NOT EXISTS idx_apps_order_index ON apps(profile_id, order_index)")
    .execute(pool)
    .await?;
  sqlx::query("CREATE INDEX IF NOT EXISTS idx_browser_tabs_profile_id ON browser_tabs(profile_id)")
    .execute(pool)
    .await?;
  sqlx::query(
    "CREATE INDEX IF NOT EXISTS idx_automation_rules_profile_id ON automation_rules(profile_id)",
  )
  .execute(pool)
  .await?;

  // Logging table indexes
  sqlx::query("CREATE INDEX IF NOT EXISTS idx_sessions_user ON sessions(user_id)")
    .execute(pool)
    .await?;
  sqlx::query("CREATE INDEX IF NOT EXISTS idx_sessions_active ON sessions(is_active)")
    .execute(pool)
    .await?;
  sqlx::query("CREATE INDEX IF NOT EXISTS idx_sessions_started ON sessions(started_at DESC)")
    .execute(pool)
    .await?;
  sqlx::query("CREATE INDEX IF NOT EXISTS idx_activity_logs_user_id ON activity_logs(user_id)")
    .execute(pool)
    .await?;
  sqlx::query("CREATE INDEX IF NOT EXISTS idx_activity_logs_action ON activity_logs(action)")
    .execute(pool)
    .await?;
  sqlx::query(
    "CREATE INDEX IF NOT EXISTS idx_activity_logs_created_at ON activity_logs(created_at DESC)",
  )
  .execute(pool)
  .await?;
  sqlx::query("CREATE INDEX IF NOT EXISTS idx_system_events_type ON system_events(event_type)")
    .execute(pool)
    .await?;
  sqlx::query("CREATE INDEX IF NOT EXISTS idx_system_events_severity ON system_events(severity)")
    .execute(pool)
    .await?;
  sqlx::query(
    "CREATE INDEX IF NOT EXISTS idx_system_events_created_at ON system_events(created_at DESC)",
  )
  .execute(pool)
  .await?;
  sqlx::query(
    "CREATE INDEX IF NOT EXISTS idx_profile_activations_user ON profile_activations(user_id)",
  )
  .execute(pool)
  .await?;
  sqlx::query(
    "CREATE INDEX IF NOT EXISTS idx_profile_activations_profile ON profile_activations(profile_id)",
  )
  .execute(pool)
  .await?;
  sqlx::query("CREATE INDEX IF NOT EXISTS idx_profile_activations_started ON profile_activations(started_at DESC)")
    .execute(pool)
    .await?;
  sqlx::query("CREATE INDEX IF NOT EXISTS idx_error_logs_type ON error_logs(error_type)")
    .execute(pool)
    .await?;
  sqlx::query("CREATE INDEX IF NOT EXISTS idx_error_logs_severity ON error_logs(severity)")
    .execute(pool)
    .await?;
  sqlx::query("CREATE INDEX IF NOT EXISTS idx_error_logs_created ON error_logs(created_at DESC)")
    .execute(pool)
    .await?;
  sqlx::query(
    "CREATE INDEX IF NOT EXISTS idx_automation_exec_rule ON automation_executions(rule_id)",
  )
  .execute(pool)
  .await?;
  sqlx::query(
    "CREATE INDEX IF NOT EXISTS idx_monitor_changes_detected ON monitor_changes(detected_at DESC)",
  )
  .execute(pool)
  .await?;
  sqlx::query("CREATE INDEX IF NOT EXISTS idx_app_launches_user ON app_launches(user_id)")
    .execute(pool)
    .await?;
  sqlx::query("CREATE INDEX IF NOT EXISTS idx_app_launches_at ON app_launches(launched_at DESC)")
    .execute(pool)
    .await?;
  sqlx::query("CREATE INDEX IF NOT EXISTS idx_sync_history_user_id ON sync_history(user_id)")
    .execute(pool)
    .await?;
  sqlx::query("CREATE INDEX IF NOT EXISTS idx_feedback_user_id ON feedback(user_id)")
    .execute(pool)
    .await?;
  sqlx::query("CREATE INDEX IF NOT EXISTS idx_feedback_status ON feedback(status)")
    .execute(pool)
    .await?;
  sqlx::query("CREATE INDEX IF NOT EXISTS idx_feedback_created_at ON feedback(created_at DESC)")
    .execute(pool)
    .await?;
  info!("Indexes created");

  // ============================================================================
  // Default Data
  // ============================================================================
  info!("Creating default user");

  // Default user for development
  sqlx::query(
    r#"
    INSERT INTO users (id, email, password_hash, username)
    VALUES ('00000000-0000-0000-0000-000000000001', 'default@smoothie.local', 'no-auth', 'default_user')
    ON CONFLICT (id) DO NOTHING
    "#,
  )
  .execute(pool)
  .await?;

  // Default user settings
  sqlx::query(
    r#"
    INSERT INTO user_settings (id, user_id)
    VALUES ('00000000-0000-0000-0000-000000000002', '00000000-0000-0000-0000-000000000001')
    ON CONFLICT (user_id) DO NOTHING
    "#,
  )
  .execute(pool)
  .await?;
  info!("Default user and settings created");

  let duration = start.elapsed();
  info!("Migration v1 completed in {}ms", duration.as_millis());
  Ok(())
}
