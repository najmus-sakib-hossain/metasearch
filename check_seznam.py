import urllib.request
import re

url = "https://search.seznam.cz/?q=test"
req = urllib.request.Request(url, headers={
    "User-Agent": "Mozilla/5.0 Firefox/120.0",
    "Accept": "text/html",
    "Accept-Language": "cs,en;q=0.9"
})
with urllib.request.urlopen(req, timeout=10) as r:
    h = r.read().decode("utf-8", errors="replace")

print("Length:", len(h))
print("data-dot-data count:", h.count("data-dot-data"))
print("organic count:", h.count("organic"))

# Find h3 a tags
h3_links = re.findall(r'<h3[^>]*>\s*<a[^>]+href="([^"]+)"[^>]*>([^<]+)', h)
print("h3 links count:", len(h3_links), h3_links[:3])

# Look for the organic result structure
organic_idx = h.find("organic")
if organic_idx > 0:
    print("\nContext around organic:")
    print(h[max(0, organic_idx-100):organic_idx+400])
