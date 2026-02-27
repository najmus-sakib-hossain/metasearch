//! Shared application state.

use std::sync::Arc;

use metasearch_core::config::Settings;
use metasearch_engine::EngineRegistry;

use crate::cache::SearchCache;

/// Shared state available to all request handlers.
pub struct AppState {
    pub settings: Settings,
    pub engine_registry: Arc<EngineRegistry>,
    pub cache: SearchCache,
}
