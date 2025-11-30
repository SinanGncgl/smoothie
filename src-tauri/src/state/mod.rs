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

    /// Get cached value or compute it
    pub async fn get_or_compute<F, T>(
        &self,
        key: &str,
        compute: F,
    ) -> crate::error::Result<T>
    where
        F: std::future::Future<Output = crate::error::Result<T>>,
        T: serde::Serialize + serde::de::DeserializeOwned,
    {
        if let Some(cached) = self.cache.get(key) {
            return serde_json::from_value((*cached).as_ref().clone())
                .map_err(|e| crate::error::SmoothieError::SerializationError(e.to_string()));
        }

        let result = compute.await?;
        let serialized = serde_json::to_value(&result)
            .map_err(|e| crate::error::SmoothieError::SerializationError(e.to_string()))?;
        self.cache.insert(key.to_string(), Arc::new(serialized));

        Ok(result)
    }

    /// Clear cache for a specific key
    pub fn invalidate_cache(&self, key: &str) {
        self.cache.remove(key);
    }

    /// Clear all cache
    pub fn clear_cache(&self) {
        self.cache.clear();
    }
}
