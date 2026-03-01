import urllib.request
import urllib.parse
import re

# First try POST (same as engine does)
url = "https://www.startpage.com/sp/search"
data = urllib.parse.urlencode({"query": "test", "cat": "web", "page": "1"}).encode()
req = urllib.request.Request(url, data=data, headers={
    "User-Agent": "Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36",
    "Accept": "text/html,application/xhtml+xml",
    "Content-Type": "application/x-www-form-urlencoded"
})
with urllib.request.urlopen(req, timeout=8) as r:
    h = r.read().decode("utf-8", errors="replace")

print("Length:", len(h))

# Check for w-gl__result
old_sel = h.count("w-gl__result")
print("Old selector (w-gl__result):", old_sel)

# Check new selectors
new_sel = h.count("wgl-title")
print("New title (wgl-title):", new_sel)

# Find result links
links = re.findall(r'href="(https?://(?!www\.startpage\.com)[^"]+)"', h)
print("External links count:", len(links), links[:3])

# Check first 1000 chars of search results area
idx = h.find("wgl-title")
if idx > 0:
    print("\nContext around wgl-title:")
    print(h[max(0,idx-500):idx+300])
