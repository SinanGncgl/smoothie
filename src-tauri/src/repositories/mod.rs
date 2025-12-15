//! Repository module - data access layer with repository pattern
//!
//! This module provides database access abstractions following the repository pattern.
//! Repositories encapsulate data access logic and provide a clean API for services.

mod app_repository;
mod audit_repository;
mod automation_repository;
mod browser_tab_repository;
mod monitor_repository;
mod profile_repository;
mod subscription_repository;
mod user_settings_repository;

pub use app_repository::AppRepository;
pub use audit_repository::AuditRepository;
pub use automation_repository::AutomationRepository;
pub use browser_tab_repository::BrowserTabRepository;
pub use monitor_repository::MonitorRepository;
pub use profile_repository::ProfileRepository;
pub use subscription_repository::SubscriptionRepository;
pub use user_settings_repository::UserSettingsRepository;
