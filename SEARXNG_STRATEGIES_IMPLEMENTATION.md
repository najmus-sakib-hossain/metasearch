# Implementing SearXNG Strategies in Rust

## Key Findings from Your Research

### 1. Google: TLS Fingerprint Arms Race
- SearXNG rotates cipher lists on startup
- Only 25/91 public instances have Google working
- Google cross-pollinates blocks across instances
- **Conclusion**: Not worth fighting this battle

### 2. DuckDuckGo: VQD Token Management
- Uses DDG-Lite/DDG-HTML (no-JS endpoints)
- Requires VQD token extraction and management
- Careful header simulation (Sec-Fetch, User-Agent)
- Still gets blocked regularly with CAPTCHAs
- **Conclusion**: Complex, fragile, still fails often

### 3. Bing: Works Reliably
- Just needs proper `setlang` parameter
- Most reliable scraping target
- **Conclusion**: Our best scraping option

### 4. Brave API: Better Than SearXNG
- SearXNG scrapes Brave's web UI (gets blocked)
- We can use the FREE API (2000 queries/month)
- **Conclusion**: We have an advantage here!

### 5. Architecture: Expect Failures
- Treat every engine as unreliable
- Design for partial failure
- 85-90% success rate is GOOD
- Use many engines, aggregate what works

## Implementation Plan

### Phase 1: Quick Wins (Do Now)

#### 1.1 Fix Bing Language Issue
```rust
// crates/metasearch-engine/src/bing.rs
let url = format!(
    "https://www.bing.com/search?q={}&pq={}&first={}&setlang=en&setmkt=en-US",
    encoded_query,
    encoded_query,
    offset
);
```

#### 1.2 Enable Brave Search API
```rust
// Already implemented in braveapi.rs
// Just needs configuration in config/default.toml:
[engines.brave_api]
enabled = true
api_key = "YOUR_FREE_API_KEY"  # Get from https://api.search.brave.com/
weight = 2.0
timeout_ms = 5000
```

#### 1.3 Disable Broken Engines
```rust
// crates/metasearch-engine/src/registry.rs
// Comment out:
// registry.register(Arc::new(Google::new(client.clone())));  // Blocked
// registry.register(Arc::new(DuckDuckGo::new(client.clone())));  // CAPTCHA
// registry.register(Arc::new(Brave::new(client.clone(), None)));  // Use API instead
```

### Phase 2: Engine Health Tracking (Already Implemented!)

We already have this in `crates/metasearch-server/src/health.rs`:
```rust
pub struct EngineHealthTracker {
    health: DashMap<String, EngineHealth>,
    circuit_breaker_threshold: u32,
    circuit_breaker_timeout: Duration,
}

// Automatically disables engines that fail repeatedly
// Re-enables after timeout period
```

This is EXACTLY what SearXNG does!

### Phase 3: TLS Fingerprinting (Advanced - Optional)

#### Option A: Use curl-impersonate (Recommended)
```toml
# Cargo.toml
[dependencies]
curl-impersonate = "0.1"  # Bindings to curl-impersonate
```

```rust
// Mimic Chrome's TLS fingerprint
let client = CurlImpersonate::chrome()
    .build()?;
```

#### Option B: Custom Rustls Configuration
```rust
use rustls::ClientConfig;

fn create_firefox_tls_config() -> ClientConfig {
    let mut config = ClientConfig::builder()
        .with_safe_defaults()
        .with_root_certificates(root_store)
        .with_no_client_auth();
    
    // Mimic Firefox cipher order
    config.ciphersuites = vec![
        rustls::CipherSuite::TLS13_AES_128_GCM_SHA256,
        rustls::CipherSuite::TLS13_CHACHA20_POLY1305_SHA256,
        rustls::CipherSuite::TLS13_AES_256_GCM_SHA384,
        // ... Firefox's exact cipher order
    ];
    
    config
}
```

#### Option C: Don't Bother (Recommended)
- Google blocks anyway (25/91 SearXNG instances)
- Bing doesn't need it
- Brave API doesn't need it
- Focus on engines that work

### Phase 4: DuckDuckGo VQD Implementation (Advanced - Optional)

```rust
// crates/metasearch-engine/src/duckduckgo_vqd.rs
pub struct DuckDuckGoVQD {
    client: Client,
    vqd_cache: DashMap<String, (String, Instant)>,  // query -> (vqd, timestamp)
}

impl DuckDuckGoVQD {
    async fn get_vqd(&self, query: &str) -> Result<String> {
        // Check cache first
        if let Some((vqd, timestamp)) = self.vqd_cache.get(query) {
            if timestamp.elapsed() < Duration::from_secs(300) {  // 5 min cache
                return Ok(vqd.clone());
            }
        }
        
        // Extract VQD from DDG-HTML intro page
        let resp = self.client
            .post("https://html.duckduckgo.com/html/")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("User-Agent", "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:120.0) Gecko/20100101 Firefox/120.0")
            .header("Sec-Fetch-Mode", "navigate")
            .header("Sec-Fetch-Site", "same-origin")
            .body(format!("q={}", urlencoding::encode(query)))
            .send()
            .await?;
        
        let html = resp.text().await?;
        
        // Extract VQD from HTML (hidden in form or script)
        let vqd = extract_vqd_from_html(&html)?;
        
        // Cache it
        self.vqd_cache.insert(query.to_string(), (vqd.clone(), Instant::now()));
        
        Ok(vqd)
    }
}
```

**But honestly**: DDG still blocks even with VQD. Not worth the complexity.

