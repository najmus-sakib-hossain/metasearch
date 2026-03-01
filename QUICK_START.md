# Quick Start Guide

## What's Fixed
✅ Failed engines now show their actual names instead of "unknown"
✅ Yahoo engine is working great!
✅ UI properly displays engine statistics

## Running Your Metasearch

### Option 1: Direct Command
```bash
cargo run --release --bin metasearch -- serve --host 127.0.0.1 --port 8888
```

### Option 2: Using the Script
```bash
bash start-metasearch.sh
```

Then open your browser to: **http://localhost:8888**

## What You'll See

### In the Search Results:
1. **Top Info Bar**: Shows responding engines (like Yahoo)
2. **Sidebar - Engines Section**:
   - ✅ X responded (green checkmark)
   - ⚠️ X failed (yellow warning) - Click "Show failed engines" to see names
3. **Results**: All results from working engines like Yahoo

## About SearXNG (Separate Project)

If you want to run SearXNG alongside your metasearch:

### After Installing Docker:
```bash
docker-compose -f docker-compose.searxng.yml up -d
```

This will start SearXNG on: **http://localhost:8080**

You can run both:
- Your Metasearch: http://localhost:8888
- SearXNG: http://localhost:8080

## Engine Status
- **Working**: Yahoo, Bing, and many others
- **Failed engines**: Now show their actual names for easier debugging
- **Total engines**: 200+ available

## Next Steps
1. Install Docker Desktop for Windows (AMD64 version)
2. Start your metasearch server
3. Optionally start SearXNG for comparison
4. Enjoy fast, private search!
