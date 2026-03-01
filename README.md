# 🔍 Metasearch

Awesome! Now please look at the integration searchXNG folder and in there you can find all 219 search engine providers in Python. We already have most of the search engines but still need some more to compete against the open source project in Rust. Please create all the other search engine providers in our Rust, learning from the actual code of the open source search engine Python, and convert it in our Rust code and support all the providers that the open source project supports. And make sure we don't have any clippy warnings and errors!!!

**A blazing-fast, privacy-respecting metasearch engine written in Rust** — inspired by [SearXNG](https://github.com/searxng/searxng).

> No tracking. No profiling. No ads. Just results.

## ✨ Features

- 🦀 **Built in Rust** — memory-safe, fearlessly concurrent, blazing fast
- 🔒 **Privacy-first** — no user tracking, no profiling, no cookies
- 🔌 **74 search engines** — web, images, news, videos, science, code, music, and more
- 📊 **Smart ranking** — result aggregation, deduplication, and weighted scoring
- 🗂️ **Categories** — General, Images, News, Videos, Science, Files, Music
- ⚡ **Async everywhere** — Tokio + Axum for maximum throughput
- 🧩 **JSON API** — programmatic access at `/api/v1/search`
- 🐳 **Docker-ready** — single command deployment
- 🛡️ **Rate limiting & bot detection** — protect your instance
- 💾 **Result caching** — Moka-powered in-memory cache

## 📁 Project Structure

```
metasearch/
├── Cargo.toml                    # Workspace root
├── crates/
│   ├── metasearch-core/          # Core types, traits, error handling
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── config.rs         # Application settings
│   │       ├── engine.rs         # SearchEngine trait + EngineMetadata
│   │       ├── error.rs          # Unified errors (thiserror)
│   │       ├── query.rs          # SearchQuery model
│   │       ├── result.rs         # SearchResult + SearchResponse
│   │       ├── ranking.rs        # Aggregation & deduplication
│   │       └── category.rs       # SearchCategory enum
│   │
│   ├── metasearch-engine/        # 74 engine implementations
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── registry.rs       # EngineRegistry::with_defaults()
│   │       ├── bing.rs           # Bing web search
│   │       ├── duckduckgo.rs     # DuckDuckGo (HTML scraping)
│   │       ├── wikipedia.rs      # Wikipedia OpenSearch API
│   │       ├── youtube.rs        # YouTube (JSON scraping)
│   │       ├── arxiv.rs          # arXiv papers
│   │       ├── github_engine.rs  # GitHub repositories
│   │       ├── ...               # 68 more engines
│   │   └── tests/
│   │       ├── engine_registry.rs  # Unit tests — no network
│   │       └── engine_probe.rs     # Integration tests — live network (#[ignore])
│   │
│   ├── metasearch-server/        # Axum web server
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── app.rs            # Router & startup
│   │       ├── state.rs          # Shared AppState
│   │       ├── cache.rs          # Moka result cache
│   │       ├── middleware/
│   │       │   ├── rate_limit.rs  # Governor rate limiter
│   │       │   └── bot_detection.rs
│   │       └── routes/
│   │           ├── search.rs     # Web UI routes
│   │           ├── api.rs        # JSON API (/api/v1/*)
│   │           └── health.rs     # Health check
│   │
│   └── metasearch-cli/           # CLI entry point
│       └── src/
│           └── main.rs           # clap CLI (serve, engines, config)
│
├── config/
│   └── default.toml              # Default configuration
├── templates/                    # Tera HTML templates
│   ├── index.html
│   └── results.html
├── static/
│   └── css/style.css             # Dark-theme UI
├── ENGINES.md                    # Full engine catalog with status
├── .github/workflows/ci.yml      # CI: fmt, clippy, build, test, probe
├── Dockerfile                    # Multi-stage Docker build
└── docker-compose.yml
```

## 🚀 Quick Start

### From source

```bash
git clone https://github.com/najmus-sakib-hossain/metasearch.git
cd metasearch
cargo run -- serve
# → 🔍 Metasearch listening on http://0.0.0.0:8888
```

### With Docker

```bash
docker compose up -d
# → http://localhost:8888
```

### CLI

```bash
# Start the server
cargo run -- serve --port 9090

# List engines
cargo run -- engines

# Print config
cargo run -- config
```

## 🔌 API

```bash
# Search
curl "http://localhost:8888/api/v1/search?q=rust+programming"

# List engines
curl "http://localhost:8888/api/v1/engines"

# Health check
curl "http://localhost:8888/health"
```

## 🧪 Testing

```bash
# Unit tests (no network)
cargo test --workspace

# Engine registry smoke tests (validates all 74 engines register correctly)
cargo test --package metasearch-engine -- --nocapture

# Live engine probe (hits real external APIs — requires network)
cargo test --package metasearch-engine --test engine_probe -- --ignored --nocapture
```

The engine probe queries every engine with a real search term and asserts that at
least 30 % return results. Engines that are blocked in CI environments (bot detection,
Cloudflare, geo-fencing) are expected failures. See [ENGINES.md](ENGINES.md) for the
full status of each engine.

## 🧰 Tech Stack

| Crate | Version | Purpose |
|-------|---------|----------|
| `axum` | 0.8 | Web framework |
| `tokio` | 1.48 | Async runtime |
| `reqwest` | 0.12 | HTTP client (rustls, gzip, brotli) |
| `serde` | 1.0 | Serialization |
| `scraper` | 0.22 | HTML parsing (CSS selectors) |
| `regex` | 1.11 | Text extraction |
| `tera` | 1.20 | HTML templates |
| `moka` | 0.12 | In-memory result cache |
| `governor` | 0.8 | Rate limiting |
| `tracing` | 0.1 | Structured logging |
| `clap` | 4.5 | CLI framework |
| `thiserror` | 2.0 | Error types |
| `urlencoding` | 2.1 | URL encoding |

## 🔧 Configuration

Copy `config/default.toml` and set optional API keys:

```toml
[freesound]
api_key = "your_freesound_key"

[spotify]
api_client_id = "your_client_id"
api_client_secret = "your_client_secret"
```

## 📜 License

AGPL-3.0 — Same as SearXNG. Free as in freedom.

## 🙏 Acknowledgements

Inspired by [SearXNG](https://github.com/searxng/searxng) — the OG privacy-respecting metasearch engine.
