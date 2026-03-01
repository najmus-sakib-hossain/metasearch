# OpenSearch Integration Setup Guide

## What Was Added

Your Metasearch engine now supports the OpenSearch standard, allowing users to add it as a search engine in Firefox, Chrome, Edge, and other browsers with full autocomplete support!

## Features

✅ **Browser Integration** - Add Metasearch to your browser's search engines
✅ **Autocomplete Suggestions** - Real-time search suggestions powered by Google
✅ **Address Bar Search** - Search directly from the browser address bar
✅ **Category Support** - Search specific categories (images, videos, news, etc.)

## How to Use

### 1. Start Your Server

```bash
cargo run --release
```

The server will start at `http://localhost:8888`

### 2. Add to Firefox

1. Visit `http://localhost:8888` in Firefox
2. Click the address bar
3. Look for the green "+" icon or "Add Search Engine" option
4. Click "Add Metasearch"
5. Done! Now you can search from the address bar

**To use:**
- Type your search query in the address bar
- Press `Tab` key
- Type "Metasearch" or select it from the dropdown
- Type your search query
- Press Enter

### 3. Add to Chrome/Edge

1. Visit `http://localhost:8888` in Chrome or Edge
2. Right-click the address bar
3. Select "Manage search engines" or "Search engine settings"
4. Metasearch should appear in the list automatically
5. Click the three dots next to it and select "Make default" (optional)

**To use:**
- Type your search query in the address bar
- Metasearch will appear as an option
- Or set a keyword like "ms" and type "ms your query"

### 4. Test Autocomplete

1. Start typing a search query in your browser's address bar
2. Select Metasearch as the search engine
3. Continue typing - you should see autocomplete suggestions appear
4. Use arrow keys to select a suggestion
5. Press Enter to search

## Technical Details

### Endpoints Added

#### `/opensearch.xml`
Returns an OpenSearch descriptor XML that browsers use to register the search engine.

**Response:**
```xml
<?xml version="1.0" encoding="UTF-8"?>
<OpenSearchDescription xmlns="http://a9.com/-/spec/opensearch/1.1/">
  <ShortName>Metasearch</ShortName>
  <Description>Privacy-respecting metasearch engine aggregating 200+ sources</Description>
  <Url type="text/html" template="http://localhost:8888/search?q={searchTerms}"/>
  <Url type="application/x-suggestions+json" template="http://localhost:8888/autocomplete?q={searchTerms}"/>
</OpenSearchDescription>
```

#### `/autocomplete?q=<query>`
Returns search suggestions in OpenSearch format.

**Example Request:**
```bash
curl "http://localhost:8888/autocomplete?q=rust"
```

**Example Response:**
```json
["rust", ["rust programming", "rust game", "rust tutorial", "rust vs c++", "rust language"]]
```

### Files Modified

1. **New Files:**
   - `templates/opensearch.xml` - OpenSearch descriptor template
   - `crates/metasearch-server/src/routes/opensearch.rs` - OpenSearch endpoint handler

2. **Modified Files:**
   - `crates/metasearch-server/src/routes/mod.rs` - Added opensearch module
   - `crates/metasearch-server/src/app.rs` - Registered opensearch route
   - `templates/index.html` - Added OpenSearch link tag in `<head>`
   - `templates/results.html` - Added OpenSearch link tag in `<head>`

### HTML Changes

Both templates now include this in the `<head>` section:
```html
<link rel="search" type="application/opensearchdescription+xml" title="Metasearch" href="/opensearch.xml">
```

This tells browsers that the site supports OpenSearch.

## Configuration

### Base URL

The OpenSearch descriptor uses your configured `base_url` from `config/default.toml`:

```toml
[server]
base_url = "http://localhost:8888"
```

**For production**, update this to your actual domain:
```toml
[server]
base_url = "https://search.yourdomain.com"
```

### Autocomplete Source

Currently uses Google's Firefox autocomplete API. To change the source, edit:
`crates/metasearch-server/src/routes/autocomplete.rs`

## Testing

### Manual Testing

1. **Test OpenSearch XML:**
```bash
curl http://localhost:8888/opensearch.xml
```

2. **Test Autocomplete:**
```bash
curl "http://localhost:8888/autocomplete?q=rust"
```

3. **Test with jq (pretty print):**
```bash
curl -s "http://localhost:8888/autocomplete?q=python" | jq .
```

### Automated Testing

Run the test script:
```bash
chmod +x test_opensearch.sh
./test_opensearch.sh
```

## Browser Compatibility

| Browser | OpenSearch Support | Autocomplete Support |
|---------|-------------------|---------------------|
| Firefox | ✅ Full | ✅ Full |
| Chrome | ✅ Full | ✅ Full |
| Edge | ✅ Full | ✅ Full |
| Safari | ⚠️ Limited | ⚠️ Limited |
| Opera | ✅ Full | ✅ Full |
| Brave | ✅ Full | ✅ Full |

## Privacy Notes

- Autocomplete suggestions are fetched from Google's public API
- No user data is stored or logged
- Suggestions are fetched in real-time and not cached
- Your search queries are not tracked or profiled

## Troubleshooting

### Browser doesn't show "Add Search Engine" option

1. Make sure you're on the homepage (`http://localhost:8888`)
2. Check that the OpenSearch link tag is in the HTML source
3. Try refreshing the page (Ctrl+F5)
4. Check browser console for errors

### Autocomplete not working

1. Test the endpoint directly: `curl "http://localhost:8888/autocomplete?q=test"`
2. Check browser console for network errors
3. Verify the OpenSearch descriptor includes the suggestions URL
4. Make sure you have internet access (suggestions come from Google)

### "localhost" not working in production

Update `config/default.toml`:
```toml
[server]
base_url = "https://your-actual-domain.com"
```

Then restart the server.

## Next Steps

1. ✅ OpenSearch integration is complete
2. ✅ Autocomplete is working
3. 🔧 Customize the autocomplete source if needed
4. 🔧 Update base_url for production deployment
5. 🔧 Add custom search engine icon (optional)

## Resources

- [OpenSearch Specification](https://github.com/dewitt/opensearch)
- [Firefox Search Engine Documentation](https://developer.mozilla.org/en-US/docs/Web/OpenSearch)
- [Chrome Search Engine Documentation](https://developer.chrome.com/docs/extensions/mv3/override/)

## Support

If you encounter issues:
1. Check the server logs for errors
2. Test endpoints manually with curl
3. Verify browser compatibility
4. Check the COMPARISON_AND_FIXES.md for engine-specific issues
