//! Engine registry — manages all available search engines.

use std::collections::HashMap;
use std::sync::Arc;

use metasearch_core::engine::SearchEngine;
use metasearch_core::category::SearchCategory;

/// Central registry of all search engines.
pub struct EngineRegistry {
    engines: HashMap<String, Arc<dyn SearchEngine>>,
}

impl EngineRegistry {
    pub fn new() -> Self {
        Self {
            engines: HashMap::new(),
        }
    }

    /// Register a new engine.
    pub fn register(&mut self, engine: Arc<dyn SearchEngine>) {
        let name = engine.metadata().name.clone();
        self.engines.insert(name, engine);
    }

    /// Get an engine by name.
    pub fn get(&self, name: &str) -> Option<&Arc<dyn SearchEngine>> {
        self.engines.get(name)
    }

    /// Get all enabled engines for a given category.
    pub fn engines_for_category(&self, category: &SearchCategory) -> Vec<Arc<dyn SearchEngine>> {
        self.engines
            .values()
            .filter(|e| e.metadata().enabled && e.metadata().categories.contains(category))
            .cloned()
            .collect()
    }

    /// List all registered engine names.
    pub fn engine_names(&self) -> Vec<String> {
        self.engines.keys().cloned().collect()
    }

    /// Number of registered engines.
    pub fn count(&self) -> usize {
        self.engines.len()
    }
}

impl Default for EngineRegistry {
    fn default() -> Self {
        Self::new()
    }
}
