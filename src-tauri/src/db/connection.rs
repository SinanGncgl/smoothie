// Database connection pool management for PostgreSQL

use sqlx::postgres::{PgPool, PgPoolOptions};
use std::time::Duration;
use tracing::{error, info};

pub async fn create_pool() -> anyhow::Result<PgPool> {
  info!("Creating PostgreSQL connection pool");
  let start = std::time::Instant::now();

  // Get database URL from environment
  let database_url = std::env::var("DATABASE_URL").unwrap_or_else(|_| {
    "postgresql://smoothie_user:smoothie_pass@localhost:5432/smoothie_dev".to_string()
  });

  info!("Connecting to PostgreSQL database");

  let pool = PgPoolOptions::new()
    .max_connections(5)
    .min_connections(1)
    .acquire_timeout(Duration::from_secs(30))
    .idle_timeout(Duration::from_secs(600))
    .max_lifetime(Duration::from_secs(1800))
    .connect(&database_url)
    .await;

  let duration = start.elapsed();

  match &pool {
    Ok(p) => {
      let pool_size = p.size();
      info!(
        "PostgreSQL connection pool created successfully in {}ms (size: {})",
        duration.as_millis(),
        pool_size
      );
    }
    Err(e) => {
      error!(
        "Failed to create PostgreSQL connection pool in {}ms: {}",
        duration.as_millis(),
        e
      );
    }
  }

  Ok(pool?)
}
