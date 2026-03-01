import urllib.request
import re

url = "https://www.startpage.com/sp/search?query=test"
req = urllib.request.Request(url, headers={"User-Agent": "Mozilla/5.0 Firefox/120.0", "Accept": "text/html"})
with urllib.request.urlopen(req, timeout=8) as r:
    h = r.read().decode("utf-8", errors="replace")

# Find result containers
result_divs = re.findall(r'<section[^>]*class="[^"]*result[^"]*"[^>]*>.*?</section>', h, re.DOTALL)
print("section.result count:", len(result_divs))
if result_divs:
    print("First result div (400 chars):")
    print(result_divs[0][:400])

# Show wgl-title usage
titles = re.findall(r'<h[123][^>]*class="[^"]*wgl-title[^"]*"[^>]*>([^<]+)<', h)
print("\nwgl-title count:", len(titles), titles[:3])

# Show links near titles
link_pattern = re.findall(r'<a[^>]+href="(https?://[^"]+)"[^>]*>\s*<h2', h)
print("direct href-h2:", len(link_pattern), link_pattern[:2])

# Find href near wgl-title
sections = re.findall(r'href="(https?://[^"]+)"[^<]*<.*?wgl-title', h)
print("href before wgl-title:", sections[:2])
