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
//!
//! ## Batch 11 — More SearXNG translations (5):
//! - Duden, Gitea, LiveSpace, Material Icons, MediathekViewWeb
//!
//! ## Batch 12 — More SearXNG translations (9):
//! - iQiyi, Jisho, Lucide, Mwmbl, Nyaa, Odysee, SVG Repo, Wallhaven, Yep
//!
//! ## Batch 13 — More SearXNG translations (9):
//! - PeerTube, pkg.go.dev, Stract, Tagesschau, Void Linux,
//!   Rumble, Pinterest, Podcast Index, Photon
//!
//! ## Batch 14 — More SearXNG translations (5):
//! - Moviepilot, Open Library, SolidTorrents, Rotten Tomatoes, SepiaSearch
//!
//! ## Batch 15 — More SearXNG translations (4):
//! - Openverse, Tootfinder, Searchcode, Tokyo Toshokan
//!
//! ## Batch 16 — Wired orphans + new engines (6):
//! - Imgur, lib.rs, Kickass Torrents, DeviantArt,
//!   360 Search Videos, SourceHut
//!
//! ## Batch 17 — More SearXNG translations (5):
//! - Chinaso, Flickr (no API), Ahmia, Naver, Radio Browser
//!
//! ## Batch 18 — More SearXNG translations (2):
//! - The Pirate Bay, OpenAlex

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

// Batch 11
pub mod duden;
pub mod gitea;
pub mod livespace;
pub mod material_icons;
pub mod mediathekviewweb;

// Batch 12
pub mod iqiyi;
pub mod jisho;
pub mod lucide;
pub mod mwmbl;
pub mod nyaa;
pub mod odysee;
pub mod svgrepo;
pub mod wallhaven;
pub mod yep;

// Batch 13
pub mod peertube;
pub mod photon;
pub mod pinterest;
pub mod pkg_go_dev;
pub mod podcastindex;
pub mod rumble;
pub mod stract;
pub mod tagesschau;
pub mod voidlinux;

// Batch 14
pub mod moviepilot;
pub mod openlibrary;
pub mod rottentomatoes;
pub mod sepiasearch;
pub mod solidtorrents;

// Batch 15
pub mod openverse;
pub mod searchcode;
pub mod tokyotoshokan;
pub mod tootfinder;

// Batch 16 (wired orphans + new)
pub mod deviantart;
pub mod imgur;
pub mod kickass;
pub mod lib_rs;
pub mod sourcehut;
pub mod three_sixty_search_videos;

// Batch 17
pub mod ahmia;
pub mod chinaso;
pub mod flickr_noapi;
pub mod naver;
pub mod radio_browser;

// Batch 18
pub mod openalex;
pub mod piratebay;

pub mod registry;

pub use registry::EngineRegistry;
