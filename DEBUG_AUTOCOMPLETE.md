# 🔍 Debugging Autocomplete Issues

## Changes Made

1. ✅ **Fixed server log** - Now shows `http://localhost:8888` instead of `http://0.0.0.0:8888`
2. ✅ **Added console logging** - JavaScript now logs all autocomplete activity to browser console
3. ✅ **Fixed compilation error** - Moved state usage before build_router call

## How to Debug Autocomplete Not Showing

### Step 1: Build and Run Server

```bash
cargo build --release
cargo run --release
```

You should see:
```
Metasearch server listening on http://localhost:8888
```

### Step 2: Test the Endpoint Directly

Open a new terminal and test:

```bash
curl "http://localhost:8888/autocomplete?q=rust"
```

**Expected output:**
```json
["rust",["rust programming","rust game","rust tutorial","rust vs c++","rust language"]]
```

**If this fails:**
- Server is not running
- Port 8888 is blocked
- Network issue

### Step 3: Test in Browser Console

1. Visit `http://localhost:8888`
2. Press `F12` to open DevTools
3. Go to **Console** tab
4. Type in the search box

**You should see logs like:**
```
Autocomplete initialized: {input: true, list: true, form: true}
Autocomplete event listeners attached
Input event: r
Input event: ru
Query too short, clearing dropdown
Input event: rus
Fetching suggestions for: rus
Autocomplete response status: 200
Autocomplete data received: ["rus", ["rust programming", ...]]
Rendering suggestions: {query: "rus", count: 5, suggestions: Array(5)}
Dropdown shown with 5 items
```

### Step 4: Check Network Tab

1. Press `F12` to open DevTools
2. Go to **Network** tab
3. Type in the search box
4. Look for `/autocomplete?q=...` requests

**Check:**
- Status should be `200 OK`
- Response should be JSON array
- No CORS errors

### Step 5: Inspect the Dropdown Element

1. Press `F12` to open DevTools
2. Go to **Elements** tab
3. Find `<ul id="autocomplete-list">`
4. Type in search box

**Check if:**
- `hidden` attribute is removed when typing
- `<li>` elements are being added
- CSS styles are applied correctly

### Step 6: Use Test Page

Open `test_autocomplete_simple.html` in your browser (with server running):

```bash
# On Windows
start test_autocomplete_simple.html

# Or just open it in your browser
```

This standalone test page will show:
- ✅ If endpoint is working
- ✅ What data is being returned
- ❌ Any errors clearly

## Common Issues and Fixes

### Issue 1: Dropdown Not Visible (CSS Issue)

**Symptom:** Console shows suggestions rendered, but nothing visible

**Check:**
```javascript
// In browser console
document.getElementById('autocomplete-list').hidden
// Should be false when suggestions are shown

document.getElementById('autocomplete-list').children.length
// Should be > 0 when suggestions are shown
```

**Fix:** Check if CSS is hiding the dropdown. Inspect element and look for:
- `display: none`
- `visibility: hidden`
- `opacity: 0`
- `z-index` too low

### Issue 2: CORS Error

**Symptom:** Console shows CORS error

**Fix:** The server already has `CorsLayer::permissive()` in `app.rs`, so this shouldn't happen. If it does, check if you're accessing from a different domain.

### Issue 3: Endpoint Returns Empty Array

**Symptom:** `["query", []]` with no suggestions

**Possible causes:**
1. Google's API is rate limiting
2. Network issue reaching Google
3. Query is blocked/filtered

**Test with curl:**
```bash
curl "http://localhost:8888/autocomplete?q=test"
```

### Issue 4: JavaScript Not Running

**Symptom:** No console logs at all

**Check:**
1. View page source - is the `<script>` tag there?
2. Browser console - any JavaScript errors?
3. Is Lucide script blocking? (shouldn't be, it's deferred)

### Issue 5: Dropdown Appears Then Disappears

**Symptom:** Flash of dropdown then gone

**Cause:** `blur` event firing too quickly

**Fix:** Already implemented - 150ms delay in blur handler

### Issue 6: Suggestions for Wrong Query

**Symptom:** Typing "rust" shows suggestions for "r"

**Cause:** Debounce timing issue

**Fix:** Already implemented - 180ms debounce

## Manual Testing Checklist

