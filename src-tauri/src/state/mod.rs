// Application state management

use crate::db::Database;
use dashmap::DashMap;
use std::sync::Arc;

pub struct AppState {
  pub db: Arc<Database>,
  // In-memory cache for frequently accessed data
  pub cache: DashMap<String, Arc<serde_json::Value>>,
}

impl AppState {
  pub fn new(db: Arc<Database>) -> Self {
    Self {
      db,
      cache: DashMap::new(),
    }
  }

  /// Clear cache for a specific key
  pub fn invalidate_cache(&self, key: &str) {
    self.cache.remove(key);
  }
}
