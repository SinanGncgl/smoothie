// Database seeding script for development

use sqlx::PgPool;
use uuid::Uuid;
use chrono::Utc;

pub async fn seed(pool: &PgPool) -> anyhow::Result<()> {
    tracing::info!("Starting database seeding...");

    let user_id = Uuid::new_v4();

    // Create test user
    sqlx::query(
        "INSERT INTO users (id, email, password_hash, username) VALUES ($1, $2, $3, $4)"
    )
    .bind(user_id)
    .bind("test@smoothie.dev")
    .bind("$2b$12$test")
    .bind("testuser")
    .execute(pool)
    .await?;

    // Create test profiles
    for i in 1..=3 {
        let profile_id = Uuid::new_v4();
        let profile_name = match i {
            1 => "Work",
            2 => "Gaming",
            _ => "Research",
        };

        sqlx::query(
            "INSERT INTO profiles (id, user_id, name, type, is_active) VALUES ($1, $2, $3, $4, $5)"
        )
        .bind(profile_id)
        .bind(user_id)
        .bind(profile_name)
        .bind(profile_name)
        .bind(i == 1)
        .execute(pool)
        .await?;

        // Create monitors for each profile
        for j in 1..=2 {
            let monitor_id = Uuid::new_v4();
            sqlx::query(
                "INSERT INTO monitors (id, profile_id, name, resolution, orientation, is_primary, x, y, width, height, display_index)
                 VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)"
            )
            .bind(monitor_id)
            .bind(profile_id)
            .bind(format!("Monitor {}", j))
            .bind("2560x1440")
            .bind("Landscape")
            .bind(j == 1)
            .bind((j - 1) * 2560)
            .bind(0)
            .bind(2560)
            .bind(1440)
            .bind(j as i32)
            .execute(pool)
            .await?;
        }
    }

    tracing::info!("Database seeding completed successfully");
    Ok(())
}
