//! Result caching layer using Moka.

use metasearch_core::result::SearchResponse;
use moka::future::Cache;
use std::time::Duration;

/// Cache for search responses.
#[derive(Clone)]
pub struct SearchCache {
    inner: Cache<String, SearchResponse>,
}

impl SearchCache {
    pub fn new(max_capacity: u64, ttl_secs: u64) -> Self {
        let inner = Cache::builder()
            .max_capacity(max_capacity)
            .time_to_live(Duration::from_secs(ttl_secs))
            .build();

        Self { inner }
    }

    /// Build a cache key from query parameters.
    pub fn cache_key(query: &str, categories: &[String], page: u32, lang: &str) -> String {
        format!("{}:{}:{}:{}", query, categories.join(","), page, lang)
    }

    /// Get a cached response.
    pub async fn get(&self, key: &str) -> Option<SearchResponse> {
        self.inner.get(key).await
    }

    /// Insert a response into cache.
    pub async fn insert(&self, key: String, response: SearchResponse) {
        self.inner.insert(key, response).await;
    }
}