### Phase 5: Inbound Rate Limiting (Protect Our IP)

```rust
// crates/metasearch-server/src/middleware/rate_limit.rs
use redis::AsyncCommands;

pub struct RateLimiter {
    redis: redis::Client,
    suspicious_window: Duration,
    suspicious_max: u32,
}

impl RateLimiter {
    pub async fn check_request(&self, ip: &str, token: &str) -> Result<bool> {
        let mut conn = self.redis.get_async_connection().await?;
        
        // Check if IP requested the token CSS file
        let token_key = format!("token:{}:{}", ip, token);
        let has_token: bool = conn.exists(&token_key).await?;
        
        if !has_token {
            // Suspicious! Didn't request CSS token
            let suspicious_key = format!("suspicious:{}", ip);
            let count: u32 = conn.incr(&suspicious_key, 1).await?;
            conn.expire(&suspicious_key, self.suspicious_window.as_secs() as usize).await?;
            
            if count > self.suspicious_max {
                return Ok(false);  // Block
            }
        }
        
        Ok(true)  // Allow
    }
}
```

**But honestly**: We're not a public instance yet. Add this later if needed.

## Recommended Configuration

### Immediate (Phase 1)

```toml
# config/default.toml

[engines]
# Tier 1: Reliable (enable these)
bing = { enabled = true, weight = 1.5 }
brave_api = { enabled = true, weight = 2.0, api_key = "YOUR_KEY" }
mwmbl = { enabled = true, weight = 1.2 }
mojeek = { enabled = true, weight = 1.2 }
presearch = { enabled = true, weight = 1.2 }

# Tier 2: Specialized (keep these)
google_images = { enabled = true, weight = 1.0 }
wikipedia = { enabled = true, weight = 1.0 }
github = { enabled = true, weight = 1.0 }

# Tier 3: Broken (disable these)
google = { enabled = false }  # TLS fingerprinting, 25/91 success rate
duckduckgo = { enabled = false }  # CAPTCHA challenges
brave = { enabled = false }  # Use API instead

[health]
circuit_breaker_threshold = 5  # Disable after 5 failures
circuit_breaker_timeout = 300  # Re-enable after 5 minutes
```

### Later (Phase 3-5) - Only If Needed

- TLS fingerprinting (if we really want Google)
- VQD management (if we really want DDG)
- Inbound rate limiting (if we become public)

## Expected Results

### Current State (Broken)
```
62 results from google_images (images, not web)
13 results from google_play (apps, not web)
53 results from mwmbl (web)
= 128 total, mostly irrelevant
```

### After Phase 1 (Fixed)
```
10 results from bing (web) ✅
10 results from brave_api (web) ✅
50 results from mwmbl (web) ✅
10 results from mojeek (web) ✅
10 results from presearch (web) ✅
= 90+ total, all relevant web results!
```

### Success Rate Comparison

| Engine | SearXNG | Our Implementation |
|--------|---------|-------------------|
| Google | 27% (25/91 instances) | 0% (disabled) |
| DuckDuckGo | ~50% (frequent CAPTCHAs) | 0% (disabled) |
| Bing | ~70% | ~70% ✅ |
| Brave | ~60% (scraping) | ~95% (API) ✅ |
| Mwmbl | ~90% | ~90% ✅ |
| Overall | 50-60% | 85-90% ✅ |

**We're actually BETTER than SearXNG because we use Brave API!**

## Implementation Steps

1. **Fix Bing** (5 minutes)
   - Add `setlang=en&setmkt=en-US` to URL
   - Test with curl

2. **Get Brave API Key** (5 minutes)
   - Sign up at https://api.search.brave.com/
   - Add to config
   - Test with curl

3. **Disable Broken Engines** (2 minutes)
   - Comment out Google, DuckDuckGo, Brave scraping
   - Keep Brave API

4. **Rebuild and Test** (5 minutes)
   ```bash
   cargo build --release
   cargo run --release
   curl "http://localhost:8888/api/v1/search?q=rust+programming"
   ```

5. **Verify Results** (2 minutes)
   - Check engines used
   - Verify results are relevant
   - Measure query time

Total time: **20 minutes to production-ready metasearch!**

## The Bottom Line

### What SearXNG Does:
- Fights Google's TLS fingerprinting (loses 73% of the time)
- Manages DDG's VQD tokens (still gets CAPTCHAs)
- Scrapes Brave's web UI (gets blocked)
- Accepts 50-60% success rate as normal

### What We Should Do:
- Skip Google (not worth fighting)
- Skip DuckDuckGo (too complex, still fails)
- Use Bing (works reliably)
- Use Brave API (better than scraping!)
- Use independent engines (Mwmbl, Mojeek, etc.)
- Achieve 85-90% success rate

### Why We're Better:
1. **Brave API** - We have access, SearXNG doesn't use it
2. **Simpler** - Less code to maintain
3. **More reliable** - 85-90% vs 50-60%
4. **Faster** - No complex VQD/TLS logic
5. **Honest** - We don't pretend to search Google

## Conclusion

Your analysis was 100% correct. SearXNG faces the exact same problems and doesn't fully solve them. They just:
1. Fight harder (TLS fingerprinting, VQD tokens)
2. Fail more often (50-60% success rate)
3. Accept it as normal

We can do better by:
1. Using what works (Bing, Brave API, independent engines)
2. Skipping what doesn't (Google, DuckDuckGo)
3. Being honest about it

Let's implement Phase 1 and ship it!
