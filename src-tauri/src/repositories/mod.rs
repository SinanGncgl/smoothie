// Repository module - data access layer with repository pattern

mod profile_repository;
mod monitor_repository;
mod app_repository;
mod browser_tab_repository;
mod automation_repository;
mod sync_repository;

pub use profile_repository::ProfileRepository;
pub use monitor_repository::MonitorRepository;
pub use app_repository::AppRepository;
pub use browser_tab_repository::BrowserTabRepository;
pub use automation_repository::AutomationRepository;
pub use sync_repository::SyncRepository;

use async_trait::async_trait;
use sqlx::PgPool;

/// Base repository trait for common CRUD operations
#[async_trait]
pub trait Repository<T, ID> {
    async fn find_by_id(&self, id: ID) -> crate::error::Result<Option<T>>;
    async fn delete(&self, id: ID) -> crate::error::Result<bool>;
}

/// Provides access to the database pool
pub trait DatabaseAccess {
    fn pool(&self) -> &PgPool;
}
