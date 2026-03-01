# Metasearch vs Metasearch2 Comparison & Fixes

## Summary of Changes Made

### 1. ✅ OpenSearch Support Added (Google/Firefox Search Suggestions)

Your website now supports being added as a search engine in Firefox, Chrome, and other browsers!

**Files Created/Modified:**
- `templates/opensearch.xml` - OpenSearch descriptor template
- `crates/metasearch-server/src/routes/opensearch.rs` - OpenSearch endpoint
- `crates/metasearch-server/src/routes/mod.rs` - Added opensearch module
- `crates/metasearch-server/src/app.rs` - Registered opensearch route
- `templates/index.html` - Added OpenSearch link tag
- `templates/results.html` - Added OpenSearch link tag

**How to Use:**
1. Start your server: `cargo run --release`
2. Visit `http://localhost:8888` in Firefox or Chrome
3. Look for the "+" icon in the address bar or search bar
4. Click to add Metasearch as a search engine
5. Now you can search directly from your browser's address bar!

### 2. ✅ Autocomplete Already Working

Your implementation already has Google-powered autocomplete working! The endpoint `/autocomplete?q=<query>` returns suggestions in the OpenSearch format `[query, [suggestions]]`.

**Implementation:**
- `crates/metasearch-server/src/routes/autocomplete.rs` - Uses Google's Firefox autocomplete API
- Both `templates/index.html` and `templates/results.html` have autocomplete JavaScript already integrated

## Key Differences: Your Implementation vs Metasearch2

### Architecture Differences

| Feature | Your Implementation | Metasearch2 |
|---------|-------------------|-------------|
| **Language** | Rust (multi-crate workspace) | Rust (single crate) |
| **Web Framework** | Axum | Actix-web |
| **Engine Architecture** | Trait-based with async/await | Function-based with async |
| **Engine Count** | 200+ engines | ~8 core engines |
| **Configuration** | TOML with Settings struct | TOML with Config struct |
| **Parsing** | Manual scraper selectors | Helper parse functions |

### Why Brave and Others Work in Metasearch2 but May Fail in Yours

#### 1. **Timeout Settings**
```toml
# Your config (config/default.toml)
request_timeout_ms = 5000  # 5 seconds

# Metasearch2 has no global timeout, relies on reqwest defaults
```

**Fix:** Increase timeout for slow engines:
```toml
request_timeout_ms = 10000  # 10 seconds
```

#### 2. **User-Agent Headers**
Your Brave implementation uses:
```rust
"Mozilla/5.0 (X11; Linux x86_64; rv:121.0) Gecko/20100101 Firefox/121.0"
```

Metasearch2 uses the same, so this should be fine.

#### 3. **Concurrent Engine Limits**
```toml
# Your config
max_concurrent_engines = 10  # Only 10 engines run at once

# Metasearch2 runs all engines in parallel with no limit
```

**Fix:** Increase concurrent engines:
```toml
max_concurrent_engines = 50  # Run 50 engines in parallel
```

#### 4. **Engine Registration**
Your implementation registers ALL 200+ engines by default in `registry.rs`:
```rust
registry.register(Arc::new(Brave::new(client.clone(), None)));
```

Metasearch2 only has ~8 engines, so less chance of conflicts.

#### 5. **Error Handling**
Your engines return `Result<Vec<SearchResult>>` and errors stop that engine.
Metasearch2 uses `eyre::Result` and continues on errors.

### Specific Engine Issues

#### Brave Engine
**Your Implementation:**
- Selector: `#results > .snippet[data-pos]:not(.standalone)`
- Title: `.title`
- Description: `.generic-snippet, .video-snippet > .snippet-description`

**Metasearch2:**
- Same selectors ✅

**Potential Issues:**
1. Brave may be blocking requests (rate limiting)
2. Timeout too short (5 seconds)
3. Brave changed their HTML structure

**Debug Steps:**
```bash
# Test Brave engine specifically
cargo test --package metasearch-engine brave -- --nocapture

# Or run the debug test
cargo test --package metasearch-engine --test debug_single -- --nocapture
```

#### Other Engines
According to `test_broken_engines.rs`, these engines have issues:
- **Parse errors:** adobe_stock, baidu, bilibili, duckduckgo_extra, reddit, stract
- **Network errors:** 360search, 360search_videos, DictZone, livespace, repology, rumble, solidtorrents, wttr

## Recommended Fixes

### 1. Update Configuration
Edit `config/default.toml`:
```toml
[search]
safe_search = 1
default_language = "en"
max_page = 10
request_timeout_ms = 10000      # Increased from 5000
max_concurrent_engines = 50     # Increased from 10
```

### 2. Add Engine-Specific Timeouts
Modify `crates/metasearch-core/src/engine.rs` to allow per-engine timeouts:
```rust
pub struct EngineMetadata {
    pub name: Cow<'static, str>,
    pub display_name: Cow<'static, str>,
    pub homepage: Cow<'static, str>,
    pub categories: SmallVec<[SearchCategory; 4]>,
    pub enabled: bool,
    pub timeout_ms: u64,  // Already exists!
    pub weight: f64,
}
```

Then in your orchestrator, use the engine's timeout instead of global timeout.

### 3. Add Better Error Logging
In `crates/metasearch-server/src/orchestrator.rs`, add detailed error logging:
```rust
match engine.search(&query).await {
    Ok(results) => { /* ... */ },
    Err(e) => {
        error!(
            engine = engine_name,
            error = %e,
            "Engine search failed"
        );
    }
}
```

### 4. Test Individual Engines
Create a test file to debug specific engines:
```bash
# Test only Brave
cargo test --package metasearch-engine brave -- --nocapture

# Test all critical engines
cargo test --package metasearch-engine --test test_critical_engines -- --nocapture
```

### 5. Compare HTML Responses
If Brave still fails, save the HTML response and compare:
```rust
// In brave.rs, temporarily add:
std::fs::write("brave_response.html", &body)?;
```

Then compare with metasearch2's response to see if selectors need updating.

## Testing the OpenSearch Integration

### Firefox
1. Visit `http://localhost:8888`
2. Click the address bar
3. Look for "Add Metasearch" option at the bottom
4. Click to add
5. Type a search query in the address bar
6. Press Tab to search with Metasearch
7. Autocomplete suggestions should appear as you type

### Chrome
1. Visit `http://localhost:8888`
2. Right-click the address bar
3. Select "Manage search engines"
4. Metasearch should appear in the list
5. Set as default or use keyword to search
6. Autocomplete suggestions should appear as you type

## Next Steps

1. ✅ OpenSearch support is now added
2. ✅ Autocomplete is already working
3. 🔧 Update config timeouts and concurrent limits
4. 🔧 Test individual engines to identify failures
5. 🔧 Update selectors for broken engines
6. 🔧 Add better error logging

## Files Modified

1. `templates/opensearch.xml` - NEW
2. `crates/metasearch-server/src/routes/opensearch.rs` - NEW
3. `crates/metasearch-server/src/routes/mod.rs` - MODIFIED
4. `crates/metasearch-server/src/app.rs` - MODIFIED
5. `templates/index.html` - MODIFIED (added OpenSearch link)
6. `templates/results.html` - MODIFIED (added OpenSearch link)

## Conclusion

Your implementation is actually more advanced than metasearch2 with 200+ engines vs 8! The main issues are likely:
1. Timeout settings too aggressive (5s vs 10s+)
2. Concurrent engine limits too low (10 vs 50+)
3. Some engines may have changed their HTML structure
4. Rate limiting from search engines

The OpenSearch integration is now complete, allowing users to add your search engine to Firefox and Chrome with full autocomplete support!
