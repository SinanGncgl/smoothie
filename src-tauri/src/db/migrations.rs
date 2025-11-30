// Database migrations for Smoothie schema

use sqlx::PgPool;

pub async fn run(pool: &PgPool) -> anyhow::Result<()> {
    run_migration_v1(pool).await?;

    tracing::info!("Database migrations completed successfully");
    Ok(())
}

async fn run_migration_v1(pool: &PgPool) -> anyhow::Result<()> {
    // Enable UUID extension
    sqlx::query("CREATE EXTENSION IF NOT EXISTS \"uuid-ossp\"")
        .execute(pool)
        .await?;

    // Users table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS users (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            email VARCHAR(255) UNIQUE NOT NULL,
            password_hash VARCHAR(255) NOT NULL,
            username VARCHAR(100) UNIQUE,
            theme VARCHAR(50) DEFAULT 'dark',
            notifications_enabled BOOLEAN DEFAULT true,
            auto_restore BOOLEAN DEFAULT true,
            created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW()
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Profiles table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS profiles (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            user_id UUID NOT NULL,
            name VARCHAR(255) NOT NULL,
            description TEXT,
            type VARCHAR(50) NOT NULL,
            is_active BOOLEAN DEFAULT false,
            created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
            updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
            last_used TIMESTAMP WITH TIME ZONE,
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Profile tags junction table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS profile_tags (
            profile_id UUID NOT NULL,
            tag VARCHAR(100) NOT NULL,
            PRIMARY KEY (profile_id, tag),
            FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Monitors table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS monitors (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            profile_id UUID NOT NULL,
            name VARCHAR(255) NOT NULL,
            resolution VARCHAR(50) NOT NULL,
            orientation VARCHAR(20) NOT NULL,
            is_primary BOOLEAN DEFAULT false,
            x INTEGER NOT NULL,
            y INTEGER NOT NULL,
            width INTEGER NOT NULL,
            height INTEGER NOT NULL,
            display_index INTEGER NOT NULL,
            FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Apps table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS apps (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            profile_id UUID NOT NULL,
            name VARCHAR(255) NOT NULL,
            bundle_id VARCHAR(255) NOT NULL,
            exe_path VARCHAR(500),
            launch_on_activate BOOLEAN DEFAULT true,
            monitor_preference INTEGER,
            created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
            FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Windows (app placements) table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS windows (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            profile_id UUID NOT NULL,
            app_id UUID NOT NULL,
            monitor_id UUID NOT NULL,
            x INTEGER NOT NULL,
            y INTEGER NOT NULL,
            width INTEGER NOT NULL,
            height INTEGER NOT NULL,
            is_maximized BOOLEAN DEFAULT false,
            state VARCHAR(50) NOT NULL,
            FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE,
            FOREIGN KEY (app_id) REFERENCES apps(id) ON DELETE CASCADE,
            FOREIGN KEY (monitor_id) REFERENCES monitors(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Browser tabs table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS browser_tabs (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            profile_id UUID NOT NULL,
            url VARCHAR(2000) NOT NULL,
            browser VARCHAR(100) NOT NULL,
            monitor_id UUID NOT NULL,
            tab_order INTEGER NOT NULL,
            favicon VARCHAR(500),
            created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
            FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE,
            FOREIGN KEY (monitor_id) REFERENCES monitors(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Automation rules table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS automation_rules (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            profile_id UUID NOT NULL,
            rule_type VARCHAR(50) NOT NULL,
            trigger_config JSONB NOT NULL,
            is_enabled BOOLEAN DEFAULT true,
            created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
            FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Sync history table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS sync_history (
            id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
            user_id UUID NOT NULL,
            profile_id UUID,
            action VARCHAR(100) NOT NULL,
            status VARCHAR(50) NOT NULL,
            synced_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT NOW(),
            FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
            FOREIGN KEY (profile_id) REFERENCES profiles(id) ON DELETE CASCADE
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create indexes for performance
    sqlx::query("CREATE INDEX IF NOT EXISTS idx_profiles_user_id ON profiles(user_id)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_monitors_profile_id ON monitors(profile_id)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_apps_profile_id ON apps(profile_id)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_browser_tabs_profile_id ON browser_tabs(profile_id)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_automation_rules_profile_id ON automation_rules(profile_id)")
        .execute(pool)
        .await?;

    sqlx::query("CREATE INDEX IF NOT EXISTS idx_sync_history_user_id ON sync_history(user_id)")
        .execute(pool)
        .await?;

    // Create a default user for development (without auth)
    sqlx::query(
        r#"
        INSERT INTO users (id, email, password_hash, username, theme, notifications_enabled, auto_restore)
        VALUES ('00000000-0000-0000-0000-000000000001', 'default@smoothie.local', 'no-auth', 'default_user', 'dark', true, true)
        ON CONFLICT (id) DO NOTHING
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}
