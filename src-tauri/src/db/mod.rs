// Database module - PostgreSQL implementation for open source version

pub mod connection;
pub mod migrations;

use sqlx::postgres::PgPool;
use tracing::{error, info};

#[derive(Clone)]
pub struct Database {
  pool: PgPool,
}

impl Database {
  /// Initialize PostgreSQL database
  pub async fn new() -> anyhow::Result<Self> {
    info!("Initializing PostgreSQL database");
    let start = std::time::Instant::now();

    let pool = match connection::create_pool().await {
      Ok(pool) => {
        let duration = start.elapsed();
        info!(
          "PostgreSQL database connection pool created in {}ms",
          duration.as_millis()
        );
        pool
      }
      Err(e) => {
        error!(
          "Failed to create PostgreSQL database connection pool: {}",
          e
        );
        return Err(e);
      }
    };

    // Run migrations
    let migration_start = std::time::Instant::now();
    match migrations::run(&pool).await {
      Ok(_) => {
        let duration = migration_start.elapsed();
        info!(
          "Database migrations completed in {}ms",
          duration.as_millis()
        );
      }
      Err(e) => {
        error!("Failed to run database migrations: {}", e);
        return Err(e);
      }
    }

    let total_duration = start.elapsed();
    info!(
      "PostgreSQL database initialization completed successfully in {}ms",
      total_duration.as_millis()
    );

    Ok(Self { pool })
  }

  /// Get connection pool reference (for backward compatibility)
  pub fn pool(&self) -> &PgPool {
    &self.pool
  }
}
