// Database module - initialization, migrations, and connection pooling

pub mod connection;
pub mod migrations;
pub mod queries;

use sqlx::postgres::PgPool;
use std::env;

pub struct Database {
    pool: PgPool,
}

impl Database {
    /// Initialize database with migrations
    pub async fn new() -> anyhow::Result<Self> {
        let database_url = Self::get_database_url();

        let pool = connection::create_pool(&database_url).await?;

        // Run migrations
        migrations::run(&pool).await?;

        Ok(Self { pool })
    }

    /// Get database URL from environment variable
    fn get_database_url() -> String {
        env::var("DATABASE_URL")
            .unwrap_or_else(|_| "postgresql://localhost:5432/smoothie".to_string())
    }

    /// Get connection pool reference
    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}
