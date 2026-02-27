//! # metasearch-engine
//!
//! Concrete search engine implementations.
//! Each engine scrapes or queries a search provider
//! and returns normalized `SearchResult` items.
//!
//! ## Engines translated from SearXNG Python source code:
//! - Bing (general, news, images) — HTML scraping
//! - arXiv (science) — Atom XML API
//! - Ask.com (general) — HTML scraping
//! - Bandcamp (music) — HTML scraping
//! - Baidu (general, Chinese) — JSON API
//! - 9GAG (social media) — JSON API
//! - Apple App Store (apps) — iTunes JSON API
//! - Bilibili (videos, Chinese) — JSON API
//! - Art Institute of Chicago (images) — JSON API
//! - Alpine Linux (packages) — HTML scraping

pub mod google;
pub mod duckduckgo;
pub mod brave;
pub mod wikipedia;
pub mod bing;
pub mod arxiv;
pub mod ask;
pub mod bandcamp;
pub mod baidu;
pub mod nine_gag;
pub mod apple_app_store;
pub mod bilibili;
pub mod artic;
pub mod alpinelinux;
pub mod registry;

pub use registry::EngineRegistry;
