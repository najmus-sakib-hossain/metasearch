//! # metasearch-engine
//!
//! Concrete search engine implementations.
//! Each engine scrapes or queries a search provider
//! and returns normalized `SearchResult` items.
//!
//! ## Batch 1 — Engines translated from SearXNG Python source:
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
//!
//! ## Batch 2 — More engines from SearXNG:
//! - GitHub (repos) — REST JSON API
//! - Hacker News (IT/news) — Algolia JSON API
//! - Docker Hub (containers) — JSON API
//! - npm (JS packages) — npms.io JSON API
//! - crates.io (Rust packages) — JSON API
//! - PyPI (Python packages) — HTML scraping
//! - Reddit (social media) — JSON API
//! - Dailymotion (videos) — JSON API
//! - Deezer (music) — JSON API
//! - eBay (shopping) — HTML scraping
//! - IMDb (movies) — suggestion JSON API
//! - SoundCloud (music) — JSON API v2
//! - Flickr (images) — HTML/JSON scraping
//!
//! ## Batch 3 — Even more engines from SearXNG:
//! - YouTube (videos) — HTML scraping (no API key)
//! - Spotify (music) — OAuth JSON API
//! - Crossref (science/papers) — JSON API
//! - Lemmy (social media) — JSON API
//! - Mastodon (social media) — JSON API
//! - Hugging Face (AI models) — JSON API
//! - Goodreads (books) — HTML scraping
//! - Bing News — HTML scraping
//! - Bing Images — HTML scraping
//! - Bing Videos — HTML scraping
//! - Genius (music/lyrics) — JSON API
//! - GitLab (repos) — JSON API

// Original engines
pub mod google;
pub mod duckduckgo;
pub mod brave;
pub mod wikipedia;

// Batch 1
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

// Batch 2
pub mod github_engine;
pub mod hackernews;
pub mod docker_hub;
pub mod npm;
pub mod crates_io;
pub mod pypi;
pub mod reddit;
pub mod dailymotion;
pub mod deezer;
pub mod ebay;
pub mod imdb;
pub mod soundcloud;
pub mod flickr;

// Batch 3
pub mod youtube;
pub mod spotify;
pub mod crossref;
pub mod lemmy;
pub mod mastodon;
pub mod huggingface;
pub mod goodreads;
pub mod bing_news;
pub mod bing_images;
pub mod bing_videos;
pub mod genius;
pub mod gitlab;

pub mod registry;

pub use registry::EngineRegistry;
