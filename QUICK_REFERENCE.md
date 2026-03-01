# 🚀 Quick Reference Card

## Start Server
```bash
cargo run --release
```
**Output:** `Metasearch server listening on http://localhost:8888`

## Test Autocomplete Endpoint
```bash
curl "http://localhost:8888/autocomplete?q=rust"
```
**Output:** `["rust",["rust programming","rust game",...]]`

## Test in Browser
1. Visit: `http://localhost:8888`
2. Press `F12` (DevTools)
3. Go to **Console** tab
4. Type in search box (2+ characters)
5. Watch console logs

## Expected Console Logs
```
Autocomplete initialized: {input: true, list: true, form: true}
Autocomplete event listeners attached
Input event: rus
Fetching suggestions for: rus
Autocomplete response status: 200
Autocomplete data received: ["rus", Array(5)]
Rendering suggestions: {query: "rus", count: 5, ...}
Dropdown shown with 5 items
```

## Debug Dropdown Not Showing

### Check 1: Endpoint Working?
```bash
curl "http://localhost:8888/autocomplete?q=test"
```

### Check 2: Console Logs?
Press `F12` → Console → Type in search box

### Check 3: Network Requests?
Press `F12` → Network → Type in search box → Look for `/autocomplete`

### Check 4: Element Exists?
Press `F12` → Console → Run:
```javascript
document.getElementById('autocomplete-list')
```

### Check 5: Force Show (Test)
Press `F12` → Console → Run:
```javascript
const list = document.getElementById('autocomplete-list');
list.innerHTML = '<li style="padding:10px;background:white;">Test</li>';
list.hidden = false;
list.style.display = 'block';
list.style.zIndex = '9999';
```

## Test Page
Open in browser: `test_autocomplete_simple.html`

## Files Changed
- ✅ `crates/metasearch-server/src/app.rs` - Server log fix
- ✅ `templates/index.html` - Console logging added
- ✅ `crates/metasearch-server/src/routes/opensearch.rs` - OpenSearch endpoint
- ✅ `config/default.toml` - Better timeouts

## Features
- ✅ 200+ search engines
- ✅ Google autocomplete on homepage
- ✅ OpenSearch browser integration
- ✅ Console debugging logs
- ✅ Server shows localhost:8888

## Documentation
- `FINAL_SUMMARY.md` - Complete summary
- `DEBUG_AUTOCOMPLETE.md` - Debugging guide
- `OPENSEARCH_SETUP.md` - Browser integration
- `QUICK_START.md` - Getting started

## Success = All These Work
- [ ] Server starts: `cargo run --release`
- [ ] Log shows: `http://localhost:8888`
- [ ] Endpoint works: `curl "http://localhost:8888/autocomplete?q=test"`
- [ ] Console shows: "Autocomplete initialized"
- [ ] Console shows: "Dropdown shown with X items"
- [ ] Dropdown visible (or check CSS)

## Quick Fixes
**Dropdown not visible?** Check CSS z-index and position
**No console logs?** Check browser console for errors
**Endpoint fails?** Check server is running
**Empty results?** Try different query

## Support
See `DEBUG_AUTOCOMPLETE.md` for comprehensive troubleshooting.
