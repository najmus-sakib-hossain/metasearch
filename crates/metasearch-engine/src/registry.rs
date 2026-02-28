//! Engine registry — manages all available search engines.

use std::collections::HashMap;
use std::sync::Arc;

use metasearch_core::category::SearchCategory;
use metasearch_core::engine::SearchEngine;
use reqwest::Client;

use crate::{
    // Batch 6
    acfun::AcFun,
    // Batch 9
    adobe_stock::AdobeStock,
    alpinelinux::AlpineLinux,
    annas_archive::AnnasArchive,
    ansa::Ansa,
    apkmirror::ApkMirror,
    apple_app_store::AppleAppStore,
    archlinux::ArchLinux,
    artic::Artic,
    artstation::ArtStation,
    arxiv::Arxiv,
    ask::Ask,
    baidu::Baidu,
    bandcamp::Bandcamp,
    base_search::BaseSearch,
    bilibili::Bilibili,
    bing::Bing,
    bing_images::BingImages,
    bing_news::BingNews,
    bing_videos::BingVideos,
    bitchute::BitChute,
    bpb::Bpb,
    brave::Brave,
    // Batch 7
    bt4g::Bt4g,
    btdigg::Btdigg,
    cachy_os::CachyOs,
    ccc_media::CccMedia,
    chefkoch::Chefkoch,
    crates_io::CratesIo,
    crossref::Crossref,
    dailymotion::Dailymotion,
    deezer::Deezer,
    destatis::Destatis,
    devicons::Devicons,
    digbt::Digbt,
    docker_hub::DockerHub,
    duckduckgo::DuckDuckGo,
    ebay::Ebay,
    emojipedia::Emojipedia,
    fdroid::Fdroid,
    findthatmeme::FindThatMeme,
    flickr::Flickr,
    freesound::Freesound,
    // Batch 8
    frinkiac::Frinkiac,
    fyyd::Fyyd,
    genius::Genius,
    github_engine::GitHub,
    gitlab::GitLab,
    goodreads::Goodreads,
    google::Google,
    hackernews::HackerNews,
    hex::Hex,
    huggingface::HuggingFace,
    imdb::Imdb,
    ina::Ina,
    ipernity::Ipernity,
    // Batch 5
    leet_x::LeetX,
    lemmy::Lemmy,
    mastodon::Mastodon,
    mixcloud::Mixcloud,
    nine_gag::NineGag,
    npm::Npm,
    pypi::PyPI,
    qwant::Qwant,
    reddit::Reddit,
    semantic_scholar::SemanticScholar,
    soundcloud::SoundCloud,
    spotify::Spotify,
    stackexchange::StackExchange,
    unsplash::Unsplash,
    vimeo::Vimeo,
    wikipedia::Wikipedia,
    yahoo::Yahoo,
    youtube::YouTube,
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

    /// Create a registry pre-loaded with all built-in engines (74 total).
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

        // ── Batch 3: Even more SearXNG translations ───────
        registry.register(Arc::new(YouTube::new(client.clone())));
        registry.register(Arc::new(Spotify::new(client.clone())));
        registry.register(Arc::new(Crossref::new(client.clone())));
        registry.register(Arc::new(Lemmy::new(client.clone())));
        registry.register(Arc::new(Mastodon::new(client.clone())));
        registry.register(Arc::new(HuggingFace::new(client.clone())));
        registry.register(Arc::new(Goodreads::new(client.clone())));
        registry.register(Arc::new(BingNews::new(client.clone())));
        registry.register(Arc::new(BingImages::new(client.clone())));
        registry.register(Arc::new(BingVideos::new(client.clone())));
        registry.register(Arc::new(Genius::new(client.clone())));
        registry.register(Arc::new(GitLab::new(client.clone())));

        // ── Batch 4: Continuing SearXNG translations ──────
        registry.register(Arc::new(Yahoo::new(client.clone())));
        registry.register(Arc::new(Qwant::new(client.clone())));
        registry.register(Arc::new(Vimeo::new(client.clone())));
        registry.register(Arc::new(Unsplash::new(client.clone())));
        registry.register(Arc::new(SemanticScholar::new(client.clone())));
        registry.register(Arc::new(StackExchange::new(client.clone())));
        registry.register(Arc::new(Freesound::new(client.clone())));

        // ── Batch 5: More SearXNG translations ────────────
        registry.register(Arc::new(LeetX::new(client.clone())));
        registry.register(Arc::new(ApkMirror::new(client.clone())));
        registry.register(Arc::new(ArchLinux::new(client.clone())));
        registry.register(Arc::new(ArtStation::new(client.clone())));
        registry.register(Arc::new(Fdroid::new(client.clone())));

        // ── Batch 6: More SearXNG translations ────────────
        registry.register(Arc::new(AcFun::new(client.clone())));
        registry.register(Arc::new(Ansa::new(client.clone())));
        registry.register(Arc::new(BitChute::new(client.clone())));
        registry.register(Arc::new(Bpb::new(client.clone())));
        registry.register(Arc::new(Chefkoch::new(client.clone())));
        registry.register(Arc::new(Emojipedia::new(client.clone())));
        registry.register(Arc::new(FindThatMeme::new(client.clone())));
        registry.register(Arc::new(Fyyd::new(client.clone())));
        registry.register(Arc::new(Mixcloud::new(client.clone())));

        // ── Batch 7: More SearXNG translations ────────────
        registry.register(Arc::new(Bt4g::new(client.clone())));
        registry.register(Arc::new(Btdigg::new(client.clone())));
        registry.register(Arc::new(CachyOs::new(client.clone())));
        registry.register(Arc::new(CccMedia::new(client.clone())));
        registry.register(Arc::new(Destatis::new(client.clone())));

        // ── Batch 8: More SearXNG translations ────────────
        registry.register(Arc::new(Frinkiac::new(client.clone())));
        registry.register(Arc::new(Hex::new(client.clone())));
        registry.register(Arc::new(Ina::new(client.clone())));
        registry.register(Arc::new(Ipernity::new(client.clone())));
        registry.register(Arc::new(Devicons::new(client.clone())));

        // ── Batch 9: More SearXNG translations ────────────
        registry.register(Arc::new(AdobeStock::new(client.clone())));
        registry.register(Arc::new(AnnasArchive::new(client.clone())));
        registry.register(Arc::new(BaseSearch::new(client.clone())));
        registry.register(Arc::new(Digbt::new(client.clone())));

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
