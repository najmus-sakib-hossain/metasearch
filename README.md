# 🔍 Metasearch

**A blazing-fast, privacy-respecting metasearch engine written in Rust** — inspired by [SearXNG](https://github.com/searxng/searxng).

> No tracking. No profiling. No ads. Just results.

## ✨ Features

- 🦀 **Built in Rust** — memory-safe, fearlessly concurrent, blazing fast
- 🔒 **Privacy-first** — no user tracking, no profiling, no cookies
- 🔌 **Pluggable engines** — Google, DuckDuckGo, Brave, Wikipedia, and more
- 📊 **Smart ranking** — result aggregation, deduplication, and weighted scoring
- 🗂️ **Categories** — General, Images, News, Videos, Science, Maps
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
│   │       ├── engine.rs         # SearchEngine trait
│   │       ├── error.rs          # Unified errors (thiserror)
│   │       ├── query.rs          # Search query model
│   │       ├── result.rs         # SearchResult + SearchResponse
│   │       ├── ranking.rs        # Aggregation & deduplication
│   │       └── category.rs       # Search categories
│   │
│   ├── metasearch-engine/        # Engine implementations
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── google.rs         # Google search
│   │       ├── duckduckgo.rs     # DuckDuckGo search
│   │       ├── brave.rs          # Brave Search (API)
│   │       ├── wikipedia.rs      # Wikipedia (fully implemented)
│   │       └── registry.rs       # Engine registry
│   │
│   ├── metasearch-server/        # Axum web server
│   │   └── src/
│   │       ├── lib.rs
│   │       ├── app.rs            # Router & startup
│   │       ├── state.rs          # Shared AppState
│   │       ├── cache.rs          # Moka result cache
│   │       ├── templates/        # Tera template management
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
├── .github/workflows/ci.yml      # CI: check, fmt, clippy, test
├── Dockerfile                    # Multi-stage Docker build
├── docker-compose.yml
└── README.md
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

## 🧰 Tech Stack

| Crate | Version | Purpose |
|-------|---------|----------|
| `axum` | 0.8 | Web framework |
| `tokio` | 1.48 | Async runtime |
| `reqwest` | 0.12 | HTTP client |
| `serde` | 1.0 | Serialization |
| `tera` | 1.20 | HTML templates |
| `scraper` | 0.22 | HTML parsing |
| `moka` | 0.12 | In-memory cache |
| `governor` | 0.8 | Rate limiting |
| `tracing` | 0.1 | Structured logging |
| `clap` | 4.5 | CLI framework |
| `thiserror` | 2.0 | Error types |

## 📜 License

AGPL-3.0 — Same as SearXNG. Free as in freedom.

## 🙏 Acknowledgements

Inspired by [SearXNG](https://github.com/searxng/searxng) — the OG privacy-respecting metasearch engine.
