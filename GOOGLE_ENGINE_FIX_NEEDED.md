# CRITICAL: Google/Bing/DuckDuckGo Engines Not Working

## Problem

The metasearch engine is returning **ZERO results** from major search engines:
- Google: 0 results
- Bing: 0 results (likely)
- DuckDuckGo: 0 results

Instead, it's returning results from:
- google_images (62 results) - IMAGE search, not web search!
- google_play (13 results) - APP store, not web search!
- mwmbl (53 results) - Small indie engine

## Root Cause

Our Google scraper implementation is outdated and missing critical parameters that SearXNG uses:

### What We're Missing:

1. **`asearch=arc` parameter** - Required for async results
2. **`async=arc_id:srp_<random>_<page>,use_ac:true,_fmt:prog`** - Special async format
3. **Random arc_id generation** - Changes every hour to avoid detection
4. **Proper XPath selectors** - SearXNG uses `.//div[contains(@class, "MjjYud")]`
5. **CONSENT cookie** - `CONSENT=YES+` to bypass cookie consent

### SearXNG's Working Implementation:

```python
# Generate random arc_id every hour
_arcid_random = (''.join(random.choices(string.ascii_letters + string.digits + "_-", k=23)), int(time.time()))
arc_id = f"arc_id:srp_{_arcid_random[0]}_1{start:02}"
async_param = f"{arc_id},use_ac:true,_fmt:prog"

# URL format
url = f"https://www.google.com/search?q={query}&start={start}&hl={lang}&lr=lang_{lang}&ie=utf8&oe=utf8&filter=0&safe={safe}&asearch=arc&async={async_param}"

# Headers
headers = {
    'User-Agent': gen_gsa_useragent(),  # Special Google Search Appliance UA
    'Accept': '*/*'
}

# Cookies
cookies = {'CONSENT': 'YES+'}

# XPath selector
results = dom.xpath('.//div[contains(@class, "MjjYud")]')
```

## Why This Matters

**Real Google search results** (from web search tool):
```
1. manfromexistence/ui: Beautifully crafted components
   https://github.com/manfromexistence/ui
   
2. manfromexistence's gists · GitHub
   https://gist.github.com/manfromexistence
```

**Our metasearch results** (broken):
```
1. Weather for manfromexistence github: 28.2°C
2. manfromexistence · GitHub (460 x 460) [IMAGE]
3. 80+ Reusable Components [IMAGE]
```

The difference is night and day!

## Solution Required

We need to update `crates/metasearch-engine/src/google.rs` to match SearXNG's implementation:

### 1. Add arc_id Generation

```rust
use std::time::{SystemTime, UNIX_EPOCH};
use rand::{Rng, distributions::Alphanumeric};

static mut ARCID_CACHE: Option<(String, u64)> = None;

fn generate_arc_id(start: u32) -> String {
    unsafe {
        let now = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs();
        
        // Regenerate every hour
        if ARCID_CACHE.is_none() || (now - ARCID_CACHE.as_ref().unwrap().1) > 3600 {
            let random: String = rand::thread_rng()
                .sample_iter(&Alphanumeric)
                .take(23)
                .map(char::from)
                .collect();
            ARCID_CACHE = Some((random, now));
        }
        
        let random_id = &ARCID_CACHE.as_ref().unwrap().0;
        format!("arc_id:srp_{}_1{:02},use_ac:true,_fmt:prog", random_id, start)
    }
}
```

### 2. Update URL Construction

```rust
let async_param = generate_arc_id(start);
let url = format!(
    "https://www.google.com/search?q={}&start={}&hl={}&lr=lang_{}&ie=utf8&oe=utf8&filter=0&safe={}&asearch=arc&async={}",
    urlencoding::encode(&query.query),
    start,
    lang,
    lang,
    safe,
    urlencoding::encode(&async_param),
);
```

### 3. Add CONSENT Cookie

```rust
.header("Cookie", "CONSENT=YES+")
```

### 4. Use Correct XPath

```rust
let result_selector = Selector::parse("div.MjjYud").unwrap();
```

## Same Issue Affects Other Engines

- **Bing**: Likely has similar bot detection
- **DuckDuckGo**: May need special headers
- **Brave**: Scraping might be blocked

## Recommended Approach

1. **Study SearXNG implementations** in `integrations/searxng/searx/engines/`
2. **Port their working code** to Rust
3. **Test each engine individually** before enabling
4. **Consider using official APIs** where available:
   - Brave Search API (we have BraveApi engine)
   - Google Custom Search API (paid)
   - Bing Search API (paid)

## Alternative: Use API-Based Engines

Instead of scraping, use engines with official APIs:
- **Brave Search API** - Free tier available
- **Marginalia** - Independent search engine
- **Mwmbl** - Already working!
- **Presearch** - Decentralized search

## Testing Commands

```bash
# Test Google specifically
curl "http://localhost:8888/api/v1/search?q=rust+programming" | grep '"engine":"google"'

# Should return results, not empty!
```

## Priority

**CRITICAL** - This is why the search engine shows irrelevant results. Without working Google/Bing/DuckDuckGo, we're essentially a broken metasearch engine.

The relevance scoring fix I made earlier is useless if the major engines return 0 results!
