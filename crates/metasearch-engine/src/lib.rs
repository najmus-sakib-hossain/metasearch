//! # metasearch-engine
//!
//! Concrete search engine implementations.
//! Each engine scrapes or queries a search provider
//! and returns normalized `SearchResult` items.
//!
//! ## Original engines (4):
//! - Google, DuckDuckGo, Brave, Wikipedia
//!
//! ## Batch 1 — SearXNG translations (10):
//! - Bing, arXiv, Ask, Bandcamp, Baidu, 9GAG,
//!   Apple App Store, Bilibili, Art Institute of Chicago, Alpine Linux
//!
//! ## Batch 2 — More SearXNG translations (13):
//! - GitHub, Hacker News, Docker Hub, npm, crates.io, PyPI,
//!   Reddit, Dailymotion, Deezer, eBay, IMDb, SoundCloud, Flickr
//!
//! ## Batch 3 — Even more SearXNG translations (12):
//! - YouTube, Spotify, Crossref, Lemmy, Mastodon, Hugging Face,
//!   Goodreads, Bing News, Bing Images, Bing Videos, Genius, GitLab
//!
//! ## Batch 4 — Continuing SearXNG translations (7):
//! - Yahoo, Qwant, Vimeo, Unsplash, Semantic Scholar,
//!   StackExchange, Freesound
//!
//! ## Batch 5 — More SearXNG translations (5):
//! - 1337x, APKMirror, Arch Linux Wiki, ArtStation, F-Droid
//!
//! ## Batch 6 — More SearXNG translations (6):
//! - Emojipedia, Fyyd, Mixcloud, BitChute, BPB, Chefkoch

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

// Batch 4
pub mod yahoo;
pub mod qwant;
pub mod vimeo;
pub mod unsplash;
pub mod semantic_scholar;
pub mod stackexchange;
pub mod freesound;

// Batch 5
pub mod leet_x;
pub mod apkmirror;
pub mod archlinux;
pub mod artstation;
pub mod fdroid;

// Batch 6
pub mod emojipedia;
pub mod fyyd;
pub mod mixcloud;
pub mod bitchute;
pub mod bpb;
pub mod chefkoch;

pub mod registry;

pub use registry::EngineRegistry;