- [ ] Server starts without errors
- [ ] Log shows `http://localhost:8888` (not 0.0.0.0)
- [ ] `curl "http://localhost:8888/autocomplete?q=test"` returns JSON
- [ ] Homepage loads at http://localhost:8888
- [ ] Search box is visible and clickable
- [ ] Browser console shows "Autocomplete initialized"
- [ ] Typing shows "Input event" logs
- [ ] After 2+ characters, shows "Fetching suggestions"
- [ ] Network tab shows `/autocomplete` request with 200 status
- [ ] Console shows "Rendering suggestions"
- [ ] Console shows "Dropdown shown with X items"
- [ ] Dropdown is visible below search box
- [ ] Suggestions have search icons
- [ ] Hovering highlights suggestions
- [ ] Arrow keys navigate suggestions
- [ ] Enter key selects suggestion
- [ ] Clicking suggestion works

## Advanced Debugging

### Enable Verbose Logging

The JavaScript now has console.log statements. To see them:
1. Open DevTools (F12)
2. Go to Console tab
3. Type in search box
4. Watch the logs

### Check CSS Specificity

If dropdown exists but isn't visible:

```javascript
// In browser console
const list = document.getElementById('autocomplete-list');
console.log('Hidden:', list.hidden);
console.log('Display:', getComputedStyle(list).display);
console.log('Visibility:', getComputedStyle(list).visibility);
console.log('Opacity:', getComputedStyle(list).opacity);
console.log('Z-index:', getComputedStyle(list).zIndex);
console.log('Position:', getComputedStyle(list).position);
console.log('Top:', getComputedStyle(list).top);
```

### Force Show Dropdown

```javascript
// In browser console
const list = document.getElementById('autocomplete-list');
list.hidden = false;
list.innerHTML = '<li>Test suggestion</li>';
```

If this shows the dropdown, the issue is with the JavaScript logic, not CSS.

### Check Parent Container

```javascript
// In browser console
const wrap = document.getElementById('search-input-wrap');
console.log('Position:', getComputedStyle(wrap).position);
// Should be 'relative' for absolute positioning to work
```

## Quick Fixes

### Fix 1: Force Dropdown to Show (Testing)

Add this to browser console:
```javascript
document.getElementById('autocomplete-list').style.display = 'block';
document.getElementById('autocomplete-list').style.position = 'absolute';
document.getElementById('autocomplete-list').style.zIndex = '9999';
document.getElementById('autocomplete-list').hidden = false;
```

### Fix 2: Test Without Debounce

In browser console:
```javascript
fetch('/autocomplete?q=rust')
  .then(r => r.json())
  .then(data => console.log('Direct fetch:', data));
```

### Fix 3: Manually Render Suggestions

In browser console:
```javascript
const list = document.getElementById('autocomplete-list');
list.innerHTML = '<li style="padding:10px;cursor:pointer;">🔍 Test Suggestion</li>';
list.hidden = false;
```

## Expected Console Output

When working correctly, you should see:

```
Autocomplete initialized: {input: true, list: true, form: true}
Autocomplete event listeners attached
Input event: r
Query too short, clearing dropdown
Input event: ru
Query too short, clearing dropdown
Input event: rus
Fetching suggestions for: rus
Autocomplete response status: 200
Autocomplete data received: (2) ['rus', Array(5)]
Rendering suggestions: {query: 'rus', count: 5, suggestions: Array(5)}
Dropdown shown with 5 items
```

## Still Not Working?

1. **Check `test_autocomplete_simple.html`** - This isolated test will show if the endpoint works
2. **Check browser console** - Look for any errors
3. **Check network tab** - Verify requests are being made
4. **Check server logs** - Look for errors in terminal
5. **Try different browser** - Rule out browser-specific issues
6. **Disable browser extensions** - Ad blockers might interfere
7. **Clear browser cache** - Old CSS/JS might be cached

## Success Indicators

✅ Server log shows `http://localhost:8888`
✅ Endpoint returns JSON: `["query", ["suggestion1", ...]]`
✅ Console shows "Autocomplete initialized"
✅ Console shows "Dropdown shown with X items"
✅ Dropdown is visible below search box
✅ Suggestions are clickable and work

## Contact/Report

If still not working after all these steps, check:
1. Browser version (should be modern Chrome/Firefox/Edge)
2. Operating system
3. Any security software that might block requests
4. Firewall settings
5. Antivirus software

The autocomplete feature is fully implemented - if it's not showing, it's likely a CSS visibility issue or the endpoint isn't returning data.
