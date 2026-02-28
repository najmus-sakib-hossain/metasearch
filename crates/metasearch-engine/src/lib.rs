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
//! ## Batch 6 — More SearXNG translations (9):
//! - AcFun, ANSA, BitChute, BPB, Chefkoch, Emojipedia,
//!   FindThatMeme, Fyyd, Mixcloud
//!
//! ## Batch 7 — More SearXNG translations (5):
//! - BT4G, BTDigg, CachyOS, CCC Media, DeStatis
//!
//! ## Batch 8 — More SearXNG translations (5):
//! - Frinkiac, Hex.pm, INA, Ipernity, Devicons
//!
//! ## Batch 9 — More SearXNG translations (4):
//! - Adobe Stock, Anna's Archive, BASE, DigBT
//!
//! ## Batch 10 — More SearXNG translations (5):
//! - Geizhals, Grokipedia, Il Post, Library of Congress, MetaCPAN

// Original engines
pub mod brave;
pub mod duckduckgo;
pub mod google;
pub mod wikipedia;

// Batch 1
pub mod alpinelinux;
pub mod apple_app_store;
pub mod artic;
pub mod arxiv;
pub mod ask;
pub mod baidu;
pub mod bandcamp;
pub mod bilibili;
pub mod bing;
pub mod nine_gag;

// Batch 2
pub mod crates_io;
pub mod dailymotion;
pub mod deezer;
pub mod docker_hub;
pub mod ebay;
pub mod flickr;
pub mod github_engine;
pub mod hackernews;
pub mod imdb;
pub mod npm;
pub mod pypi;
pub mod reddit;
pub mod soundcloud;

// Batch 3
pub mod bing_images;
pub mod bing_news;
pub mod bing_videos;
pub mod crossref;
pub mod genius;
pub mod gitlab;
pub mod goodreads;
pub mod huggingface;
pub mod lemmy;
pub mod mastodon;
pub mod spotify;
pub mod youtube;

// Batch 4
pub mod freesound;
pub mod qwant;
pub mod semantic_scholar;
pub mod stackexchange;
pub mod unsplash;
pub mod vimeo;
pub mod yahoo;

// Batch 5
pub mod apkmirror;
pub mod archlinux;
pub mod artstation;
pub mod fdroid;
pub mod leet_x;

// Batch 6
pub mod acfun;
pub mod ansa;
pub mod bitchute;
pub mod bpb;
pub mod chefkoch;
pub mod emojipedia;
pub mod findthatmeme;
pub mod fyyd;
pub mod mixcloud;

// Batch 7
pub mod bt4g;
pub mod btdigg;
pub mod cachy_os;
pub mod ccc_media;
pub mod destatis;

// Batch 8
pub mod devicons;
pub mod frinkiac;
pub mod hex;
pub mod ina;
pub mod ipernity;

// Batch 9
pub mod adobe_stock;
pub mod annas_archive;
pub mod base_search;
pub mod digbt;

// Batch 10
pub mod geizhals;
pub mod grokipedia;
pub mod il_post;
pub mod loc;
pub mod metacpan;

pub mod registry;

pub use registry::EngineRegistry;
