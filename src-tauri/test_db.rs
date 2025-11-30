use std::env;
use sqlx::postgres::PgPool;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    dotenv::dotenv().ok();

    let database_url = env::var("DATABASE_URL")
        .unwrap_or_else(|_| "postgresql://localhost:5432/smoothie".to_string());

    println!("Testing database connection to: {}", database_url);

    let pool = PgPool::connect(&database_url).await?;
    println!("✅ Database connection successful!");

    // Test a simple query
    let result: (i64,) = sqlx::query_as("SELECT 1 as test")
        .fetch_one(&pool)
        .await?;

    println!("✅ Query test successful! Result: {}", result.0);

    pool.close().await;
    println!("✅ Database connection closed successfully");

    Ok(())
}