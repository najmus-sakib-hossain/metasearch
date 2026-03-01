import urllib.request
import json

url = "https://www.tagesschau.de/api2u/search/?searchText=test&pageSize=3&resultPage=all"
req = urllib.request.Request(url, headers={"User-Agent": "Mozilla/5.0 Firefox/120.0"})
with urllib.request.urlopen(req, timeout=8) as r:
    d = json.loads(r.read())
items = d.get("searchResults", [])
print("Count:", len(items))
for item in items:
    print("Title:", item.get("title"))
    for k, v in item.items():
        if "detail" in k.lower() or "url" in k.lower() or "link" in k.lower() or "web" in k.lower():
            print(f"  {k} = {str(v)[:80]}")
