//! Engine registry — manages all available search engines.

use std::collections::HashMap;
use std::sync::Arc;

use metasearch_core::engine::SearchEngine;
use metasearch_core::category::SearchCategory;
use reqwest::Client;

use crate::{
    google::Google,
    duckduckgo::DuckDuckGo,
    brave::Brave,
    wikipedia::Wikipedia,
    bing::Bing,
    arxiv::Arxiv,
    ask::Ask,
    bandcamp::Bandcamp,
    baidu::Baidu,
    nine_gag::NineGag,
    apple_app_store::AppleAppStore,
    bilibili::Bilibili,
    artic::Artic,
    alpinelinux::AlpineLinux,
    github_engine::GitHub,
    hackernews::HackerNews,
    docker_hub::DockerHub,
    npm::Npm,
    crates_io::CratesIo,
    pypi::PyPI,
    reddit::Reddit,
    dailymotion::Dailymotion,
    deezer::Deezer,
    ebay::Ebay,
    imdb::Imdb,
    soundcloud::SoundCloud,
    flickr::Flickr,
};

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

    /// Create a registry pre-loaded with all built-in engines (27 total).
    pub fn with_defaults(client: Client) -> Self {
        let mut registry = Self::new();

        // ── Original engines ──────────────────────────────
        registry.register(Arc::new(Google::new(client.clone())));
        registry.register(Arc::new(DuckDuckGo::new(client.clone())));
        registry.register(Arc::new(Brave::new(client.clone(), None)));
        registry.register(Arc::new(Wikipedia::new(client.clone())));

        // ── Batch 1: SearXNG translations ─────────────────
        registry.register(Arc::new(Bing::new(client.clone())));
        registry.register(Arc::new(Arxiv::new(client.clone())));
        registry.register(Arc::new(Ask::new(client.clone())));
        registry.register(Arc::new(Bandcamp::new(client.clone())));
        registry.register(Arc::new(Baidu::new(client.clone())));
        registry.register(Arc::new(NineGag::new(client.clone())));
        registry.register(Arc::new(AppleAppStore::new(client.clone())));
        registry.register(Arc::new(Bilibili::new(client.clone())));
        registry.register(Arc::new(Artic::new(client.clone())));
        registry.register(Arc::new(AlpineLinux::new(client.clone())));

        // ── Batch 2: More SearXNG translations ────────────
        registry.register(Arc::new(GitHub::new(client.clone())));
        registry.register(Arc::new(HackerNews::new(client.clone())));
        registry.register(Arc::new(DockerHub::new(client.clone())));
        registry.register(Arc::new(Npm::new(client.clone())));
        registry.register(Arc::new(CratesIo::new(client.clone())));
        registry.register(Arc::new(PyPI::new(client.clone())));
        registry.register(Arc::new(Reddit::new(client.clone())));
        registry.register(Arc::new(Dailymotion::new(client.clone())));
        registry.register(Arc::new(Deezer::new(client.clone())));
        registry.register(Arc::new(Ebay::new(client.clone())));
        registry.register(Arc::new(Imdb::new(client.clone())));
        registry.register(Arc::new(SoundCloud::new(client.clone())));
        registry.register(Arc::new(Flickr::new(client.clone())));

        registry
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
