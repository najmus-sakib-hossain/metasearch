//! Shared application state.

use std::sync::Arc;

use metasearch_core::config::Settings;
use metasearch_engine::EngineRegistry;

use crate::cache::SearchCache;
use crate::health::EngineHealthTracker;
use crate::orchestrator::SearchOrchestrator;
use crate::templates::TemplateEngine;

/// Shared state available to all request handlers.
pub struct AppState {
    pub settings: Settings,
    pub engine_registry: Arc<EngineRegistry>,
    pub cache: SearchCache,
    pub templates: TemplateEngine,
    /// High-level search coordinator (cache + health + fan-out).
    pub orchestrator: Arc<SearchOrchestrator>,
    /// Per-engine health tracker shared with the orchestrator.
    pub health: Arc<EngineHealthTracker>,
}
