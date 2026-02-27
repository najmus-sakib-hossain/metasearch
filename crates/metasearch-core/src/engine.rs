//! Search engine trait definition.

use async_trait::async_trait;
use serde::{Deserialize, Serialize};

use crate::category::SearchCategory;
use crate::error::Result;
use crate::query::SearchQuery;
use crate::result::SearchResult;

/// Metadata describing a search engine.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EngineMetadata {
    /// Unique identifier (e.g., "google", "duckduckgo").
    pub name: String,

    /// Human-readable display name.
    pub display_name: String,

    /// URL of the engine's homepage.
    pub homepage: String,

    /// Categories this engine supports.
    pub categories: Vec<SearchCategory>,

    /// Whether this engine is enabled by default.
    pub enabled: bool,

    /// Timeout for requests to this engine (ms).
    pub timeout_ms: u64,

    /// Weight for result scoring (higher = more trusted).
    pub weight: f64,
}

/// The trait that every search engine must implement.
#[async_trait]
pub trait SearchEngine: Send + Sync {
    /// Return metadata about this engine.
    fn metadata(&self) -> &EngineMetadata;

    /// Perform a search and return results.
    async fn search(&self, query: &SearchQuery) -> Result<Vec<SearchResult>>;

    /// Optional: return autocomplete suggestions.
    async fn autocomplete(&self, _partial: &str) -> Result<Vec<String>> {
        Ok(Vec::new())
    }
}
