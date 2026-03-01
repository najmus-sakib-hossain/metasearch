import urllib.request
import re

url = "https://www.startpage.com/sp/search?query=test"
req = urllib.request.Request(url, headers={
    "User-Agent": "Mozilla/5.0 Firefox/120.0",
    "Accept": "text/html"
})
with urllib.request.urlopen(req, timeout=10) as r:
    h = r.read().decode("utf-8", errors="replace")

print("Length:", len(h))
print("wgl-title count:", h.count("wgl-title"))
print("result class:", h.count('class="result'))

# Look for the structure around wgl-title
idx = h.find("wgl-title")
if idx > 0:
    print("\nContext (800 chars around wgl-title):")
    print(h[max(0, idx-400):idx+400])
else:
    # Look for w-gl__result or any result class
    idx2 = h.find("w-gl")
    if idx2 > 0:
        print("\nContext around w-gl:")
        print(h[max(0, idx2-200):idx2+400])
    # Show first 2000 chars
    print("\nFirst 1000 chars:")
    print(h[:1000])
