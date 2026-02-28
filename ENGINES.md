# Metasearch Engines

A catalog of all **74 search engines** supported by Metasearch, organized by category.

## Status Legend

| Symbol | Meaning |
|--------|---------|
| ✅ | Returns results reliably |
| ⚠️ | Works with limitations (geo-restricted, rate-limited, query-sensitive) |
| 🔑 | Requires an API key in `config/default.toml` |
| 🚧 | Stub — implementation in progress |
| ❌ | Currently blocked (bot detection, Cloudflare, JS-only SPA) |

---

## General Web

| Engine | Homepage | Status | Notes |
|--------|----------|--------|-------|
| Ask | <https://www.ask.com> | ❌ | JavaScript-rendered SPA |
| Baidu | <https://www.baidu.com> | ✅ | Chinese web search |
| Bing | <https://www.bing.com> | ✅ | |
| Brave Search | <https://search.brave.com> | 🚧 | Stub — API key integration pending |
| DuckDuckGo | <https://duckduckgo.com> | ⚠️ | HTML scraping; may be rate-limited |
| Google | <https://www.google.com> | 🚧 | Stub — scraping blocked |
| Qwant | <https://www.qwant.com> | ❌ | Cloudflare challenge |
| Yahoo | <https://search.yahoo.com> | ✅ | |

## Images

| Engine | Homepage | Status | Notes |
|--------|----------|--------|-------|
| Adobe Stock | <https://stock.adobe.com> | ✅ | |
| ArtStation | <https://www.artstation.com> | ✅ | |
| Artic (Art Institute of Chicago) | <https://www.artic.edu> | ✅ | |
| Bing Images | <https://www.bing.com/images> | ✅ | |
| DevIcons | <https://devicon.dev> | ✅ | Tech logo icons |
| FindThatMeme | <https://findthatmeme.com> | ✅ | |
| Flickr | <https://www.flickr.com> | ✅ | |
| Frinkiac | <https://frinkiac.com> | ✅ | Simpsons screencaps |
| Ipernity | <https://www.ipernity.com> | ✅ | |
| Unsplash | <https://unsplash.com> | ✅ | |

## Videos

| Engine | Homepage | Status | Notes |
|--------|----------|--------|-------|
| ACFun | <https://www.acfun.cn> | ⚠️ | Chinese video; geo-restricted |
| Bilibili | <https://www.bilibili.com> | ❌ | Requires session cookies (412) |
| Bing Videos | <https://www.bing.com/videos> | ✅ | |
| BitChute | <https://www.bitchute.com> | ✅ | |
| CCC Media | <https://media.ccc.de> | ✅ | Chaos Computer Club talks |
| Dailymotion | <https://www.dailymotion.com> | ✅ | |
| INA | <https://www.ina.fr> | ❌ | Migrated to JS-rendered SPA |
| Vimeo | <https://vimeo.com> | ❌ | 403 Forbidden |
| YouTube | <https://www.youtube.com> | ✅ | |

## News

| Engine | Homepage | Status | Notes |
|--------|----------|--------|-------|
| ANSA | <https://www.ansa.it> | ⚠️ | Italian news agency |
| Bing News | <https://www.bing.com/news> | ✅ | |
| Hacker News | <https://news.ycombinator.com> | ✅ | |

## Music & Audio

| Engine | Homepage | Status | Notes |
|--------|----------|--------|-------|
| Bandcamp | <https://bandcamp.com> | ✅ | |
| Deezer | <https://www.deezer.com> | ⚠️ | IP rate-limited |
| Freesound | <https://freesound.org> | 🔑 | Requires `freesound.api_key` |
| Fyyd | <https://fyyd.de> | ❌ | 500 Internal Server Error |
| Genius | <https://genius.com> | ❌ | Cloudflare challenge |
| Mixcloud | <https://www.mixcloud.com> | ✅ | |
| SoundCloud | <https://soundcloud.com> | ✅ | |
| Spotify | <https://www.spotify.com> | 🔑 | Requires `spotify.client_id` + `client_secret` |

## Science & Academia

| Engine | Homepage | Status | Notes |
|--------|----------|--------|-------|
| arXiv | <https://arxiv.org> | ✅ | |
| BASE Search | <https://www.base-search.net> | ⚠️ | Access restricted by IP |
| CrossRef | <https://www.crossref.org> | ✅ | DOI metadata |
| Semantic Scholar | <https://www.semanticscholar.org> | ⚠️ | Rate-limited without API key |

## Technology & Code

| Engine | Homepage | Status | Notes |
|--------|----------|--------|-------|
| APKMirror | <https://www.apkmirror.com> | ⚠️ | Android app mirror |
| Apple App Store | <https://apps.apple.com> | ✅ | |
| CachyOS Packages | <https://packages.cachyos.org> | ⚠️ | Rolling release packages |
| Crates.io | <https://crates.io> | ✅ | Rust packages |
| Docker Hub | <https://hub.docker.com> | ✅ | |
| F-Droid | <https://f-droid.org> | ⚠️ | Android FOSS packages |
| GitHub | <https://github.com> | ✅ | |
| GitLab | <https://gitlab.com> | ✅ | |
| Hex | <https://hex.pm> | ✅ | Elixir/Erlang packages |
| HuggingFace | <https://huggingface.co> | ✅ | ML models & datasets |
| npm | <https://www.npmjs.com> | ✅ | JavaScript packages |
| PyPI | <https://pypi.org> | ❌ | Migrated to JS-rendered SPA |

## Q&A & Communities

| Engine | Homepage | Status | Notes |
|--------|----------|--------|-------|
| Lemmy | <https://lemmy.ml> | ✅ | Federated forum |
| Mastodon | <https://mastodon.social> | ✅ | Federated social |
| Reddit | <https://www.reddit.com> | ❌ | Bot detection (returns HTML) |
| Stack Exchange | <https://stackexchange.com> | ✅ | |

## Books & Files

| Engine | Homepage | Status | Notes |
|--------|----------|--------|-------|
| Anna's Archive | <https://annas-archive.org> | ❌ | Connection refused from most IPs |
| DigBT | <https://digbt.org> | ❌ | Site down |
| Goodreads | <https://www.goodreads.com> | ✅ | |

## Torrents

| Engine | Homepage | Status | Notes |
|--------|----------|--------|-------|
| BT4G | <https://bt4g.org> | ❌ | 403 Forbidden |
| BTDigg | <https://btdig.com> | ❌ | 429 Too Many Requests |
| KickAss Torrents | <https://kickasstorrents.to> | ⚠️ | |
| Leet-X (1337x) | <https://1337x.to> | ⚠️ | Selector may be outdated |

## Lifestyle & Shopping

| Engine | Homepage | Status | Notes |
|--------|----------|--------|-------|
| Chefkoch | <https://www.chefkoch.de> | ✅ | German recipes |
| eBay | <https://www.ebay.com> | ❌ | Bot detection |
| IMDB | <https://www.imdb.com> | ✅ | Films & TV |

## Reference & Culture

| Engine | Homepage | Status | Notes |
|--------|----------|--------|-------|
| Alpine Linux Packages | <https://pkgs.alpinelinux.org> | ✅ | Returns empty for generic queries |
| Arch Linux Wiki | <https://wiki.archlinux.org> | ⚠️ | Bot detection on some IPs |
| BPB | <https://www.bpb.de> | ✅ | German political education |
| Destatis | <https://www.destatis.de> | ✅ | German federal statistics |
| DeviantArt | <https://www.deviantart.com> | ✅ | |
| Emojipedia | <https://emojipedia.org> | ✅ | |
| NineGag | <https://9gag.com> | ✅ | |
| Wikipedia | <https://www.wikipedia.org> | ✅ | |

---

## Running the Engine Probe

To test all engines against live external APIs:

```bash
cargo test -p metasearch-engine --test engine_probe -- --ignored --nocapture
```

The test queries every engine with `"rust programming"` and asserts that at least
30 % return results. Engines blocked in CI environments (bot detection, geo-fencing,
Cloudflare) are expected failures and do not affect the pass threshold.

## Adding a New Engine

1. Create `crates/metasearch-engine/src/<name>.rs` implementing the `SearchEngine` trait.
2. Expose the module in `crates/metasearch-engine/src/lib.rs`.
3. Register it in `EngineRegistry::with_defaults()` inside `src/registry.rs`.
4. Add a row to this file.

                        (attribute.language == 'en' and infobox_id_lang is None) or attribute.language != 'en'
                        ),
                        attributes.append({"label": data_label, "value": data_value, "entity": "P625"})
                        content=(
                        content=f"Resource Group in Subscription: {data['subscriptionId']}",
                        coordinates = info
                        f" | extend matchscore = name startswith '{query}'"
                        f" | extend normalizedName = tolower(tostring(name))"
                        f" | project id,name,type,kind,subscriptionId,resourceGroup"
                        f" | sort by matchscore desc, normalizedName asc"
                        f" | take 30"
                        f" | where (name contains ('{query}'))"
                        f" | where (type =~ ('Microsoft.Resources/subscriptions/resourcegroups'))"
                        f"ResourceContainers"
                        infobox_id = url
                        infobox_id_lang = attribute.language
                        metadata.append(t.strip())
                        results.append({'title': infobox_title, 'url': url, "content": infobox_content})
                        result_content = "%s: %s" % (pod_title, content)
                        result_content = pod_title + ': ' + subpod['plaintext']
                        thumbnail = data_image_map.get(img_id[0])
                        title=data["name"],
                        url=(
                        value = '-' + t[1]
                        value = t[0]
                    "author": f"{item['user']['username']} ({item['user']['full_name']})",
                    "author": result.get("author"),
                    "commandName": "Microsoft.ResourceGraph",
                    "content": ", ".join(tags),
                    "content": content,
                    "content": item.get("content_major", ""),
                    "content": item["pkg_desc"],
                    "content": result.get("description", ""),
                    "content": sec_name,
                    "filesize": item.get("filesize"),
                    "iframe_src": resp.search_params['base_url'] + '/embed/' + videoid,
                    "img_format": "SVG",
                    "img_format": item.get("type"),
                    "img_src": fullsize_image,
                    "img_src": img_src,
                    "img_src": item.get("bigPicUrl"),
                    "img_src": item.get("picUrl", ""),
                    "img_src": item.get('originalUrl'),
                    "img_src": replace_url.get("ObjURL"),
                    "length": length,
                    "package_name": package_name,
                    "publishedDate": datetime.fromtimestamp(item["pkg_builddate"]),
                    "publishedDate": publishedDate,
                    "publishedDate": published_date,
                    "query": (
                    "query": f"Resources | where name contains '{query}' | take 30",
                    "resolution": f"{item.get('orgWidth')} x {item.get('orgHeight')}",
                    "resolution": f"{item['width']} x {item['height']}",
                    "resolution": f"{width} x {height}",
                    "source": item.get("ch_site_name", ""),
                    "source": item.get("fromURLHost"),
                    "source": item.get("site"),
                    "source": item.get('source'),
                    "tags": [arch],
                    "template": "images.html",
                    "template": "videos.html",
                    "template": 'images.html',
                    "template": 'packages.html',
                    "thumbnail": thumbnail,
                    "thumbnail_src": item.get("img"),
                    "thumbnail_src": item.get("picUrl", ""),
                    "thumbnail_src": item.get("thumbURL"),
                    "thumbnail_src": item.get('thumb'),
                    "thumbnail_src": thumb,
                    "title": f"{package_name} ({repo})",
                    "title": html_to_text(item.get("fromPageTitle")),
                    "title": html_to_text(item.get('title')),
                    "title": icon_name,
                    "title": item.get("title"),
                    "title": item.get("title", ""),
                    "title": item["title"],
                    "title": result.get("title", ""),
                    "title": title,
                    "url": f"https://packages.cachyos.org/package/{repo}/{arch}/{package_name}",
                    "url": img_src,
                    "url": item.get("imgUrl"),
                    "url": item.get("url", ""),
                    "url": item.get('link'),
                    "url": item["url"],
                    "url": replace_url.get("FromURL"),
                    "url": url,
                    "version": item["pkg_version"],
                    "views": humanize_number(result['viewCount']),
                    # "normal" results (not infobox) include official website and Wikipedia links.
                    # Append if it's a single result
                    # Date: 2016-02-21 21:44 UTC
                    # Extend if the result is a list
                    # first the local wikipedia URL, and as fallback the english wikipedia URL
                    # ignore google_play_artist_id: service shutdown
                    # ignore instance: Wikidata value from "Instance Of" (Qxxxx)
                    # ignore wiki_maps_trigger: reference to a javascript
                    # prefer mp4 (minimal data rates)
                    # pygments enumerates lines from 1, highlight the next line
                    # There is already an URL for the website
                    # update the infobox_id with the wikipedia URL
                    $sortBy: SearchProductExplorerSort) {
                    'author': f"{attrs['user']['username']}",
                    'author': pub_info,
                    'backlink': backlink_json.get("backlink"),
                    'city', address_raw.get('town', address_raw.get('village'))  # noqa
                    'content': '',
                    'content': api_result.get('extract', ''),
                    'content': attrs["description"],
                    'content': content or '',
                    'content': content,
                    'content': extract_text(eval_xpath(result, content_xpath)),
                    'content': f"Hashtag has been used {uses_count} times by {user_count} different users",
                    'content': f"[{result['albums'][0]['title']}] {result['artists'][0]['name']} - {result['title']}",
                    'content': html_to_text(item.get('description', '')),
                    'content': html_to_text(result['description']),
                    'content': item.get('description', ''),
                    'content': result.get('description') or '',
                    'content': result['description'],
                    'content': result['note'],
                    'country': properties.get('country'),
                    'country': result.get('country'),
                    'crawl_date': crawl_date,
                    'fields': [
                    'filesize': humanized_filesize,
                    'house_number': properties.get('housenumber'),
                    'house_number': result.get('subThoroughfare'),
                    'id': wikipedia_link,
                    'iframe_src': "https://embed.spotify.com/?uri=spotify:track:" + result['id'],
                    'iframe_src': embed_url,
                    'iframe_src': f'{url}/iframe/track/{track_id}/{album_id}',
                    'iframe_src': result['images'].get('image460sv', {}).get('url'),
                    'image': get_wikipedia_image(wd_result.get('image', {}).get('value')),
                    'image_name': backlink_json.get("image_name"),
                    'image_sign': get_wikipedia_image(wd_result.get('sign', {}).get('value')),
                    'image_symbol': get_wikipedia_image(wd_result.get('symbol', {}).get('value')),
                    'img_src': api_result.get('thumbnail', {}).get('source'),
                    'img_src': attrs["image"]["download_link"],
                    'img_src': clean_url(result['urls']['regular']),
                    'img_src': fullsize_image,
                    'img_src': item.get('image'),
                    'img_src': result['image'],
                    'img_src': result['images']['image700']['url'],
                    'infobox': title,
                    'itemLabel': wd_result['itemLabel']['value'],
                    'key': k,
                    'label': get_key_label('phone', user_language),
                    'label': get_key_label('website', user_language),
                    'label': k_label,
                    'length': duration,
                    'locality': properties.get(
                    'locality': result.get('locality'),
                    'metadata': ' / '.join(metadata),
                    'metadata': metadata,
                    'name': result['name'],
                    'postcode': properties.get('postcode'),
                    'postcode': result.get('postCode'),
                    'publishedDate': datetime.fromtimestamp(result['creationTs']),
                    'publishedDate': datetime.strptime(result['created_at'][:10], "%Y-%m-%d"),
                    'publishedDate': publishedDate,
                    'query': query,
                    'resolution': f"{attrs['width']}x{attrs['height']}",
                    'resolution': f'{width} x {height}',
                    'road': properties.get('street'),
                    'road': result.get('thoroughfare'),
                    'template': 'images.html',
                    'template': 'videos.html',
                    'thumbnail': item.get('image'),
                    'thumbnail': result.get('avatar'),
                    'thumbnail': thumbnail,
                    'thumbnail_src': attrs["image"]["small"],
                    'thumbnail_src': clean_url(result['urls']['thumb']),
                    'thumbnail_src': item.get('thumbnail'),
                    'thumbnail_src': thumb,
                    'thumbnail_src': thumbnail,
                    'title': attrs["title"],
                    'title': extract_text(eval_xpath(result, title_xpath)),
                    'title': html_to_text(item['title']),
                    'title': result.get('alt_description') or 'unknown',
                    'title': result['name'],
                    'title': result['title'],
                    'title': result['username'] + f" ({result['followers_count']} followers)",
                    'title': title,
                    'url': 'tel:' + telephone,
                    'url': backlink_json.get("url"),
                    'url': clean_url(result['links']['html']),
                    'url': extract_text(eval_xpath(result, url_xpath)),
                    'url': f"{base_url}/photo/{attrs['slug']}-{attrs['id']}/",
                    'url': f'{url}/album/{album_id}/track/{track_id}',
                    'url': item.get('link'),
                    'url': link,
                    'url': result['link'] or '',
                    'url': result['uri'],
                    'url': result['url'],
                    'url': result_url,
                    'url': source,
                    'url': url,
                    'urls': [{'title': 'Wikipedia', 'url': wikipedia_link}],
                    'url_label': telephone,
                    'url_label': url,
                    'value': v,
                    )
                    ),  # noqa
                    ):
                    and not result['extratags'].get('contact:website')
                    and not result['extratags'].get('website')
                    answer=html_to_text(answer),
                    attributes.append({"label": data_label, "value": area_to_str(data_value), "entity": "P2046"})
                    attributes.append({"label": data_label, "value": data_value})
                    audio_src=media_url,
                    content += "..."
                    content=content,
                    content_parts.append("%s: %s " % (gettext("Channel"), c))
                    content_parts.append("%s: %s " % (gettext("Source"), s))
                    content_parts.append(d)
                    continue
                    date = datetime.strptime(item, 'Date: %Y-%m-%d %H:%M UTC')
                    definitions=word['meanings'],
                    definitions=[translation['definition']] if translation['definition'] else [],
                    else:
                    embedded=media_url,
                    eval_xpath(item, ".//div[contains(@class, 'total_dsc_wrap')]//a[contains(@class, 'api_txt_lines')]")
                    examples=[translation['example']] if translation['example'] else [],
                    filename=unquote(pathlib.Path(media_url).name),
                    filesize=size,
                    highlighted_lines_index.add(len(lines) + 1)
                    highlight_groups.pop(0)
                    if "list" in display_type and (attribute.kwargs.get('official') or attribute_type == WDArticle):
                    if attribute_type == WDArticle and (
                    if data_value.get("globe") == "http://www.wikidata.org/entity/Q2":
                    if img_id:
                    if pod_id != "Input":
                    if t and t.strip():
                    if value.startswith('-'):
                    iframe_src = rec['recording_url']
                    iframe_src=media_url,
                    img_format=imageinfo["mime"],
                    img_id = content_nodes[0].xpath('.//img/@id')
                    img_src = get_thumbnail(value)
                    img_src=imageinfo["url"],
                    img_src_priority = attribute.priority
                    infobox_title = content
                    infobox_urls.append({'title': attribute.get_label(language), 'url': url, 'entity': attribute.name})
                    infobox_urls.append({'title': attribute.get_label(language), 'url': url, **attribute.kwargs})
                    length = timedelta(milliseconds=length)
                    length=duration,
                    mimetype=mimetype,
                    osm_zoom = area_to_osm_zoom(data_value.get("amount"))
                    params['filesize'] = size_re.search(item).group()
                    params['publishedDate'] = date
                    pass
                    print("CONFLICT: babel %s --> %s, %s" % (sxng_tag, conflict, country_tag))
                    print("CONFLICT: babel %s --> %s, %s" % (sxng_tag, conflict, market_code))
                    pub_date = datetime.fromtimestamp(pub_date)
                    res.add(res.types.KeyValue(kvmap=result))
                    res.types.MainResult(
                    resolution=f"{imageinfo['width']} x {imageinfo['height']}",
                    results.add(results.types.LegacyResult({"suggestion": suggestion}))
                    results.append(item)
                    results.append(parsed_results)
                    results.append(video_info)
                    results.append({"suggestion": from_to_prefix + synonym})
                    results.extend(parsed_results)
                    result['extratags']['contact:website'] = website
                    result['extratags']['wikipedia'] = user_language + ':' + wikipedia_name
                    result_chunks.append({'label': pod_title, 'value': subpod['plaintext']})
                    return res
                    return True
                    script.getparent().remove(script)
                    size=size,
                    suggestions.append(suggestion)
                    synonyms=translation['synonyms'],
                    t = extract_text(item)
                    t = value.split('-')
                    tag = css_class.removeprefix(prefix)
                    tag.removeprefix("#") for tag in eval_xpath_list(item, "./div[contains(@class, 'tags')]/a/text()")
                    tags.append(tag.replace("-", " ").title())
                    template="default.html",
                    template="images.html",
                    template="videos.html",
                    text=result['translation'],
                    text=word['word'],
                    thumbnail = news_media[0].get('pict', {}).get('url', None)
                    thumbnail=thumbnail,
                    thumbnail_src=thumbnail,
                    title += ' (' + title_raw['reading'] + ')'
                    title=title,
                    tmp_result['thumbnail'] = extract_url(thumbnail_xpath_result, search_url)
                    url=search_res.get("AbstractURL", ""),
                    url=url,
                    urls.append({"title": data_label, "url": external_url})
                    website
                    ],
                    _command_logger.debug('skipped result:', raw_result)
                    {
                    {'label': attribute.get_label(language), 'value': value, 'entity': attribute.name}
                    }
                  schema:inLanguage "%LANGUAGE%" .
                  schema:isPartOf/wikibase:wikiGroup "wikipedia";
                  schema:name ?wikipediaName;
                ".//div[contains(@class, 'sds-comps-image') and contains(@class, 'sds-rego-thumb-overlay')]//img[@src]/@src",
                "audio_src": uri,
                "author": author,
                "author": channel,
                "author": f"{item.get('userName')} (ID: {item.get('userId')})",
                "author": hit["author"],
                "author": item['channel']['channel_name'],
                "content": " / ".join([c for c in content if c]),
                "content": content,
                "content": description,
                "content": fields.get("abstract", ""),
                "content": html_to_text(content),
                "content": html_to_text(item.get("lyrics")),
                "content": html_to_text(item.get('summary')),
                "content": html_to_text(item['description']),
                "content": item['abstract'],
                "content": meta.get("description", ""),
                "content": package.get("description", ""),
                "content": package["description"],
                "content": {
                "engine_data": token,
                "homepage": package["docs_html_url"],
                "homepage": package["links"].get("homepage"),
                "httpMethod": "POST",
                "iframe_src": "https://w.soundcloud.com/player/?url=" + uri,
                "iframe_src": 'https://www.bitchute.com/embed/' + item['video_id'],
                "iframe_src": get_embeded_stream_url(entry["play_url"]),
                "iframe_src": get_embeded_stream_url(url),
                "iframe_src": iframe_src,
                "iframe_src": iframe_url,
                "img_src": item.get("bigPicUrl"),
                "img_src": proxy_full_image_url,
                "key": "next_page_token",
                "layout": "html",
                "length": duration,
                "length": formatted_duration,
                "length": item['duration'],
                "length": length,
                "license_name": ", ".join(meta.get("licenses", [])),
                "limit": results_per_page,
                "links": links,
                "maintainer": ", ".join(meta.get("maintainers", [])),
                "maintainer": package.get("author", {}).get("name"),
                "metadata": ' | '.join(metadata),
                "metadata": metadata,
                "name": "resourceGroups",
                "name": "resources",
                "package_name": package["name"],
                "page": page_num,
                "pageNum": page_num,
                "pageSize": results_per_page,
                "paramList": f"page_num={page_num},page_size={results_per_page}",
                "pn": (page_num - 1) * results_per_page,
                "position": 0,
                "publishedDate": datetime.fromtimestamp(hit["created_at_i"]),
                "publishedDate": datetime.strptime(item["date_published"], "%Y-%m-%dT%H:%M:%S.%fZ"),
                "publishedDate": formatted_date,
                "publishedDate": parser.parse(result["last_modified"]),
                "publishedDate": pubdate_original,
                "publishedDate": publishedDate,
                "publishedDate": published_date,
                "q": query,
                "query": query,
                "requestHeaderDetails": {
                "requestHeaderDetails": {"commandName": "Microsoft.ResourceGraph"},
                "resolution": f"{item['width']} x {item['height']}",
                "rn": results_per_page,
                "source": 'pixiv.net',
                "source": item.get("site"),
                "source_code_url": package["links"].get("repository"),
                "start": (page_num - 1) * results_per_page,
                "tags": package["keywords"],
                "tags": tags,
                "template": "images.html",
                "template": "packages.html",
                "template": "videos.html",
                "thumbnail": image_url,
                "thumbnail": item.get("image_url").replace("http://", "https://"),
                "thumbnail": item.get('image').replace("http://", "https://"),
                "thumbnail": item['thumbnail_url'],
                "thumbnail": odysee_thumbnail,
                "thumbnail": thumbnail,
                "thumbnail_src": item.get("img"),
                "thumbnail_src": proxy_image_url,
                "title": extract_text(eval_xpath(item, ".//a[contains(@class, 'info_title')]")),
                "title": f"{item['song_name']} | {item['song_singer']}",
                "title": hit.get("title") or f"{gettext('author')}: {hit['author']}",
                "title": html_to_text(item.get('title')),
                "title": item.get("title"),
                "title": item['title'],
                "title": item['video_name'],
                "title": module,
                "title": package["name"],
                "title": result["title"],
                "title": title,
                "tn": "json",
                "tn": "resultjson_com",
                "url": "/providers/Microsoft.ResourceGraph/resources?api-version=2024-04-01",
                "url": "https://metacpan.org/pod/" + module,
                "url": 'https://www.bitchute.com/video/' + item['video_id'],
                "url": entry["url"],
                "url": f"https://news.ycombinator.com/item?id={object_id}",
                "url": f'https://crates.io/crates/{package["name"]}',
                "url": href_base.format(category=categ, entry_id=entry_id),
                "url": item.get("imgUrl"),
                "url": item.get("play_url"),
                "url": item.get('url'),
                "url": item['url'].split('&ueid')[0],
                "url": package["html_url"],
                "url": package["links"]["npm"],
                "url": proxy_full_image_url,
                "url": result["url"],
                "url": url,
                "version": meta.get("latest_version"),
                "version": package.get("version"),
                "version": package["newest_version"] or package["max_version"] or package["max_stable_version"],
                "version": version,
                "views": item['view_count'],
                "views": result.get("likes_count"),
                "wd": query,
                "word": query,
                "{numCitations} citations from the year {firstCitationVelocityYear} to {lastCitationVelocityYear}"
                # "thumbnail": item.get('image_url') or None, # these are not thumbs / to large
                # "usually written as kana: <kana>"
                # 'cs' : 1,
                # 'ei': 'GASaY6TxOcy_xc8PtYeY6AE',
                # 'prmd': 'vin',
                # 'sa': 'N',
                # 'sstk': 'AcOHfVkD7sWCSAheZi-0tx_09XDO55gTWY0JNq3_V26cNN-c8lfD45aZYPI8s_Bqp8s57AHz5pxchDtAGCA_cikAWSjy9kw3kgg'
                # 'ved': '2ahUKEwik3ZbIzfn7AhXMX_EDHbUDBh0Q_skCegQIARAG',
                # 'version': item.get('updated_at'),
                # 'vet': '12ahUKEwik3ZbIzfn7AhXMX_EDHbUDBh0QxK8CegQIARAC..i',
                # 'yv': 3,
                # (the infobox contain only one image).
                # * imdb_id / facebook_profile / youtube_channel / youtube_video / twitter_profile
                # * instagram_profile / rotten_tomatoes / spotify_artist_id / itunes_artist_id / soundcloud_id
                # * netflix_id
                # -createdAt: sort by date ascending / createdAt: date descending
                # abbreviation, archaism, etc.
                # and len(fragment)), so only check the first highlight group
                # append result
                # append unless it's not an actual answer
                # Avoid reading more results than available.
                # Bing will send back the results from 0 to 10 and no error.
                # by a different symbol --> then we split at the first space.
                # eval_xpath_getindex(source, './/a', 0, None),
                # eval_xpath_getindex(source, './div/span', 3, None),
                # exclude languages with too few articles
                # For example, if there is 100 results from some search and we try to get results from 120 to 130,
                # formally known as use_mobile_ui
                # frequently its articles are updated.
                # from some locations (DE and others?) the s2 link do
                # geocoordinate link
                # get website if not already defined
                # get_select() method : there is group_concat(distinct ...;separator=", ")
                # guaranteed match in the code (all indices are in the range 0
                # https://github.com/searxng/searxng/issues/3790
                # if no input pod was found, title is first plaintext pod
                # If we compare results count with the first parameter of the request we can avoid this "invalid"
                # In other languages (tested with zh-TW) a colon is represented
                # Is it an external URL ?
                # link to the magnet
                # link to the torrent file
                # miss the 'desc' key.
                # Note: ignore the unit (must be km² otherwise the calculation is wrong)
                # okay, we have a valid magnet link, let's add it to the result
                # overwrite wikipedia link
                # print("ignore lang: %s <-- %s" % (cc_tag, lang_tag))
                # pylint: disable=unexpected-keyword-arg
                # qwant-news does not support all locales from qwant-web:
                # replace the current image only the priority is lower
                # response a 'Please wait ..' but does not deliver the thumbnail
                # results.
                # returns an empty string, such video from a qwant-web query
                # Rough indicator of a Wikipedia’s quality, showing how
                # Should use normalized value p:P2046/psn:P2046/wikibase:quantityAmount
                # some videos do not have a description: while qwant-video
                # split text into key / value
                # split the value here
                # test https://docs.searxng.org/_static/searxng-wordmark.svg
                # test https://notexists
                # test https://pngimg.com/uploads/dot/dot_PNG4.png
                # the API ensures these are sorted already, and we have a
                # the last item in the JSON list is empty, the JSON string ends with "}, {}]"
                # There is no google.cn, we use .com.hk for zh-CN
                # this attribute is an image.
                # this not one of the common google results *section*
                # title sometimes contains HTML tags / see
                # Use Repository Avatar and fall back to Owner Avatar if not set.
                # use the area to get the OSM zoom
                # Workaround: ddg may return a double quote
                ''.join(extra)[:-1],
                ', '.join(defn_raw['parts_of_speech']),
                ', '.join(result.get('tags', [])),
                './/img[@class="T75of stzEZd" or @class="T75of etjhNc Q8CSx "]/@src',
                '.google.com',
                '; '.join(defn_raw['english_definitions']),
                'address': address,
                'address': {
                'address_label': get_key_label('addr', user_language),
                'asearch': 'arc',
                'async': str_async,
                'async': ui_async(start),
                'attributes': attributes,
                'attributes': infobox_attributes,
                'author': ', '.join(result['artist_titles']),
                'author': author,
                'author': f"{result['pinner'].get('full_name')} ({result['pinner']['username']})",
                'author': photo['ownername'],
                'author': result.get('account', {}).get('displayName'),
                'author': result['sellerName'],
                'author': section['ownerText']['runs'][0]['text'],
                'author': username,
                'boundingbox': boundingbox,
                'boundingbox': result['boundingbox'],
                'content': " ".join(html_to_text(content).strip().split()),
                'content': " | ".join(content),
                'content': "%(description)s" % item,
                'content': "%(medium_display)s // %(dimensions)s" % result,
                'content': ", ".join(tags) + " / " + ", ".join(icon_categories),
                'content': "No category or tags." if len(content) == 0 else ' '.join(content),
                'content': "\n".join(content),
                'content': ' / '.join(content),
                'content': ' / '.join([i for i in content_items if i]),
                'content': ' | '.join(content),
                'content': '',
                'content': ''.join(fragment['text'] for fragment in result['snippet']['text']['fragments']),
                'content': ', '.join(content),
                'content': ', '.join(tags),
                'content': (result.get('rich_summary') or {}).get('display_description') or "",
                'content': content,
                'content': entry["techDocDigest"]["summary"],
                'content': entry['site'],
                'content': eval_xpath_getindex(item, content_xpath, 0),
                'content': extract_text(eval_xpath(item, './p')),
                'content': extract_text(eval_xpath(result, content_xpath)),
                'content': extract_text(eval_xpath(result, info_text_xpath)),
                'content': extract_text(eval_xpath(result, news_content_xpath)),
                'content': extract_text(result_data),
                'content': f"{result['category']} / {result['purity']}",
                'content': f"{result['short_desc']} - {humanize_bytes(result['filename_size'])}",
                'content': html.unescape(content),
                'content': html_to_text(content),
                'content': html_to_text(entry.get("ImageInfo", "")),
                'content': html_to_text(entry["description"]),
                'content': html_to_text(entry["snippet"]),
                'content': html_to_text(result.get('description') or ''),
                'content': html_to_text(result['content']),
                'content': infobox_content,
                'content': info_item.get('content', ''),
                'content': item.get('alt'),
                'content': item.get('description'),
                'content': item['description'],
                'content': json['result']['response'],
                'content': markdown_to_text(result['comment']['content']),
                'content': markdown_to_text(result['community'].get('description', '')),
                'content': markdown_to_text(result['person'].get('bio', '')),
                'content': metadata.get('desc'),
                'content': photo['description']['_content'],
                'content': pkg_list[0]['content'],
                'content': result['description'],
                'content': result['extract'][0]['value'],
                'content': result['teaser']['text'],
                'content': result['topic']
                'context': {"client": {"clientName": "WEB", "clientVersion": "2.20210310.12.01"}},
                'continuation': params['engine_data']['next_page_token'],
                'count': 10,
                'country': address_raw.get('country'),
                'country_code': address_raw.get('country_code'),
                'data': data,
                'engine_data': nextpage_url.replace("http://", "https://"),
                'files': files,
                'filesize': filesize,
                'filesize': fullDescription[1],
                'filesize': humanize_bytes(item['meme_file_size']),
                'filesize': humanize_bytes(result['file_size']),
                'filter': '0',
                'format': tineye_match['image_format'],
                'geojson': geojson,
                'geojson': {'type': 'Point', 'coordinates': [result['center']['lng'], result['center']['lat']]},
                'height': tineye_match['height'],
                'homepage': extract_text(eval_xpath(result, './td[contains(@class, "url")]/a/@href')),
                'homepage': item.get('homepage'),
                'homepage': item.get('readme_url'),
                'homepage': item.get('website'),
                'house_number': address_raw.get('house_number'),
                'id': infobox_id,
                'id': info['title'],
                'iframe_src': "https://player.vimeo.com/video/" + videoid,
                'iframe_src': "https://www.youtube-nocookie.com/embed/" + videoid,
                'iframe_src': 'https://www.youtube-nocookie.com/embed/' + section['videoId'],
                'iframe_src': iframe_src,
                'iframe_src': item['url_video_hd'].replace("http://", "https://"),
                'iframe_src': result.get('embedUrl'),
                'iframe_src': result['url_resolved'].replace("http://", "https://"),
                'img_format': img_format[1] if len(img_format) >= 2 else None,
                'img_format': result['file_type'],
                'img_src': backlink['url'],
                'img_src': base_url + extract_text(eval_xpath(result, "./a/img/@src")),
                'img_src': base_url + extract_text(eval_xpath(result, image_img_src_xpath)),  # type: ignore
                'img_src': entry["url"].replace("http://", "https://"),
                'img_src': extract_text(eval_xpath(result, ".//img/@src")),
                'img_src': extract_text(eval_xpath(result, img_src_xpath)),
                'img_src': image_api + '/%(image_id)s/full/843,/0/default.jpg' % result,
                'img_src': IMAGE_URL.format(base=BASE, episode=episode, timestamp=timestamp),
                'img_src': img if item['type'] == 'IMAGE' else thumb,
                'img_src': img_list[-1],
                'img_src': img_src,
                'img_src': img_src_url.format(icon_name=result["name"], svg_type=svg_type),
                'img_src': info.get('image'),
                'img_src': main_image['url'],
                'img_src': metadata['murl'],
                'img_src': result['path'],
                'img_src': result['url'],
                'img_src': thumbnail_src,
                'img_src': _clean_url(base_image_url),
                'infobox': cf_ai_model_display_name,
                'infobox': infobox_title,
                'infobox': info['title'],
                'key': 'nextpage',
                'label': get_key_label(k, user_language),
                'latitude': geojson['coordinates'][1],
                'latitude': result['center']['lat'],
                'latitude': result['lat'],
                'leech': 'N/A',
                'leech': leech,
                'length': (">= " if end_time is None else "") + timestring,
                'length': datetime.timedelta(seconds=item['length']),
                'length': duration,
                'length': item['hms'],
                'length': length,
                'length': section['lengthText']['simpleText'],
                'license_name': extract_text(eval_xpath(result, './td[contains(@class, "license")]')),
                'license_name': extract_text(eval_xpath(result, license_name_xpath)),
                'license_name': lic.get('name'),
                'license_url': base_url + extract_text(eval_xpath(result, license_url_xpath)),
                'license_url': lic_url,
                'links': links,
                'locality': address_raw.get(
                'longitude': geojson['coordinates'][0],
                'longitude': result['center']['lng'],
                'longitude': result['lon'],
                'magnetlink': magnetlink,
                'magnetlink': magnet_link,
                'maintainer': extract_text(eval_xpath(result, './td[contains(@class, "maintainer")]')),
                'maintainer': item.get('namespace', {}).get('name'),
                'maintainer': item.get('owner', {}).get('login'),
                'maintainer': item.get('owner', {}).get('username'),
                'metadata': ' | '.join(metadata),
                'metadata': ', '.join(metadata),
                'metadata': extract_text(eval_xpath(result, author_xpath)),
                'metadata': f"Rank: {result['rank']} || {result['episode_count']} episodes",
                'metadata': f"{result['author']}, {result['episodeCount']} episodes",
                'metadata': metadata,
                'name': address_name,
                'nsfw': safesearch_table[params['safesearch']],
                'osm': osm,
                'package_name': ' | '.join(x['package_name'] for x in pkg_list),
                'package_name': extract_text(eval_xpath(result, './td[contains(@class, "package")]')),
                'package_name': item.get('name'),
                'package_name': package["name"],
                'package_name': package_name,
                'package_name': re.sub(r"\(|\)", "", extract_text(eval_xpath(result, package_name_xpath))),
                'package_name': result['name'],
                'package_name': title,
                'popularity': extract_text(eval_xpath(result, download_count_xpath)),
                'popularity': item.get('stargazers_count'),
                'popularity': item.get('stars_count'),
                'popularity': item.get('star_count'),
                'popularity': popularity,
                'postcode': address_raw.get('postcode'),
                'price': price,
                'publishedDate': backlink['crawl_date'],
                'publishedDate': datetime.fromtimestamp(int(info_item['posted_at'])),
                'publishedDate': datetime.fromtimestamp(result['newestItemPubdate']),
                'publishedDate': datetime.strptime(counts['published'][:19], '%Y-%m-%dT%H:%M:%S'),
                'publishedDate': datetime.strptime(pubDate, '%a,%d %b %Y %H:%M:%S %z'),
                'publishedDate': datetime.strptime(result['comment']['published'][:19], '%Y-%m-%dT%H:%M:%S'),
                'publishedDate': datetime.strptime(result['created_at'], '%Y-%m-%d %H:%M:%S'),
                'publishedDate': datetime.strptime(result['post']['published'][:19], '%Y-%m-%dT%H:%M:%S'),
                'publishedDate': datetime.strptime(result['status_since'], '%Y-%m-%d %H:%M:%S'),
                'publishedDate': fixed_date,
                'publishedDate': formatted_date,
                'publishedDate': parse(result['currentVersionReleaseDate']),
                'publishedDate': parse(result['publishedAt']),
                'publishedDate': parser.parse(created_at),  # type: ignore
                'publishedDate': parser.parse(extract_text(eval_xpath(result, './td[contains(@class, "bdate")]'))),
                'publishedDate': parser.parse(item.get("updated_at") or item.get("created_at")),
                'publishedDate': parser.parse(item.get('last_activity_at') or item.get("created_at")),
                'publishedDate': publishedDate,
                'publishedDate': published_date,
                'publishedDate': start_time,
                'q': query,
                'resolution': f"{main_image['width']}x{main_image['height']}",
                'resolution': img_format[0],
                'resolution': resolution,
                'resolution': result['resolution'].replace('x', ' x '),
                'resultType': 'videos',
                'road': address_raw.get('road'),
                'search': query,
                'searchTarget': 'search-index',  # Vidiversum
                'seed': 'N/A',
                'seed': seed,
                'shipping': shipping,
                'sort': '-match',  # sort by *match descending*
                'source': (result.get('rich_summary') or {}).get('site_name'),
                'source': backlink['url'],
                'source': source,
                'source_code_url': item.get('clone_url'),
                'source_code_url': item.get('http_url_to_repo'),
                'source_country': source_country,
                'start': (params['pageno'] - 1) * 10,
                'start': start,
                'tags': eval_xpath_list(result, tags_xpath),
                'tags': item.get('tag_list', []),
                'tags': item.get('topics', []),
                'tags': result['repository'],
                'tags': [extract_text(eval_xpath(result, './td[contains(@class, "repo")]'))],
                'tags': [x['tags'] for x in pkg_list],
                'tbm': "vid",
                'template': 'images.html',
                'template': 'map.html',
                'template': 'packages.html',
                'template': 'products.html',
                'template': 'torrent.html',
                'template': 'videos.html',
                'thumbnail': entry["cover_img"],
                'thumbnail': entry["image_src"].replace("http://", "https://"),
                'thumbnail': entry["picurl"],
                'thumbnail': eval_xpath_getindex(item, thumbnail_xpath, 0),
                'thumbnail': extract_text(eval_xpath(result, thumbnail_xpath)),
                'thumbnail': item.get('avatar_url') or item.get('owner', {}).get('avatar_url'),
                'thumbnail': item.get('avatar_url'),
                'thumbnail': item.get('owner', {}).get('avatar_url'),
                'thumbnail': item['thumb_url'],
                'thumbnail': recipe['previewImageUrlTemplate'].replace("<format>", thumbnail_format),
                'thumbnail': result.get('favicon', '').replace("http://", "https://"),
                'thumbnail': result.get('thumbnailUrl') or result.get('previewUrl'),
                'thumbnail': result['artworkUrl100'],
                'thumbnail': result['avatar_url'],
                'thumbnail': result['community'].get('icon', result['community'].get('banner')),
                'thumbnail': result['image'],
                'thumbnail': result['smallImageURL'],
                'thumbnail': section['thumbnail']['thumbnails'][-1]['url'],
                'thumbnail': thumbnail,
                'thumbnail': thumbnailUrl,
                'thumbnail_src': entry["largeimage"].replace("http://", "https://"),
                'thumbnail_src': extract_text(eval_xpath(result, thumbnail_src_xpath)),
                'thumbnail_src': img_list[0],
                'thumbnail_src': metadata['turl'],
                'thumbnail_src': result['images']['236x']['url'],
                'thumbnail_src': result['thumbs']['small'],
                'thumbnail_src': thumbnail_src,
                'thumbnail_src': THUMB_URL.format(base=BASE, episode=episode, timestamp=timestamp),
                'thumbnail_src': tineye_match['image_url'],
                'thumbnail_src': _clean_url(base_image_url + THUMBNAIL_SUFFIX),
                'title': " ".join(html_to_text(title).strip().split()),
                'title': "%(channel)s: %(title)s (%(hms)s)" % item,
                'title': ' '.join(x['text'] for x in section['title']['runs']),
                'title': ' | '.join(x['title'] for x in pkg_list),
                'title': '',
                'title': ''.join(title_parts),
                'title': 'Jisho.org',
                'title': backlink['image_name'],
                'title': entry["techDocDigest"]["title"],
                'title': entry["titleEsc"],
                'title': episode,
                'title': extract_text(eval_xpath(item, './h2/a')),
                'title': extract_text(eval_xpath(item, title_xpath)),
                'title': extract_text(eval_xpath(result, ".//img/@alt")),
                'title': extract_text(eval_xpath(result, "./a/img/@alt")),
                'title': extract_text(eval_xpath(result, './td[contains(@class, "package")]')),
                'title': extract_text(eval_xpath(result, image_title_xpath)),
                'title': extract_text(eval_xpath(result, news_title_xpath)),
                'title': extract_text(eval_xpath(result, title_xpath)),
                'title': extract_text(eval_xpath(result, title_xpath)).replace(" SVG File", "").replace("Show ", ""),
                'title': extract_text(link),
                'title': extract_text(title_element),
                'title': f"{result['title'].strip()} by {result['artist']} {result.get('displayYear', '')}",
                'title': html.unescape(result['title']),
                'title': html_to_text(entry["raw_title"]),
                'title': html_to_text(entry["title"]),
                'title': info_item.get('title'),
                'title': item.get('full_name'),
                'title': item.get('name'),
                'title': item.get('source_site'),
                'title': item['title'],
                'title': metadata.get('vt', ''),
                'title': package_name,
                'title': photo['title'],
                'title': recipe['title'],
                'title': result.get('title') or result.get('grid_title'),
                'title': result["name"].replace("_", "").title(),
                'title': result['community']['title'],
                'title': result['label'],
                'title': result['name'],
                'title': result['person']['name'],
                'title': result['post']['name'],
                'title': result['teaser']['title'],
                'title': result['title'] + " (%(date_display)s) // %(artist_display)s" % result,
                'title': result['title'],
                'title': result['trackName'],
                'title': title,
                'torrentfile': torrent_link,
                'type': get_tag_label(result.get('category'), result.get('type', ''), user_language),
                'type': result.get('poiCategory'),
                'type_icon': result.get('icon'),
                'url': "https://%s.com/q/%s" % (api_site, result['question_id']),
                'url': 'https://artic.edu/artworks/%(id)s' % result,
                'url': backlink['backlink'],
                'url': base_url + "/" + extract_text(eval_xpath(result, url_xpath)),
                'url': base_url + extract_text(eval_xpath(result, "./a/@href")),
                'url': base_url + extract_text(eval_xpath(result, './td[contains(@class, "package")]/a/@href')),
                'url': base_url + extract_text(eval_xpath(result, url_xpath)),
                'url': base_url + extract_text(eval_xpath(result, url_xpath)),  # type: ignore
                'url': base_url + result['teaser']['link']['url'],
                'url': base_youtube_url + section['videoId'],
                'url': build_flickr_url(photo['owner'], photo['id']),
                'url': entry["play_url"],
                'url': entry["techDocDigest"]["url"],
                'url': entry["url"],
                'url': entry["web_url"],
                'url': extract_text(eval_xpath(item, "./span[contains(@class, 'url partner')]")),
                'url': extract_text(eval_xpath(result, "./a/@href")),
                'url': extract_text(eval_xpath(result, image_url_xpath)),
                'url': extract_text(eval_xpath(result, news_url_xpath)),
                'url': extract_text(eval_xpath(result, url_xpath)),
                'url': f"{base_url}/doc/{info_item['user_id']}/{info_item['doc_id']}",
                'url': href,
                'url': item.get('html_url'),
                'url': item.get('source_page_url'),
                'url': item.get('web_url'),
                'url': item['frontend_link'],
                'url': item['url_video_hd'].replace("http://", "https://"),
                'url': link,
                'url': matrix_url + '/#/' + result['alias'],
                'url': metadata['murl'],
                'url': metadata['purl'],
                'url': pdbe_entry_url.format(pdb_id=result['pdb_id']),
                'url': pkg_url,
                'url': recipe['siteUrl'],
                'url': result.get('link') or f"{base_url}/pin/{result['id']}/",
                'url': result['comment']['ap_id'],
                'url': result['community']['actor_id'],
                'url': result['foreign_landing_url'],
                'url': result['htmlURL'],
                'url': result['link'],
                'url': result['person']['actor_id'],
                'url': result['placecardUrl'],
                'url': result['post']['ap_id'],
                'url': result['trackViewUrl'],
                'url': result['url'],
                'url': result_url,
                'url': RESULT_URL.format(base=BASE, query=urlencode({'p': 'caption', 'e': episode, 't': timestamp})),
                'url': result_url.format(icon_name=result["name"], query=result["name"], fill=0 if outlined else 1),
                'url': title_element.get('href'),
                'url': url + 'structure/' + result['id'],
                'url': url,
                'url': urljoin(base_url, link.get('href')),  # type: ignore
                'url': video_url,
                'url': _clean_url(f"{about['website']}/images/{result['objectID']}"),
                'urls': infobox_urls,
                'url_label': url_label,
                'version': extract_text(eval_xpath(result, './td[contains(@class, "version")]')),
                'version': extract_text(eval_xpath(result, version_xpath)),
                'version': f"v{result['version']}_{result['revision']}",
                'version': pkg_list[0]['version'],
                'views': humanize_number(result['views']),
                'width': tineye_match['width'],
                (extract_url(x, search_url) for x in eval_xpath_list(dom, url_xpath)),
                )
                ),
                ),  # noqa
                ):
                **google_info['params'],
                + f" // {result['alias']}"
                + f" // {result['members']} members"
                + f" // {result['server']}",
                - relativedelta(**{f"{params['time_range']}s": 1})  # type: ignore
                .get("continuationCommand", {})
                .get("continuationEndpoint", {})
                .get("token", "")
                0,
                abstract=result.get("abstract", ""),
                alt_forms.append(title)
                alt_forms.append(title_raw['reading'])
                and api_result['detail'] == 'title-invalid-characters'
                answer=gettext('Show route in map ..'),
                answer=zero_click,
                api_result['type'] == 'https://mediawiki.org/wiki/HyperSwitch/errors/bad_request'
                area = attribute_result.get('P2046')
                args['countrycode'] = countrycode
                author = get_text_from_json(video.get('ownerText', {}))
                author=result.get("author", ""),
                authors.add(name)
                authors.append(author_name)
                authors=authors,
                authors=item.get("author_name", []),
                authors=result['entry_author_list'][0],
                base_url + "/" + extract_text(eval_xpath(result, ".//a[contains(@class, 'listview__name-link')]/@href"))
                break
                buffer = []
                caption=f"Demo Offline Engine Result #{count}",
                codelines=sorted(lines.items()),
                codelines=[(i + 1, line) for (i, line) in enumerate(lines)],
                comments=comments,
                content = ' // '.join(content_parts)
                content = content.lstrip(_pub_date).strip("- \n\t")
                content = content[:500] + '...'
                content = extract_text(eval_xpath(r, '.'))
                content = get_text_from_json(video.get('descriptionSnippet', {}))
                content = item.text[:300]
                content = item['desc']
                content = replace_pua_chars(content)
                content.append(info)
                content.append(item)
                content.append(v)
                content=abstract,
                content=content,
                content=description,
                content=extract_text(
                content=extract_text(eval_xpath(item, "./p")),
                content=extract_text(eval_xpath(item, content_xpath)),
                content=extract_text(_content) or "",
                content=html_to_text(' | '.join(content)),
                content=html_to_text(item["snippet"]),
                content=html_to_text(result["paperAbstract"]["text"]),
                content=item.get("description", ""),
                content=repo['description'],
                content=result.get("description", ""),
                content=result.get("fullText", "") or "",
                content=result.get("summary", ""),
                content=result["description"],
                content=_get_most_common([pkg.get("summary") for pkg in repositories]),
                content_list.append(item['dot_text'])
                content_parts = []
                continue
                count += 1
                crawl_date = datetime.min
                crawl_date = datetime.strptime(crawl_date, '%Y-%m-%d')
                d, s, c = item.get('desc'), item.get('source'), item.get('channel')
                data.get('address'),
                data.get('business_scope'),
                data.get('capital'),
                data.get('company_status'),
                data.get('company_type'),
                data.get('controled_type'),
                data.get('establish_time'),
                data_label = coordinates.get("label")
                data_label = info.get("label")
                data_type: str = info.get("data_type", "")
                data_value = coordinates.get("value")
                data_value = info.get("value")
                date = item.text
                datetime.now()
                def_text = def_text[len(def_abbr) :].strip()
                doi=doi,
                doi=result.get("doi"),
                doi=result.get("doiInfo", {}).get("doi"),
                duration = datetime.timedelta(seconds=int(seconds))
                duration = parse_duration_string(duration)
                e.getparent().remove(e)
                editor=", ".join(result.get("contributors", [])),
                elif data_type == "area":
                elif data_type == "coordinates":
                elif data_type == "string" and data_label == "Website":
                elif data_type == "string":
                elif data_type in ["instance", "wiki_maps_trigger", "google_play_artist_id"]:
                elif rec['mime_type'] == 'video/mp4':
                else:
                embedded=embedded,
                engine_traits.custom['lang_region'][sxng_tag] = eng_lang
                engine_traits.custom['supported_domains']['CN'] = 'www' + domain  # type: ignore
                engine_traits.regions[region_tag(Locale(lang, code))] = region
                eval_xpath_getindex(source, './/span[@aria-label]/@aria-label', 0, None),
                eval_xpath_getindex(_content, ".//span[contains(@class, 't-secondary')]", 0, default="")
                except:  # pylint: disable=bare-except
                external_url: str | None = get_external_url(data_type, data_value)  # type: ignore
                extra.append(', '.join(defn_raw['tags']) + '. ')
                extra.append(defn_raw['tags'][0] + ', ' + defn_raw['info'][0] + '. ')
                extract_text(eval_xpath(item, publish_date_xpath)), "%b %d, %Y %I:%M %p %Z"
                filename=f"{item['path']}",
                filename=result.get("filename", ""),
                filename=result["filename"],
                firstCitationVelocityYear=result["citationStats"]["firstCitationVelocityYear"],
                for script in item.xpath(".//script"):
                for synonym in item.get('synonyms', []):
                for url in value.split(', '):
                geojson['coordinates'][0],
                geojson['coordinates'][1],
                hl_lines=highlighted_lines_index,
                html_url = val
                html_url=html_url,
                if 'reading' in title_raw:
                if (
                if after <= (i + offset) < before:
                if attribute.priority > img_src_priority:
                if c:
                if conflict != country_tag:
                if conflict != market_code:
                if css_class.startswith(prefix):
                if d:
                if data_value == '""':
                if end < count:
                if external_url is not None:
                if isinstance(parsed_results, list):
                if item is not None:
                if item:
                if len(item.text) > 300:
                if len(thumbnail_xpath_result) > 0:
                if length is not None:
                if news_media:
                if not iframe_src:
                if not infobox_title:
                if part in tag:
                if pod_is_result or not result_content:
                if precision >= 1:
                if pub_date is not None:
                if result is None:
                if s:
                if start <= count and count <= end:  # pylint: disable=chained-comparison
                if subpod['plaintext'] != '(requires interactivity)':
                if suggestion != heading and suggestion is not None:
                if thumbnail.startswith('data:image'):
                if url:
                if video_info and video_info["title"] and video_info["url"]:
                if wikipedia_name:
                img_format = format_name.lower()
                img_src = item['media']
                infobox_attributes.append(
                isbn=item.get("isbn", [])[:5],
                issn=[issn],
                item = KeyValue(kvmap=kvmap)
                item = MainResult(**kvmap)  # type: ignore
                item = _get_image_result(item)
                item,
                item.definitions.append(word['definition'])
                item.examples.append(re.sub(r"<|>", "", example).lstrip('- '))
                item.text = def_text
                item["iframe_src"] = iframe_src
                item["template"] = "videos.html"
                item['thumbnail'] = image_url.format(image_id=result['image'], filename=result['image_filename'])
                journal=", ".join(journals),
                journal='',
                journal=journal,
                journal=result.get("venue", {}).get("text") or result.get("journal", {}).get("name"),
                journal=result['journal'],
                key_title="Name",
                kvmap=kvmap,
                label = label[:-1]
                label, value = text.split(' ', 1)
                label, value = text.split(':', 1)
                lang = x[0]
                lastCitationVelocityYear=result["citationStats"]["lastCitationVelocityYear"],
                latest_version = repo["version"]
                latitude = data_value.get("latitude")
                leftover = raw_results[-1]
                length = datetime.timedelta(seconds=length)
                length = get_text_from_json(video.get('lengthText', {}))
                length = item['duration']
                length = None
                length = time.strftime("%H:%M:%S", length)
                length = time.strftime("%M:%S", length)
                length = timedelta(minutes=timediff.minute, seconds=timediff.second)
                license_name=_get_most_common(_flatten([pkg.get("licenses", []) for pkg in repositories])),
                lines.append("".join(buffer))
                link.attrib.get('data-author'),
                links[v] = l
                logger.debug('ignoring item from the result_xpath list: missing content of title "%s"', title)
                logger.debug('ignoring item from the result_xpath list: missing title')
                logger.debug('ignoring item from the result_xpath list: missing url of title "%s"', title)
                logger.warning("Unknown suggestion key encountered: %s", s_key)
                longitude = data_value.get("longitude")
                magic = '_V1_' + magic
                magnet_link = url
                MainResult(
                maintainer=(extract_text(eval_xpath(item, "./h4/a[1]")) or "").removeprefix("~"),
                map(extract_text, eval_xpath_list(dom, cached_xpath)),
                map(extract_text, eval_xpath_list(dom, content_xpath)),
                map(extract_text, eval_xpath_list(dom, title_xpath)),
                message = ','.join(description)
                message = DOWNLOAD_ERROR
                message = FORMAT_NOT_SUPPORTED
                message = NO_SIGNATURE_ERROR
                message="google_scholar: unusual traffic detected",
                metadata.append(f"{x} {v}")
                metadata.append(time)
                metadata=metadata,
                metadata=result.get("kicker", {}).get("name"),
                msg_superseded=msg_superseded, url=superseded_url, pdb_id=result['superseded_by']
                mtype=mtype,
                news_media = item.get('media', [])
                number=number,
                numCitations=result["citationStats"]["numCitations"],
                osm_zoom = area_to_osm_zoom(area) if area else 19
                package_name=extract_text(eval_xpath(item, "./h4/a[2]")),
                package_name=_get_most_common([pkg.get("visiblename") for pkg in repositories]),
                page='',
                page=result['journal_page'],
                pages=pages,
                params['content'] = item
                params['magnetlink'] = magnet
                parsed_results = parsers(initial_data)
                pass
                pdf_url = doc["url"]
                pdf_url = val
                pdf_url=pdf_url,
                pdf_url=result.get("downloadUrl", {}) or result.get("sourceFulltextUrls", {}),
                popularity_infos.insert(0, f"{pull_count} pulls")
                print("CONFLICT: babel %s --> %s, %s" % (sxng_lang, conflict, eng_lang))
                print("CONFLICT: babel %s --> %s, %s" % (sxng_tag, conflict, eng_lang))
                print("CONFLICT: babel %s --> %s, %s" % (sxng_tag, conflict, eng_tag))
                print("CONFLICT: babel %s --> %s, %s" % (sxng_tag, conflict, lang_tag))
                print("CONFLICT: babel %s --> %s, %s" % (sxng_tag, conflict, ui_lang))
                print("ERROR: can't determine babel locale of startpage's locale %s" % eng_tag)
                print("ERROR: title tag from %s (%s) is unknown" % (netloc, sxng_tag))
                print(f"CONFLICT: babel {sxng_tag} --> {conflict}, {bing_ui_lang}")
                properties.get('extent')[0],
                properties.get('extent')[1],
                properties.get('extent')[2],
                properties.get('extent')[3],
                published = min(published_dates)
                publishedDate = datetime.strptime(date, date_format)
                publishedDate = datetime.strptime(img_date, "%Y-%m-%d %H:%M")
                publishedDate = parser.parse(result['pubDate'])
                publishedDate=date,
                publishedDate=parser.isoparse(result["display_time"]),
                publishedDate=published,
                publishedDate=publishedDate,
                publishedDate=published_date,
                publishedDate=pub_date,
                publishedDate=_extract_published_date(result.get("age")),
                published_date = datetime.fromisoformat(result["publishedDate"].replace("Z", "+00:00"))
                published_date = datetime.fromtimestamp(entry["time"])
                published_date = datetime.fromtimestamp(int(entry["publish_time"]))
                published_date = datetime.fromtimestamp(int(entry["timestamp"]))
                published_date = datetime.fromtimestamp(int(entry["VideoPubDate"]))
                published_date = datetime.fromtimestamp(int(item.get("publish_time")))
                published_date = datetime.fromtimestamp(int(match.group(1)))
                published_date = datetime.strptime(create_time.strip(), "%Y-%m-%d")
                published_date = datetime.strptime(entry['date'], "%Y-%m-%d")
                published_date = datetime.strptime(upload_time, "%Y/%m/%d %H:%M")
                published_date = None
                publisher=publisher,
                pub_date = datetime(year=int(year.text), month=int(month.text), day=int(day.text))
                pub_date = item['date']
                pub_date = None
                pub_date = parser.parse(pub_date)
                pub_date = _extract_published_date(_pub_date)
                p_text += " // " + smpl
                raise ValueError("adobe_stock engine: adobe_content_types: '%s' is invalid" % t)
                raise ValueError('submitted query params is not allowed', param, 'allowed params:', query_enum)
                repository=repo['html_url'],
                repository=result["repo"],
                res.add(
                res.types.File(
                res.types.LegacyResult(
                res.types.MainResult(
                resolution = f'{info_item["width"]}x{info_item["height"]}'
                result = __parse_single_result(raw_result)
                result,
                result.get("description"),
                result.get("label_name"),
                result.get('channel', {}).get('displayName'),
                result.get('channel', {}).get('name') + '@' + result.get('channel', {}).get('host'),
                results.append(
                results.append(func(hit))
                results.append(_get_news_result(item))
                results.append(_get_web_result(item))
                results.append(_result(video, album_info))
                results.append({'title': title, 'content': content, 'url': urljoin(base_url, res_url)})
                results.append({'url': url, 'title': title, 'content': content, 'is_onion': is_onion})
                results.types.Answer(
                results.types.LegacyResult(
                results.types.Translations.Item(
                result['wikidata'] = {
                result_chunks.append(
                result_chunks.append({'label': pod_title, 'image': subpod['img']})
                result_chunks.append({'label': pod_title, 'value': content})
                result_url = result['link']
                result_url = result['url']
                res["length"] = length
                res_url = eval_xpath(r, './/a[@class="wikilink1"]/@href')[-1]
                ret.append(value)
                ret.extend(do_query(value, q))
                ret.extend(do_query(value, q[1:]))
                return format_method(value, language)
                return get_label(o, lang).lower()
                return str(year - 1)
                return True
                return unit
                return value
                return []
                return {}
                section["continuationItemRenderer"]
                size=result.get("size", ""),
                size_data = photo['sizes']['data'][image_size]['data']
                strip_new_lines=ghc_strip_new_lines,
                strip_whitespace=ghc_strip_whitespace,
                strip_whitespace=True,
                subtype=subtype,
                suggestion = result_to_text(text, ddg_result.get("Result", ""))
                suggestion = result_to_text(topic_result.get("Text", ""), topic_result.get("Result", ""))
                superseded_url = pdbe_entry_url.format(pdb_id=result['superseded_by'])
                suspended_time=0, message=f"VQD missed (page: {params['pageno']}, locale: {params['searxng_locale']})"
                sxng_tag = language_tag(babel.Locale.parse(ui_lang, sep="-"))
                sxng_tag = locales.region_tag(babel.Locale.parse(eng_lang))
                sxng_tag = region_tag(babel.Locale.parse(babel_region_tag, sep='_'))
                sxng_tag = region_tag(babel.Locale.parse(ui_lang, sep="-"))
                tags=item.get("subject", [])[:10] + item.get("place", [])[:10],
                tags=list({pkg.get("repo") for pkg in repositories}),  # ensure that tags are unique
                tags=result.get("fieldOfStudy", []),
                tags=result.get("fieldsOfStudy"),
                tags=tags,
                tags=[
                template="packages.html",
                template='packages.html',
                text += f" : {p_text}"
                thumbnail = ""
                thumbnail = 'https://i.ytimg.com/vi/' + videoid + '/hqdefault.jpg'
                thumbnail = 'https://www.bing.com/' + thumbnail
                thumbnail = data_image_map[img_id]
                thumbnail = item['thumbnail']
                thumbnail = None
                thumbnail = resp.search_params['base_url'] + thumbnail
                thumbnail = thumb.get("url", "")
                thumbnail = thumbnail.replace('https://s2.qwant.com', 'https://s1.qwant.com', 1)
                thumbnail = thumbnail[0]
                thumbnail = url + thumbnail
                thumbnail=cover,
                thumbnail=item.get('tiny_image', ''),
                thumbnail=resize_url(result.get("thumbnail", {}), height=80),
                thumbnail=result.get("image"),
                thumbnail=result.get("thumbnail", {}).get("src"),
                thumbnail=thumbnail,
                thumbnail_xpath_result = eval_xpath_list(result, thumbnail_xpath)
                time=result.get("time", ""),
                timediff = datetime.strptime(entry['duration'], "%M:%S")
                timediff = datetime.strptime(video_duration.strip(), "%M:%S")
                timediff = datetime.strptime(video_length, "%M:%S")
                title += f" ({record['title'][0]})"
                title = extract_text(eval_xpath(r, './/a[@class="wikilink1"]/@title'))
                title = get_text_from_json(video.get('title', {}))
                title = item.text
                title = title.strip() + "\u2026"
                title = title_raw['word']
                title=cve_id,
                title=extract_text(eval_xpath(item, ".//a[contains(@class, 'link_tit')]")),
                title=extract_text(eval_xpath(item, "./h4")),
                title=extract_text(eval_xpath(item, title_xpath)),
                title=extract_text(_title) or "",
                title=f"{repo['full_name']} · {item['name']}",
                title=f'{result["name"]} - {result["filename"]}',
                title=item.get('name'),
                title=item["title"],
                title=pkgname,
                title=result.get("label", ""),
                title=result.get("title"),
                title=result["title"],
                title=result["title"]["text"],
                title=result["web"],
                title=result['citation_title'],
                title=title,
                tmp_result['cached_url'] = cached_url + extract_text(eval_xpath_list(result, cached_xpath, min_len=1))
                tmp_result['is_onion'] = True
                torrent_link = url
                try:
                type=item.get("type"),
                type=pub_type,
                type=result.get("documentType", "") or "",
                url = 'https' + url[4:]
                url = alternatePaperLinks[0].get("url")
                url = attribute.get_geo_url(attribute_result, osm_zoom=osm_zoom)
                url = base_youtube_url + videoid
                url = item.text
                url='https://grokipedia.com/page/' + item["slug"],
                url=base_url + (extract_text(eval_xpath(item, "./h4/a[2]/@href")) or ""),
                url=base_url + result["canonical_url"],
                url=eval_xpath(div_result, ".//h2/a/@href")[0],
                url=eval_xpath_getindex(doc, '//div[@id="zero_click_abstract"]/a/@href', 0),
                url=eval_xpath_getindex(item, ".//a[contains(@class, 'link_tit')]/@href", 0),
                url=f"{base_url}/project/{pkgname}/versions",
                url=f"{base_url}/{item['key']}",
                url=f"{base_url}{eval_xpath_getindex(item, url_xpath, 0)}",
                url=f"{route_url}/?point={point1}&point={point2}",
                url=f'https://nvd.nist.gov/vuln/detail/{cve_id}',
                url=f'{base_url}/app/{app_id}',
                url=item["html_url"],  # pyright: ignore[reportAny]
                url=item["url"],
                url=result["link"],
                url=result["url"],
                url=url,
                urls.append({"title": "OpenStreetMap", "url": _url, "entity": "P625"})
                url_id = 'wikimedia_image'
                v = str(v).strip()
                value = value[len(WDURLAttribute.HTTP_WIKIMEDIA_IMAGE) :]
                value_title="Value",
                version=latest_version,
                video_info = extract_video_data(video_block)
                volume='',
                volume=result['journal_volume'],
                volume=volume,
                website = wd_result.get('website', {}).get('value')
                wikidata_property_names.append("wd:" + attribute.name)
                wikipedia_name = wd_result.get('wikipediaName', {}).get('value')
                year=result['citation_year'],
                year=result['release_year'],
                [after, before] = highlight_groups[0]
                ],
                _url: str = get_earth_coordinates_url(latitude, longitude, osm_zoom)  # type: ignore
                {
                {'exists': {'field': 'documentation'}},
                {'role': 'assistant', 'content': cf_ai_model_assistant},
                {'role': 'system', 'content': cf_ai_model_system},
                {'role': 'user', 'content': params['query']},
                {'term': {'authorized': 1}},
                {'term': {'indexed': 1}},
                {'term': {'status': 'latest'}},
                {'url': url, 'title': title, 'iframe_src': iframe_src.format(audioid=result['id']), 'content': content}
                }
                },
              a particular language.
              be used to decode the XML result ('utf8').
              be used to interpret the query string ('utf8').
              originating in a particular country.
              WDURLAttribute('P4033', url_path_prefix='/@')
             COALESCE( NULLIF(url_video_hd,''), NULLIF(url_video_sd,''), url_video) AS url,
             description AS content
             schema:inLanguage "{language}" ;
             schema:isPartOf <https://{language}.wikipedia.org/> ;
             schema:name ?articleName{language} . }""".replace(
            " | ".join(map(str, item.get('content', [])))
            ",".join(SEARCH_TYPES.keys()),
            "adobe_stock engine: adobe_content_types must be a list of strings not %s" % type(adobe_content_types)
            "advancedSyntax": "true",
            "author": ", ".join(result["artist_titles"]),
            "content": "%(medium_display)s // %(dimensions)s" % result,
            "content": extract_text(content),
            "content": video_intro,
            "Content-Type": "application/json",
            "db": "pubmed",
            "engine_data": json["nextpage"],
            "fields": "id,title,artist_display,medium_display,image_id,date_display,dimensions,artist_titles",
            "getRankingInfo": "true",
            "hits": number_of_results,
            "hitsPerPage": results_per_page,
            "id": ",".join(pmids),
            "iframe_src": iframe_src,
            "iframe_src": _frontend_url() + '/embed' + result.get("url", ""),
            "ignorePlurals": "false",
            "img_src": image_api + "/%(image_id)s/full/843,/0/default.jpg" % result,
            "key": "nextpage",
            "leech": result["leechers"],
            "length": length,
            "limit": page_size,
            "magnet:?xt=urn:btih:" + result["info_hash"] + "&dn=" + result["name"] + "&tr=" + "&tr=".join(trackers)
            "magnetlink": magnetlink,
            "minProximity": 7,
            "minWordSizefor1Typo": 4,
            "minWordSizefor2Typos": 8,
            "numericFilters": '[]',
            "page": (params["pageno"] - 1),
            "page": params["pageno"],
            "page": params['pageno'],
            "pageSize": page_size,
            "publishedDate": parser.parse(time.ctime(uploaded / 1000)) if uploaded != -1 else None,
            "publishedDate": published_date,
            "q": query,
            "query": query,
            "queryType": "prefixLast",
            "restrictSearchableAttributes": '["title","comment_text","url","story_text","author"]',
            "retmode": "xml",
            "retstart": (params["pageno"] - 1) * number_of_results,
            "searchField": "ALL",
            "seed": result["seeders"],
            "sortDirection": "ASC",
            "sortOrder": "RELEVANCY",
            "tagFilters": '["story",[]]',
            "tags": "front_page",
            "template": "images.html",
            "template": "torrent.html",
            "term": query,
            "thumbnail": video_cover,
            "title": extract_text(title),
            "title": result.get("title", ""),
            "title": result["name"],
            "title": result["title"] + " (%(date_display)s) // %(artist_display)s" % result,
            "title": title,
            "typoTolerance": "true",
            "url": "https://artic.edu/artworks/%(id)s" % result,
            "url": extract_text(link),
            "url": link,
            "url": url,
            "url": _frontend_url() + result.get("url", ""),
            "views": humanize_number(result["views"]),
            "wc_search_type: %s isn't a valid file type (%s)",
            "X-S2-Client": "webapp-browser",
            "X-S2-UI-Version": get_ui_version(),
            #  * search on imdb.com, look at the URL of the thumbnail on the right side of the screen
            #  * search using the imdb engine, compare the imageUrl and thumbnail URL
            #  <span class="algoSlug_icon" # data-priority="2">Web</span>
            # 'oc' will be ignored.
            # ...
            # 280,414 : size of the image (add white border)
            # a gap in the end means that no data was found
            # add alternative cached url if available
            # add only if result is ascii (otherwise "normalization" didn't work)
            # add padding
            # add thumbnail if available
            # album only contains a single video
            # append result
            # better boundingbox calculation?
            # Bug on their end, time sometimes returns "</a>"
            # but 'ca' is already in the list of engine_traits.languages -->
            # continue if invalid osm-type
            # Cut off the hours that are already in the past
            # DDG recognizes this as a request from a bot. This lowers the
            # decode base64 encoded URL
            # Don"t try to call follow up pages without a vqd value.
            # e.g the "No item found..." message
            # expand title to add some sort of warning message
            # get the first value of u parameter
            # get thumbnail
            # html_url=html_url,
            # https://github.com/osm-search/Nominatim/issues/1662
            # if the value of shortDescription set, but is None, return empty string
            # ignore adds
            # ignore channels
            # ignore header columns "tr/th"
            # ignore randomly interspersed advertising adds
            # in case of sectiontitle create a link to the section in the wiki page
            # In my tests a video tag in the WEB search was most often not a
            # inconsistent
            # it's called bookmark by pinterest, but it's rather a nextpage
            # Korean alphabet
            # Make sure that the element is free of:
            # Malayalam is one of 22 scheduled languages of India.
            # measure.
            # network. Please try again later.
            # obsoleted entries don't have preview images
            # Occitanis not known by babel, its closest relative is Catalan
            # Our systems have detected unusual traffic from your computer
            # parameter to get the next results
            # print("%-20s: %s <-- %s" % (v["label"], country_tag, sxng_tag))
            # print("ERROR: %s -> %s is unknown by babel" % (x.get("data-name"), eng_lang))
            # print("ERROR: %s is unknown by babel" % (eng_lang))
            # print("ERROR: %s [%s] is unknown by babel" % (cols[0], eng_tag))
            # print(f"ERROR: lang - no iso code in {lang}")
            # print(f"ERROR: language tag {babel_lang} is unknown by babel")
            # QL75 : JPEG quality (?)
            # recipe to get the magic value:
            # remove "a1" in front
            # reputation of the SearXNG IP and DDG starts to activate CAPTCHAs.
            # set suspend time to zero is OK --> ddg does not block the IP
            # should not happen, disowned photo? Show it anyway
            # silently ignore unknown languages
            # since we can't construct a proper body from the response, we'll make up our own
            # skip non valid entries in the result table
            # some instances return a partial thumbnail url
            # Some locales (at least China) do not have a "next page" button and DDG
            # Sometime Quark will return non-standard format like "1天前", set published_date as None
            # Sometimes Bing will send back the first result page instead of the requested page as a rate limiting
            # ss_doc variant 1
            # ss_kv variant 1
            # ss_kv variant 1 & 2
            # ss_kv variant 2
            # ss_kv, ss_pic, ss_text, ss_video, baike, structure_web_novel use the same struct as ss_doc
            # the api url differs from the frontend, hence use piped.video as default
            # the language doesn't have any iso code, and hence can't be parsed
            # There are articles with hundreds of authors
            # These seem to be files published along with papers. Not something
            # this is a [ZITATION] block
            # traditional chinese used in ..
            # use first result as summary
            # use the DOI reference
            # UX280 : resize to width 320
            # video, except the ones from youtube ..
            # we check if the url is partial, and prepend the base_url if it is
            # will return a HTTP/2 403 Forbidden for a request of such a page.
            # you'd search for.
            '%LANGUAGE%', sparql_string_escape(user_language)
            'addition': parse_addition,
            'ai_page': parse_ai_page,
            'analyzer': 'camelcase',
            'attributes': result_chunks,
            'author': video_result['channel']['channel_name'],
            'Authorization': 'Basic '
            'baike': parse_ss_doc,
            'baike_sc': parse_baike_sc,
            'bookmarks': [params['engine_data'].get('bookmark', '')],
            'Connection': 'keep-alive',
            'content': ' | '.join(content),
            'content': '',
            'content': content,
            'content': html.unescape(post.get('blurb', '')),
            'content': html_to_text(item['description']),
            'content': html_to_text(video_result['description']),
            'content': item.get("short_description"),
            'content': item["text_in_grid"]["snippet"],
            'content': result['user']['name'],
            'content': result_content,
            'DNT': '1',
            'endpoint': '/v5/general/v1/search/image',
            'endpoint': '/v5/general/v1/search/video',
            'endpoint': 'https://image.baidu.com/search/acjson',
            'endpoint': 'https://kaifa.baidu.com/rest/v1/search',
            'endpoint': 'https://quark.sm.cn/s',
            'endpoint': 'https://vt.sm.cn/api/pic/list',
            'endpoint': 'https://www.baidu.com/s',
            'engine_data': json_resp['resource_response']['bookmark'],
            'fantizhengwen': 'zh_Hant',
            'fields': 'id,title,artist_display,medium_display,image_id,date_display,dimensions,artist_titles',
            'fields': ['documentation', 'documentation.*'],
            'filesize': extract_text(stats[1]),
            'finance_shuidi': parse_finance_shuidi,
            'from': (params["pageno"] - 1) * page_size,
            'future': True,
            'hangul': 'ko',
            'Host': 'tineye.com',
            'https://cdn.apple-mapkit.com/ma/bootstrap?apiVersion=2&mkjsVersion=5.72.53&poi=1',
            'iframe_src': iframe_src.format(url=r_url),
            'img_format': img_format,
            'img_src': img_src,
            'img_src': item["original_image"]["url"],
            'infobox': infobox_title,
            'key': 'bookmark',
            'kk_yidian_all': parse_kk_yidian_all,
            'language': params['searxng_locale'],
            'leech': extract_text(stats[2]),
            'length': length,
            'length': video_result['player']['duration_str'],
            'life_show_general_image': parse_life_show_general_image,
            'limit': nb_per_page,
            'magnetlink': magnet,
            'maintainer': item["publisher"].get("name"),
            'malayam': 'ml',
            'med_struct': parse_med_struct,
            'messages': [
            'metadata': ' | '.join(metadata),
            'metadata': ', '.join(item for item in metadata if item),
            'metadata': extract_text(categ),
            'metadata': item.get('source'),
            'music_new_song': parse_music_new_song,
            'must': [
            'nature_result': parse_nature_result,
            'news_uchq': parse_news_uchq,
            'norsk': 'nb',
            'offset': (params['pageno'] - 1) * 10,
            'order': 'beste',
            'order': 'desc',
            'p': params['pageno'],
            'package_name': item.get("name"),
            'page': params['pageno'] - 1,
            'page': params['pageno'],
            'pagesize': pagesize,
            'params': {
            'params': {'start_index': (params["pageno"] - 1) * results_per_page, 'rn': results_per_page},
            'popularity': ', '.join(popularity_infos),
            'publishedDate': parse(video_result['published']),
            'publishedDate': parser.parse(item.get("updated_at") or item.get("created_at")),
            'publishedDate': parser.parse(item['CreatedAt']),
            'publishedDate': publishedDate,
            'q': query,
            'queries': [
            'query': query,
            'resolution': f'{item["original_image"]["width"]} x {item["original_image"]["height"]}',
            'resolution': resolution,
            'result': 'this is what you get',
            'seed': extract_text(stats[3]),
            'selectedRegion': region,
            'sinhalese': 'si',
            'site': api_site,
            'size': 10,
            'size': page_size,
            'sort': api_sort,
            'sortBy': 'timestamp',
            'sortOrder': 'desc',
            'source': item["result"]["site_title"],
            'source': source,
            'ss_doc': parse_ss_doc,
            'ss_kv': parse_ss_doc,
            'ss_note': parse_ss_note,
            'ss_pic': parse_ss_doc,
            'ss_text': parse_ss_doc,
            'ss_video': parse_ss_doc,
            'structure_web_novel': parse_ss_doc,
            'tags': architectures,
            'TE': 'trailers',
            'template': "torrent.html",
            'template': 'images.html',
            'template': 'packages.html',
            'template': 'products.html',
            'template': 'videos.html',
            'thumbnail': extract_text(eval_xpath(result, ".//img[contains(@class, 'listview__image')]/@src")),
            'thumbnail': f'{absolute_url(video_result["vid_thumb_url"])}?auth={ta_token}',
            'thumbnail': item["logo_url"].get("large") or item["logo_url"].get("small"),
            'thumbnail': item['image'],
            'thumbnail': result['pictures']['medium'],
            'thumbnail': thumbnail,
            'thumbnail_src': item["thumbnail"]["url"],
            'thumbnail_src': thumbnail_src,
            'title': 'Wolfram|Alpha (' + infobox_title + ')',
            'title': extract_text(eval_xpath(result, ".//h3[contains(@class, 'listview__name')]")),
            'title': extract_text(title),
            'title': html.unescape(result['title']),
            'title': item.get("name"),
            'title': item["result"]["page_title"],
            'title': item['Name'],
            'title': result['name'],
            'title': title,
            'title': video_result['title'],
            'title': _fix_title(item['title'], item['link']),
            'torrentfile': torrentfile,
            'travel_dest_overview': parse_travel_dest_overview,
            'travel_ranking_list': parse_travel_ranking_list,
            'type': 'most_fields',
            'type': 'suggest',
            'upstream': {'topics': result},
            'url': (
            'url': base_url + ("/_/" if is_official else "/r/") + item.get("slug", ""),
            'url': img_src,
            'url': item["result"]["referrer_url"],
            'url': item['link'],
            'url': resp.request.headers['Referer'],
            'url': resp.search_params['base_url'] + url,
            'url': r_url,
            'url': url,
            'urls': [{'title': 'Wolfram|Alpha', 'url': resp.request.headers['Referer']}],
            'value': row.get("value"),
            'views': humanize_number(video_result['stats']['view_count']),
            '{language}', self.language
            '{name}', self.name
            (
            (gettext('bitrate'), 'bitrate'),
            (gettext('clicks'), 'clickcount'),
            (gettext('votes'), 'votes'),
            )
            ),
            ).format(
            ).timestamp()
            ):
            + "/"
            + "px-"
            + ' - '
            + base64.b64encode("{}:{}".format(api_client_id, api_client_secret).encode()).decode()
            + extract_text(app.xpath('./div/div/span[@class="package-license"]')).strip()
            + img_src_name_first
            + img_src_name_md5[0:2]
            + img_src_name_md5[0]
            + img_src_name_second
            + img_src_size
            - account: ``libreoffice``
            - domain: ``fosstodon.org``
            - result url: https://fosstodon.org/@libreoffice
            - ``Accept: '*/*``
            - ``cr`` parameter: restricts search results to documents
            - ``hl`` parameter: specifies the interface language of user interface.
            - ``ie`` parameter: sets the character encoding scheme that should
            - ``lr`` parameter: restricts search results to documents written in
            - ``oe`` parameter: sets the character encoding scheme that should
            .get('appendContinuationItemsAction')['continuationItems'][1]
            .get('continuationCommand')['token']
            .get('continuationItemRenderer')['continuationEndpoint']
            .replace("'", "")
            .replace("custom-thumb", "img-master")
            .replace("_custom1200.jpg", "_master1200.jpg")
            .replace("_square1200.jpg", "_master1200.jpg")
            .replace('{0}', format_time(timestamp, 'full', tzinfo=None, locale=locale))
            .replace('{1}', format_date(timestamp, 'short', locale=locale))
            :py:func:`urllib.parse.urlencode`).
            A instance of :py:obj:`babel.core.Locale` build from the
            address = None
            address = {'name': properties.get('name')}
            address.update(
            address_name = address_raw.get('address29')
            address_name = address_raw.get(result['category'])
            Adds Property `P4033 <https://www.wikidata.org/wiki/Property:P4033>`_
            album_id = result['albums'][0]['id']
            alternatePaperLinks = result.get("alternatePaperLinks")
            amount = float(area.get("amount", ""))
            answer="this is a dummy answer ..",
            api_result = resp.json()
            architectures.extend(arch['name'] for arch in rate_plan.get("architectures", []) if arch['name'])
            args["offset"] = params.get("pageno", 1) - 1
            args["tf"] = time_range_map.get(params["time_range"])
            args['language'] = lang
            args['nextpage'] = nextpage
            attributes.append({'label': label, 'value': value})
            attribute_type = type(attribute)
            author = result['item']['creators'][0]['title']
            author=channel_result['channel_name'],
            authors = authors[:15] + ["et al."]
            authors.append(display_name)
            authors=authors,
            author_name = f"{f.text if f is not None else ''} {l.text if l is not None else ''}".strip()
            backlinks.append(
            base_url.format(language=resp.search_params['language']) + 'wiki/' + quote(title.replace(' ', '_').encode())
            bing_ui_lang = bing_ui_lang + '-' + bing_ui_lang_map.get(bing_ui_lang, bing_ui_lang)
            boundingbox = [
            boundingbox = [box['southLat'], box['northLat'], box['westLng'], box['eastLng']]
            box = result['displayMapRegion']
            break
            buf = leftover + line.decode('utf-8')
            buffer.append(letter)
            CACHE.set(key="guest_client_id", value=guest_client_id)
            catalog_engine2code[unaccented_name] = lang_code
            category = category.attrib.get('title')
            category_config['news']['params']["cate"] = chinaso_news_source
            category_config['news']['params']["type"] = 'EPAPER'
            client_id = cids.groups()[0]
            cmd.append(c)
            cmd.extend(params)
            code = code.lstrip("\n")
            code = code.lstrip()
            code = code.rstrip("\n")
            code = code.rstrip()
            code.
            comments = gettext(
            comments=" / ".join(_list("pubnote")),
            comments=field("publicationName"),
            conflict = engine_traits.regions.get(sxng_tag)
            content += "(%s) " % entry['rank']
            content += ' // is answered'
            content += entry['s']
            content += str(entry['y']) + " - "
            content = "%s / %s" % (x, content)
            content = ' '.join(x['text'] for x in section['descriptionSnippet']['runs'])
            content = '{msg_superseded}: {url} ({pdb_id})'.format(
            content = '{} - {} - {}'.format(result['artist']['name'], result['album']['title'], result['title'])
            content = '{} - {} - {}'.format(result['artists'][0]['name'], result['album']['name'], result['name'])
            content = content.format(
            content = content[:300] + '...'
            content = data['selftext']
            content = extract_text(content_nodes)
            content = extract_text(eval_xpath_list(result, content_xpath))
            content = extract_text(item.xpath('.//div[contains(@class, "fz-mid space-txt")]'))
            content = extract_text(item.xpath('.//p[contains(@class, "txt-info")]'))
            content = extract_text(item.xpath('.//span[@class="res-list-summary"]'))
            content = extract_text(_content)  # type: ignore
            content = f"{views} views - {rumbles} rumbles - ${earned}"
            content = f"{views} views - {rumbles} rumbles"
            content = markdown_to_text(content)
            content = result['highlights'][0]['value']
            content = subpod.xpath(plaintext_xpath)[0].text
            content = [
            content.append("Encompassing work: " + result['encompassingWork'])
            content.append("Themes: " + result['themes'])
            content.append(f"{extract_text(eval_xpath(spec, './dt'))}: {extract_text(eval_xpath(spec, './dd'))}")
            content.append(f'category - {category}')
            content.append(f'tags - {tags}')
            content.append(tags)
            content.insert(0, recipe['subtitle'])
            content=content,
            content=extract_text(content),
            content=extract_text(eval_xpath(result, "./div[@class='content']/div[@class='text']")),
            content=field("abstract"),
            content=html_to_text(" | ".join(contents)),
            content=html_to_text(channel_result['channel_description']),
            content=html_to_text(_str("abstract")),
            content=result["description"],
            contents.append(f"Description: {entry['description']}")
            contents.append(f"Downloads: {entry['downloads']:,}")
            contents.append(f"Likes: {entry['likes']}")
            contents.append(f"Tags: {', '.join(entry['tags'])}")
            content_between_tags = extr(html_sample, '{"location":"/images/search/', 'false}}}')
            content_list = [result.get(x) for x in ['abstract', 'summary']]
            content_nodes = eval_xpath(result, './/div[contains(@data-sncf, "1")]')
            content_parts.append(f"Duration: {minutes} min")
            continue
            continue  # ignore anime results that which aren't actually torrents
            converting IDs to full URLs.
            coordinates = None
            countrycode = params['searxng_locale'].split('-')[-1].upper()
            counts[item] = counts.get(item, 0) + 1
            cover = f"https://archive.org/services/img/{item['lending_identifier_s']}"
            crawl_date = backlink_json.get("crawl_date")
            created = datetime.fromtimestamp(data['created_utc'])
            cur.execute(query_to_run, query_params)
            custom_query[key] = custom_query.pop(query_key)
            custom_query[query_key] = value
            data = json.loads(match)
            data.append(
            data.get('summaryProps', {}).get('content')
            data.get('titleProps', {}).get('content')
            data["vqd"] = vqd
            data_image = data_image[: end_pos + 1]
            day = eval_xpath_getindex(accepted_date, "./Day", 0)
            def_abbr = extract_text(def_item.xpath('.//abbr')).strip()
            def_text = extract_text(def_item).strip()
            delta = (now if end_time is None else end_time) - start_time
            delta = AGO_TIMEDELTA[ago.group(2)]
            description = suggestions.get('description')
            discovery_filters.append(query_part)
            div_1, div_2 = eval_xpath(item, './div')[:2]
            doi=field("DOI"),
            doi=_list("doi")[0],
            domain = domain.strip()
            duration = item.get('duration')
            duration = None
            duration = timedelta(seconds=duration)
            duration=result["video"]["duration"],
            elif 'images' in results_categ['display_type']:
            elif 'img' in subpod:
            elif 'link' in result:
            elif attribute_type == WDGeoAttribute:
            elif attribute_type == WDImageAttribute:
            elif image:
            elif is_iterable(value):
            elif item.attrib["name"] == "dcdescription":
            elif item.attrib["name"] == "dclink":
            elif item.attrib["name"] == "dctitle":
            elif item.startswith('Comment:'):
            elif item.startswith('Date:'):
            elif item["format"] == "pdf":
            elif mainline_type == 'images':
            elif mainline_type == 'news':
            elif mainline_type == 'videos':
            elif r.tag == "dd":
            elif results_categ['display_type'] == 'news-bing':
            elif s_key == 'Download Error':
            elif s_key == 'NO_SIGNATURE_ERROR':
            else str(item.get('content'))
            else:
            embedded = url
            embed_url = get_embeded_stream_url(f"https://www.youtube.com/watch?v={video_id}")
            embed_url = get_embeded_stream_url(url)
            encoded_url = encoded_url + '=' * (-len(encoded_url) % 4)
            encoded_url = param_u[2:]
            endpositions.append(endpos)
            end_time = datetime.fromtimestamp(result.get("endTimeStamp") / 1000)
            engine_settings.get("wc_search_type"),
            engine_traits.all_locale = "all"
            engine_traits.all_locale = 'wt-wt'
            engine_traits.all_locale = 'ZZ'
            engine_traits.all_locale = cc_tag
            engine_traits.custom["content"].append(x.get("value"))
            engine_traits.custom["ext"].append(x.get("value"))
            engine_traits.custom['supported_domains'][region] = 'www' + domain  # type: ignore
            engine_traits.custom['title'][sxng_tag] = title  # type: ignore
            engine_traits.custom['WIKIPEDIA_LANGUAGES'].append(eng_tag)
            engine_traits.custom['wiki_netloc'][sxng_tag] = netloc
            engine_traits.languages[locale.language] = code
            engine_traits.languages[sxng_tag] = eng_tag
            engine_traits.regions[region_tag(locale)] = code
            engine_traits.regions[region_tag(sxng_locale)] = eng_country
            engine_traits.regions[sxng_tag] = country_tag
            engine_traits.regions[sxng_tag] = eng_tag
            engine_traits.regions[sxng_tag] = market_code
            eng_tag_list.add(_locale)
            eng_territory, eng_lang = eng_tag.split('-')
            eval_xpath(
            eval_xpath(result, content_xpath)
            eval_xpath_getindex(entry, './h3/span[@class="package-snippet__created"]/time/@datetime', 0)
            eval_xpath_getindex(item, ".//a[starts-with(@href, '/search')]", 1, default=None), allow_none=True
            eval_xpath_getindex(result, './/div[contains(@class, "gqF9jc")]', 0, default=None), allow_none=True
            eval_xpath_getindex(result, './/div[contains(@class, "ITZIwc")]', 0, default=None), allow_none=True
            eval_xpath_getindex(result, './/h3[contains(@class, "LC20lb")]', 0, default=None), allow_none=True
            eval_xpath_getindex(result, './/span[contains(@class, "k1U36b")]', 0, default=None), allow_none=True
            except (ValueError, AttributeError):
            except (ValueError, TypeError):
            except babel.UnknownLocaleError:
            except Exception:  # pylint: disable=broad-except
            except OverflowError:
            except parser.ParserError:
            except ValueError:
            except:  # pylint: disable=bare-except
            extra.append(', '.join(defn_raw['info']).capitalize() + '. ')
            extra.append('Only applies to: ' + ', '.join(defn_raw['restrictions']) + '. ')
            extract_text(app.xpath('./div/div/span[@class="package-summary"]')).strip()
            extract_text(eval_xpath(result, ".//div[@class='gs_a']"))
            extract_text(eval_xpath(result, ".//div[@class='gs_fl']/a[starts-with(@href,'/scholar?cites=')]")) or ""
            extract_text(eval_xpath(result, ".//div[contains(@class, 'listview__offercount')]")),
            extract_text(eval_xpath(result, ".//div[contains(@class, 'stars-rating-label')]")),
            extra_data = data.get('extraData', {})
            f = eval_xpath_getindex(author, "./ForeName", 0, default=None)
            f" presearch_session=;"
            f" use_local_search_results=false;"
            f" use_safe_search={safesearch_map[params['safesearch']]}"
            f" | {gettext('active users')}: {counts.get('users_active_half_year', 0)}"
            f" | {gettext('comments')}: {result['counts']['comments']}"
            f" | {gettext('community')}: {result['community']['title']}"
            f" | {gettext('posts')}: {counts.get('posts', 0)}"
            f" | {gettext('user')}: {user}"
            f"&#x25B2; {result['counts']['upvotes']} &#x25BC; {result['counts']['downvotes']}"
            f"Anzahl der Zutaten: {recipe['ingredientCount']}",
            f"b=1;"
            f"Schwierigkeitsstufe (1-3): {recipe['difficulty']}",
            f"Zubereitungszeit: {recipe['preparationTime']}min",
            f"{gettext('subscribers')}: {counts.get('subscribers', 0)}"
            files = int(files)
            files = None
            filesize = item_data['viewerData']['dups'][0]['fileSizeInBytes']
            filters.append(f"language:{iso2}")
            firstURL = ddg_result.get("FirstURL")
            first_result = False
            for data in result["content"]["data"]:
            for e in p.xpath('.//span[@class="algoSlug_icon"]'):
            for example in word.get('examples_target', []):
            for info in infobox.get("content", {}):
            for info in [
            for item in (
            for item in content_nodes:
            for item in definition.get('list', []):
            for lang in languages.get_official_languages(code, de_facto=True):
            for prefix in ("category", "tag"):
            for raw_result in raw_results:
            for result in wd_to_results.get(wd_id, []):
            for tag in result["altnames"] + result["tags"]:
            for tag in tags:
            for topic_result in _topic_results:
            for url, title, content in zip(
            for url, title, content, cached in zip(
            for video in album_info["videos"]:
            for video_block in video_blocks:
            for x in [
            forecast_data = _weather_data(geoloc, forecast)
            forecast_data.datetime = weather.DateTime(date.replace(hour=index * time_slot_len + 1))
            format_method = getattr(self, date_format[0])
            found = regex.search(raw_result)
            fullsize_image = item_data['viewerData']['dups'][0]['url']
            func = parse.get(hit['type'])
            gallery_url, (eval_xpath_getindex(link, './/img', 0).attrib['src']).replace(base_url, '')
            get_datetime_format(format, locale=locale)
            Google subdomain :py:obj:`google_domains` that fits to the country
            google_info['params']['hl'] = ceid_lang
            google_info['params']['hl'] = ceid_lang + '-' + ceid_region
            google_info['params']['hl'] = ceid_lang + '-' + ceid_suffix
            headers={'Authorization': 'Bearer ' + token_response.text},
            headers['Accept-Language'] = f"{l.language}-{l.territory},{l.language};" "q=0.9,*;" "q=0.5"
            height = item.get("height")
            height = item_data['viewerData']['dups'][0]['h']
            highlighted_lines_index: set[int] = set()
            hourly_data[key] = json_data["hourly"][key][index]
            html_sample, '{"location":"/images/search/', 'advRsyaSearchColumn":null}}', default="fail"
            html_url = doc_url
            humanized_filesize = humanize_bytes(filesize)
            if ':' in text:
            if 'dot_text' in item:
            if 'magnet' in url:
            if 'pubDate' in result:
            if 'url' in result:
            if 'word' not in title_raw:
            if (
            if alternatePaperLinks:
            if articles < 10000:
            if attribute.name not in WIKIDATA_PROPERTIES:
            if attribute_type in (WDURLAttribute, WDArticle):
            if author_name:
            if babel_tag == 'lang_region':
            if cached_xpath:
            if conflict != bing_ui_lang:
            if conflict != eng_lang:
            if conflict != eng_tag:
            if conflict != lang_tag:
            if conflict != ui_lang:
            if conflict:
            if content and pod_id not in image_pods:
            if coordinates:
            if countrycode in traits.custom['countrycodes']:  # type: ignore
            if crawl_date:
            if defn_raw.get('info'):
            if def_abbr:
            if doc["linkType"] not in ("crawler", "doi"):
            if duration:
            if expected_start > result_len:
            if func:
            if i == 0:
            if iframe_src:
            if image_size in photo['sizes']['data']:
            if img_date:
            if img_id and img_id in data_image_map:
            if info
            if info and info[-1] != " ":
            if info_item.get("width") and info_item.get("height"):
            if int(depth) < 20:
            if isinstance(description, list):
            if isinstance(item.get('content'), list)
            if is_onion:
            if item.attrib["name"] == "dcdate":
            if item.startswith('Size:'):
            if item:
            if item["format"] == "html":
            if item["platform"] != "web":
            if item[format_name] == 'Yes':
            if key == qkey:
            if l.territory:
            if l:
            if lang_tag not in engine_traits.languages.keys():
            if len(content) > 500:
            if len(highlight_groups) > 0:
            if len(title) > 50:
            if length.tm_hour:
            if length:
            if letter == "\n":
            if magnet.startswith('magnet'):
            if mainline_type == 'web':
            if match:
            if name:
            if next_page_token:
            if not content:
            if not domain or domain in [
            if not found:
            if not image_url_name.endswith('_V1_'):
            if not info_item.get('mediakey'):
            if not isinstance(backlink_json, dict):
            if not is_broken_text(text):
            if not is_iterable(value):
            if not item.text:
            if not item:
            if not raw_html:
            if not thumbnail.startswith("https://www.bing.com"):
            if not title:
            if not url:
            if not videoid:
            if not video_blocks:
            if o:
            if param not in query_enum:
            if parsers:
            if part in icon_name:
            if part in result["name"] or part in result["tags"] or part in result["categories"]:
            if part in result["name"]:
            if published_dates:
            if pull_count:
            if qwant_categ == 'news' and _locale.lower() not in qwant_news_locales:
            if r.tag == "dt":
            if raw_results[-1]:
            if raw_url is None:
            if real_unit is None:
            if record["title"][0].lower().strip() != title.lower().strip():
            if rec['mime_type'].startswith('video'):
            if region == 'HK':
            if region not in (code, name):
            if repo.get("status") == "newest":
            if result.get('image'):
            if results_categ['display_type'] == 'web-google':
            if result['type'] != 'Organic':
            if result_type == "MainResult":
            if smpl:
            if subpod['plaintext'] != '' and pod_id not in image_pods:
            if s_key == "Invalid image URL":
            if t not in ADOBE_VALID_TYPES:
            if thumb:
            if thumbnail and not urlparse(thumbnail).netloc:
            if thumbnail:
            if thumbnail[0] == '/':
            if thumbnail_xpath:
            if time != "":
            if title_tag is None:
            if url.startswith('http://'):
            if v:
            if value.startswith(WDURLAttribute.HTTP_WIKIMEDIA_IMAGE):
            if videoid is not None:
            if word.get('definition'):
            if x
            if x[1] not in ['Hant', 'Hans']:
            if year < 0:
            if _pub_date:
            iframe_src = get_embeded_stream_url(url)
            image = subpod.xpath(image_xpath)
            image_url = image_url_name + magic + '.' + image_url_prefix
            image_url_name, image_url_prefix = image_url.rsplit('.', 1)
            img = eval_xpath(div_1, './/img')[0]
            img_date = item.get("bdImgnewsDate")
            img_id = eval_xpath_getindex(result, './/img/@id', 0, default=None)
            img_results.append(params)
            img_src = f"{cdn_base_url}/icons/{result['name']}/{result['name']}-{image_type}.svg"
            img_src = img_src.split(' ')[0]
            img_src = parsed_url._replace(path=parsed_url.path.split('/v1')[0]).geturl()
            img_src = photo['url_o']
            img_src = photo['url_z']
            img_src = result['wikidata']['image_sign']
            img_src = result['wikidata']['image_symbol']
            img_src = thumbnail_src.replace('240.jpg', '640.jpg')
            img_src=result["properties"]["url"],
            img_src_name_second = img_src_name + ".png"
            index[7]
            info
            info = extract_text(eval_xpath(result, xpath))
            infobox_content.append('</ul><small>Wikipedia, CC BY-SA 3.0.</small><ul>')
            infobox_title = pod['subpods'][0]['plaintext']
            info_item = loads(info_js)
            initial_data = data.get('data', {}).get('initialData', {})
            isbn=[x for x in [field("isbn")] if x],
            isbn=_list("isbn"),
            issn=[x for x in [field("issn")] if x],
            issn=_list("issn"),
            item = item.strip()
            item = _strip_leading_strings(html_to_text(item))
            item = {
            item.definitions.append(def_text)
            item.publishedDate = datetime(*(record["published"]["date-parts"][0] + [1, 1][:3]))
            item.url = record["resource"]["primary"]["URL"]
            item["content"] = result.get("shortDescription", "") or ""
            item["content"] = result.get("uploaderName", "") or ""
            item["iframe_src"] = iframe_src
            item["length"] = datetime.timedelta(seconds=length)
            item["price"] = f"Bestes Angebot: {best_price[1]}€"
            item["template"] = "default.html"
            item["template"] = "videos.html"
            item["thumbnail"] = result.get("thumbnail", "")
            item["thumbnail"] = result["thumbnail"]["src"]
            item['content'] = ', '.join([result['class'], result['info'], result['more']])
            item['content'] = current['content_txt'][0]
            item['content'] = html_to_text(' | '.join([x for x in content_list if x]))
            item['iframe_src'] = iframe_src.format(video_id=res['id'])
            item['metadata'] = html_to_text(result.get('meta_short', ''))
            item['publishedDate'] = datetime.strptime(timestamp, timestamp_format)
            item['publishedDate'] = pub_date
            item['thumbnail'] = result['image']
            item['url'] = base_url + result['path']
            item['url'] = result['url']
            journal = ""
            journal = record.get("container-title", [None])[0] if "title" in record else ""
            journal=field("publicationName"),
            journal=journal,
            journals = [j.get("title") for j in result["journals"] if j.get("title")]
            json_data = '{"location":"/images/search/' + content_between_tags + 'false}}}'
            json_data = json.loads(match)
            json_str = line
            kvmap = dict(zip(col_names, map(str, row)))
            kvmap = dict(zip(cur.column_names, map(str, row)))
            kvmap = dict(zip(titles, map(str, row)))
            kvmap["metadata"] = {"index": result["_index"], "id": result["_id"], "score": result["_score"]}
            kwargs: dict[str, t.Any] = _get_result(item, resp.search_params["base_url"])
            l = babel.Locale.parse(ui_lang, sep="-")
            l = eval_xpath_getindex(author, "./LastName", 0, default=None)
            l = package.get(k)
            l, r = babel_region_tag.split('-')
            lang_tag = lang_map.get(lang_tag, lang_tag)
            lang_tag = lang_tag.split('_')[0]  # zh_Hant --> zh
            latest_version = _get_most_common([repo.get("version") for repo in repositories])
            length = int(result.get("duration", 0) / 1000)
            length = parse_duration_string(extract_text(eval_xpath(item, ".//span[contains(@class, 'time')]")))
            length = time.gmtime(result.get("lengthSeconds"))
            length = time.strftime("%H:%M:%S", length)
            length = time.strftime("%M:%S", length)
            length=result["video"]["duration"],
            lic_url = f"https://spdx.org/licenses/{lic.get('spdx_id')}.html"
            line = process.stdout.readline()
            lines.append("...")
            lines[int(line)] = code
            link = result['external_urls']['spotify']
            links.append(
            locale = babel.Locale.parse(lang_map.get(eng_lang, eng_lang), sep="-")
            locale = babel.Locale.parse(lang_map.get(eng_lang, eng_lang), sep='-')
            locale = babel.Locale.parse(sxng_locale, sep='-')
            locale = language_name_locale_map[eng_lang.lower()]
            locale = Locale(code)
            logger.debug("ignore exception (publishedDate): %s", e)
            logger.debug("skip legend enty %s : %s", x, index)
            logger.debug("vqd value from duckduckgo.com request: '%s'", value)
            logger.debug('cannot find valid image size: {0}'.format(repr(photo['sizes']['data'])))
            logger.debug('The SPARQL request returns duplicate entities: %s', str(attribute_result))
            logger.error("init: app_js GET %s failed", url)
            logger.error("no handle for %s --> %s", item["asset_type"], item)
            logger.error("unknown result type: %s", item_type)
            logger.error("vqd: can't parse value from ddg response (return empty string)")
            logger.error('skip unknown category tag %s in %s', entry_id[:2], entry_id)
            logger.error(e, exc_info=True)
            magic = 'QL75_UX280_CR0,0,280,414_'
            magnet = links[0].attrib.get('href')
            MainResult(
            market_code = f"{lang_tag}-{cc_tag}"  # zh-tw
            market_code = map_market_codes.get(market_code, market_code)
            match = re.search(r"timeConvert\('(\d+)'\)", timestamp)
            match for match in text_matches if match["object_type"] == "FileContent" and match["property"] == "content"
            max_index = max(max_index, pos)
            message="get_sc_code: got redirected to https://www.startpage.com/sp/captcha",
            message="get_sc_code: [PR-695] querying new sc timestamp failed! (%s)" % resp.url,
            metadata += f" | {authors}"
            metadata = extract_text(eval_xpath(div_2, './div[@class]'))
            metadata = f"Severity: {severity} | CVSS Score: {cvss_score}"
            metadata = f"{gettext('points')}: {points}" f" | {gettext('comments')}: {num_comments}"
            metadata = [source]
            metadata.append(f'{codec} ' + gettext('radio'))
            metadata.append(f'{gettext("comments")}: {comments}')
            metadata.append(gettext("answered"))
            metadata.append(status)
            missing.append(cfg_name)
            missing_opts.append(opt)
            month = eval_xpath_getindex(accepted_date, "./Month", 0)
            msg = f"Expected results to start at {expected_start}, but got results starting at {start}"
            msg_superseded = gettext("This entry has been superseded by")
            mtype, subtype = (mtype.split("/", 1) + [""])[:2]
            n.getparent().remove(n)
            name: str | None = i.get("name")
            new_result["iframe_src"] = iframe_src.format(type='album', result_id=result_id)
            new_result["iframe_src"] = iframe_src.format(type='track', result_id=result_id)
            new_result["publishedDate"] = dateparse(date.replace("released ", ""))
            new_result['thumbnail'] = thumbnail[0]
            next_page_token = (
            None,
            number = int(ago.group(1))
            number=field("number"),
            o = CURRENCIES.iso4217_to_name(currency[1], lang)
            or data.get('desc')
            or data.get('message', {}).get('replyContent')
            or data.get('show_body')
            or data.get('title')
            or properties.get('osm_key') == 'leisure'
            or properties.get('osm_key') == 'shop'
            or properties.get('osm_key') == 'tourism'
            osm_type = 'node'
            osm_type = 'relation'
            osm_type = 'way'
            osm_zoom = 17
            pages=",".join(_list("page")),
            pages="-".join([x for x in [field("startingPage"), field("endingPage")] if x]),
            pages=field("page"),
            params=resp.search_params,
            params["url"] += "&range={lrange}".format(lrange=lang[1])
            params["url"] += f"&filters[]={discovery_filter}"
            params["url"] = None
            params['content'] = content
            params['img_src'] = data['url']
            params['leech'] = int_or_zero(extract_text(stats[1]))
            params['publishedDate'] = created
            params['publishedDate'] = datetime.fromtimestamp(float(result["added"]))
            params['publishedDate'] = datetime.strptime(extract_text(stats[4]), '%b %d, %Y')
            params['seed'] = int_or_zero(extract_text(stats[0]))
            params['template'] = 'images.html'
            params['thumbnail_src'] = thumbnail
            params['url'] += time_range_url.format(time_range=time_range_dict[params['time_range']])
            params['url'] = params['url'] + "&lang=" + language
            param_u = parsed_url_query["u"][0]
            parsed_url = urllib.parse.urlparse(img_src)
            parsed_url = urlparse(url)
            parsed_url_query = parse_qs(url_query)
            parsers = source_category_parsers.get(source_category)
            pass
            path = "/nextpage/search"
            pdf_url = doc_url
            pdf_url=pdf_url,
            position_to_token[pos] = token
            precision = date_format[1]
            print("ERROR: %s (%s) -> %s is unknown by babel" % (name, eng_tag, region))
            print("ERROR: %s -> %s is unknown by babel" % (ceid, sxng_locale))
            print("ERROR: %s is unknown by babel" % eng_tag)
            print("ERROR: %s is unknown by babel" % lang_tag)
            print("ERROR: can't determine babel locale of Brave's (UI) language %s" % ui_lang)
            print("ERROR: can't determine babel locale of quant's locale %s" % eng_tag)
            print("ERROR: can't map from google country %s (%s) to a babel region." % (x.get('data-name'), eng_country))
            print("ERROR: item unknown --> %s" % item)
            print("ERROR: language %s (%s) is unknown by babel" % (name, eng_lang))
            print("ERROR: language (%s) is unknown by babel" % (babel_lang))
            print("INFO:  google UI language %s (%s) is unknown by babel" % (eng_lang, x.text.split("(")[0].strip()))
            print(f"ERROR: language name of startpage's language {lang_code} is unknown by babel")
            print(f"ERROR: region tag {region['iso_3166_1']} is unknown by babel")
            properties.get('osm_key') == 'amenity'
            proxy_image_url.replace("/c/250x250_80_a2/", "/")
            pubdate_original = dateutil.parser.parse(pubdate_original)
            published = _parse_date(str(item.get("first_publish_year")))
            publishedDate = datetime.fromtimestamp(result['extension']['publishingDate'])
            publishedDate = datetime.strptime(result["pubDate"], "%Y-%m-%d")
            publishedDate = datetime.strptime(result['recipe']['submissionDate'][:19], "%Y-%m-%dT%H:%M:%S")
            publishedDate = None
            publishedDate = parser.parse(item['date'])
            publishedDate = parser.parse(publishedDate)
            publishedDate = parser.parse(time.ctime(result.get("published", 0)))
            publishedDate=datetime.fromisoformat(_str("date")),
            publishedDate=published,
            publishedDate=published_date,
            publishedDate=pub_date,
            publishedDate=_extract_published_date(result["age"]),
            published_date = datetime.fromtimestamp(int(item.get("publish_time")))
            published_date = datetime.fromtimestamp(int(item.get('source', {}).get('time')))
            published_date = datetime.fromtimestamp(timestamp)
            published_date = datetime.strptime(
            published_date = datetime.strptime(entry["createdAt"], "%Y-%m-%dT%H:%M:%S.%fZ")
            published_date = datetime.strptime(item.get('time'), "%Y-%m-%d")
            published_date = datetime.strptime(release_time, "%Y-%m-%d")
            published_date = dateutil.parser.parse(date_string, dayfirst=True)
            published_date = None
            published_dates = [date for date in map(_parse_date, published) if date]
            publisher = ""
            publisher=field("publisher"),
            publisher=_str("pub") + " " + _str("year"),
            pubmed_data, "./History//PubMedPubDate[@PubStatus='accepted']", 0, default=None
            pub_date = datetime.now() - delta * number
            pub_type = pub_type[1:-1].lower()
            pull_count = rate_plan.get("repositories", [{}])[0].get("pull_count")
            Py-Dictionary with additional HTTP headers (can be passed to
            Py-Dictionary with additional request arguments (can be passed to
            p_text: str = extract_text(p_item)  # type: ignore
            query=q,
            query_params["gpc"] = f"stf={past},{now}|stftype=1"
            query_params["numericFilters"] = f"created_at_i>{timestamp}"
            query_params["paramList"] += f",timestamp_range={past}-{now}"
            r = r.split('_')[-1]
            raise RuntimeError("Can't determine Semantic Scholar UI version")
            raise RuntimeError("Response from https://www.google.com/supported_domains is not OK.")
            raise RuntimeError('non-zero return code when running command', cmd, return_code)
            raise SearxEngineAccessDeniedException(
            raise SearxEngineAccessDeniedException()
            raise SearxEngineAPIException("failed to obtain secret key")
            raise SearxEngineAPIException(msg)
            raise SearxEngineCaptchaException(
            raise SearxEngineCaptchaException()
            raise SearxEngineTooManyRequestsException()
            raise ValueError
            raise ValueError('requested path is outside of configured working directory')
            raise ValueError(f"Invalid duckduckgo category: {ddg_category}")
            raise ValueError(f"Unsupported mastodon type: {mastodon_type}")
            raise ValueError(f"Unsupported yep search type: {search_type}")
            raw_html = json_data.get("html", "")
            raw_results = buf.split(result_separator)
            raw_results = raw_results[:-1]
            raw_url = eval_xpath_getindex(result, './/a/@href', 0, None)
            real_unit = WIKIDATA_UNITS.get(wikidata_entity)
            region = domain.split('.')[-1].upper()
            region = eng_lang + '_' + eng_territory.upper()
            relatedTopics.append({"name": ddg_result.get("Name", ""), "suggestions": suggestions})
            replace_url = item.get("replaceUrl", [{}])[0]
            request's headers)
            res = dict(enumerate(_valkey_client.lrange(key, 0, -1)))
            res = _valkey_client.hgetall(key)
            res = {
            res.add(
            res.add(item)
            res.add(res.types.KeyValue(kvmap=kvmap))
            res.add(res.types.KeyValue(kvmap={row[0]: row[1]}))
            res.types.Answer(
            res.types.Code(
            res.types.File(
            res.types.KeyValue(
            res.types.LegacyResult(
            res.types.MainResult(
            res.types.Paper(
            resolution = None
            result = parse_audio_item(item)
            result = parse_image_item(item)
            result = parse_video_item(item)
            result, ".//div[contains(@class, 'video-snippet') and @data-macro='video']", 0, default=[]
            results += get_results(attribute_result, attributes, language)
            results.add(
            results.add(results.types.LegacyResult({"title": heading, "url": firstURL}))
            results.add(results.types.LegacyResult({"url": urls[0]["url"], "title": heading, "content": content}))
            results.append(
            results.append(get_infobox(alt_forms, result_url, definitions))
            results.append(item)
            results.append(res)
            results.append(tmp_result)
            results.append(_images_result(result))
            results.append(_image_result(result))
            results.append(_news_result(result))
            results.append(_result(album_info, album_info))
            results.append(_story(item))
            results.append(_video(item))
            results.append(_video_result(result))
            results.append(_web_result(result))
            results.append({'number_of_results': int(extract_text(number_of_results))})
            results.append({'suggestion': extract_text(suggestion)})
            results.append({'suggestion': suggestion['key']})
            results.append({'url': url, 'title': title, 'content': content, 'thumbnail': thumbnail})
            results.types.Answer(
            results.types.LegacyResult(
            results.types.MainResult(
            result["extratags"] = {}
            result['item'].get('created_published_date'),
            result['item'].get('notes', [None])[0],
            result['item'].get('part_of', [None])[0],
            result['item'].get('summary', [None])[0],
            result['thumbnail'] = base_url + avatar
            result[delimiter['keys'][i]] = elements[i]
            result[result_key] = raw_result[found.start() : found.end()]
            result_element, './/div[@class="c8774a" or @class="e69e8d a11657"]', 0, default=None
            result_index += 1
            result_item['author'] = ', '.join(author)
            result_item['source'] += ' (%s)' % file_size
            result_item['source'] += ' | ' + copyright_notice
            result_item['source'] += ' | ' + freshness_date
            result_json['onResponseReceivedCommands'][0]
            result_len = int(result_len_container)
            result_url = ''
            res["author"] = result.get("user", {}).get("full_name") or None
            res["thumbnail"] = thumbnail or None
            res["views"] = result.get("playback_count", 0) or None
            res['valkey_key'] = key
            res_dict = {'url': url, 'title': title, 'content': content}
            res_dict = {'url': url, 'title': title, 'publishedDate': publishedDate, 'content': content}
            res_obj.thumbnail = base_url + thumbnail
            res_url = eval_xpath(r, './/a[@class="wikilink1"]/@href')[-1]
            res_url = item.get('url', None)
            ret.append(res)
            return
            return ""
            return "{} {}".format(amount, unit)
            return currency[1]
            return datetime.strptime(pubDate, '%a, %d %b %Y %H:%M:%S %z')
            return datetime.strptime(value, fmt)
            return f"https://{domain}{self.url_path_prefix}{account}"
            return get_earth_coordinates_url(latitude, longitude, osm_zoom)
            return get_external_url(url_id, value)
            return latitude + ' ' + longitude
            return line.split("= ")[1][:-1].replace('"', ""), resp.cookies
            return match.groups()[0]
            return None
            return real_unit["symbol"]
            return results
            return str(year)
            return value + " " + get_label_for_entity(unit, language)
            return _fetch_results(cur)
            return {}
            search_type = 'search_by_date'
            seen_entities.add(entity_url)
            servers.append(srv)
            Set result['extratags']['contact:website'] if not defined
            Set result['extratags']['wikipedia'] if not defined
            Set result['wikidata'] to { 'image': ..., 'image_sign':..., 'image_symbal':... }
            size = humanize_bytes(size)
            smpl: str = extract_text(p_list[i].xpath("./i[@class='smpl']"))  # type: ignore
            source += ' @ Flickr'
            source = item.get('source')
            source = item_data['snippet']['url']
            source=result["source"],
            source_category = extra_data.get('sc')
            start = 1
            start = int(start_str)
            start_str, result_len_container = re.split(r'-\d+', result_len_container)
            start_time = datetime.fromtimestamp(result.get("startTimeStamp") / 1000)
            suggestions: list[str] = []
            suspended_time=900, message="Alibaba CAPTCHA detected. Please try again later."
            sxng_tag = catalog_engine2code[name]
            sxng_tag = language_tag(babel.Locale.parse(babel_lang, sep="-"))
            sxng_tag = language_tag(babel.Locale.parse(babel_lang.replace('-', '_')))
            sxng_tag = language_tag(babel.Locale.parse(eng_tag))
            sxng_tag = language_tag(babel.Locale.parse(lang_tag, sep="-"))
            sxng_tag = locales.language_tag(babel.Locale.parse(babel_tag))
            sxng_tag = locales.language_tag(babel.Locale.parse(lang_map.get(eng_tag, eng_tag), sep='-'))
            sxng_tag = locales.region_tag(babel.Locale.parse(region))
            sxng_tag = region_tag(babel.Locale.parse("%s_%s" % (lang_tag, country_tag.upper())))
            sxng_tag = region_tag(babel.Locale.parse('%s_%s' % (lang_tag, cc_tag.upper())))
            sxng_tag = region_tag(babel.Locale.parse(eng_tag))
            sxng_tag = region_tag(babel.Locale.parse(eng_tag, sep='_'))
            sxng_tag = region_tag(babel.Locale.parse(l + '_' + r, sep='_'))
            synonyms.append(p_text)
            s_key = suggestions.get('key', '')
            tags = ', '.join(tags)
            tags = [x for x in result.get("tags").split(';') if x and x != 'null']
            tags.append(name)
            tags=record.get("keyword", []),
            tags=record.get("subject"),
            tags=_list("keyword"),
            telephone = result['telephone']
            template="default.html",
            template="images.html",
            template="videos.html",
            text = ddg_result.get("Text", "")
            text = html_to_text(item)
            text = text[: -len(x)]
            text_results.append(params)
            The country code that is used by google (e.g. ``US`` or ``TW``)
            The language code that is used by google (e.g. ``lang_en`` or
            thumb = item_data['image']
            thumb = next((th for th in thumbs if th["quality"] == "sddefault"), None)
            thumbnail = base_url + result['teaser']['image']['sources'][-1]['url']
            thumbnail = content_nodes[0].xpath('.//img/@src')
            thumbnail = eval_xpath_getindex(
            thumbnail = eval_xpath_getindex(item, ".//div[contains(@class, 'thumb_single')]//img/@data-lazysrc", 0)
            thumbnail = eval_xpath_getindex(item, ".//img[contains(@class, 'thumb')]/@src", 0)
            thumbnail = f"https://img.youtube.com/vi/{video_id}/hqdefault.jpg"
            thumbnail = f"https:{thumbnail}"
            thumbnail = imagelink.attrib.get('src')
            thumbnail = images[0]
            thumbnail = img.get('src')
            thumbnail = None
            thumbnail = result["artwork_url"] or result["user"]["avatar_url"]
            thumbnail = result['images']['image700']['url']
            thumbnail = result['images']['imageFbThumbnail']['url']
            thumbnail = result['logo']
            thumbnail = result['post']['thumbnail_url'] + '?format=webp&thumbnail=208'
            thumbnail = url
            thumbnail=f'{absolute_url(channel_result["channel_thumb_url"])}?auth={ta_token}',
            thumbnail=thumbnail,
            thumbnail_query_result = query(result, thumbnail_query)[0]
            thumbnail_src = extract_text(eval_xpath(images[result_index], './@src'))
            thumbnail_src = img_src
            thumbnail_src = photo['sizes']['data']['n']['data']['url']
            thumbnail_src = photo['sizes']['data']['z']['data']['url']
            thumbnail_src = photo['url_n']
            thumbnail_src = photo['url_z']
            thumbnail_src=result["thumbnail"]["src"],
            thumbs = result.get("videoThumbnails", [])
            time = html_to_text(item.get('time')).strip()
            timeout=2.0,
            timestamp = (
            timestring = dates.format_timedelta(delta, granularity='second')
            title += " (%s)" % entry['q']
            title += ' / ' + sectiontitle
            title += f" {w}"
            title = extract_text(eval_xpath(div_2, './div[@title]'))
            title = extract_text(eval_xpath_list(result, title_xpath, min_len=1))
            title = extract_text(title_tag)
            title = gettext('{title} (OBSOLETE)').format(title=result['title'])
            title = html_to_text(result['content'])[:75]
            title = item.get('title', None)
            title = item_data['snippet']['title']
            title = record["container-title"][0]
            title = record["title"][0] if "title" in record else record.get("container-title", [None])[0]
            title = result['name']
            title = result['title']
            title = title.strip('[]')
            title = title_map.get(sxng_tag)
            title, content, thumbnail = construct_body(result)
            title=channel_result['channel_name'],
            title=entry["id"],
            title=extract_text(eval_xpath(result, "./div[@class='content']/h2[@class='title']/a")),
            title=extract_text(title),
            title=extract_text(title_tag),
            title=field("title"),
            title=html_to_text(_list("title")[0]),
            title=result["title"],
            title=title,
            title_tag = eval_xpath_getindex(result, './/div[contains(@role, "link")]', 0, default=None)
            tmp_result = {'url': url, 'title': title, 'content': content}
            tmp_result['is_onion'] = True
            tmp_result['thumbnail'] = thumbnail_prefix + to_string(thumbnail_query_result)
            to the wikidata query.  This field might return for example
            track_id = result['id']
            translations=data,
            tree = html.fromstring(raw_html)
            try:
            type=field("contentType"),
            type=field("type"),
            unit = unit.replace('http://www.wikidata.org/entity/', '')
            uri = quote_plus(result.get("uri"))
            url += '#' + quote(sectiontitle.replace(' ', '_').encode())
            url = "https://core.ac.uk/works/" + str(result["id"])
            url = "https://doi.org/" + str(result["doi"])
            url = base64.urlsafe_b64decode(encoded_url).decode()
            url = base_invidious_url + videoid
            url = base_url + "/paper/%s" % result["id"]
            url = base_url + item.get('href')
            url = build_flickr_url(photo['ownerNsid'], photo['id'])
            url = extract_text(item.xpath('.//h3[contains(@class, "res-title")]/a/@href'))
            url = extract_url(eval_xpath_list(result, url_xpath, min_len=1), search_url)
            url = f"https://doi.org/{doi}"
            url = f"{base_url}/{entry['id']}"
            url = f"{base_url}/{huggingface_endpoint}/{entry['id']}"
            url = f"{base_url}{url}"
            url = f'{base_url.rstrip("/")}/?videoId={video_result["youtube_id"]}'
            url = f'{base_url.rstrip("/")}{video_result["media_url"]}'
            url = img_src
            url = link.attrib.get('href')
            url = result.get("links")[0]
            url = result.get("permalink_url")
            url = result["downloadUrl"]
            url = result["sourceFulltextUrls"]
            url = result['link']  # pylint: disable=redefined-outer-name
            url = result['urls'][0]
            url = result['url_resolved']
            url = unquote(raw_url[7:].split('&sa=U')[0])  # remove the google redirector
            url="https://example.org",
            url=base_url + extract_text(eval_xpath(result, "./div[@class='content']/h2[@class='title']/a/@href")),
            url=channel_url,
            url=f"https://ui.adsabs.harvard.edu/abs/{doc.get('bibcode')}/",
            url=f"{url}/{params['from_lang'][1]}/{params['to_lang'][1]}/{params['query']}",
            url=field("URL"),
            url=html_url,
            url=result["url"],
            url=url,
            urls.append({"title": text, "url": firstURL})
            url_id = self.url_id
            url_label = result.get('wikidata', {}).get('itemLabel') or url_label
            url_query = urlparse(url).query
            user_count = sum(int(entry['accounts']) for entry in result['history'])
            uses_count = sum(int(entry['uses']) for entry in result['history'])
            v = result.get(x)
            v = result.get(y)
            val = f"'{val}'"
            val = item["value"].replace("http://", "https://", 1)
            value = _strip_leading_strings(value)
            value=str(form_vqd),
            video = video_container.get('videoRenderer', {})
            videoid = result.get("videoId", None)
            videoid = video.get('videoId')
            video_blocks = tree.xpath('//div[contains(@class, "search-video")]')
            video_id = parse_qs(parsed_url.query).get('v', [None])[0]
            video_url = f"{base_url}{video_url}"
            views=humanize_number(channel_result['channel_subs']),
            views=_str("read_count"),
            volume=field("volume"),
            volume=_str("volume"),
            wd_id = wd_result['item']['value'].replace('http://www.wikidata.org/entity/', '')
            wd_to_results.setdefault(wd_id, []).append(result)
            weather_answer.forecasts.append(forecast_data)
            width = item.get("width")
            width = item_data['viewerData']['dups'][0]['w']
            wikidata_entity = unit[len(prefix) :]
            wikidata_ids.append('wd:' + wd_id)
            working_dir = realpath(working_dir)
            x
            year = eval_xpath_getindex(accepted_date, "./Year", 0)
            yield cursor
            [account, domain] = [x.strip("@ ") for x in value.rsplit('@', 1)]
            ]
            ],
            ]:
            _clean_up_node(p_item)
            _compiled_parse_regex[result_key] = re.compile(regex, flags=re.MULTILINE)
            _IMG_SRC_NEW_URL_PREFIX
            _locale = "{lang}_{country}".format(lang=lang, country=country)
            _pub_date = extract_text(
            _topic_results: list[dict[str, str]] = ddg_result.get("Topics", [])  # pyright: ignore[reportAssignmentType]
            ``account@domain``.  If provided, value are rewritten to
            ``https://<domain><url_path_prefix><account>``.  For example::
            ``lang_zh-TW``)
            ``libreoffice@fosstodon.org`` and the URL built from this is then:
            ``searxng_locale`` value.
            {
            {'type': qwant_categ, 'items': mainline},
            {'url': result_url, 'title': ", ".join(alt_forms), 'content': content[:300] + (content[300:] and '...')}
            }
            },
           "de": "Spezial:Suche",
           "de": "wiki.archlinux.de",
           "en": "en.wikipedia.org",
           "gsw": "als.wikipedia.org",
           "zh": "Special:\u641c\u7d22"
           "zh": "wiki.archlinuxcn.org"
           "zh": "zh.wikipedia.org",
           "zh-classical": "zh-classical.wikipedia.org"
           ..
          "rating": "{safe_search}"
          "time_range": {time_range},
         "title": {
         "wiki_netloc": {
         }
         },
        """
        "a": "h_",
        "Accept": "text/html,application/xhtml+xml,application/xml;q=0.9,image/webp,*/*;q=0.8",
        "Accept-Language": "en-US;q=0.5,en;q=0.3",
        "action": "query",
        "after": search_after(params["time_range"]),
        "api_key": api_key,
        "app_locale": app_locale_map.get(params["language"].split("-")[0], "en"),
        "as_sdt": "2007",  # include patents / to disable set "0,5"
        "as_vis": "0",  # include citations / to disable set "1"
        "author": item.get("artist_name"),
        "author": item["author"],
        "authors": [extract_text(author) for author in author_elements],
        "authors": [extract_text(eval_xpath_getindex(item, ".//a[starts-with(@href, '/search')]", 0))],
        "authors": [],
        "bpia": "1",
        "ca": "ca",
        "Cache-Control": "max-age=0",
        "client_id": c_id,
        "client_id": guest_client_id,
        "client_secret": c_secret,
        "coAuthors": [],
        "comment": "More info on api: https://www.ncbi.nlm.nih.gov/books/NBK25501/",
        "Connection": "keep-alive",
        "content": aa_content,
        "content": content,
        "content": extract_text(eval_xpath(item, ".//div[contains(@class, 'relative')]")),
        "content": html_to_text(
        "content": html_to_text(' '.join(content_list)),
        "content": html_to_text(content),
        "content": html_to_text(data.get('data', {}).get('abstract')),
        "content": html_to_text(data.get('message', {}).get('content_text')),
        "content": html_to_text(data.get('strong', {}).get('baike_text')),
        "content": html_to_text(data.get('summary', {}).get('content')),
        "content": html_to_text(data.get('title', {}).get('title_tag')),
        "content": item["asset_type"],
        "count": results_per_page,
        "current": ",".join(data_of_interest),
        "current_page": params["pageno"],
        "day": "today",
        "de-DE": "de-de",
        "dir": search_dir,
        "direction": -1,
        "DNT": "1",
        "en-CA": "en-ca",
        "en-GB": "en-gb",
        "en-US": "en-us",
        "es": "es",
        "ext": aa_ext,
        "extra": {{
        "facet": soundcloud_facet,
        "fields": "*",
        "filters": "ctype:articoli",
        "fl": ",".join(ads_field_list),
        "forecast_days": 3,
        "format": "json",
        "fr-CA": "fr-ca",
        "fr-FR": "fr-fr",
        "frame": "1",
        "from": page_size * (params['pageno'] - 1),
        "from": start_index,
        "generator": "search",
        "getQuerySuggestions": False,
        "grant_type": "client_credentials",
        "gsrlimit": number_of_results,
        "gsrnamespace": "6",  # https://www.mediawiki.org/wiki/Help:Namespaces#Renaming_namespaces
        "gsroffset": number_of_results * (params["pageno"] - 1),
        "gsrprop": "snippet",
        "gsrsearch": f"filetype:{filetype} {query}",
        "highlight": 0,
        "hourly": ",".join(data_of_interest),
        "iframe_src": audio_data["preview"]["url"],
        "iframe_src": item["video_small_preview_url"],
        "iiprop": "url|size|mime",
        "iiurlheight": "180",  # needed for the thumb url
        "img_format": item["format"],
        "img_src": item["content_thumb_extra_large_url"],
        "include": "channel,thumbnail_url,title,description,duration,release_time",
        "ja-JP": "ja-jp",
        "k": query,
        "keyword": query,
        "l": eng_region,
        "lang": "en",
        "lang": lang,
        "lang": resp.search_params.get("searxng_locale", "en"),  # ui language
        "latitude": location.latitude,
        "length": isodate.parse_duration(item["time_duration"]),
        "length": timedelta(seconds=round(audio_data["duration"] / 1000)) if audio_data["duration"] else None,
        "limit": nb_per_page,
        "limit": results_per_page,
        "list" in display_type
        "longitude": location.longitude,
        "max_results": arxiv_max_results,
        "mediaType": "video",
        "metadata": item["asset_type"],
        "mode": "all",
        "month": "month",
        "month": "thismonth",
        "o": "json",
        "offset": (params["pageno"] - 1) * nb_per_page,
        "offset": (params["pageno"] - 1) * results_per_page,
        "offset": (params['pageno'] - 1) * results_per_page,
        "offset": 20 * (params["pageno"] - 1),
        "offset": start_index,
        "operationName": "SearchProductExplorer",
        "order": "date_d",
        "order": adobe_order,
        "orderby": sort_order,
        "p": nb_per_page,
        "p": params["pageno"],
        "page": params["pageno"],
        "page": params['pageno'],
        "page": {pageno},
        "pageSize": 10,
        "page_size": results_per_page,
        "per-page": 10,
        "performTitleMatch": True,
        "per_page": results_per_page,
        "pg": params["pageno"],
        "pn": params["pageno"],
        "prop": "info|imageinfo",
        "pro_first": 1,
        "pt-BR": "pt-br",
        "publishedDate": datetime.fromisoformat(audio_data["release_date"]) if audio_data["release_date"] else None,
        "publishedDate": datetime.fromisoformat(item["creation_date"]),
        "publishedDate": published_date,
        "publisher": extract_text(
        "publisher": _text(item, './/a[@title="Publisher"]'),
        "q": query,
        "q": resp.search_params["query"],
        "qs": query,
        "query": graphql_query,
        "query": query,
        "queryString": query,
        "requests": [
        "resolution": f"{item['content_original_width']}x{item['content_original_height']}",
        "resultType": "records",
        "rowCount": results_per_page,
        "rows": ads_rows,
        "s": nb_per_page * (params["pageno"] - 1),
        "s": query,
        "scope": "https://management.azure.com/.default",
        "search": "{query}",
        "search": query,
        "searchid": "3131712",
        "search_page": params["pageno"],
        "search_query": f"{arxiv_search_prefix}:{query}",
        "search_type": "pagination",
        "search_type": "video",
        "Sec-GPC": "1",
        "single_column": "0",
        "size": page_size,
        "size": results_per_page,
        "sl": resp.search_params['from_lang'][1],
        "sort": "date_d",
        "sort": "relevance",
        "sort": "relevance_score:desc",
        "sort": aa_sort,
        "sorting": "relevance",
        "source": "web",
        "sq-AL": "sq-al"
        "start": (params["pageno"] - 1) * 10,
        "start": (params["pageno"] - 1) * 48,
        "start": (params["pageno"] - 1) * arxiv_max_results,
        "start": 10,
        "start": 15,
        "start": 48,
        "start": 50,
        "start": ads_rows * (params["pageno"] - 1),
        "s_mode": "s_tag_full",
        "template": "images.html",
        "template": "videos.html",
        "text": query,
        "thumbnail": data.get('data', {}).get('img').replace("http://", "https://"),
        "thumbnail": data.get('message', {}).get('video_img').replace("http://", "https://"),
        "thumbnail": extract_text(eval_xpath_getindex(item, ".//img/@src", 0, default=None), allow_none=True),
        "thumbnail": item["thumbnail_url"],
        "thumbnail": thumbnail,
        "thumbnail_src": item["thumbnail_url"],
        "timeformat": "unixtime",
        "timezone": "auto",  # use timezone of the location
        "title": extract_text(eval_xpath(item, "./div//a[starts-with(@href, '/md5')]")),
        "title": html_to_text(
        "title": html_to_text(data.get('company_name')),
        "title": html_to_text(data.get('data', {}).get('title')),
        "title": html_to_text(data.get('strong', {}).get('title')),
        "title": html_to_text(data.get('title')),
        "title": html_to_text(data.get('title', {}).get('content')),
        "title": html_to_text(data.get('title', {}).get('text')),
        "title": item["title"],
        "title": title,
        "title": _text(item, './/*[@itemprop="name"]'),
        "tl": resp.search_params['to_lang'][1],
        "tmpl_version": "releases",
        "type": "illust_and_ugoira",
        "type": 2,
        "type": _text(item, './/div[contains(@class, "property__file")]//div[contains(@class, "property_value")]'),
        "u": "bing",
        "uid": base64.b64encode(secrets.token_bytes(16)).decode("utf-8"),
        "uinfo": "sw-1920-sh-1080-ww-1125-wh-999",
        "Upgrade-Insecure-Requests": "1",
        "url": "https://www.ncbi.nlm.nih.gov/home/develop/api/",
        "url": base_url + item.xpath('(.//a[starts-with(@href, "/book/")])[1]/@href')[0],
        "url": base_url_choice + eval_xpath_getindex(item, "./a/@href", 0),
        "url": data.get('data', {}).get('url'),
        "url": data.get('message', {}).get('statistics', {}).get('nu'),
        "url": data.get('source', {}).get('dest_url'),
        "url": data.get('source', {}).get('url'),
        "url": data.get('sourceProps', {}).get('dest_url')
        "url": data.get('strong', {}).get('baike_url'),
        "url": data.get('title', {}).get('url'),
        "url": data.get('title_url'),
        "url": item["content_url"],
        "uselang": uselang,
        "User-Agent": gen_useragent(),
        "variables": {"offset": offset, "limit": page_size, "query": query, "sortBy": "RELEVANCE"},
        "venues": [],
        "vqd": vqd,
        "web": "1",
        "website": "reuters",
        "week": "thisweek",
        "week": "week",
        "where": "image",
        "where": "news",
        "where": "video",
        "where": "web",
        "word": query,
        "year": "thisyear",
        "year": "year",
        "Your IP address is" not in zero_click
        "zh-CN",
        "zh-HK",
        "zh-MO",
        "zh-MY",
        "zh-SG",
        "zh-TW",
        "zh_Hans",
        "zh_Hant",
        "__refresh__": "true",
        #  doi
        # !ddi paris :es-AR --> {'ad': 'es_AR', 'ah': 'ar-es', 'l': 'ar-es'}
        # "https://lh3.googleusercontent.com/DjhQh7DMszk.....z=-p-h100-w100"
        # "p": params["pageno"] - 1,
        # "payment:*" in KEY_ORDER matches "payment:cash", "payment:debit card", etc...
        # "per_page": 10,
        # "thumbnail": base_url + item["thumbnail_url"],
        # 'ams_country': eng_region.split('_')[1],
        # 'app', 'audio', 'video',
        # 'pt-pt' and 'pt-br' --> 'pt-br'
        # (see the seen_entities variable)
        # - https://github.com/searxng/searxng/issues/4646#issuecomment-2817848019
        # --> using LegacyResult
        # 110n 15,000.00 (EN) --> 15.000,00 (DE)
        # 32bit and dbg packages don't have their own package templates
        # a type for the video.html template is not yet implemented
        # a video and not to a video stream --> SearXNG can't use the video template.
        # add "normalized" language name (i.e. français becomes francais and español becomes espanol)
        # add as result ? as answer ? problem always in english
        # add as result ? problem always in english
        # add market codes from official languages of the country ..
        # add native name exactly as it is
        # add official languages of the country ..
        # add rest of adressdata, if something is already found
        # address calculation
        # All the links are not PDFs, even if the URL ends with ".pdf"
        # already a '-' delemitter in the language.  For instance 'pt-PT' -->
        # and all definitions (as description) truncated to 300 characters.
        # and for example https://taginfo.openstreetmap.org/keys/currency:EUR#values
        # append result
        # append suggestion
        # Appends either a text or an image, depending on which one is more suitable
        # authors
        # baidu's JSON encoder wrongly quotes / and ' characters by \\ and \'
        # Broader XPath to find any <img> element
        # but there is also currency=EUR (currently not handled)
        # calculate datetime
        # catch obsolete entries and mark them accordingly
        # category in which our torrent belongs
        # ceid includes a ':' character which must not be urlencoded
        # cited by
        # comments
        # content string contains all information not included into template
        # convert files to int if possible
        # country_list contains duplicates that differ only in upper/lower case
        # Create a Locale object for the current locale
        # currency:EUR --> get the name from the CURRENCIES variable
        # currently the api seems to always return null before the first tag,
        # data["kl"] = ""
        # dates returned by the BASE API are not several formats
        # DDG does not accept queries with more than 499 chars
        # defaults
        # deterministic ;)
        # different result types (e.g. mainline[0]['type'] returns type of the
        # does not set a region (e.g. 'en-CA' / canada) we cannot hand over a region.
        # don't ask why it is only sometimes / its M$ and they have never been
        # don't change anything, re-use the existing values
        # don't url-encode the query if it's in the request body
        # don't urlencode this because wildly different AND bad results
        # don't use nextpage when user selected to jump back to page 1
        # each row is an Translations.Item
        # either there's a package with status "newest" or we assume that the
        # embedded
        # ensure picking only the code contents in the blob
        # Entries that are purely from Wikipedia are excluded.
        # example: if 'zh' is not found, check 'en'
        # example: if 'zh-hk' is not found, check 'zh'
        # example: if still not found, use the first entry
        # Extra data. Since they're not documented, this implementation is based solely by the author's assumptions.
        # extract and convert creation date
        # extract post information
        # Extract severity (Low, Medium, High, or Critical) and CVSS score, if available
        # facilitate preview support for known mime types
        # Fallback for video_id from URL if not found via XPath
        # FILTER NOT EXISTS { ?item p:{name}/psv:{name}/wikibase:timeValue ?{name}bis FILTER (?{name}bis < ?{name}) }
        # fix content string
        # fix offset date for line 644 webapp.py check
        # For a bigger thumbnail, keep only the url_z, not the url_n
        # For results, we'll return the URL, all alternative forms (as title),
        # for VODs (videos on demand)
        # From the biggest to the lowest format
        # generator (gsr optins) https://commons.wikimedia.org/w/api.php?action=help&modules=query%2Bsearch
        # get name
        # get osm-type
        # Get the not cropped version of the thumbnail when the image height is not too important
        # get the real URL
        # Get title
        # Get URL - try different options
        # gets app_js and search for the client_id
        # Handle authors
        # Handle journals
        # Handle publisher
        # Handle thumbnail
        # Handle video embed URL
        # has the layout not changed yet?
        # HINT: no mater what the value is, without API token videos can't shown
        # HINT: this list probably needs to be supplemented
        # hold the cached value for 5min
        # however only using contains(@class, 'content') would e.g. also match `site-name-content`
        # https://commons.wikimedia.org/w/api.php
        # https://commons.wikimedia.org/w/api.php?action=help&modules=query
        # https://developer.imdb.com/documentation/key-concepts#imdb-ids
        # https://developers.dailymotion.com/api/#global-parameters
        # https://developers.dailymotion.com/api/#video-filters
        # https://en.wikipedia.org/wiki/ISO_8601#Durations
        # https://taginfo.openstreetmap.org/keys/currency#values
        # I have not yet seen any torrents without magnet links, but
        # if arg 'pq' is missed, sometimes on page 4 we get results from page 1,
        # if search query is empty show results from HN's front page
        # if thumbnail field contains a valid URL, we need to change template
        # ignore brand:wikidata
        # ignore result without title
        # imageinfo: https://commons.wikimedia.org/w/api.php?action=help&modules=query%2Bimageinfo
        # images are sorted in ascending quality
        # imageUrl is the image itself, it is not a thumb!
        # in the response.  The result items are directly in the list
        # in which regions this language is an official one, but then we still
        # Insufficient rights for the `ams_country' parameter of route `GET /videos'
        # is a item that can't be displayed in a infobox.
        # is converted by bing to 'en-us' or 'de-de'.  But only if there is not
        # is it thumbnail or img_src??
        # is returned / e.g. search term is "user: foo" instead "user:foo"
        # it's better to be prepared to stumble upon one some day
        # item.pdf_url = record.get("link", [{"URL": None}])[0]["URL"]
        # journal
        # keep result size moderate; OpenAlex default is 25
        # Language (e.g. 'en' or 'de') from https://www.bing.com/account/general
        # language parameter is ignored
        # leech count
        # let's try to calculate the torrent size
        # license can be None
        # Like Wordnik, we'll return the first result in an infobox too.
        # link to apple when you press the play button.
        # link to the html or pdf document
        # link to the page
        # minor optimization : no need to check VALUE_TO_LINK if extratags is empty
        # most commonly used version is the latest released (non-alpha) version
        # name as the query
        # netloc & path
        # no more info in the first row, start parsing the second one
        # note: piped returns -1 for all upload times when filtering for music
        # on a invalid search term the status code 422 "Unprocessable Content"
        # Only append results with valid title and url
        # pagination uses Zero-based numbering
        # paging is broken in searchcode.com's API
        # parse general results
        # parse image results
        # parse the first row
        # pdf
        # performs an IP-based geolocation of the user, we don't want that in
        # pick for the first alternate link, but not from the crawler
        # popularity is of type str ..
        # precision: day
        # precision: less than a year
        # precision: minute
        # precision: month
        # precision: second.
        # precision: year
        # Presearch narrows down the search by region.  In SearXNG when the user
        # Process alternative forms
        # Published date
        # Put empty kl in form data if language/region set to all
        # pylint: disable=too-many-nested-blocks
        # Quark returns a variety of different sc values on a single page, depending on the query type.
        # Queries on News, Images and Videos do not have a list named 'mainline'
        # Queries without auth are heavily rate limited.
        # query example: "EC1M 5RF London"
        # query will be cached by wikidata
        # query won't be cached by wikidata
        # relevance sorting works only with `search`
        # remove ahmia url and extract the actual url for the result
        # remove HTML from snippet
        # response and the HTTP status code is set in the 'status' element.
        # result items in mainline[0]['items']
        # results.add(results.types.Answer(answer=message))
        # result['items'].
        # SearXNG ;-)
        # see https://github.com/osm-search/Nominatim/issues/1521
        # see https://github.com/searxng/searxng/pull/1456#issuecomment-1193105023
        # see https://wiki.openstreetmap.org/wiki/Key%3Acurrency
        # seed count
        # seems, wolframalpha resets its token in every hour
        # set suspend time to zero is OK --> ddg does not block the IP
        # show item in the result list if 'list' is in the display options or it
        # skip images that are blurred
        # so strip that unless it's not already there
        # Sometime Quark will return 0, set published_date as None
        # sometimes there's just no preview image, hence we skip the image
        # Springer's API key is a hex value
        # ss_kv variant 1
        # ss_kv variant 2
        # stream tags
        # tags
        # that's a bug at imgur's side:
        # the duration only seems to be valid if the video is less than 60 mins
        # The first <a> tag in the <article> contains the link to the article
        # The href attribute of the <a> tag is a google internal link, we have
        # The image URL is located in a preceding sibling <img> tag, e.g.:
        # The players are just playing 30sec from the title.  Some of the player
        # The pub_date is mostly a string like 'yesterday', not a real
        # the response from search-index repository is very slow
        # The results in the video category are most often links to pages that contain
        # The WEB query contains a list named 'mainline'.  This list can contain
        # there are other classes like 'site-name-content' we don't want to match,
        # These URL are long but not personalized (double checked via tor).
        # this filter is too slow, so the response function ignore duplicate results
        # thus, we explicitly also require the spaces as class separator
        # time & duration
        # timezone date or time.  Therefore we can't use publishedDate.
        # title and content sometimes containing characters such as &amp; &#39; &quot; etc...
        # to decode
        # To remove duplicate, add
        # to simplify the page count lets use the default of 10 images per page
        # to simplify the page count lets use the default of 35 images per page
        # TODO get infobox.meta.value where .label="article_title"    # pylint: disable=fixme
        # TODO parse video, audio and file results
        # torrent downloads count
        # torrent title
        # Try DOI first
        # url="https://open-meteo.com/en/docs",
        # use duckduckgo's mapkit token
        # use wildcards to match more than just packages with the exact same
        # vqd is required to request other pages after the first one
        # We could possibly use searx.locales.get_official_locales to determine
        # when a search term has none results, loc sends a JSON in a HTTP 404
        # wiki.archlinux.org is protected by anubis
        # will be blocked because of a cross-origin request and some players will
        # Without the auth header the query fails, so add a dummy instead.
        # wouldn't know which region should be given more weight / Presearch
        # zh_Hans --> zh
        #'simple: – invented code used for the Simple English Wikipedia (not the official IETF code en-simple)
        '''
        ', {"value":"first item"}'
        ', {"value":"second item"}'
        ', {"value":"third item"}'
        './/torznab:attr[@name="{attribute_name}"]'.format(attribute_name=attribute_name),
        '0': ('format_8', 1000000000),
        '1': ('format_8', 100000000),
        '10': ('format_10', 1),  # month
        '11': ('format_11', 0),  # day
        '12': ('format_13', 0),  # hour (not supported by babel, display minute)
        '13': ('format_13', 0),  # minute
        '14': ('format_14', 0),  # second
        '2': ('format_8', 10000000),
        '3': ('format_8', 1000000),
        '4': ('format_8', 100000),
        '5': ('format_8', 10000),
        '6': ('format_8', 1000),
        '7': ('format_8', 100),
        '8': ('format_8', 10),
        '9': ('format_9', 1),  # year
        'abd': '1',
        'abe': '1',
        'abp': '1',
        'Accept': 'application/json',
        'Accept': 'application/json, text/javascript, */*; q=0.01',
        'Accept-Language': "en-US,en;q=0.5",  # bing needs to set the English language
        'action': 'query',
        'ak': 'aka',
        'als': 'gsw',
        'alternatives': 3,
        'an': 'arg',
        'any': query,
        'arch': query_arch or alpine_arch,
        'async': '1',
        'async': 'content',
        'backlinks': backlinks,
        'bat-smg': 'sgs',
        'be-tarask': 'bel',
        'bool': {
        'cat': startpage_categ,
        'category': params['category'],
        'cbk-zam': 'cbk',
        'cc': engine_region.split('-')[-1],
        'client': 'web',
        'content': '',
        'content': ''.join(infobox_content),
        'content': album_info.get("brief", {}).get("value", ""),
        'content': content,
        'content': content.strip(),
        'content': html_to_text(result['excerpt']),
        'content': html_to_text(result['snippet']),
        'content': item.get('firstSentence', ''),
        'content': item['firstSentence'],
        'content': result.get('description', ''),
        'content': result['description'],
        'Content-Type': 'application/json',
        'contentdom': search_type,
        'context': {},
        'Cookie': (
        'cookies': {},
        'count': 35,
        'count': page_size,
        'country': None,
        'da': 'dk',  # da --> da-dk
        'de': 'Spezial:Suche',
        'domain': match_json.get("domain"),
        'en': 'us',  # en --> en-us
        'english_uk',  # SearXNG lang 'en' already maps to 'english'
        'fa': 'ویژه:جستجو',
        'family_filter': family_filter_map.get(params['safesearch'], 'false'),
        'fields': ','.join(result_fields),
        'files': get_attribute(item, 'files'),
        'filesize': filesize,
        'filesize': humanize_bytes(int(filesize)) if filesize else None,
        'filesize': match_json.get("filesize"),
        'filter': piped_filter,
        'first': (int(params.get('pageno', 1)) - 1) * 35 + 1,
        'first': page * 10 + 1,
        'fiu-vro': 'vro',
        'fl': 1,
        'form': 'PTFTNR',
        'format': 'json',
        'from': (params['pageno'] - 1) * page_size,
        'fs': query,
        'general': {
        'gtp': f"474_list%3D{params['pageno']}",
        'headers': {},
        'height': match_json.get("height"),
        'hidebroken': 'true',
        'hits': number_of_results,
        'hitsPerPage': page_size,
        'https://'
        'https://accounts.spotify.com/api/token',
        'https://developer.apple.com/library/archive/documentation/AudioVideo/Conceptual/'
        'https://framagit.org/framasoft/peertube/search-index/-/raw/master/client/src/components/Filters.vue',
        'https://raw.githubusercontent.com/OdyseeTeam/odysee-frontend/master/ui/constants/supported_browser_languages.js',  # pylint: disable=line-too-long
        'iframe_src': get_embeded_stream_url(result['content']),
        'iframe_src': result['sources'].get('embed'),
        'iframe_src': video_url,
        'image',
        'images': {
        'image_format': match_json.get("format"),
        'image_url': match_json.get("image_url"),
        'img_format': result.get('format'),
        'img_src': list(result['sources'].values())[-1],
        'img_src': result.get('rawImageUrl'),
        'img_src': result['image'],
        'img_src': result['image_id'],
        'indexName': 'prod_all-images',
        'InfiniteScroll': 1,
        'infobox': infobox_title,
        'it': {
        'iTuneSearchAPI/UnderstandingSearchResults.html#//apple_ref/doc/uid/TP40017632-CH8-SW1'
        'ja': '特別:検索',
        'lang': lang,
        'language': None,
        'language': params['language'],
        'languages': eng_lang,
        'leech': _map_leechers(leechers, seeders, peers),
        'length': length,
        'length': result['duration'],
        'length': timedelta(seconds=result['duration']),
        'limit': 10,
        'limit': 20,
        'limit': limit,
        'limit': max_result_count,
        'limit': number_of_results,
        'limit': page_size,
        'list': 'search',
        'locale': None,
        'm': 'package',
        'magnetlink': None,
        'map-bms': 'map',
        'maximumRecords': number_of_results,
        'metadata': result.get('uploader'),
        'multi_match': {
        'name': f"*{query}*",
        'name': query,
        'nds-nl': 'nds',
        'news': {'endpoint': '/v5/general/v1/web/search', 'params': {'pn': params["pageno"], 'ps': results_per_page}},
        'no': 'nb-NO',
        'no_correct': 'false',
        'nrm': 'nrf',
        'NSTN/3.60.474802233.release Dalvik/2.1.0 (Linux; U; Android 12;' f' {google_info.get("country", "US")}) gzip'
        'offset': (params['pageno'] - 1) * limit,
        'offset': (params['pageno'] - 1) * number_of_results,
        'offset': 10 * (params['pageno'] - 1),
        'offset': offset,
        'options': {
        'oq': query,
        'order': 'votes',
        'overlay': match_json.get("overlay"),
        'p': params['pageno'] - 1,
        'p': params['pageno'],
        'P122',  # basic form of government
        'P123',  # publisher
        'P159',
        'P162',  # producer
        'P17',  # country
        'P170',  # creator
        'P175',  # performer
        'P176',  # manufacturer
        'P178',  # developer
        'P264',  # record label
        'P27',  # country of citizenship
        'P272',  # production company
        'P35',  # head of state
        'P36',  # capital
        'P37',
        'P400',  # platform (videogames, computing)
        'P449',  # original network
        'P495',  # country of origin
        'P50',  # author
        'P569',  # date of birth
        'P57',  # director
        'P570',  # date of death
        'P571',  # inception date
        'P576',  # dissolution date
        'P58',  # screenwriter
        'P580',  # start date
        'P582',  # end date
        'P6',  # head of government
        'P619',  # date of spacecraft launch
        'P620',
        'P750',  # distributed by
        'P86',
        'page': params['pageno'] - 1,
        'page': params['pageno'],
        'pageno': (params['pageno'] - 1) * page_size + first_page_num,
        'pageno': params['pageno'],
        'pageSize': results_per_page,
        'pagi': params['pageno'],
        'params': {},
        'password_protected': 'false',
        'payload[nid]': 65350,
        'per_page': results_per_page,
        'pg': params['pageno'],
        'pn': 10,
        'pq': query,
        'private': 'false',
        'profile': 'default',
        'publishedDate': datetime.fromtimestamp(result['date']),
        'publishedDate': datetime.strptime(item['date'][:19], '%Y-%m-%dT%H:%M:%S'),
        'publishedDate': datetime.strptime(result['first_seen'][:19], '%Y-%m-%dT%H:%M:%S'),
        'publishedDate': parser.parse(result['uploadDate']),
        'publishedDate': publishedDate,
        'publishedDate': published_date,
        'publishedDate': _map_published_date(pubDate),
        'purity': safesearch_map[params['safesearch']],
        'q': ' '.join(q),
        'q': params['query'],
        'q': query,
        'qs': 'thumbs',
        'query': query,
        'query': urlencode({'q': query})[2:],
        'query': urlencode({'query': query}),
        'query[term]': query,
        'resolution': '%s x %s' % (result['width'], result['height']),
        'resolution': resolution,
        'resolve': 'false',
        'resource': search_mode,
        'resultPage': params['pageno'] - 1,
        'reverse': 'true',
        'roa-rup': 'rup',
        'rw': 'new',
        'safe': min(params['safesearch'], 1),
        'safeSearch': safesearch_map[params['safesearch']],
        'safe_search': safe_search,
        'sc': get_sc_code(params),
        'score': match_json.get("score"),
        'search': query,
        'searchText': query,
        'seed': seeders,
        'setlang': engine_region.split('-')[0],
        'SFX': page,
        'size': match_json.get("size"),
        'size': page_size,
        'sort': "data:desc",
        'sort': 'relevance',
        'sort': sort,
        'sort[direction]': 'descending',
        'source': params['from_lang'][1],
        'source': result['provider'],
        'source': result['source'],
        'srlimit': number_of_results,
        'sroffset': offset,
        'srprop': srprop,
        'srsearch': query,
        'srsort': srsort,
        'srwhat': search_type,
        'start': (params['pageno'] - 1) * page_size,
        'startRecord': offset,
        'subdomain': None,
        't': 'device',
        'tags': match_json.get("tags"),
        'target': params['to_lang'][1],
        'template': 'images.html',
        'template': 'torrent.html',
        'template': 'videos.html',
        'templateQueryString': query,
        'term': query,
        'text',
        'thumbnail': album_info.get("img", ""),
        'thumbnail': hit['result']['image_url'],
        'thumbnail': hit['result']['song_art_image_thumbnail_url'],
        'thumbnail': item.get('teaserImage', {}).get('imageVariants', {}).get('16x9-256'),
        'thumbnail': result['images'].get('small') or result['images'].get('medium'),
        'thumbnail': result['sources'].get('thumbnail'),
        'thumbnail': res['cover_art_url'],
        'thumbnail': thumbnailUrl,
        'thumbnail_ratio': 'original',  # original|widescreen|square
        'thumbnail_src': list(result['sources'].values())[0],
        'thumbnail_src': result['src'],
        'thumbnail_src': result['thumbnail'],
        'thumbnail_src': thumbnailUrl,
        'time_range': params['time_range'],
        'time_range': time_range,
        'title': get_attribute(item, 'title'),
        'title': hit['result']['full_title'],
        'title': hit['result']['name'],
        'title': html_to_text(result['title']),
        'title': item['title'],
        'title': result.get('name'),
        'title': result.get('title', ''),
        'title': result['title'],
        'title': res['full_title'],
        'title': title,
        'title': video.get("title", ""),
        'toggle_all': 1,  # load item specs
        'torrentfile': None,
        'type': mastodon_type,
        'type': search_type,
        'type_': lemmy_type,
        'url': base_url + result["href"],
        'url': hit['result']['url'],
        'url': item['shareURL'] if use_source_url else item['detailsweb'],
        'url': result['clickUrl'],
        'url': result['content'],
        'url': result['host_page'],
        'url': result['url'],
        'url': res['url'],
        'url': url,
        'url': video.get("pageUrl", "").replace("http://", "https://"),
        'url': _map_result_url(guid, comments),
        'urls': [
        'User-Agent': gen_useragent() + " Pixabay",
        'User-Agent': gen_useragent(),
        'User-Agent': searxng_useragent(),
        'userset': 1,
        'v': 1,
        'v=1&vm=p&fl=1&vl=lang_fr'
        'videos': {
        'vl': f'lang_{lang}',
        'vm': safesearch_dict[params['safesearch']],
        'width': match_json.get("width"),
        'wildcard': r'%' + query.replace(' ', r'%') + r'%',
        'with_date': time_range_dict.get(params['time_range'], ''),
        'x-bootstrap-cache-miss': '1',
        'x-fetch-bootstrap': '1',
        'X-Pinterest-AppState': 'active',
        'X-Pinterest-PWS-Handler': 'www/ideas.js',
        'X-Pinterest-Source-Url': '/ideas/',
        'X-Requested-With': 'XMLHttpRequest',
        'zh': 'Special:搜索',
        'zh-hk': 'en-hk',  # not sure why, but at M$ this is the market code for Hongkong
        'zh-min-nan': 'nan',
        'zh-yue': 'yue',
        '[ {"value": "%s"}'
        '\"': '\\\"',
        '\'': '\\\'',
        '\b': '\\\b',
        '\f': '\\\f',
        '\n': '\\\n',
        '\r': '\\\r',
        '\t': '\\\t',
        '\uf522': '\u2192',  # right arrow
        '\uf74c': 'd',  # differential
        '\uf74d': '\u212f',  # euler's number
        '\uf74e': 'i',  # imaginary number
        '\uf7b1': '\u2115',  # set of natural numbers
        '\uf7b4': '\u211a',  # set of rational numbers
        '\uf7b5': '\u211d',  # set of real numbers
        '\uf7bd': '\u2124',  # set of integer numbers
        '\uf7d9': '=',
        '\\': '\\\\'
        ']' % engine_settings.get('name')
        (
        ("$skip", (params["pageno"] - 1) * page_size),
        ("$top", "10"),
        ("applyOperator", "false"),
        ("documentation", "Documentation"),
        ("expandScope", "true"),
        ("facet", "category"),
        ("facet", "products"),
        ("facet", "tags"),
        ("homepage", "Project homepage"),
        ("includeQuestion", "false"),
        ("locale", params["language"]),
        ("partnerId", "LearnSite"),
        ("repository", "Source code"),
        ("scoringprofile", "semantic-answers"),
        ("search", query),
        (:py:obj:`searx.enginelib.traits.EngineTraits`)
        )
        ) from exc
        ),
        ):
        **google_info["params"],
        + "&extensions[]={zlib_ext}"
        + "&languages[]={lang}"
        + "&yearFrom={zlib_year_from}"
        + "&yearTo={zlib_year_to}"
        + "/api/v1/search/videos?"
        + "/s/{search_query}/?page={pageno}"
        + "/search?"
        + "?"
        + '/search'
        + '?'
        + ('&ceid=%s' % ceid)
        + f'&async=_fmt:json,p:1,ijn:{params["pageno"] - 1}'
        + google_info['subdomain']
        + urlencode(
        + urlencode({'q': query, 'tbm': "isch", **google_info['params'], 'asearch': 'isch'})
        + [
        .get('appendContinuationItemsAction')['continuationItems'][0]
        .get('contents', [])
        .get('itemSectionRenderer')['contents']
        .get('primaryContents', {})
        .get('sectionListRenderer', {})
        .get('twoColumnSearchResultsRenderer', {})
        .replace('%GROUP_BY%', ' '.join(group_by))
        .replace('%LANGUAGE%', language)
        .replace('%SELECT%', ' '.join(select))
        .replace('%WHERE%', '\n  '.join(where))
        .replace('%WIKIBASE_LABELS%', '\n      '.join(wikibase_label))
        :param url_id: ID matching one key in ``external_urls.json`` for
        :param url_path_prefix: Path prefix if the values are of format
        <small><a href="https://www.edrdg.org/wiki/index.php/JMdict-EDICT_Dictionary_Project">JMdict</a> 
        <ul>
        >>> build_sb_cookie(cookie_params)
        >>> cookie_params = {'v': '1', 'vm': 'p', 'fl': '1', 'vl': 'lang_fr'}
        ?item wikibase:apiOutputItem mwapi:item.
        a ``searxng_locale`` key should be in the dictionary.
        abstract: str = eval_xpath_getindex(entry, xpath_summary, 0).text
        accepted_date = eval_xpath_getindex(
        actual_token = http_get(
        address = {}
        address.update(
        address_name = address_raw.get(result['type'])
        add_date(p)
        add_label(p)
        after_date = datetime.now() - AGO_TIMEDELTA[time_range]
        ago = AGO_RE.search(pub_date)
        album_info = entry.get("albumInfo", {})
        alt_forms = []
        and "premium feature" in json_data["message"].lower()
        and "URL Decoded:" not in zero_click
        and "Your user agent:" not in zero_click
        and <a href="https://www.edrdg.org/enamdict/enamdict_doc.html">JMnedict</a> 
        and img_src is None
        and json_data["status"].lower() == "fail"
        and len(infobox_attributes) == 0
        and len(infobox_content) == 0
        and len(infobox_urls) == 1
        and len(title) > len(domain)
        and not title.startswith(domain + " ")
        and not title.startswith(domain + "/")
        answer_type = search_res.get("AnswerType")
        api_key=api_key,
        app_content = (
        app_id = item.get('id')
        app_title = extract_text(app.xpath('./div/h4[@class="package-name"]/text()'))
        app_url = app.xpath('./@href')[0]
        arch = item["pkg_arch"]
        architectures = []
        arch_path = arch_path.group(0)
        arch_path = void_arch
        args = {
        args["ct"] = params["searxng_locale"].split("-")[0].upper()
        args["filter"] = ",".join(filters)
        args["goggles_id"] = Goggles
        args["key"] = api_key
        args["mailto"] = mailto
        args["p"] = safe_search
        args["s"] = (params["pageno"] - 1) * 100
        args["since"] = (datetime.now() - relativedelta(**kwargs)).strftime("%Y%m%d")  # type: ignore
        args["sort"] = ads_sort
        args["spellcheck"] = "1"
        args["start_date"] = start_date.isoformat()
        args["time"] = params["time_range"]
        args['api_key'] = api_key
        args['count'] = 10
        args['count'] = 50
        args['country'] = eng_region.split('_')[1]
        args['created_after'] = datetime.timestamp(created_after)
        args['date'] = time_range_map[params['time_range']]
        args['date_from'] = time_range_map[params['time_range']]
        args['fmt'] = search_type
        args['l'] = q_locale.split('_')[0]
        args['language'] = engine_language
        args['llm'] = 'false'
        args['locale'] = q_locale
        args['locale'] = q_locale.lower()
        args['localization'] = eng_region
        args['lr'] = 'lang_' + params['language'].split('-')[0]
        args['lui'] = engine_language
        args['offset'] = (params['pageno'] - 1) * args['count']
        args['p'] = params['pageno']
        args['page'] = params["pageno"]
        args['page'] = params['pageno']
        args['s'] = 10 * (params['pageno'] - 1)
        args['s'] = params['safesearch']
        args['safesearch'] = params['safesearch']
        args['segment'] = 'startpage.udog'
        args['srenablerewrites'] = '1'
        args['tgp'] = 3
        args[f"filters[content_type:{content_type}]"] = 1 if content_type in adobe_content_types else 0
        articles = int(cols[4].replace(',', '').replace(',', ''))
        attachments = result.get('media_attachments', [])
        attributes = []
        attributes.append(WDAmountAttribute(name))
        attributes.append(WDArticle('en'))  # wikipedia (english)
        attributes.append(WDAttribute(name))
        attributes.append(WDDateAttribute(name))
        attributes.append(WDImageAttribute(name, url_id, priority))
        attributes.append(WDLabelAttribute(name))
        attributes.append(WDURLAttribute(name, url_id, url_path_prefix, kwargs))
        attribute_result = {key: value['value'] for key, value in result.items()}
        attrs = result["attributes"]
        author = ecma_unescape(photo.get('realname', ''))
        author = extract_text(result_dom.xpath(author_xpath))
        author = item["author"]
        author = item["result"].get('iptc', {}).get('creator')
        author = None
        authors = ', '.join(author['name'] for author in result['extension'].get('authors', []))
        authors = _extract_authors(item)
        authors, journal, publisher, publishedDate = parse_gs_a(
        authors: list[str] = doc["author"]
        authors: list[str] = [" ".join(author["creator"].split(", ")[::-1]) for author in record["creators"]]
        authors: list[str] = [author.text for author in eval_xpath_list(entry, xpath_author_name)]
        authors: list[str] = [author[0]["name"] for author in result.get("authors", [])]
        authors: list[str] = []
        authors: set[str] = set()
        author_obj = auth.get("author", {})
        auth_plugin=auth_plugin,
        avatar = post.get('avatar_template', '').replace('{size}', '96')
        babel_lang = lang.get('iso_639')
        babel_lang = map_lang.get(eng_lang, eng_lang)
        babel_region_tag = {'no_NO': 'nb_NO'}.get(eng_tag, eng_tag)  # norway
        babel_tag = ddg_lang_map.get(eng_lang, eng_lang)
        backlink = tineye_match['backlinks'][0]
        base_image_url = result['thumbnail'].split("?")[0]
        base_url
        base_url = 'https://' + netloc + '/wzh/index.php?'
        base_url.rstrip("/")
        bd:serviceParam wikibase:endpoint "www.wikidata.org";
        best_price = extract_text(eval_xpath(result, ".//a[contains(@class, 'listview__price-link')]")).split(" ")
        bing_ui_lang = eng_lang.lower()
        boundingbox = None
        buffer: list[str] = []
        by <a href="https://www.edrdg.org/edrdg/licence.html">EDRDG</a>, CC BY-SA 3.0.</small>
        CACHE.set("X-S2-UI-Version", value=ret_val, expire=300)
        CACHE.set('ahmia-tokens', token_str, expire=60 * 60)
        CACHE.set(key="code", value=token, expire=3600)
        CACHE.set(SECRET_KEY_DB_KEY, secret_key)
        catalog_engine2code[native_name] = lang_code
        catch_bad_response(resp)
        categ = eval_xpath(result, './/a[contains(@class, "category")]')
        categ = search_categories.get(entry_id[:2])
        category = eval_xpath_getindex(result, xpath_category, 0, '')
        category = result.get("category/name")
        category=bt4g_category,
        cc_tag = parse_qs(urlparse(href).query)['cc'][0]
        ceid_lang.split('-')
        channel = item["channel"]
        channel_url = absolute_url(f'/channel/{channel_result["channel_id"]}')
        chars: ' '
        cids = cid_re.search(resp.content.decode())
        claim_id = item["claimId"]
        classes = extract_text(eval_xpath(result, "./@class")).split(" ")
        cleaned_url = parse_qs(urlparse(raw_url).query).get('redirect_url', [''])[0]
        cloud_cover=data["cloudCover"] * 100,
        cloud_cover=data["cloudcover"],
        cloud_cover=data["cloud_cover"],
        code: str = match['fragment']
        codec = result.get('codec')
        code_matches = [
        cols = row.xpath('./td')
        cols = [c.text_content().strip() for c in cols]
        col_from, col_to = td_list
        col_names = [cn[0] for cn in cur.description]
        col_names = [i[0] for i in cur.description]
        comments = result.get('posts_count', 0)
        comments = _extract_comments(item)
        comments: str = ""
        comments: str = "" if comments_elements is None else comments_elements.text
        comments: str = (
        comments_elements = eval_xpath_getindex(entry, xpath_comment, 0, default=None)
        condition=WEATHERKIT_TO_CONDITION[data["conditionCode"]],
        condition=WMO_TO_CONDITION[data["weather_code"]],
        condition=WWO_TO_CONDITION[data["weatherCode"]],
        conflict = engine_traits.custom["ui_lang"].get(sxng_tag)  # type: ignore
        conflict = engine_traits.languages.get(sxng_lang)
        conflict = engine_traits.languages.get(sxng_tag)
        conflict = engine_traits.regions.get(sxng_tag)
        connect.row_factory = sqlite3.Row
        content += " %s" % result['owner']['display_name']
        content += " // score: %s" % result['score']
        content = " ".join(f"{engdef}." for _, engdef, _ in definitions)
        content = " / ".join(item.get("first_sentence", []))
        content = ""
        content = "-"
        content = "No description available"
        content = "[%s]" % ", ".join(result['tags'])
        content = ' '.join(content.split())
        content = ' / '.join([x for x in [pub_origin, pub_date] if x])
        content = ''
        content = 'Category: "{category}". Downloaded {downloads} times.'
        content = '{0} - {1}'.format(metadata['du'], info)
        content = (
        content = audio_data["album"] + " - " + content
        content = content + search_res.get("Abstract", "")
        content = content + search_res.get("Definition", "")
        content = content.format(category=category, downloads=downloads)
        content = content.strip().replace('\n', ' | ')
        content = content[date_pos:]
        content = eval_xpath(result, './/p')
        content = eval_xpath_getindex(result, './/div[contains(@class, "compText")]', 0, default='')
        content = eval_xpath_list(result, ".//p[contains(@class, 'desc')]")
        content = extract_text(
        content = extract_text(content)
        content = extract_text(eval_xpath(item, ".//span[contains(@class, 'sds-comps-text-type-body1')]"))
        content = extract_text(eval_xpath(newsitem, './/div[@class="snippet"]'))
        content = extract_text(eval_xpath(result, './/p'))
        content = extract_text(eval_xpath(result, './/span[@class="wMUdtb"]'))
        content = extract_text(eval_xpath(result, content_xpath))
        content = extract_text(eval_xpath(result, publishedDate_xpath)) + extract_text(
        content = extract_text(eval_xpath(spot, './/div[@class="LbQbAe"]'))
        content = extract_text(eval_xpath_getindex(entry, './p', 0))
        content = extract_text(item.xpath('.//div[contains(@class, "text-layout")]//p[contains(@class, "star-wiki")]'))
        content = extract_text(item.xpath('.//p[@class="res-desc"]'))
        content = extract_text(item.xpath('.//p[@class="txt-info"]'))
        content = extract_text(result.xpath('.//div[@class="files"]'))
        content = extract_text(result.xpath('.//div[@class="searchresult"]'))
        content = extract_text(result.xpath('.//p'))
        content = extract_text(result_dom.xpath(content_xpath))
        content = hit.get("url") or html_to_text(hit.get("comment_text")) or html_to_text(hit.get("story_text"))
        content = hit['highlights'][0]['value']
        content = hit['result'].get('title_with_featured', '')
        content = html.tostring(excerpt, encoding='unicode', method='text', with_tail=False)
        content = html_to_text(ecma_unescape(photo.get('description', '')))
        content = html_to_text(item["snippet"])
        content = html_to_text(result.get("snippet", ""))
        content = html_to_text(result.get('snippet', ''))
        content = html_to_text(res['description'])
        content = None
        content = query(result, content_query)[0]
        content = result.xpath('.//div[@class="subhead"]/text()')
        content = result["description"][:128]
        content = result['comment'].get('content', '').strip()
        content = result['post'].get('body', '').strip()
        content = result['snippet']['description']
        content = title
        content = unescape(entry.get("abs", ""))
        content = [
        content = [f'Price: {price:.2f} {currency}', f'Platforms: {platforms}']
        content = [item.get(i) for i in ['language', 'description'] if item.get(i)]
        content = []
        content = _field_txt(medline_citation, ".//Abstract/AbstractText//text()")
        content.append(f"{i18n_book_rating}: {book_rating}")
        content.append(f"{i18n_file_quality}: {file_quality}")
        content.append(f"{i18n_language}: {language.capitalize()}")
        content: str = ""
        content: str = extract_text(content, allow_none=True)
        content: str = extract_text(eval_xpath(result, ".//div[@class='gs_rs']")) or ""
        content: str = _reconstruct_abstract(item.get("abstract_inverted_index")) or ""
        content=' | '.join(content_parts),
        contents = []
        content_between_tags = extr(
        content_id = video_data.get("content_id", "")
        content_items = [
        content_parts.append(f"Country: {', '.join(countries)}")
        content_parts.append(f"Director(s): {', '.join(directors)}")
        content_parts.append(f"Genre(s): {', '.join(genres)}")
        content_parts.append(f"Original title: {original_title}")
        content_parts.append(f"Rating: {item['rating']}/10 ({item['stats']['ratingCount']} votes)")
        content_parts.append(item['category'])
        conversion_rate = float(json.loads(json_resp)["to"][0]["mid"])
        conversion_rate,
        cookie['language'] = engine_language
        cookie['language_ui'] = engine_language
        cookie['search_results_region'] = engine_region
        cookie_parts.append(f"{key}={value}")
        copyright_notice = item["result"].get('iptc', {}).get('copyright_notice')
        count += 1
        countries = [country['name'] for country in item['countries']]
        country:
        countrycodes.add(_reg)
        country_tag = v["value"]
        counts = result['counts']
        cover = ""
        created_after = datetime.now() - time_delta
        created_at = extract_text(
        create_time = extract_text(video_block.xpath('.//span[contains(@class, "info__create-time")]'))
        cur.execute(query_to_run, query_params)
        currency = item.get('price', {}).get('currency', 'USD')
        currency = key_name.split(':')
        current=_weather_data(geoloc, json_data["currentWeather"]),
        current=_weather_data(geoloc, json_data["current_condition"][0]),
        current=_weather_data(location, json_data["current"]),
        cve_id = item["cve"]["id"]
        cvss_score = info.get("baseScore")
        data = get_data(result, user_language, link_keys)
        data = json.loads(text)
        data = post['data']
        data = resp.json()
        data.append(results.types.Translations.Item(text=result['translation']))
        data={'grant_type': 'client_credentials'},
        database=database,
        data["api"] = "d.js"
        data["b"] = ""
        data["dc"] = offset + 1
        data["df"] = t_range
        data["kl"] = "wt-wt"
        data["kl"] = eng_region
        data["nextParams"] = ""
        data["o"] = "json"
        data["s"] = offset
        data["v"] = "l"
        data['facetFilters'] = facet_filters
        data_exposure_log = video_block.get('data-exposure-log')
        data_image_map[img_id] = data_image
        data_image_map[last.group(1)] = last.group(2)
        date = datetime.fromisoformat(day["date"])
        date = datetime.now()  # needed in case no dcdate is available for an item
        date = datetime.strptime(item["cve"]["published"], "%Y-%m-%dT%H:%M:%S.%f")
        date = datetime.strptime(item["updated_at"].split("T")[0], "%Y-%m-%d")
        date = eval_xpath_getindex(result, '//div[@class="released"]/text()', 0, default=None)
        date = extract_text(eval_xpath(result, date_xpath))
        date_format = WDDateAttribute.DATE_FORMAT.get(precision)
        date_pos = content.find('...') + 4
        date_string = content[0 : date_pos - 5]
        db=db,
        decode_responses=True,
        definitions = get_definitions(page)
        definitions.append(
        del extratags['image']
        del extratags['wikimedia_commons']
        del row['_id']
        delimiter = engine_settings['delimiter']
        depth = float(cols[11].replace('-', '0').replace(',', ''))
        desc = extract_text(info_row.xpath('./td[@class="desc-bot"]')[0])
        description = item["cve"]["descriptions"][0]["value"]
        description = item["description"]
        description = item["description"] or ""
        directors = [director['name'] for director in item['directors']]
        display_name = author_obj.get("display_name")
        doc = html.fromstring(resp.text)
        doctype = extract_text(eval_xpath(result, doctype_xpath))
        doc_type = extract_text(eval_xpath(result, ".//span[@class='gs_ctg2']"))
        doc_url = eval_xpath_getindex(result, ".//div[@class='gs_or_ggsm']/a/@href", 0, default=None)
        doi = _doi_to_plain(item.get("doi"))
        doi = _field_txt(medline_citation, ".//ELocationID[@EIdType='doi']/text()")
        doi: str = "" if doi_element is None else doi_element.text
        doi: str = result.get("doi")
        doi_element = eval_xpath_getindex(entry, xpath_doi, 0, default=None)
        dom = fromstring(resp.text)
        dom = html.fromstring(resp.text)
        dom, "//div[contains(@class, 'sds-comps-base-layout') and contains(@class, 'sds-comps-full-layout')]"
        dom, '//a[@class="active" and contains(@href,"/suchen/dudenonline")]/span/text()', 0, default=None
        dom, '//div[@id="searchpage-root"]//div[@class="Layout--left"]/div[@class="f2c528"]'
        domain = lang2domain.get(lang, f'{lang}.search.yahoo.com')
        downloads = int_or_zero(result.xpath(xpath_downloads))
        duration = extract_text(
        duration = item["duration"]
        duration = None
        duration = result.get('duration')
        duration = utils.parse_duration_string(item["duration"])
        earned = extract_text(result_dom.xpath(earned_xpath))
        elem = eval_xpath_getindex(xml, xpath_str, 0, default="")
        elements = raw_result.split(delimiter['chars'], maxsplit=len(delimiter['keys']) - 1)
        elif "Topics" in ddg_result:
        elif "track" == itemtype:
        elif 'url_z' in photo:
        elif 'z' in photo['sizes']['data']:
        elif ddg_category == 'news':
        elif ddg_category == 'videos':
        elif defn_raw.get('info'):
        elif int(comments) > 1:
        elif item["asset_type"].lower() == "audio":
        elif item["asset_type"].lower() == "video":
        elif item_type == 'video':
        elif key_type == 'list':
        elif mastodon_type == "hashtags":
        elif piped_filter == 'music_songs':
        elif properties.get('osm_type') == 'R':
        elif properties.get('osm_type') == 'W':
        elif result.get("downloadUrl"):
        elif result.get("id"):
        elif result.get("sourceFulltextUrls"):
        elif result["name"] == "resources":
        elif result['mediaType'] == 'video':
        elif result_type == 'Animated':
        elif search_type == "images":
        elif search_type == "news":
        elif url:
        else:
        else:  # 400
        embed_url = None
        enclosure_url = enclosure.get('url')
        endpoint=endpoint,
        endpos = url_string.rfind(ending)
        end_pos = data_image.rfind('=')
        end_time = None
        engine_settings["base_url"] = engine_settings["base_url"][:-1]
        engine_traits.all_locale = ENGINE_TRAITS["z-library"]["all_locale"]
        engine_traits.custom = ENGINE_TRAITS["z-library"]["custom"]
        engine_traits.custom["sort"].append(x.get("value"))
        engine_traits.custom["ui_lang"][sxng_tag] = ui_lang
        engine_traits.custom['ceid'][locales.region_tag(locale)] = ceid
        engine_traits.custom['wiki_netloc'][eng_tag] = wiki_url.netloc
        engine_traits.languages = ENGINE_TRAITS["z-library"]["languages"]
        engine_traits.languages[sxng_lang] = 'lang_' + eng_lang
        engine_traits.languages[sxng_lang] = eng_lang
        engine_traits.languages[sxng_tag] = bing_ui_lang
        engine_traits.languages[sxng_tag] = eng_lang
        engine_traits.languages[sxng_tag] = eng_tag
        engine_traits.languages[sxng_tag] = eng_tag  # type: ignore
        engine_traits.languages[sxng_tag] = lang_tag
        engine_traits.regions[sxng_tag] = eng_tag
        eng_country = x.get("value")
        eng_lang = parse_qs(urlparse(href).query)['setlang'][0]
        eng_lang = x.get("value")
        eng_lang: str = traits.get_language(sxng_lang, 'English')  # type: ignore
        eng_tag = cols[3]
        eng_tag = extract_text(eval_xpath_list(a, ".//span"))
        eng_tag = item['code']
        eng_tag = item['locale']
        eng_tag = lang.group(1)
        eng_tag = lang['name']
        eng_tag = option.get('value')
        entity_id = result['item']['value'].replace('http://www.wikidata.org/entity/', '')
        entity_url = attribute_result['item']
        entry_id = entry['id']
        environment_variables = engine_settings['environment_variables']
        episode = result['Episode']
        error_code = data.get('error_code')
        except (ValueError, TypeError) as e:
        except (ValueError, TypeError):
        except (ValueError, TypeError, SearxEngineXPathException):
        except babel.UnknownLocaleError:
        except Exception as e:  # pylint: disable=broad-except
        except Exception:  # pylint: disable=broad-except
        except json.JSONDecodeError:
        except JSONDecodeError:
        except KeyError:
        except parser.ParserError:
        except SearxEngineXPathException:
        except ValueError:
        except:  # pylint: disable=bare-except
        excerpt = result.xpath('.//div[@class="torrent_excerpt"]')[0]
        expected_start = _page_offset(resp.search_params.get("pageno", 1))
        explicit = "No"
        extra = f' ({extra})' if extra else ''
        extra = []
        extratags = result['extratags']
        fargs['query'] = query
        feels_like=weather.Temperature(val=data["apparent_temperature"], unit="°C"),
        feels_like=weather.Temperature(val=data["FeelsLikeC"], unit="°C"),
        feels_like=weather.Temperature(val=data['temperatureApparent'], unit="°C"),
        fields = result["_source"]
        files = (result.xpath('.//span[@class="torrent_files"]/text()') or ['1'])[0]
        filesize = 0
        filesize = enclosure.get('length')
        filesize = eval_xpath_getindex(result, xpath_filesize, 0, '')
        filesize = extract_text(eval_xpath(result, './/td[contains(@class, "size")]/text()'))
        filesize = f"{files_data[FILESIZE]} {files_data[FILESIZE_MULTIPLIER]}"
        filesize = humanize_bytes(int(size_str))
        filesize = result.xpath('.//span[@class="torrent_size"]/text()')[0]
        files_data = extract_text(result.xpath('.//div[@class="tail"]')).split()
        file_size = item.get('gsa', {}).get('file_size')
        filter_category = filter_category_and_value[0]
        filter_category_and_value = query_part.split("-", 1)
        finally:
        firstURL = ddg_result.get("FirstURL")
        fixed_date = datetime.strptime(p_date, '%Y-%m-%dT%H:%M:%S%z')
        for author in eval_xpath_list(medline_citation, "./Article/AuthorList/Author"):
        for backlink_json in match_json["backlinks"]:
        for code, name in territories.items():
        for css_class in classes:
        for date_format in ['%Y-%m-%dT%H:%M:%SZ', '%Y-%m-%d', '%Y-%m', '%Y']:
        for definition in info['definitions']:
        for def_item in src.xpath('following-sibling::ul[1]/li'):
        for discovery_filter in discovery_filters:
        for doc in result.get("alternatePaperLinks", []):
        for domain in resp.text.split():  # type: ignore
        for format_name in ('SVG', 'PNG', 'WebP'):
        for hit in section['hits']:
        for i in range(len(elements)):  # pylint: disable=consider-using-enumerate
        for i in result.get("authors", []):
        for i, letter in enumerate(code):
        for i, p_item in enumerate(p_list):
        for image_size in image_sizes:
        for image_type in result["versions"]["svg"]:
        for index, forecast in enumerate(day["hourly"]):
        for item in data.get('data', {}).get('hit', {}).get('imgInfo', {}).get('item', []):
        for item in data["data"]:
        for item in data["searchList"]["searchList"]:
        for item in desc.split('|'):
        for item in entry:
        for item in eval_xpath(section, './/a'):
        for item in info.get('about', []):
        for item in items:
        for item in json_resp.get('images', []):
        for item in json_resp.get('news', []):
        for item in json_resp.get('videos', []):
        for item in mainline_items:
        for item in results_categ.get('results', []):
        for item in section.get('list_container', []):
        for item in url_list:
        for item in [info.get('subtitle'), info.get('description')]:
        for k, v in linked_terms.items():
        for key in data_of_interest:
        for kvmap in search_keys(query):
        for lang in v['langs']:
        for lang_tag in babel.languages.get_official_languages(cc_tag, de_facto=True):
        for lang_tag in babel.languages.get_official_languages(country_tag, de_facto=True):
        for line, code in result["lines"].items():
        for link in result.xpath(xpath_torrent_links):
        for locale in get_official_locales(code, engine_traits.languages):
        for match in matches:
        for n in node.xpath(x):
        for p in content:
        for param in params:
        for part in query_parts:
        for pos in positions:
        for rate_plan in item.get("rate_plans", []):
        for rec in item['recordings']:
        for repo in repositories:
        for result in eval_xpath_list(dom, results_xpath):
        for result in json_data['results']:
        for result_key, regex in parse_regex.items():
        for result_key, regex in _compiled_parse_regex.items():
        for row in cur.fetchall():
        for row in cur:
        for row in _valkey_client.hscan_iter(qset, match='*{}*'.format(rest)):
        for spec in eval_xpath_list(result, ".//div[contains(@class, 'specs-grid__item')]"):
        for subpod in pod['subpods']:
        for subpod in subpods:
        for suggestion in eval_xpath(dom, suggestion_xpath):
        for suggestion in eval_xpath_list(dom, '//div[contains(@class,"AlsoTry")]//td'):
        for suggestion in json_results['tags']:
        for sxng_locale in sxng_locales:
        for sxng_tag in sxng_tag_list:
        for t in adobe_content_types:
        for title_raw in page['japanese']:
        for translation in definition['list']:
        for video_container in section.get('itemSectionRenderer', {}).get('contents', []):
        for w in content.split(" "):
        for wd_result in wikidata_json.get('results', {}).get('bindings', {}):
        for word in translation["list"]:
        for word in translation['word_choices']:
        for x in ['state', 'country']:
        for x, y in [
        for xpath in (release_year_xpath, score_xpath, cast_xpath):
        for _, item_data in json_resp['initialState']['serpList']['items']['entities'].items():
        forecast_data = _weather_data(geoloc, forecast)
        forecast_data = _weather_data(location, hourly_data)
        forecast_data.datetime = weather.DateTime(datetime.fromtimestamp(time))
        forecast_data.datetime = weather.DateTime(forecast_time)
        forecast_time = date_parser.parse(forecast['forecastStart'])
        form = form[0]
        formatted_date = datetime.fromtimestamp(date.timestamp())
        formatted_date = datetime.fromtimestamp(release_date.timestamp())
        formatted_date = datetime.fromtimestamp(unix_date)
        formatted_duration = format_duration(duration)
        form_vqd = eval_xpath(form, '//input[@name="vqd"]/@value')[0]
        fp['query'] = query
        freshness_date = item["result"].get("freshness_date")
        FROM film
        fullDescription = entry.find("description").text.split('<br>')
        fullsize_image = re.sub(r'/\d{6,}/', '/', thumb).replace("smaller_square", "large")
        genres = [genre['label'] for genre in item['genresInfos']]
        geojson = r.get('geometry')
        geojson = {'type': 'Point', 'coordinates': [result['lon'], result['lat']]}
        github_slug = re.sub(r"-(32bit|dbg)$", "", result['name'])
        guest_client_id = get_client_id()
        headers:
        headers=params["headers"],
        headers={
        headers["Accept-Language"] = f"{ui_lang},{ui_lang}-{ui_lang.upper()};q=0.7"
        highlight_groups = [highlight_group['indices'] for highlight_group in match['matches']]
        host=host,
        hourly_data = {}
        href = base64.urlsafe_b64decode(href + '====')
        href = base_url + page_a.attrib.get('href')
        href = eval_xpath_getindex(result, './article/a/@href', 0)
        href = href.decode()
        href = href.split('/')[-1]
        href = href.split('?')[0]
        href = href[href.index(b'http') :].split(b'\xd2')[0]
        href = urljoin(url, eval_xpath_getindex(result, './td[contains(@class, "name")]/a[2]/@href', 0))
        href = urljoin(url, link.attrib.get('href'))
        html_data = html.fromstring(resp.text)
        html_sample = unescape(html.tostring(html_data, encoding='unicode'))
        html_url = ""
        html_url: str = ""
        https://dev.arcxp.com/photo-center/image-resizer/resizer-v2-how-to-transform-images/#query-parameters
        http_response = get(SPARQL_ENDPOINT_URL + '?' + urlencode({'query': query}), headers=get_headers(), **kwargs)
        http_response = get(SPARQL_EXPLAIN_URL + '&' + urlencode({'query': query}), headers=get_headers())
        http_response = post(SPARQL_ENDPOINT_URL, data={'query': query}, headers=get_headers(), **kwargs)
        http_response = post(SPARQL_EXPLAIN_URL, data={'query': query}, headers=get_headers())
        humidity=weather.RelativeHumidity(data["humidity"] * 100),
        humidity=weather.RelativeHumidity(data["humidity"]),
        humidity=weather.RelativeHumidity(data["relative_humidity_2m"]),
        icon_categories = [category.title() for category in result["categories"]]
        icon_name, tags = result
        if "-" in result_len_container:
        if ".svg" in img_src_name.split()[0]:
        if "/sorry/index?continue" in resp.headers["Location"]:
        if "album" == itemtype:
        if "citationStats" in result:
        if "content" in infobox:
        if "continuationItemRenderer" in section:
        if "FirstURL" in ddg_result:
        if "lending_identifier_s" in item:
        if "pubDate" in result:
        if "published" in record and "date-parts" in record["published"]:
        if "resource" in record and "primary" in record["resource"] and "URL" in record["resource"]["primary"]:
        if "videoId" not in result["id"]:
        if "videos" in album_info:
        if "window.searchId = " in line:
        if '-' in babel_region_tag:  # pyright: ignore[reportOperatorIssue]
        if '-' not in bing_ui_lang:
        if 'chars' not in engine_settings['delimiter'] or 'keys' not in engine_settings['delimiter']:
        if 'descriptionSnippet' in section:
        if 'displayMapRegion' in result:
        if 'highlights' in result:
        if 'id' not in result:
        if 'logo' in result:
        if 'n' in photo['sizes']['data']:
        if 'ownerNsid' not in photo:
        if 'q' in entry:
        if 'rank' in entry:
        if 's' in entry:
        if 'subpods' not in pod:
        if 'telephone' in result:
        if 'url_n' in photo:
        if 'url_o' in photo:
        if 'videoRenderer' not in section:
        if 'y' in entry:
        if (
        if accepted_date is not None:
        if address.get('name'):
        if address_raw.get('address29'):
        if ago:
        if api_result.get('type') == 'standard':
        if author:
        if authors:
        if babel_tag == 'skip':
        if baidu_category == 'general':
        if baidu_category == 'it':
        if c == '{{QUERY}}':
        if cached_xpath:
        if categ is None:
        if category:
        if cc_tag == 'clear':
        if ceid in _skip_values:
        if ceid_region in ['AT', 'BE', 'CH', 'IL', 'SA', 'IN', 'BD', 'PT']:
        if ceid_region.lower() == ceid_lang:
        if chinaso_news_source == 'EPAPER':
        if cids and len(cids.groups()):
        if codec and codec.lower() != 'unknown':
        if commonprefix([realpath(query_path), working_dir]) != working_dir:
        if conflict:
        if content:
        if content_between_tags == "fail":
        if copyright_notice:
        if create_time:
        if current.get('content_txt'):
        if date:
        if date_format is not None:
        if ddg_category == 'images':
        if defn_raw.get('restrictions'):
        if defn_raw.get('tags'):
        if doc_type == "[PDF]":
        if doi:
        if duration and duration > timedelta(minutes=60):
        if duration:
        if earned:
        if endpos > -1:
        if end_pos > 0:
        if eng_country == 'ZZ':
        if eng_country in skip_countries:
        if eng_lang == 'wt_WT':
        if eng_lang in ("", "_empty", "nl-BE", "und") or eng_lang.startswith("anti__"):
        if eng_lang is None:
        if eng_tag == 'all':
        if eng_tag == 'oc':
        if eng_tag == 'wt-wt':
        if eng_tag in ('en_EN', 'ar_AA'):
        if eng_tag in locale_lang_list:
        if eng_tag in skip_eng_tags:
        if entity_url not in seen_entities and entity_url not in DUMMY_ENTITY_URLS:
        if entry.get("date") and entry.get("duration"):
        if entry.get("description"):
        if entry.get("downloads"):
        if entry.get("likes"):
        if entry.get("publish_time"):
        if entry.get("tags"):
        if entry.get("time"):
        if entry.get("timestamp"):
        if entry.get("VideoPubDate"):
        if error_code == 24:
        if eval_xpath(item, "./span[contains(@class, 'tooltip')]"):
        if expected_start != start:
        if file_size:
        if filter_category in filter_types:
        if firstURL is not None and text is not None:
        if first_result:
        if freshness_date:
        if get_key_rank(k) is None:
        if ghc_strip_new_lines:
        if ghc_strip_whitespace:
        if guest_client_id:
        if hide_obsolete:
        if huggingface_endpoint != 'models':
        if i > 0 and ghc_insert_block_separator:
        if iframe_src:
        if image is None and len(attributes) == 0 and len(urls) == 1 and len(relatedTopics) == 0 and len(content) == 0:
        if imagelink is not None:
        if image_url:
        if img_src:
        if int(comments) > 1:
        if isinstance(answer, str) and answer_type not in ["calc", "ip"]:
        if isinstance(display_name, str) and display_name != "":
        if isinstance(name, str) and name != "":
        if is_onion:
        if item.get('date'):
        if item:
        if item["asset_type"].lower() in ["image", "premium-image", "illustration", "vector"]:
        if item_type in ('story', 'webview'):
        if journal == "…":
        if json_data.get('status') == 404:
        if k == "all":
        if k in ignore_keys:
        if key_type == 'hash':
        if k_label:
        if l and l.territory:
        if labels is None:
        if lang:
        if language in supported_languages:
        if latitude and longitude:
        if len(authors) > 15:
        if len(best_price) > 1:
        if len(content) > 300:
        if len(currency) > 1:
        if len(elements) != len(delimiter['keys']):
        if len(filter_category_and_value) < 2:
        if len(images) > 0:
        if len(index) != 8:
        if len(iso2) == 2:
        if len(lang) == 2:
        if len(links) == 2:
        if len(params['searxng_locale'].split('-')) > 1:
        if len(q) == 1:
        if len(result.xpath("./td")) < 9:
        if len(result_len_container) > 0:
        if len(stats) == 3:
        if len(td_list) != 2:
        if len(thumbnail_src) < 25:
        if len(unaccented_name) == len(unaccented_name.encode()):
        if len(upload_time) > 0:
        if len(video_length) > 0:
        if len(video_tag):
        if len(x) > 1:
        if len(_content):
        if length.tm_hour:
        if length:
        if lic.get('spdx_id'):
        if line.startswith("[{"):
        if link is None:
        if loc.english_name is None:
        if mainline_type != qwant_categ:
        if mainline_type == 'ads':
        if mastodon_type == "accounts":
        if match:
        if minutes > 0:
        if mtype in ["audio", "video"]:
        if mtype in ["image"] and subtype in ["bmp", "gif", "jpeg", "png"]:
        if mtype:
        if netloc != 'wiki.archlinux.org':
        if nextpage:
        if not any(query_part in keyword for query_part in query_parts):
        if not auth:
        if not babel_lang:
        if not cols:
        if not content:
        if not engine_settings.get(cfg_name):
        if not engine_settings.get(opt, ""):
        if not entry.get("title") or not entry.get("play_url"):
        if not entry.get("title") or not entry.get("url"):
        if not entry.get("titleEsc") or not entry.get("url"):
        if not ghc_highlight_matching_lines:
        if not img_list:
        if not img_src:
        if not info_js:
        if not isabs(engine_settings['working_dir']):
        if not isinstance(result.get('extratags'), dict):
        if not item.get("imageinfo", []):
        if not kvmap:
        if not metadata:
        if not native_name:
        if not properties:
        if not published:
        if not raw_value:
        if not region:
        if not resp.ok:
        if not resp.ok:  # type: ignore
        if not result.get("title"):
        if not result:
        if not result["image_id"]:
        if not result['image_id']:
        if not res_url:
        if not ret_val:
        if not rs:
        if not size_data:
        if not subpods:
        if not sxng_locales:
        if not thumbnail and video_id:
        if not tineye_match['backlinks']:
        if not title:
        if not tmp_result:
        if not url and result.get("links"):
        if not url:
        if not val.strip():
        if not value:
        if not video_id and url and 'youtube.com' in url:
        if not x.get("value").startswith("anti__"):
        if params.get("pageno", 1) - 1:
        if params["searxng_locale"].startswith("zh"):
        if params["time_range"] not in time_range_args:
        if params['time_range'] in time_range_dict:
        if params['time_range']:
        if part == '':
        if parts_of_speech and parts_of_speech[0] == 'Wikipedia definition':
        if piped_filter == 'videos':
        if pod_id == 'Input' or not infobox_title:
        if points != 0 or num_comments != 0:
        if pos == 'Wikipedia definition':
        if properties.get('extent'):
        if properties.get('osm_type') == 'N':
        if pubdate_original:
        if published:
        if publishedDate is not None:
        if publishedDate:
        if publisher in url:
        if pub_date is not None:
        if pub_type:
        if query_key == '{{KEY}}':
        if query_value == '{{VALUE}}':
        if raw_date:
        if recipe['submissionDate']:
        if recipe['subtitle']:
        if record["type"] == "book-chapter":
        if record["type"] == "component":
        if region == 'skip':
        if res:
        if resp.search_params['discovery']:
        if resp.status_code == 403:
        if resp.status_code == 422:
        if result.get("encompassingWork"):
        if result.get("endTimeStamp"):
        if result.get("journals"):
        if result.get("startTimeStamp"):
        if result.get("tags"):
        if result.get("themes"):
        if result.get('has_accepted_answer'):
        if result.get('snippet', '').startswith('#REDIRECT'):
        if result.get('urls'):
        if result["kind"] in ("track", "playlist"):
        if result["name"] == "resourceGroups":
        if result["thumbnail"] is not None:
        if result['extension'].get('publishingDate'):
        If result['extratags']['wikidata'] or r['extratags']['wikidata link']:
        if result['images']['image700']['height'] > 400:
        if result['is_answered']:
        if result['item'].get('creators'):
        if result['journal']:
        if result['mediaType'] in ('photo', 'illustration', 'vector'):
        if result['post'].get('thumbnail_url'):
        if result['status'] == 'OBS':
        if result['status'] in pdb_unpublished_codes:
        if result['teaser']['image']:
        if result['type'] == 'music':
        if result['type'] == 'story':
        if result['type'] == 'track':
        if result_data is None:
        if result_type == 'Photo':
        if res['allow_embed']:
        if return_code != 0:
        if rtype == "video":
        if script.status_code != 200:
        if search_results.get("data", {}).get("error_data", {}).get("captchaUrl") is not None:
        if search_type == "web":
        if search_type == 'image':
        if seconds:
        if sectiontitle:
        if self.url_id:
        if self.url_path_prefix:
        if severity and cvss_score is not None:
        if show_avatar and avatar:
        if show_metadata:
        if size:
        if skip_premium and (recipe['isPremium'] or recipe['isPlus']):
        if source is not None:
        if source:
        if srv not in servers:
        if start_time:
        if sxng_tag is None:
        if sxng_tag not in locales.LOCALE_NAMES:
        if tags and len(tags) > 0:
        if tags:
        if ta_link_to_mp4:
        if text.lower().endswith(x):
        if thumbnail and thumbnail.startswith("//"):
        if thumbnail and thumbnail.startswith('data:image'):
        if thumbnail:
        if thumbnail_query:
        if time < json_data["current"]["time"]:
        if timestamp != 0:
        if timestamp:
        if time_range_map.get(params["time_range"]):
        if title == "":
        if title and content and url:
        if title and url:
        if title.startswith('['):
        if torrentfile is None or magnet is None:
        if type(attribute) in (WDAttribute, WDAmountAttribute, WDURLAttribute, WDDateAttribute, WDLabelAttribute):
        if unit is not None:
        if unit.startswith(prefix):
        if url is None and result.get("doi"):
        if url is None or title_tag is None or not urlparse(url).netloc:  # partial url likely means it's an ad
        if url is None:
        if url.startswith("/link?url="):
        if url.startswith('https://wikidata.org'):
        if url.startswith('https://www.bing.com/ck/a?'):
        if url_info[1] != '' and url_info[2] != '':
        if val.startswith('!') and get_node(EXTERNAL_BANGS, val[1:]):
        if value == '' or value is None:
        if value is not None and value != '':
        if value:
        if video_duration:
        if video_id:
        if video_url.startswith("/vc/np"):
        if vqd:
        if wc_search_type == "audio":
        if wc_search_type == "file":
        if wc_search_type == "image":
        if wc_search_type == "video":
        if wd_id and wd_id not in wikidata_ids:
        if x:
        if year < 1584:
        if _reg not in babel_reg_list:
        if _text and premium_keytext in _text:
        iframe_src = f"{base_url}/player/ac{content_id}"
        iframe_src = f"{embed_url}/watch/{video_id}"
        iframe_src = get_embeded_stream_url(result["url"])
        iframe_src = None
        iframe_url = f"https://odysee.com/$/embed/{name}:{claim_id}"
        iframe_url = f"https://player.bilibili.com/player.html?aid={video_id}&high_quality=1&autoplay=false&danmaku=0"
        image = urljoin("https://duckduckgo.com", image)
        imageinfo = item["imageinfo"][0]
        imagelink = eval_xpath_getindex(newsitem, './/a[@class="imagelink"]//img', 0, None)
        images = [attachment['preview_url'] for attachment in attachments if attachment['type'] == 'image']
        image_source = item.get("source")
        image_url = entry.get('i', {}).get('imageUrl')
        image_url = item["url"]
        img = 'https://s3.thehackerblog.com/findthatmeme/' + item['image_path']
        img = extract_text(
        img = extract_text(eval_xpath(spot, './/img[@class="T75of bzqKMd"]/@src'))
        img_format = ' '.join(result.xpath('.//div[@class="imgpt"]/div/span/text()')).strip().split(" · ")
        img_format = None
        img_list = result.get('image_url')
        img_src = (
        img_src = extract_text(eval_xpath(result, img_src_xpath))
        img_src = extratags['image']
        img_src = f"{cdn_base_url}/icons/{icon_name}.svg"
        img_src = f'{cdn_base_url}/{img_format}/{item["Reference"]}.{img_format}'
        img_src = get_external_url('wikimedia_image', extratags['wikimedia_commons'])
        img_src = None
        img_src = result['wikidata']['image']
        img_src = size_data['url']
        img_src = thumbnail_src.replace("b.", ".")
        img_src_name = unquote(img_src.replace(_IMG_SRC_DEFAULT_URL_PREFIX, "").split("?", 1)[0].replace("%20", "_"))
        img_src_name_first = img_src_name
        img_src_name_md5 = md5(img_src_name.encode("utf-8")).hexdigest()
        img_src_name_second = img_src_name
        img_src_size = img_src.replace(_IMG_SRC_DEFAULT_URL_PREFIX, "").split("?", 1)[1]
        img_src_size = img_src_size[img_src_size.index("=") + 1 : img_src_size.index("&")]
        index='multi',
        info = ' - '.join(result.xpath('.//div[@class="mc_vtvc_meta_block"]//span/text()')).strip()
        info = item["cve"].get("metrics", {}).get("cvssMetricV31", [{}])[0].get("cvssData", {})
        infobox: dict[str, t.Any] = search_res.get("Infobox", {})  # pyright: ignore[reportAssignmentType]
        infobox_content.append(f'<li>{pos}{engdef}{extra}</li>')
        infobox_content.append(f'<p><i>Other forms:</i> {", ".join(alt_forms[1:])}</p>')
        infobox_id = abstractURL
        infobox_id = definitionURL
        infobox_id = replace_http_by_https(infobox_id)
        infobox_title = ""
        infobox_title = search_results.xpath(input_xpath)[0].text
        info_js = extr(extract_text(result), '] = ', '};') + '}'
        info_row = rows[i + 1]
        int(key, 16)
        iso2 = language.split("-")[0].split("_")[0]
        issn = _field_txt(medline_citation, "./Article/Journal/ISSN/text()")
        is_official = image_source in ["store", "official"]
        item = MainResult(
        item = res.types.LegacyResult(
        item = res.types.Paper(
        item = results.types.Translations.Item(text="")
        item = results.types.Translations.Item(text=text, synonyms=synonyms)
        item = {
        item = {'title': result['title']}
        item = {'url': url, 'title': title, 'content': content, 'metadata': metadata}
        item = {'url': url, 'title': title, 'content': content, 'thumbnail': thumbnail}
        item = {}
        item.authors = [a.get("given", "") + " " + a.get("family", "") for a in record.get("author", [])]
        item.isbn = record.get("isbn") or [i["value"] for i in record.get("isbn-type", [])]
        item.transliteration = translation['target_transliteration']
        items = enumerate(iterable)
        items = iterable.items()
        items = json.get('content', {}).get('items', [])
        itemtype = extract_text(result.xpath('.//div[@class="itemtype"]')).lower()
        item['hms'] = str(datetime.timedelta(seconds=item['duration']))
        item['publishedDate'] = parser.parse(current['file_modified_dt'])
        item['title'] = current['title_txt_txt_en']
        item['url'] = current['id']
        item_list.append(item)
        item_list.insert(0, results.types.Translations.Item(text=autotranslate.text))
        item_type = item.get('type')
        journal = ""
        journal = _field_txt(medline_citation, "./Article/Journal/Title/text()")
        journal, publisher, pages, volume, number, published_date = _extract_biblio(item)
        journal: str = ""
        journal: str = "" if journal_element is None else journal_element.text
        journal: str = ", ".join(journal_year[0:-1])
        journals = []
        journal_element = eval_xpath_getindex(entry, xpath_journal, 0, default=None)
        json = js_obj_str_to_python(match.strip())
        json_data = '{"location":"/images/search/' + content_between_tags + 'advRsyaSearchColumn":null}}'
        json_data = resp.json()
        json_resp = loads(json_data)
        json_results = json
        json_results = json['results']
        key = 'article{language}'.replace('{language}', self.language)
        key, value = query.split(':')
        key, values = query.split(':')
        keys: ['line']
        keyword = item['Reference'].lower()
        key_rank = KEY_RANKS.get(k.split(':')[0] + ':*')
        key_type = _valkey_client.type(key)
        kvmap = {
        kvmap = {key: str(value) for key, value in result.items()}
        kvmap = {key: str(value) for key, value in row.items()}
        kvmap = {key: str(value) if not key.startswith("_") else value for key, value in result["_source"].items()}
        kvmap = {str(k): str(v) for k, v in row.items()}
        kwargs = _parse_result(item)
        kwargs = {
        kwargs = {_delta_kwargs[params['time_range']]: 1}
        kwargs: dict[str, t.Any] = {
        kwargs['password'] = password
        kwargs['username'] = username
        k_label = get_key_label(k, user_language)
        l = locales.get_locale(params['searxng_locale'])
        l = re.findall(r"\s*(.*)\s+to\s+(.+)", resp.search_params["query"])
        l.append(ext.get("value") or "")
        l.append(year.get("value") or "")
        label
        labels = labels.get(k)
        lang = params["language"].split("-")
        lang = params['language'][:2]
        lang = result['name']['xml:lang']
        lang = traits.get_language(params['searxng_locale'])  # type: ignore
        lang=lang,
        language = params['language'].split('_')[0]
        language:
        language_name_locale_map[loc.english_name.lower()] = loc
        language_param: traits.get_language(params['searxng_locale'], traits.custom['language_all']),
        lang_tag = line.strip().split(": ")[0].replace("'", "")
        latest_version = None
        latitude = result.get(self.name + 'Lat')
        leech = extract_text(eval_xpath(result, './/td[contains(@class, "leeches")]'))
        leech = int_or_zero(result.xpath(xpath_leeches))
        length = extract_text(result_dom.xpath(length_xpath))
        length = None
        length = result.get("duration")
        length = time.gmtime(res.get('duration'))
        lic = item.get('license') or {}
        lic_url = None
        line = process.stdout.readline()
        lines = {}
        lines, highlighted_lines_index = extract_code(code_matches)
        lines.append("".join(buffer))
        link = entry.find("guid").text
        link = eval_xpath_getindex(newsitem, './/a[@class="title"]', 0, None)
        link = eval_xpath_getindex(result, './/div[@class="itemurl"]/a', 0, default=None)
        link = eval_xpath_getindex(result, './/div[@class="mw-search-result-heading"]/a', 0)
        link = eval_xpath_getindex(result, './/h2/a', 0, None)
        link = eval_xpath_getindex(result, './/h5/a', 0)
        link = result.xpath('.//div[@class="torrent_name"]//a')[0]
        link = url + "description.php?id=" + result["id"]
        links = name_row.xpath('./td[@class="desc-top"]/a')
        links = []
        links = {linked_terms.get(k.lower(), k): v for k, v in meta.get("links").items()}
        links = {}
        links, link_keys = get_links(result, user_language)
        links.append(
        link_keys.add(k)
        loc = babel.Locale.parse(locale)
        locale = babel.Locale.parse(sxng_locale, sep='-')
        locale = None
        locale:
        location = weather.GeoLocation.by_query(query)
        location=location,
        logger.debug("AnswerType='%s' Answer='%s'", answer_type, answer)
        logger.debug("get_sc_code: using cached value: %s", sc_code)
        logger.debug("get_vqd: re-use cached value: %s", value)
        logger.debug("suspend startpage API --> https://github.com/searxng/searxng/pull/695")
        logger.debug("X-S2-UI-Version: %s", ret_val)
        logger.debug('get_thumbnail() redirected: %s', img_src)
        logger.debug('SPARQL endpoint error %s', http_response.content.decode())
        logger.debug(args["since"])
        logger.debug(query)
        logger.error(
        logger.error("Fetched server list is empty!")
        logger.error("init: GET %s failed", url)
        logger.error("invalid api_key (%s): see https://about.marginalia-search.com/article/api", api_key)
        logger.error("missing api_key: see https://about.marginalia-search.com/article/api")
        logger.error("missing recoll configuration: %s", missing)
        logger.error("missing values for options: %s", ", ".join(missing_opts))
        logger.error("none vqd value from duckduckgo.com: HTTP %s", resp.status_code)
        logger.error("Springer's API key is not set or invalid.")
        logger.error("vqd: got HTTP %s from duckduckgo.com", resp.status_code)
        logger.info("using client_id '%s' for soundclud queries", client_id)
        logger.info(message)
        logger.warning("missing valid client_id for soundclud queries")
        longitude = result.get(self.name + 'Long')
        magnet = eval_xpath_getindex(result, './/a[contains(@class, "dl-magnet")]/@href', 0, None)
        magnetlink = (
        magnetlink = entry.find("link").text
        magnetlink = result.xpath('.//div[@class="tail"]//a[@class="title"]/@href')[0]
        magnetlink = result.xpath('.//div[@class="torrent_magnet"]//a')[0].attrib['href']
        magneturl = get_torznab_attribute(item, 'magneturl')
        magnet_link = ""
        mainline = data.get('result', {}).get('items', [])
        mainline = data.get('result', {}).get('items', {}).get('mainline', {})
        mainline = [
        mainline_items = row.get('items', [])
        mainline_type = row.get('type', 'web')
        main_image = result['images']['orig']
        match = SECRET_KEY_RE.search(script.text)
        matches = re.findall(pattern, text, re.DOTALL)
        media_url: str = imageinfo["url"]
        medline_citation: ElementType = eval_xpath_getindex(pubmed_article, "./MedlineCitation", 0)
        message = f'HTTP Status Code: {resp.status_code}'
        meta = package["meta"]
        metadata = ""
        metadata = ' | '.join(metadata)
        metadata = (
        metadata = html_to_text(result.get('categorysnippet', ''))
        metadata = json.loads(result.xpath('.//a[@class="iusc"]/@m')[0])
        metadata = json.loads(result.xpath('.//div[@class="vrhdata"]/@vrhm')[0])
        metadata = list(filter(None, [video_result['channel']['channel_name'], *video_result.get('tags', [])]))[:5]
        metadata = result.xpath('.//a[@class="iusc"]/@m')
        metadata = result['extension']['overline']
        metadata = [
        metadata = [item.get(field) for field in ['category_l1', 'catsy'] if item.get(field)]
        metadata = [meta for meta in (doctype, date) if meta != ""]
        metadata = []
        metadata.append('@' + post.get('username', ''))
        mimetype: str = imageinfo["mime"]
        minutes = item['duration'] // 60
        module = fields["documentation"]
        msg = ",".join(data.get('message', ['unknown']))
        mtype = subtype = result.get("mtype", "")
        mwapi:language "%LANGUAGE%".
        mwapi:search "%QUERY%";
        name
        name = (c or {}).get("display_name")
        name = e.get('name')
        name = entity_id
        name = extract_text(option).lower()  # type: ignore
        name = item["name"]
        name = result['name']['value']
        name = WIKIDATA_PROPERTIES.get((entity_id, 'en'))
        name = WIKIDATA_PROPERTIES.get((entity_id, language))
        name = WIKIDATA_PROPERTIES.get((entity_id, language.split('-')[0]))
        name_row = rows[i]
        native_name = babel.Locale(lang_code).get_language_name()
        native_name = native_name.lower()
        netloc = urlparse(a.get('href')).netloc
        new_result = {
        nextpage = params['engine_data'].get('nextpage')
        now = int(time.time())
        number_of_results_string = re.sub('[^0-9]', '', number_of_results_element)
        num_comments = hit.get("num_comments") or 0
        object_id = hit["objectID"]
        obtain_token()
        odysee_thumbnail = f"https://thumbnails.odycdn.com/optimize/s:390:0/quality:85/plain/{thumbnail_url}"
        offset = 10 + (params["pageno"] - 2) * 15  # Page 2 = 10, Page 2+n = 10 + n*15
        offset = original_code_lenght - len(code)
        or data.get('normal_url')
        or data.get('url'),
        or result['category'] == 'leisure'
        or result['category'] == 'shop'
        or result['category'] == 'tourism'
        order_by = 'time'
        order_by=order_by,
        original_code_lenght = len(code)
        osm = {'type': osm_type, 'id': properties.get('osm_id')}
        osm = {'type': osm_type, 'id': result['osm_id']}
        osm = {}
        package = entry["package"]
        packages[pkg_url] = pkg_list
        package_name = extract_text(eval_xpath(result, title_xpath))
        package_name = item["pkg_name"]
        page=params["pageno"],
        page=params['pageno'],
        pageno=params["pageno"],
        pageno=params['pageno'],
        page_a = result.xpath(xpath_title)[0]
        page_size=page_size,
        paper = res.types.Paper(
        params = {
        params = {'template': 'torrent.html', 'url': links[-1].attrib.get('href'), 'title': extract_text(links[-1])}
        params = {'url': urljoin(base_url, data['permalink']), 'title': data['title']}
        params:
        params["amount"] * conversion_rate,
        params["amount"],
        params["base_url"] = base_url
        params["base_url"] = random.choice(base_url)
        params["cookies"]["df"] = t_range
        params["cookies"]["kl"] = eng_region
        params["cookies"]["p"] = safe_search  # "-2", "1"
        params["from_iso4217"],
        params["from_name"],
        params["headers"]["Accept-Language"] = "en-US,en;q=0.5"
        params["searxng_locale"],
        params["to_iso4217"],
        params["to_name"],
        params["url"] += "&date={timerange}".format(timerange=time_range_dict[params["time_range"]])
        params["url"] = f"{base_url}/api/discovery?{urlencode(args)}"
        params["url"] = f"{base_url}/api/search?{urlencode(args)}"
        params["url"] = None
        params['auth'] = (username, password)
        params['auth'] = DigestAuth(http_digest_auth_user, http_digest_auth_pass)
        params['base_url'] = base_url
        params['base_url'] = random.choice(base_url)
        params['cookies']['ad'] = eng_lang
        params['cookies']['ah'] = eng_region
        params['cookies']['l'] = eng_region
        params['data'] = dumps(
        params['data'] = request_body.format(**fargs)
        params['data'] = request_body.format(**fp)
        params['filesize'] = humanize_bytes(int(result["size"]))
        params['headers']['Api-Key'] = api_key
        params['headers']['Api-Username'] = api_username
        params['headers']['Authorization'] = "placeholder"
        params['headers']['Authorization'] = auth_key
        params['headers']['Authorization'] = f"Bearer {ghc_auth['token']}"
        params['headers']['Authorization'] = f"token {ghc_auth['token']}"
        params['headers']['Content-Type'] = 'application/json'
        params['headers']['User-Agent'] = searxng_useragent()
        params['headers']['X-API-Key'] = api_key
        params['language'] = 'en'
        params['language'] = 'en-us'
        params['language'] = params['language'].split('-')[0]
        params['method'] = 'POST'
        params['raise_for_httperror'] = True
        params['time_range']
        params['url'] += '&' + urlencode({'d': time_range_dict[params['time_range']]})
        params['url'] += '&boostLanguages[]=' + eng_lang
        params['url'] += '&languageOneOf[]=' + eng_lang
        params['url'] += '&relevanceLanguage=' + params['language'].split('-')[0]
        params['url'] += '&startDate=' + time.isoformat()
        params['url'] += f'&filters=ex1:"ez{time_ranges[params["time_range"]]}"'
        params['url'] = f"{base_url_images}?{urlencode(query_params_images)}"
        params['url'] = f"{base_url_web}?{urlencode(query_params_web)}"
        params['url'] = f"{base_url}/search?{urllib.parse.urlencode({'q': query})}"
        params['url'] = nextpage_url
        params['url'] = next_page_url
        params['url'] = search_url.format(offset=offset, query=quote(query))
        params['url'] = search_url.format(query=quote_plus(query), page=params['pageno'])
        params['url'] = search_url_fmt.format(query=quote(query))
        Parsed datetime object or None if parsing fails
        parse_regex = engine_settings['parse_regex']
        parts_of_speech = page.get('senses') and page['senses'][0].get('parts_of_speech')
        pass
        password=password or None,
        password=password,
        past = now - time_range_dict[params["time_range"]]
        pattern = r'<script\s+type="application/json"\s+id="s-data-[^"]+"\s+data-used-by="hydrate">(.*?)</script>'
        pdf_element = eval_xpath_getindex(entry, xpath_pdf, 0, default=None)
        pdf_url = ""
        pdf_url: str = ""
        pdf_url: str = "" if pdf_element is None else pdf_element.attrib.get("href")
        photo = model_export['main'][index[0]][int(index[1])][index[2]][index[3]][index[4]][index[5]][int(index[6])][
        picture
        pixiv_proxy = random.choice(pixiv_image_proxies)
        pkg_list = packages.get(pkg_url, [])
        pkg_list.append(
        pkg_url = f"{pkg_repo_url}/tree/master/srcpkgs/{github_slug}"
        platforms = ', '.join([platform for platform, supported in item.get('platforms', {}).items() if supported])
        play_categ=play_categ,
        pmid: str = eval_xpath_getindex(medline_citation, ".//PMID", 0).text
        pod_id = pod.get('id', '')
        pod_id = pod.xpath(pod_id_xpath)[0]
        pod_is_result = pod.get('primary', None)
        pod_is_result = pod.xpath(pod_primary_xpath)
        pod_title = pod.get('title', '')
        pod_title = pod.xpath(pod_title_xpath)[0]
        point1, point2 = [urllib.parse.quote_plus(p) for p in l[0]]
        points = hit.get("points") or 0
        popularity = babel.numbers.parse_decimal(popularity, locale='en_US')
        popularity = extract_text(eval_xpath(result, popularity_xpath)).strip()
        popularity = flask_babel.format_decimal(popularity)
        popularity_infos = [f"{item.get('star_count', 0)} stars"]
        port=port,
        pos = f'<i>{pos}</i>: ' if pos else ''
        precision = result.get(self.name + 'timePrecision')
        pressure=weather.Pressure(val=data["pressure"], unit="hPa"),
        pressure=weather.Pressure(val=data["pressure_msl"], unit="hPa"),
        price = extract_text(result_dom.xpath(price_xpath))
        price = item.get('price', {}).get('final', 0) / 100
        print("ERROR: can't determine languages from Odysee")
        print("ERROR: can't determine languages from peertube")
        print("ERROR: response from bing is not OK.")
        print("ERROR: response from Brave is not OK.")
        print("ERROR: response from dailymotion/languages is not OK.")
        print("ERROR: response from dailymotion/locales is not OK.")
        print("ERROR: response from DuckDuckGo is not OK.")
        print("ERROR: response from peertube is not OK.")
        print("ERROR: response from Startpage is not OK.")
        print("ERROR: response from wiki.archlinux.org is not OK.")
        print("ERROR: response from Wikipedia is not OK.")
        print(f"  --> {exc}")
        print(f"ERROR: zlibrary domain '{base_url}' is seized?")
        print(f"ERROR: zlibrary domain is seized: {base_url}")
        properties = r.get('properties')
        proxy_full_image_url = (
        proxy_image_url = image_url.replace("https://i.pximg.net", pixiv_proxy)
        pubDate = entry.find("pubDate").text
        pubdate = result['snippet']['publishedAt']
        pubdate_original = item.get('pubdate_original')
        published = datetime.strptime(record["publicationDate"], "%Y-%m-%d")
        published = item.get("publish_date")
        publishedDate = datetime.fromisoformat(result["created"])
        publishedDate = datetime.fromtimestamp(result['date'] / 1000)
        publishedDate = datetime.fromtimestamp(res['created_time'], None)
        publishedDate = datetime.strptime(eval_xpath_getindex(entry, xpath_published, 0).text, "%Y-%m-%dT%H:%M:%SZ")
        publishedDate = datetime.strptime(year.strip(), "%Y")
        publishedDate = extract_text(eval_xpath(result, updated_xpath))
        publishedDate = None
        publishedDate = package.get("date")
        publishedDate = parser.parse(pubdate)
        publishedDate = parser.parse(result['created_at'])
        publishedDate = parser.parse(result['created_time'])
        publishedDate: datetime | None
        published_date = datetime.fromtimestamp(int(data.get('source', {}).get('time')))
        published_date = datetime.now() - timedelta(days=int(re.match(r'\d+', date_string).group()))  # type: ignore
        published_date = None
        published_date = package.get("updated_at")
        published_date = parser.parse(published_date)
        published_date_raw: Raw date string from the API
        publisher = result.get("publisher", "").strip("'")
        pubmed_data: ElementType = eval_xpath_getindex(pubmed_article, "./PubmedData", 0)
        pub_date = extract_text(eval_xpath(result, './article//time'))
        pub_date = extract_text(result.xpath('.//span[contains(@class,"s-time")]'))
        pub_date = None
        pub_info = extract_text(
        pub_origin = extract_text(eval_xpath(result, './article//a[@data-n-tid]'))
        pub_type: str = extract_text(eval_xpath(result, ".//span[@class='gs_ctg2']")) or ""
        Py-Dictionary with the key/value pairs:
        p_date = extract_text(result_dom.xpath(published_date))
        p_list = col_to.xpath(".//p")
        q = {'$eq': query}
        q = {'$regex': _re}
        q.append('after:' + after_date.strftime('%Y-%m-%d'))
        q.append(part)
        q: str = str(params["data"]["q"])
        qset, rest = query.split(" ", 1)
        query += ' (' + eng_lang + ')'
        query = params['search_urls']['data:image']
        query = params['search_urls']['http']
        query = query.replace(arch_path, '').strip()
        query = query.replace(query_arch, '').strip()
        query = query.title()
        query = re.sub(key, val, query)
        query = SORT_RE.sub("", query)
        query = wikidata_image_sparql.replace('%WIKIDATA_IDS%', sparql_string_escape(wikidata_ids_str)).replace(
        query=quote(query),
        query=urlencode({"q": query}),
        query=urlencode({'q': query}),
        query=urlencode({'s': query}),
        query_arch = query_arch.group(0)
        query_params = {
        query_params.update({"ai_type": 1})
        query_params["adv_t"] = time_range_dict.get(params['time_range'])
        query_params["etime"] = 'now'
        query_params["filters"] += f";{time_range_args.get(params['time_range'], 'pub_date:da_sempre')}"
        query_params["nso"] = f"p:{time_range_dict[params['time_range']]}"
        query_params["sitePublishDate"] = time_range_dict[params['time_range']]
        query_params["start"] = (params["pageno"] - 1) * naver_category_dict[naver_category]["start"] + 1
        query_params["stime"] = time_range_dict[params['time_range']]
        query_params["s_from"] = time_range_dict.get(params['time_range'])
        query_params["tl_request"] = time_range_dict.get(params['time_range'])
        query_params["tsn"] = 1
        query_params["where"] = naver_category_dict[naver_category]["where"]
        query_params['df'] = default_fields
        query_params['first'] = _page_offset(page)  # see also arg FORM
        query_params['fl'] = field_list
        query_params['FORM'] = 'PERE%s' % (page - 2)
        query_params['FORM'] = 'PERE'
        query_params['form'] = 'VRFLTR'
        query_params['language'] = lang
        query_params['periodo'] = time_range_args.get(params['time_range'])
        query_params['qf'] = query_fields
        query_params['qft'] = ' filterui:videoage-lt%s' % time_map[params['time_range']]
        query_params['qft'] = 'filterui:age-lt%s' % time_map[params['time_range']]
        query_params['qft'] = time_map.get(params['time_range'], 'interval="9"')
        query_params['sort'] = sort
        query_params['start'] = rows * (params['pageno'] - 1)
        query_params['start'] = start_date.strftime('%Y-%m-%d')
        query_params['time_filter'] = time_range_dict[params['time_range']]
        query_params_images.update({"p": params["pageno"] - 1})
        query_params_web.update({"p": params["pageno"] - 1})
        query_params_web["lang"] = lang
        query_path = expanduser(query_path)
        query_path = params[-1]
        QUERY_TEMPLATE.replace('%QUERY%', sparql_string_escape(query))
        query_url += '&' + urlencode({'safe': filter_mapping[params['safesearch']]})
        query_url += '&' + urlencode({'tbs': 'qdr:' + time_range_dict[params['time_range']]})
        raise httpx.TooManyRedirects(f"location {resp.headers['Location'].split('?')[0]}")
        raise LookupError("Couldn't obtain AWS api url for PDImageArchive")
        raise LookupError("Failed to fetch config location (and as such the API url) for PDImageArchive")
        raise LookupError("Failed to obtain AWS api url for PDImageArchive")
        raise RuntimeError("Response from Anna's search page is not OK.")
        raise RuntimeError("Response from Google's preferences is not OK.")
        raise RuntimeError("Response from zlibrary's search page is not OK.")
        raise RuntimeError(f"Azure authentication failed (status {resp.status_code}): {resp.text}")
        raise SearxEngineAccessDeniedException()
        raise SearxEngineAPIException("failed to obtain secret key")
        raise SearxEngineAPIException("failed to parse response") from e
        raise SearxEngineAPIException("Invalid response")
        raise SearxEngineAPIException("No API key provided")
        raise SearxEngineAPIException('Cloudflare AI error: ' + json['error'])
        raise SearxEngineAPIException(f"Invalid response: {e}") from e
        raise SearxEngineAPIException(f"Unsupported category: {baidu_category}")
        raise SearxEngineAPIException(f"Unsupported category: {naver_category}")
        raise SearxEngineAPIException(f"Unsupported category: {quark_category}")
        raise SearxEngineAPIException(f"Unsupported Hugging Face endpoint: {huggingface_endpoint}")
        raise SearxEngineAPIException(f"{msg} ({error_code})")
        raise SearxEngineAPIException(json_data["error"]["msg"])
        raise SearxEngineAPIException(resp_json["error"])
        raise SearxEngineAPIException(resp_json["error"]["msg"])
        raise SearxEngineAPIException(search_results.get("description"))
        raise SearxEngineAPIException(search_results['error']['message'])
        raise SearxEngineAPIException(search_res['error'].get('message'))
        raise SearxEngineCaptchaException(
        raise SearxEngineCaptchaException()
        raise SearxEngineCaptchaException(message="CAPTCHA (gs_captcha_f)")
        raise SearxEngineCaptchaException(suspended_time=0, message=f"CAPTCHA ({params['data'].get('kl')})")
        raise SearxEngineException()
        raise SearxException(f"zlibrary domain is seized: {base_url}")
        raise ValueError(
        raise ValueError("adobe_stock engine: adobe_content_types is unset")
        raise ValueError("adobe_stock engine: adobe_order is unset")
        raise ValueError("adobe_stock engine: categories is unset")
        raise ValueError("can't find JS/JSON data in the given text")
        raise ValueError("missing invidious base_url")
        raise ValueError("missing required config `base_url`")
        raise ValueError('collection cannot be empty')
        raise ValueError('engine command : missing configuration key: command')
        raise ValueError('engine MRS, base_url is unset')
        raise ValueError('failed to init settings for parsing lines: missing delimiter or parse_regex')
        raise ValueError('failed to init settings for parsing lines: too many settings')
        raise ValueError('gitea engine: base_url is unset')
        raise ValueError('index cannot be empty')
        raise ValueError('missing torznab base_url')
        raise ValueError('only SELECT query is supported')
        raise ValueError('query format must be "key:value"') from e
        raise ValueError('query format must be key:value') from e
        raise ValueError('query format must be key:value1,value2') from e
        raise ValueError('query_str cannot be empty')
        raise ValueError('search_type "%s" is  not one of %s' % (search_type, valid_types))
        raise ValueError('tubearchivist engine: base_url is unset')
        raise ValueError('tubearchivist engine: ta_token is unset')
        raise ValueError('unsupported query type', engine_settings['query_type'])
        raise ValueError(f"Invalid search type {search_type}")
        raise ValueError(f"invalid setting content: {aa_content}")
        raise ValueError(f"invalid setting ext: {aa_ext}")
        raise ValueError(f"invalid setting ext: {zlib_ext}")
        raise ValueError(f"invalid setting sort: {aa_sort}")
        raise ValueError(f"invalid setting year_from: {zlib_year_from}")
        raise ValueError(f"invalid setting year_to: {zlib_year_to}")
        raise ValueError(f"unknown google play category: {play_categ}")
        raise ValueError(f"unsupported adobe_order: {adobe_order}")
        raise ValueError(f"Unsupported category: {chinaso_category}")
        raise ValueError(f"Unsupported DuckDuckGo category: {engine_settings['ddg_category']}")
        raise ValueError(f"Unsupported news source: {chinaso_news_source}")
        raise ValueError(f'presearch search_type: {search_type}')
        ratingCount
        raw_date = result.get("publishedDate") or result.get("depositedDate")
        raw_url = extract_url(eval_xpath_list(result, url_xpath, min_len=1), search_url)
        raw_value = extratags.get(k)
        recipe = result['recipe']
        region = ddg_reg_map.get(eng_tag)
        region, lang = ceid.split(':')
        region_param: traits.get_region(params['searxng_locale'], traits.custom['region_all']),
        relative_url = eval_xpath_getindex(item, url_xpath, 0)
        release_date = datetime.strptime(release_time.split("T")[0], "%Y-%m-%d")
        release_time = item["release_time"]
        repo = item["repo_name"]
        repo: dict[str, str] = item['repository']  # pyright: ignore[reportAny]
        res = None
        res = results.types.MainResult(
        res = {
        res = {'url': url, 'title': extract_text(result), 'content': ''}
        res = {'url': url, 'title': title, 'thumbnail': thumbnail}
        res.add(
        res.add(item)
        res.add(paper)
        res.add(res.types.KeyValue(kvmap=kvmap))
        res.add(res.types.LegacyResult(**kwargs))
        res.add(res.types.LegacyResult(correction=extract_text(correction)))
        res.add(res.types.LegacyResult(suggestion=extract_text(suggestion)))
        res.add(res.types.MainResult(url=result["url"], title=result["title"], content=result.get("description", "")))
        res.add(res.types.Paper(**kwargs))
        res.add(result=result)
        res.append(res.types.LegacyResult({'suggestion': extract_text(suggestion)}))
        res.append(res_obj)
        res.types.Answer(
        resolution = f"{result['width']}x{result['height']}"
        resolution = f"{size_data['width']} x {size_data['height']}"
        resp = get('https://www.google.com/supported_domains')
        resp = get(base_url, headers={'User-Agent': gen_useragent()})
        resp = get(base_url, timeout=3)
        resp = get(base_url, verify=False)
        resp = http_get('https://www.wolframalpha.com/input/api/v1/code?ts=9999999999999999999', timeout=2.0)
        resp = http_get(url)
        resp.status_code == 403
        resp_json = resp.json()
        result = extract_text(a[0])
        result = parse_item(item)
        result = result[result['type']]
        result = text
        result = topics.get(post['topic_id'], {})
        result = {
        result = {'template': 'torrent.html'}
        result.update({'publishedDate': datetime.fromtimestamp(timestamp)})
        result: dict[str, t.Any] = build_result(item)
        results = parse_search_query(json_resp.get('results', {}))
        results = []
        results.add(
        results.add(item)
        results.add(result=res)
        results.add(results.types.LegacyResult(**kwargs))
        results.add(results.types.LegacyResult({"url": abstractURL, "title": heading}))
        results.add(results.types.Translations(translations=item_list, url=resp.search_params["url"]))
        results.add(results.types.Translations(translations=[item], url=resp.search_params["url"]))
        results.append(
        results.append(item)
        results.append(new_result)
        results.append(params)
        results.append(res)
        results.append(result)
        results.append(result_item)
        results.append(res_dict)
        results.append(tmp_result)
        results.append({"suggestion": extract_text(eval_xpath(suggestion, './/div[@class="Epkrse "]'))})
        results.append({"suggestion": from_to_prefix + info["typo"]})
        results.append({"url": url, "title": title, "content": content, "img_src": img})
        results.append({'correction': extract_text(correction)})
        results.append({'number_of_results': int(number_of_results_string)})
        results.append({'number_of_results': number_of_results})
        results.append({'suggestion': extract_text(suggestion)})
        results.append({'suggestion': suggestion})
        results.append({'title': title, 'content': "", 'url': urljoin(base_url, res_url)})
        results.append({'url': app_url, 'title': app_title, 'content': app_content, 'thumbnail': thumbnail})
        results.append({'url': cleaned_url, 'title': title, 'content': content, 'is_onion': True})
        results.append({'url': infobox_urls[0]['url'], 'title': infobox_title, 'content': infobox_content})
        results.append({'url': url, 'title': title, 'content': content})
        results.append({'url': wikipedia_link, 'title': title, 'content': api_result.get('description', '')})
        results.extend(response_json[i])
        results.types.Translations(
        results_json.get('contents', {})
        result["publishedDate"] = datetime.strptime(year, '%Y')
        result["thumbnail"] = thumbnail
        result['author'] = author.encode(errors='ignore').decode()
        result['category'] == 'amenity'
        result['content'] = content.encode(errors='ignore').decode()
        result['content'] = extract_text(eval_xpath(tag, './/span[@class="font11px lightgrey block"]'))
        result['filesize'] = extract_text(eval_xpath(tag, './/td[contains(@class, "nobr")]'))
        result['iframe_src'] = music_player.format(api_path=api_path)
        result['leech'] = int_or_zero(extract_text(eval_xpath(tag, './/td[contains(@class, "red")]')))
        result['magnetlink'] = _map_magnet_link(magneturl, guid, enclosure_url, link)
        result['seed'] = int_or_zero(extract_text(eval_xpath(tag, './/td[contains(@class, "green")]')))
        result['source'] = source.encode(errors='ignore').decode()
        result['title'] = extract_text(eval_xpath(tag, './/a[contains(@class, "cellMainLink")]'))
        result['title'] = title.encode(errors='ignore').decode()
        result['torrentfile'] = _map_torrent_file(link, enclosure_url)
        result['url'] = resp.search_params['base_url'] + url
        result_data = eval_xpath_getindex(
        result_id = parse_qs(urlparse(link.get('href')).query)["search_item_id"][0]
        result_item = {
        result_json['onResponseReceivedCommands'][0]
        result_len_container = "".join(eval_xpath(dom, '//span[@class="sb_count"]//text()'))
        result_len_container = re.sub('[^0-9]', '', result_len_container)
        result_type = result['type']
        result_url = urljoin(BASE_URL, page['slug'])
        res_obj = MainResult(
        return
        return ""
        return """  OPTIONAL { ?item p:{name} ?{name}Node .
        return """OPTIONAL { ?article{language} schema:about ?item ;
        return """OPTIONAL { ?item p:{name}/psv:{name} [
        return "?article{language} ?articleName{language}".replace('{language}', self.language)
        return "?{name} rdfs:label ?{name}Label .".replace('{name}', self.name)
        return "?{name}Lat ?{name}Long".replace('{name}', self.name)
        return "OpenStreetMap"
        return "OPTIONAL { ?item wdt:{name} ?{name} . }".replace('{name}', self.name)
        return "Wikipedia ({language})".replace('{language}', self.language)
        return '(group_concat(distinct ?{name};separator=", ") as ?{name}s)'.replace('{name}', self.name)
        return '(group_concat(distinct ?{name}Label;separator=", ") as ?{name}Labels)'.replace('{name}', self.name)
        return '<' + str(type(self).__name__) + ':' + self.name + '>'
        return '?{name} ?{name}timePrecision ?{name}timeZone ?{name}timeCalendar'.replace('{name}', self.name)
        return '?{name} ?{name}Unit'.replace('{name}', self.name)
        return (
        return authors, "", publisher, None
        return client_id
        return comments
        return cookie
        return doc.get(k, [])
        return element.get("value")
        return enclosure_url
        return extract_text(elem, allow_none=True) or ""
        return f"{cited_by_count} citations"
        return f"{first_page}-{last_page}"
        return False
        return format_date(timestamp, format='full', locale=locale)
        return format_date(timestamp, format='MMMM y', locale=locale)
        return format_date(timestamp, format='yyyy', locale=locale)
        return format_datetime(isoparse(value), format='full', locale=locale)
        return get_label_for_entity(self.name, language)
        return guid
        return leechers
        return link
        return links, link_keys
        return magneturl
        return None
        return params
        return parser.parse(date)
        return parser.parse(published_date_raw)
        return parse_next_page_response(resp.text)
        return parse_web_lite(resp)
        return property_element.text
        return public_token, private_token
        return random.choice(base_url)
        return reduce(lambda a, b: a + b.get('text', ''), element.get('runs'), '')
        return res
        return response_apps(resp)
        return response_movies(resp)
        return result
        return result.get(key)
        return result.get(self.name + 'Labels')
        return result.get(self.name + 's')
        return results
        return ret
        return sc_code
        return self.get_select()
        return servers
        return str(doc.get(k, ""))
        return str(first_page)
        return str(int(peers) - int(seeders))
        return str(last_page)
        return str(record.get(k, ""))
        return time.strftime("%H:%M:%S", length)
        return time_range_url.format(start=time(), end=str(int(time()) - time_range_dict[time_range]))
        return token
        return True
        return url_string
        return value
        return []
        return [], "", "", None
        return _general_results(dom)
        return _get_comments(json)
        return _get_communities(json)
        return _get_posts(json)
        return _get_users(json)
        return _image_results(dom)
        return _news_results(dom)
        return _parse_images(json_resp)
        return _parse_news(resp)
        return _parse_search(resp)
        return _parse_videos(json_resp)
        return __CACHED_API_URL
        return {
        return_code = process.wait(timeout=timeout)
        ret_val = eval_xpath_getindex(doc, "//meta[@name='s2-ui-version']/@content", 0)
        ret_val["as_ylo"] = datetime.now().year - 1
        ret_val['params']['cr'] = 'country' + country
        ret_val['params']['lr'] = ''
        re_transliteration_unsupported, translation['target_transliteration']
        rs = json  # pylint: disable=invalid-name
        rs = query(json, results_query)  # pylint: disable=invalid-name
        rs = rs[0]  # pylint: disable=invalid-name
        rtype = result.get("type", None)
        rumbles = extract_text(result_dom.xpath(rumbles_xpath))
        r_url = result['url']
        safe_search = safe_search_map[params['safesearch']]
        script = get(script_src)
        sc_code = eval_xpath(dom, search_form_xpath + '//input[@name="sc"]/@value')[0]
        search_args["safesearch"] = "strict"
        search_args["time_range"] = time_range_map.get(params["time_range"])
        search_query=quote(query),
        search_query=quote(query), api_key=api_key, torznab_categories=",".join([str(x) for x in torznab_categories])
        search_results = loads(resp.text)
        search_results = {}
        search_term=quote(query),
        search_type = 'search_by_date'
        search_url += '&apikey={api_key}'
        search_url += '&cat={torznab_categories}'
        search_url_fmt = base_url + 'suchen/dudenonline/{query}'
        seconds: str = imageinfo.get("duration")
        secret_key = _get_secret_key()
        section = section['videoRenderer']
        sectiontitle = result.get('sectiontitle')
        sec_name = extract_text(eval_xpath(section, './header'))
        seed = extract_text(eval_xpath(result, './/td[contains(@class, "seeds")]'))
        seed = int_or_zero(result.xpath(xpath_seeds))
        self.kwargs = kwargs
        self.kwargs = kwargs or {}
        self.language = language
        self.name = name
        self.priority = priority
        self.url_id = url_id
        self.url_path_prefix = url_path_prefix
        service="duckduckgo weather",
        service="Open-meteo",
        service="wttr.in",
        set_vqd(
        set_vqd(query=query, value=value, params=params)
        severity = info.get("baseSeverity")
        shipping = extract_text(result_dom.xpath(shipping_xpath))
        size = imageinfo.get("size")
        size_data = None
        size_str = ''.join(filter(str.isdigit, result['filesize']))
        sort = sort_order_map.get(sort_order_path.group(1))
        source = ' '.join(result.xpath('.//div[@class="imgpt"]//div[@class="lnkw"]//a/text()')).strip()
        source = ecma_unescape(photo.get('username', ''))
        source = eval_xpath_getindex(newsitem, './/div[contains(@class, "source")]', 0, None)
        source_category_parsers = {
        source_country = extract_text(result_dom.xpath(source_country_xpath))
        sp_region_names.append(option.get('value'))
        srv = "https://" + url
        start_date = datetime.now() - timedelta(days=time_diff_days)
        start_time = None
        stats = eval_xpath_list(result, './/div[contains(@class, "stats")]/div', min_len=5)
        stats = info_row.xpath('./td[@class="stats"]/span')
        status = gettext("closed") if result.get('closed', '') else gettext("open")
        subdomain:
        subpods = pod.xpath(subpods_xpath)
        suggestions = json_data.get('suggestions', {})
        super().__init__('wikipedia')
        super().__init__(name)
        super().__init__(name, url_id)
        sxng_lang = language_tag(locale)
        sxng_locale = _ceid_locale_map.get(ceid, lang + '-' + region)
        sxng_locales = get_official_locales(eng_country, engine_traits.languages.keys(), regional=True)
        sxng_tag = catalog_engine2code.get(eng_tag)
        sxng_tag = language_tag(babel.Locale.parse(a.get('lang'), sep='-'))
        sxng_tag = sxng_tag.split('_')[0]
        synonyms = []
        tags = ''
        tags = ', '.join(result.get('tags', '').split(','))
        tags = list(entry.get("flags", {}).keys()) + package.get("keywords", [])
        tags = [tag.title() for tag in result["tags"]]
        tags = []
        tags = _extract_tags(item)
        tags: list[str] = [str(tag) for tag in tag_elements]
        tag_elements = eval_xpath(entry, xpath_category)
        tag_label = labels.get('en')
        tag_label = labels.get(lang.split('-')[0])
        tag_label = labels.values()[0]
        td_list = result.xpath("./td")
        temperature=weather.Temperature(val=data["temperature_2m"], unit="°C"),
        temperature=weather.Temperature(val=data['temperature'], unit="°C"),
        temperature=weather.Temperature(val=tempC, unit="°C"),
        text = ddg_result.get("Text")
        text = f"{extract_text(col_from)}"
        text = text.replace(k, v)
        text = text.replace(r"\/", "/").replace(r"\'", "'")
        text=urlencode({'text': query}), api_key=api_key, nb_per_page=nb_per_page, page=params['pageno']
        text_matches: list[dict[str, str]] = item['text_matches']  # pyright: ignore[reportAny]
        thumb = 'https://s3.thehackerblog.com/findthatmeme/thumb/' + item.get('thumbnail', '')
        thumb = item["smaller_square_cover_url"]
        thumbnail = ''
        thumbnail = app.xpath('./img[@class="package-icon"]/@src')[0]
        thumbnail = base_url + eval_xpath_getindex(result, './/img/@src', 0)
        thumbnail = data.get('picListProps', [])[0].get('src').replace("http://", "https://")
        thumbnail = data['thumbnail']
        thumbnail = embedded = ""
        thumbnail = eval_xpath_getindex(result, ".//div[contains(@class, 'image-wrapper')]//img/@src", 0, default="")
        thumbnail = eval_xpath_getindex(result, './/img/@data-src', 0, None)
        thumbnail = eval_xpath_getindex(result, './/img/@src', 0, default=None)
        thumbnail = extract_text(eval_xpath(result, "./div[@class='image']/a/img/@src"))
        thumbnail = extract_text(eval_xpath(result, thumbnail_xpath))
        thumbnail = extract_text(item.xpath('.//div[@class="img-box"]/a/img/@src'))
        thumbnail = extract_text(result.xpath('preceding-sibling::a/figure/img/@src'))
        thumbnail = extract_text(result_dom.xpath(thumbnail_xpath))
        thumbnail = get_thumbnail(get_img_src(result))
        thumbnail = item["pic"]
        thumbnail = item['medias']['picture']
        thumbnail = None
        thumbnail = pdbe_preview_url.format(pdb_id=result['pdb_id'])
        thumbnail = result.xpath('.//div[@class="art"]/img/@src')
        thumbnail = result.xpath('.//div[contains(@class, "mc_vtvc_th")]//img/@src')[0]
        thumbnail = result['pictures']['sizes'][-1]['link']
        thumbnail = result['snippet']['thumbnails']['high']['url']
        thumbnail = res['thumbnail_360_url']
        thumbnail = thumbnail.replace("http://", "https://")
        thumbnail: str = eval_xpath_getindex(result, ".//a[contains(@class, 'thumbnail')]//img/@src", 0, default="")
        thumbnail: str = imageinfo["thumburl"]
        thumbnail=thumbnail,
        thumbnailUrl = base_url + result['thumbnailUrl']
        thumbnailUrl = result.get("thumbnailUrl")
        thumbnail_src = extract_text(eval_xpath(result, thumbnail_xpath))
        thumbnail_src = urljoin(
        thumbnail_url = item["thumbnail_url"]
        time = datetime.now().date() + time_range_table[params['time_range']]
        timeout=2,
        timeout=60,
        timestamp = extract_text(item.xpath('.//script[contains(text(), "timeConvert")]'))
        timestamp = int(data.get('sourceProps', {}).get('time'))
        timestamp = isoparse(value)
        timestamp = result.get('timestamp')
        timestamp = result['Timestamp']
        timestring = ""
        time_diff_days = time_range_dict[params['time_range']]
        time_diff_days = time_range_duration_map[params["time_range"]]
        time_range = time_range_url.format(time_range_val=time_range_val)
        time_ranges = {'day': '1', 'week': '2', 'month': '3', 'year': f'5_{unix_day-365}_{unix_day}'}
        time_range_val = time_range_map.get(params.get('time_range'))
        time_slot_len = 24 // len(day["hourly"])
        tineye_match = parse_tineye_match(match_json)
        title = ""
        title = ' '.join(result.xpath('.//div[@class="infnmpt"]//a/text()')).strip()
        title = address_name
        title = ecma_unescape(photo.get('title', ''))
        title = entry.find("title").text
        title = entry['l']
        title = eval_xpath(result, 'string(.//h2/a)').strip()
        title = eval_xpath_getindex(result, './/h5[contains(@class, "title")]', 0, None)
        title = eval_xpath_getindex(result, title_xpath, 0, default='')
        title = eval_xpath_list(result, ".//span[contains(@class, 'snippet-title')]")
        title = extract_text(
        title = extract_text(eval_xpath(item, ".//span[contains(@class, 'sds-comps-text-type-headline1')]/text()"))
        title = extract_text(eval_xpath(r, './/a[@class="wikilink1"]/@title'))
        title = extract_text(eval_xpath(result, ".//h3[1]//a"))
        title = extract_text(eval_xpath(result, './/span[@class="DdYX5"]'))
        title = extract_text(eval_xpath(result, './article/h3[1]'))
        title = extract_text(eval_xpath(result, './td[contains(@class, "name")]/a[2]'))
        title = extract_text(eval_xpath(result, title_xpath))
        title = extract_text(eval_xpath(spot, './/div[@class="vWM94c"]'))
        title = extract_text(eval_xpath_getindex(entry, './h3/span[@class="package-snippet__name"]', 0))
        title = extract_text(item.xpath('.//h3/a'))
        title = extract_text(item.xpath('.//h3[contains(@class, "res-title")]/a'))
        title = extract_text(item.xpath('.//h3[contains(@class, "vr-title")]/a'))
        title = extract_text(link)
        title = extract_text(page_a)
        title = extract_text(result.xpath('.//a[@title]'))
        title = extract_text(result.xpath('.//h4/a'))
        title = extract_text(result_dom.xpath(title_xpath))
        title = item["title"]
        title = properties.get('name')
        title = query(result, title_query)[0]
        title = re.sub(r"APP\d+ (FC-)?", "", title, count=1)
        title = result.get("title")
        title = result.get('card', {}).get('title')
        title = result.get('display_name')
        title = result.xpath('.//div[@class="heading"]/a/text()')
        title = result["name"]
        title = result['name']
        title = result['snippet']['title']
        title = result['title']
        title = res['title']
        title = title.removeprefix(domain)
        title = title.replace("_vapp.mxf", "")
        title = title.split(",", 1)[0]
        title = title.split(".", 1)[0]
        title = unescape(entry["title"])
        title = unescape(extract_text(eval_xpath(result, title_xpath)))
        title = utils.html_to_text(item["title"])
        title = video_data.get("title", "")
        title, address = get_title_address(result)
        title.startswith(domain)
        title: str = ""
        title: str = eval_xpath_getindex(entry, xpath_title, 0).text
        title: str = eval_xpath_getindex(medline_citation, ".//Article/ArticleTitle", 0).text
        title: str = extract_text(title)
        title: str = item.get("title", "")
        title: str = item["title"].replace("File:", "").rsplit(".", 1)[0]
        title=title + (f' ({year})' if year else ''),
        titles = [column_desc.name for column_desc in cur.description]
        title_element = eval_xpath_getindex(result_element, './/h3/a', 0)
        title_parts = [title['value'] for title in result['title']]
        title_tag = eval_xpath_getindex(result, ".//div[contains(@class, 'title')]", 0, default=None)
        title_xpath = './/div[contains(@class,"compTitle")]/a/h3/span'
        tmp_result = extract_response_info(result)
        tmp_result['content'] = ""
        tmp_result['content'] = content_filter(to_string(content))
        tmp_result['title'] = title_filter(to_string(title))
        tmp_result['url'] = url_prefix + to_string(url)
        token = (
        token = resp.json()["code"]
        token['last_updated'] = update_time
        token['value'] = loads(actual_token.text)['authInfo']['access_token']
        token_response = http_get('https://duckduckgo.com/local.js?get_mk_token=1', timeout=2.0)
        token_str = _get_tokens()
        topics[item['id']] = item
        torrentfile = eval_xpath_getindex(result, './/a[contains(@class, "dl-torrent")]/@href', 0, None)
        torrent_link = ""
        traits.all_locale,
        try:
        ui_lang = option.get("value")
        unaccented_name = ''.join(filter(lambda c: not combining(c), normalize('NFKD', native_name)))
        unit = result.get(self.name + "Unit")
        unix_date = item["pubdate"]
        unix_day = int(time.time() / 86400)
        uploaded = result.get("uploaded", -1)
        upload_time = eval_xpath_getindex(item, upload_time_xpath, 0)
        uri = result["download"]
        url += f"&height={int(height)}"
        url += f"&width={int(width)}"
        url = ""
        url = (
        url = base_url + extract_text(eval_xpath_getindex(entry, './@href', 0))  # type: ignore
        url = base_url + extract_text(result_dom.xpath(url_xpath))
        url = base_url + link.attrib.get('href') + '#downloads'
        url = base_url + result.attrib.get('href')
        url = base_url + url_relative
        url = base_url + videoid
        url = base_youtube_url + videoid
        url = eval_xpath_getindex(item, ".//a[@href and @nocr='1']/@href", 0)
        url = eval_xpath_getindex(item, ".//a[contains(@class, 'info_title')]/@href", 0)
        url = eval_xpath_getindex(result, ".//a[contains(@class, 'result-header')]/@href", 0, default=None)
        url = eval_xpath_getindex(result, './/a[@jsname="UWckNb"]/@href', 0, default=None)
        url = eval_xpath_getindex(result, './/h2/a', 0).get('href')
        url = eval_xpath_getindex(result, './/h4/a/@href', 0, None)
        url = eval_xpath_getindex(result, './/h5[contains(@class, "title")]/a/@href', 0, None)
        url = eval_xpath_getindex(result, url_xpath, 0, default=None)
        url = eval_xpath_getindex(tag, './/a[contains(@class, "cellMainLink")]/@href', 0, None)
        url = extract_text(item.xpath('.//h3/a/@href'))
        url = extract_text(item.xpath('.//h3[contains(@class, "res-title")]/a/@data-mdurl'))
        url = extract_text(item.xpath('.//h3[contains(@class, "vr-title")]/a/@href'))
        url = extract_text(result_dom.xpath(url_xpath))
        url = extract_url(eval_xpath(result, ".//a/@href"), search_url)
        url = extract_url(eval_xpath(spot, './a[@class="Qfxief"]/@href'), search_url)
        url = f"https://odysee.com/{name}:{claim_id}"
        url = f"{base_url}/p/{post['id']}"
        url = f"{base_url}/v/ac{content_id}"
        url = f"{base_url}/watch/{video_id}"
        url = f'https://live.space/watch/{username}'
        url = item["arcurl"]
        url = link.attrib.get('href')
        url = parse_url(url)
        url = query(result, url_query)[0]
        url = random.choice(url)
        url = result.get("url", "").replace("file://" + mount_prefix, dl_prefix)
        url = result["item"].get("link")
        url = result['homepage']
        url = result_base_url.format(osm_type=osm_type, osm_id=properties.get('osm_id'))
        url = result_id_url.format(osm_type=osm_type, osm_id=result['osm_id'])
        url = result_lat_lon_url.format(lat=result['lat'], lon=result['lon'], zoom=12)
        url = res['url']
        url = socket.gethostbyaddr(_ip)[0]
        url = urljoin(base_url, link.attrib.get('href'))
        url = urljoin(base_url, url)
        url = urljoin(URL, result.xpath('.//a[@title]/@href')[0])
        url = url[:-1]
        url = web_lite_url + '?'
        url, html_url, pdf_url = _extract_links(item)
        url, osm, geojson = get_url_osm_geojson(result)
        url, url_label = mapping_function(raw_value)
        url: str = eval_xpath_getindex(entry, xpath_id, 0).text
        url: str = eval_xpath_getindex(result, ".//h3[1]//a/@href", 0)
        url: str = imageinfo["descriptionurl"]
        url: str = pubmed_url + pmid
        url: str = result.get("primaryPaperLink", {}).get("url")
        url: str | None = eval_xpath_getindex(result, ".//a/@href", 0, default=None)
        url: str | None = None
        url=f"https://duckduckgo.com/?q={quote_plus(query)}&iar=images&t=h_",
        url=url,
        urls.append({"title": search_res.get("AbstractSource", ""), "url": abstractURL, "official": True})
        urls.append({"title": search_res.get("DefinitionSource", ""), "url": definitionURL})
        url_info = urlparse(thumbnail)
        url_list: list[dict[str, str]] = record["url"]
        url_params['b'] = params['pageno'] * 7 + 1  #  8, 15, 21, etc.
        url_params['bct'] = 0
        url_params['btf'] = btf
        url_params['iscqry'] = ''
        url_params['pz'] = 7
        url_params['xargs'] = 0
        url_params[name] = value
        url_relative = eval_xpath_getindex(result, url_xpath, 0)
        url_xpath = './/div[contains(@class,"compTitle")]/a/@href'
        uselang = params["searxng_locale"].split("-")[0]
        user = result['creator'].get('display_name', result['creator']['name'])
        user=username,
        username = result.get("user", {}).get("userName", "")
        user_language = 'en' if user_language == 'all' else user_language.split('-')[0]
        value = 'https://' + value[len(http) :]
        value = attribute.get_str(attribute_result, language)
        value = e.get('value')
        value = extr(resp.text, 'vqd="', '"')
        value = result.get(self.name + 's')
        value = result.get(self.name)
        value = value.split(',')[0]
        version = extract_text(eval_xpath_getindex(entry, './h3/span[@class="package-snippet__version"]', 0))
        videoid = result['id']['videoId']
        videoid = result['uri'].split('/')[-1]
        video_cover = extract_text(video_block.xpath('.//div[contains(@class, "video__cover")]/a/img/@src')[0])
        video_data = json.loads(data_exposure_log)
        video_duration = extract_text(video_block.xpath('.//span[contains(@class, "video__duration")]'))
        video_id = eval_xpath_getindex(result, './/div[@jscontroller="rTuANe"]/@data-vid', 0, default=None)
        video_id = item["aid"]
        video_id = relative_url.rsplit('?', maxsplit=1)[0].split('/')[-1]
        video_intro = extract_text(video_block.xpath('.//div[contains(@class, "video__main__intro")]'))
        video_length = eval_xpath_getindex(item, video_length_xpath, 0)
        video_tag = eval_xpath_getindex(
        video_url = entry.get("url")
        views = extract_text(result_dom.xpath(views_xpath))
        vqd = get_vqd(query=query, params=params)
        wd_id = extratags.get('wikidata', extratags.get('wikidata link'))
        weather_answer.forecasts.append(forecast_data)
        while line:
        wikibase:api "EntitySearch";
        wikibase:limit 1;
        wikidata_ids_str = " ".join(wikidata_ids)
        wikidata_json = send_wikidata_query(query)
        WIKIDATA_PROPERTIES[(entity_id, lang)] = name.capitalize()
        WIKIDATA_PROPERTIES[k] = v['symbol']
        wiki_url = row.xpath('./td[4]/a/@href')[0]
        wiki_url = urllib.parse.urlparse(wiki_url)
        wind_from=weather.Compass(data["windDirection"]),
        wind_from=weather.Compass(data["wind_direction_10m"]),
        wind_from=weather.Compass(int(data["winddirDegree"])),
        wind_speed=weather.WindSpeed(val=data["windSpeed"], unit="mi/h"),
        wind_speed=weather.WindSpeed(val=data["windspeedKmph"], unit="km/h"),
        wind_speed=weather.WindSpeed(val=data["wind_speed_10m"], unit="km/h"),
        with contextlib.closing(connect.cursor()) as cursor:
        with contextlib.suppress(UnknownLocaleError):
        with _connection.cursor() as cur:
        working_dir = engine_settings['working_dir']
        x = lang.split('-')
        x = x.get('year')
        year = int(value)
        yield str(index), value
        zlib_ext=zlib_ext,
        zlib_year_from=zlib_year_from,
        zlib_year_to=zlib_year_to,
        ]
        ],
        ]:
        _arcid_random = (''.join(random.choices(_arcid_range, k=23)), int(time.time()))
        _CACHE = EngineCache("duckduckgo")  # pyright: ignore[reportUnreachable]
        _clean_up_node(col_from)
        _clear_cached_api_url()
        _content = eval_xpath_getindex(div_result, './/a[contains(@class, "result__snippet")]', 0, [])
        _content = eval_xpath_getindex(result, ".//div[contains(concat(' ', @class, ' '), ' content ')]", 0, default="")
        _ip: str = ip_tuple[4][0]  # type: ignore
        _q.append(val)
        _re = re.compile('.*{0}.*'.format(re.escape(query)), re.I | re.M)
        _reg = region['iso_3166_1'].upper()
        _text = extract_text(eval_xpath(result, premium_xpath))
        _title = eval_xpath(div_result, ".//h2/a")
        _use_old_values()
        {
        { "as_ylo" : 2024 }
        {"date": {"order": "desc"}},
        {"_score": {"order": "desc"}},
        {'torznab': 'http://torznab.com/schemas/2015/feed'},
        }
        },
        }}
       "custom": {
       categories: [general, web]
       eng_lang = get_ddg_lang(traits, params["searxng_locale"])
       eng_region = traits.get_region(params["searxng_locale"], traits.all_locale)
       https://framagit.org/framasoft/peertube/search-index/-/commit/8ed5c729
       https://framagit.org/framasoft/peertube/search-index/-/commit/8ed5c729#3d8747f9a60695c367c70bb64efba8f403721fad_0_291
       https://github.com/MarginaliaSearch/MarginaliaSearch/blob/master/code/services-application/api-service/java/nu/marginalia/api/model/ApiSearchResult.java
       https://github.com/MarginaliaSearch/MarginaliaSearch/blob/master/code/services-application/api-service/java/nu/marginalia/api/model/ApiSearchResults.java
       ORDER BY duration DESC
       page https://html.duckduckgo.com/html do not offer a language selection
       to the user.
       traits.custom['wiki_netloc'] = {
       WHERE title LIKE :wildcard OR description LIKE :wildcard
       `DDG-lite <https://lite.duckduckgo.com/lite>`__ and the *no Javascript*
       }
       },
      "en-CA": "ca",
      "fr-CA": "ca",
      "ui_lang": {
      %WIKIBASE_LABELS%
      - https://..
      - https://search.lomig.me
      - https://search.webproject.link
      - https://yacy.ecosys.eu
      - https://yacy.searchlab.eu
      - `backlink`, the original website URL.
      - `crawl_date`, the date the image was crawled.
      - `url`, the image URL to the image.
      ..
      0: '&filter=none'
      0: none, 1: moderate, 2:strict
      1: '&filter=moderate'
      2: '&filter=strict'
      <https://github.com/TinEye/pytineye/blob/main/pytineye/api.py>`__):
      <input type="hidden" class="abp" id="abp-input" name="abp" value="1">
      <input type="hidden" name="cat" value="web">
      <input type="hidden" name="lui" value="english">
      <input type="hidden" name="sc" value="Q7Mt5TRqowKB00">
      <input type="hidden" name="t" value="device">
      <input type="text" name="query"  value="" ..>
      ?item rdfs:label ?itemLabel .
      ?item schema:description ?itemDescription .
      Added: 2005-10-16
      and image URLs. List items are instances of :py:obj:`dict`, (`Backlink
      ar_AA --> ar_EG, ar_AE, ar_SA
      bd:serviceParam wikibase:language "%LANGUAGE%,en".
      category
      countries {
      dateRelease
      day: 1
      Description: Norwegian Bokmål
      Dimensions of the full-size image.
      directors {
      duration
      en_EN --> en_GB, en_US
      genresInfos {
      id
      Image height in pixels (negative values are ignored). If only height is
      Image width in pixels (negative values are ignored). If only width is
      Is a full-size image (>MB).
      Macrolanguage: no
      medias {
      month: 30
      originalTitle
      otherwise the engine is inactive.
      rating
      Reuters has a *resizer* `REST-API for the images`_, this is the URL of the
      SELECT ?item
      SELECT title || ' (' || time(duration, 'unixepoch') || ')' AS title,
      service. This URL includes the ``&auth`` argument, other arguments are
      specified, the height matches the original aspect ratio.
      specified, the width matches the original aspect ratio.
      stats {
      Subtag: nb
      Suppress-Script: Latn
      the languages and the list of all
      title
      token: "<token>"
      type: "bearer"
      type: "none"
      type: "personal_access_token"
      type: language
      url
      VALUES ?item { %ATTRIBUTES% }
      website: https://gitlab.com/
      website: https://gitlab.gnome.org
      week: 7
      WHERE { ?item wdt:P279* wd:Q12132 }
      wikidata_id: Q16639197
      wikidata_id: Q44316
      year: 365
      yearOfProduction
      ``&width=<int>`` and ``&height=<int>``.
      {{
      }
      }}
     # Recommended by OpenAlex: join the polite pool with an email address
     - "./your-new-config.yml:/etc/nginx/sites-available/default:ro"
     -v ./your-new-config.yml:/etc/nginx/sites-available/default
     ...
     <https://docs.meilisearch.com/learn/getting_started/installation.html>`_
     <https://www.elastic.co/guide/en/elasticsearch/reference/current/index.html>`_
     <https://www.elastic.co/guide/en/elasticsearch/reference/current/install-elasticsearch.html>`_
     api_key: '<KEY>'
     api_key: ...
     api_order: views
     api_username: 'system'
     azure_client_id: "your_client_id"
     azure_client_secret: "your_client_secret"
     azure_tenant_id: "your_tenant_id"
     azure_token_expiration_seconds: 5000
     base_url: 'https://forums.paddling.com/'
     categories: science, scientific publications
     categories: ['social media', 'sports']
     categories: [images]
     categories: [news]
     categories: [videos]
     chinaso_category: images
     chinaso_category: news
     chinaso_category: videos
     chinaso_news_source: all
     database: my_database
     engine: azure
     engine: chinaso
     engine: discourse
     engine: marginalia
     engine: mariadb_server
     engine: mysql_server
     engine: openalex
     engine: postgresql
     engine: pubmed
     engine: reuters
     engine: semantic_scholar
     engine: wikicommons
     engine: zlibrary
     Header set Content-Security-Policy "img-src 'self' data: ;"
     limit: 5
     mailto: "[email protected]"
     network: chinaso news
     password: password
     query_str: 'SELECT * from my_table WHERE my_column = %(query)s'
     query_str: 'SELECT * from my_table WHERE my_column=%(query)s'
     shortcut: chinaso
     shortcut: chinasoi
     shortcut: chinasov
     shortcut: mar
     shortcut: oa
     shortcut: paddle
     shortcut: pub
     shortcut: reu
     shortcut: se
     shortcut: zlib2010s
     show_avatar: true
     sort_order: "relevance"
     timeout: 5.0
     username: searxng
     wc_search_type: audio
     wc_search_type: file
     wc_search_type: image
     wc_search_type: video
     zlib_ext: 'EPUB'
     zlib_year_from: '2010'
     zlib_year_to: '2020'
     ['ls', '-l', '-h', '{{QUERY}}']
     }
    " format. TinEye only supports images that are JPEG, PNG, GIF, BMP, TIFF or WebP."
    " visual detail to successfully identify matches."
    """
    """Assemble a Bing-Image request."""
    """Assemble a Bing-News request."""
    """Assemble a Bing-Video request."""
    """Assemble a Bing-Web request."""
    """Assemble a request (`wikipedia rest_v1 summary API`_)."""
    """Assemble a Startpage request.
    """Assemble request for the Peertube API"""
    """Assemble request for the SepiaSearch API"""
    """Authenticates to Azure using Oauth2 Client Credentials Flow and returns
    """Build a result from a XML item."""
    """Build sB cookie parameter from provided parameters.
    """Build the content parts for an item"""
    """Build the request params."""
    """Build TinEye HTTP request using ``search_urls`` of a :py:obj:`engine_type`."""
    """Build up the ``params`` for the online request.  In this example we build a
    """Build Yahoo search request."""
    """Check of engine's settings."""
    """Checks if delimiter based parsing or regex parsing is configured correctly"""
    """Composing various (language) properties for the google engines (:ref:`google
    """Create the API request."""
    """duckduckgo may return something like ``<a href="xxxx">http://somewhere Related website<a/>``
    """Dynamic setup of the engine settings.
    """Extract and parse the published date from the API response.
    """Fetch :ref:`languages <brave languages>` and :ref:`regions <brave
    """Fetch :ref:`languages <startpage languages>` and :ref:`regions <startpage
    """Fetch languages & regions from DuckDuckGo.
    """Fetch languages and countrycodes from RadioBrowser
    """Fetch languages and other search arguments from Anna's search form."""
    """Fetch languages and other search arguments from zlibrary's search form."""
    """Fetch languages and regions from Bing-News."""
    """Fetch languages and regions from Bing-Web."""
    """Fetch languages from Archlinux-Wiki.  The location of the Wiki address of a
    """Fetch languages from Google."""
    """Fetch languages from peertube's search-index source code.
    """Fetch languages from Wikipedia.  Not all languages from the
    """Fetch locales & languages from dailymotion.
    """Format of the response from UI's async request.
    """Generates a URL for Reuter's thumbnail with the dimensions *width* and
    """Get an actual ``sc`` argument from Startpage's search form (HTML page).
    """Get attribute from item."""
    """Get DuckDuckGo's language identifier from SearXNG's locale.
    """Get image URL from either wikidata or r['extratags']"""
    """Get key label from OSM_KEYS_TAGS"""
    """Get label from labels in OSM_KEYS_TAGS
    """Get OSM key rank
    """Get response from Bing-Images"""
    """Get response from Bing-Video"""
    """Get response from google's search request"""
    """Get tag label from OSM_KEYS_TAGS"""
    """Get Thumbnail image from wikimedia commons
    """Get torznab special attribute from item."""
    """Get url, osm and geojson"""
    """Google search request"""
    """Google-Image search request"""
    """Google-News search request"""
    """Google-Scholar search request"""
    """Google-Video search request"""
    """Implements a :py:obj:`Context Manager <contextlib.contextmanager>` for a
    """In case of CAPTCHA ddg response its own *not a Robot* dialog and is not
    """In case of CAPTCHA Google Scholar open its own *not a Robot* dialog and is
    """Initialization of the ADS_ engine, checks whether the :py:obj:`api_key`
    """Initialization of the CORE_ engine, checks whether the :py:obj:`api_key`
    """Initialization of the engine.
    """Initialization of the Recoll engine, checks if the mandatory values are
    """Initialization of the Springer engine, checks whether the
    """Initialization of the Wikimedia engine, checks if the value configured in
    """Initialize the engine."""
    """Marginalia's ApiSearchResults_ class definition.
    """Marginalia's ApiSearchResult_ class definition.
    """Parse a single item from the SensCritique API response"""
    """Parse HTTP response from TinEye."""
    """Parse out the result items from the response.  In this example we parse the
    """Parse response from Google Scholar"""
    """parse response"""
    """Parse results from Qwant's API"""
    """Parse results from Qwant-Lite"""
    """Parse the text written in green.
    """Parse the XML response and return a list of results."""
    """Parse video response from SepiaSearch and Peertube instances."""
    """Parse video response from Tubearchivist instances."""
    """parse ``{"unit": "https://www.wikidata.org/entity/Q712226", "amount": "+20.99"}``"""
    """Parses command line output based on configuration"""
    """Process the API response and return results."""
    """Query (offline) engine and return results.  Assemble the list of results
    """Qwant search request"""
    """remove yahoo-specific tracking-url"""
    """Return key, value of result['extratags']
    """Return links from result['extratags']"""
    """Return title and address
    """Returns a dictionary with a time range arguments based on
    """Returns the vqd_ vakue that fits to the *query* (and HTTP User-Agent_
    """Returns the Wikipedia language tag and the netloc that fits to the
    """Scrap *results* from the response (see :ref:`result types`)."""
    """Takes parsed JSON from the API server and turns it into a :py:obj:`dict`
    """The ``base_url`` must be set in the configuration, if ``base_url`` is not
    """To avoid a redirect, the !bang directives in the query string are
    """Update nominatim_json using the result of an unique to wikidata
    """Uses languages evaluated from :py:obj:`wikipedia.fetch_wikimedia_traits
    "113": "clear sky",
    "116": "partly cloudy",
    "119": "cloudy",
    "122": "fair",
    "143": "fair",
    "176": "light rain showers",
    "179": "light snow showers",
    "182": "light sleet showers",
    "185": "light sleet",
    "200": "rain and thunder",
    "227": "light snow",
    "230": "heavy snow",
    "248": "fog",
    "260": "fog",
    "263": "light rain showers",
    "266": "light rain showers",
    "281": "light sleet showers",
    "284": "light snow showers",
    "293": "light rain showers",
    "296": "light rain",
    "299": "rain showers",
    "302": "rain",
    "305": "heavy rain showers",
    "308": "heavy rain",
    "311": "light sleet",
    "314": "sleet",
    "317": "light sleet",
    "320": "heavy sleet",
    "323": "light snow showers",
    "326": "light snow showers",
    "329": "heavy snow showers",
    "332": "heavy snow",
    "335": "heavy snow showers",
    "338": "heavy snow",
    "350": "light sleet",
    "353": "light rain showers",
    "356": "heavy rain showers",
    "359": "heavy rain",
    "362": "light sleet showers",
    "365": "sleet showers",
    "368": "light snow showers",
    "371": "heavy snow showers",
    "374": "light sleet showers",
    "377": "heavy sleet",
    "386": "rain showers and thunder",
    "389": "heavy rain showers and thunder",
    "392": "snow showers and thunder",
    "395": "heavy snow showers",
    "abstract",
    "all": "any",
    "apparent_temperature",
    "ar": "ar",  # Arabic
    "AR": "ar.search.yahoo.com",  # Argentina
    "arxiv": "http://arxiv.org/schemas/atom",
    "ar_DZ": "lang_region",
    "ar_JO": "lang_region",
    "ar_SA": "lang_region",
    "atom": "http://www.w3.org/2005/Atom",
    "audio": "audio",
    "author",
    "author": "Author",
    "be",  # Belarusian
    "bg": "bg",  # Bulgarian
    "bibcode",
    "bitbucket": "Bitbucket",
    "Blizzard": "heavy snow",
    "BlowingDust": "fog",
    "BlowingSnow": "heavy snow",
    "bn_IN": "lang_region",
    "BR": "br.search.yahoo.com",  # Brazil
    "Breezy": "partly cloudy",
    "bug tracker": "Issue tracker",
    "buvid3": "".join(random.choice(string.hexdigits) for _ in range(16)) + "infoc",
    "b_ut": "7",
    "CA": "ca.search.yahoo.com",  # Canada
    "changelog": "Changelog",
    "CL": "cl.search.yahoo.com",  # Chile
    "Clear": "clear sky",
    "Cloudy": "cloudy",
    "cloud_cover",
    "CO": "co.search.yahoo.com",  # Colombia
    "comment",
    "Could not read that image url. This may be due to an unsupported file"
    "cs": "cs",  # Czech
    "ct-ca": "skip",  # ct-ca and es-ca both map to ca_ES
    "custom": {
    "da": "da",  # Danish
    "date",
    "day": "pd",
    "day": 1,
    "day": timedelta(days=1),
    "de",  # German
    "de": "de",
    "de": "de",  # German
    "DE": "de.search.yahoo.com",  # Germany
    "description": "Service from DuckDuckGo.",
    "de_CH": "lang_region",
    "doc": "Documentation",
    "docs": "Documentation",
    "documentation": "Documentation",
    "doi",
    "Drizzle": "light rain",
    "el": "el",  # Greek
    "en",  # English
    "en": "en",
    "en": "en",  # English
    "en_AU": "lang_region",
    "en_CA": "lang_region",
    "en_GB": "lang_region",
    "eo_XX": "eo",
    "es": "es",
    "es": "es",  # Spanish
    "ES": "espanol.search.yahoo.com",  # Espanol
    "es-ca": "ca_ES",
    "es_AR": "lang_region",
    "es_CL": "lang_region",
    "es_CO": "lang_region",
    "es_CR": "lang_region",
    "es_EC": "lang_region",
    "es_MX": "lang_region",
    "es_PE": "lang_region",
    "es_UY": "lang_region",
    "es_VE": "lang_region",
    "et": "et",  # Estonian
    "FEED_LIVE_VERSION": "V8",
    "fi": "fi",  # Finnish
    "file": "multimedia|office|archive|3d",
    "Flurries": "light snow",
    "Foggy": "fog",
    "fr",  # French
    "fr": "fr",
    "fr": "fr",  # French
    "FR": "fr.search.yahoo.com",  # France
    "FreezingDrizzle": "light sleet",
    "FreezingRain": "sleet",
    "Frigid": "clear sky",
    "fr_BE": "lang_region",
    "fr_CA": "lang_region",
    "fr_CH": "lang_region",
    "GB": "uk.search.yahoo.com",  # United Kingdom
    "general": {
    "github repository": "GitHub",
    "github": "GitHub",
    "gitlab": "GitLab",
    "Hail": "heavy rain",
    "Haze": "fog",
    "he": "he",  # Hebrew
    "header_theme_version": "undefined",
    "HeavyRain": "heavy rain",
    "HeavySnow": "heavy snow",
    "HK": "hk.search.yahoo.com",  # Hong Kong
    "hk-tzh": "zh_HK",
    "home_feed_column": "4",
    "Hot": "clear sky",
    "hr": "hr",  # Croatian
    "http://www.wikidata.org/entity/" + wid for wid in ("Q4115189", "Q13406268", "Q15397819", "Q17339402")
    "hu": "hu",  # Hungarian
    "Hurricane": "rain and thunder",
    "i-wanna-go-back": "-1",
    "id",  # Indonesian
    "id-en": "id_ID",
    "image": "bitmap|drawing",
    "images": {
    "IN": "in.search.yahoo.com",  # India
    "innersign": "0",
    "io_XX": "skip",
    "isbn",
    "IsolatedThunderstorms": "rain and thunder",
    "issn",
    "issues": "Issue tracker",
    "it": "it",
    "it": "it",  # Italian
    "ja": "ja",  # Japanese
    "jp-jp": "ja_JP",
    "keyword",
    "kk",  # Kazakh
    "ko": "ko",  # Korean
    "kr-kr": "ko_KR",
    "language": "cz",
    "language": "de",
    "language": "en",
    "language": "it",
    "language": "ja",
    "language": "ko",
    "language": "zh",
    "language": 'de',
    "language": 'fr',
    "language": 'ja',
    "lt": "lt",  # Lithuanian
    "lv": "lv",  # Latvian
    "month": "pm",
    "month": 30,
    "month": timedelta(days=31),
    "MostlyClear": "clear sky",
    "MostlyCloudy": "partly cloudy",
    "MX": "mx.search.yahoo.com",  # Mexico
    "news": {
    "nl": "nl",
    "nl": "nl",  # Dutch
    "nl_BE": "lang_region",
    "no": "no",  # Norwegian
    "no-no": "nb_NO",
    "oc": "fr",
    "od_IN": "skip",
    "official_api_documentation": "?",
    "official_api_documentation": "http://api.artic.edu/docs/",
    "official_api_documentation": "https://about.marginalia-search.com/article/api/",
    "official_api_documentation": "https://api-dashboard.search.brave.com/documentation",
    "official_api_documentation": "https://api-docs.npms.io/",
    "official_api_documentation": "https://api.core.ac.uk/docs/v3",
    "official_api_documentation": "https://api.crossref.org/swagger-ui/",
    "official_api_documentation": "https://api.semanticscholar.org/",
    "official_api_documentation": "https://api.sklik.cz/",
    "official_api_documentation": "https://commons.wikimedia.org/w/api.php",
    "official_api_documentation": "https://crates.io/data-access",
    "official_api_documentation": "https://dev.springernature.com/docs/live-documentation/",
    "official_api_documentation": "https://developers.google.com/custom-search",
    "official_api_documentation": "https://developers.soundcloud.com/docs/api/guide",
    "official_api_documentation": "https://docs.discourse.org/",
    "official_api_documentation": "https://docs.gitlab.com/ee/api/",
    "official_api_documentation": "https://docs.openalex.org/how-to-use-the-api/api-overview",
    "official_api_documentation": "https://docs.presearch.io/nodes/api",
    "official_api_documentation": "https://duckduckgo.com/api",
    "official_api_documentation": "https://freesound.org/docs/api",
    "official_api_documentation": "https://github.com/chubin/wttr.in#json-output",
    "official_api_documentation": "https://github.com/hexpm/hexpm/blob/main/lib/hexpm_web/controllers/api/package_controller.ex",
    "official_api_documentation": "https://hn.algolia.com/api",
    "official_api_documentation": "https://huggingface.co/docs/hub/en/api",
    "official_api_documentation": "https://info.arxiv.org/help/api/user-manual.html",
    "official_api_documentation": "https://jisho.org/forum/54fefc1f6e73340b1f160000-is-there-any-kind-of-search-api",
    "official_api_documentation": "https://join-lemmy.org/api/",
    "official_api_documentation": "https://learn.microsoft.com/en-us/\
    "official_api_documentation": "https://man.sr.ht/",
    "official_api_documentation": "https://open-meteo.com/en/docs",
    "official_api_documentation": "https://openlibrary.org/developers/api",
    "official_api_documentation": "https://searchcode.com/api/",
    "official_api_documentation": "https://stract.com/beta/api/docs/#/search/api",
    "official_api_documentation": "https://torznab.github.io/spec-1.3-draft",
    "official_api_documentation": "https://ui.adsabs.harvard.edu/help/api/api-docs.html",
    "official_api_documentation": "https://warehouse.readthedocs.io/api-reference/index.html",
    "official_api_documentation": "https://www.lesbonscomptes.com/recoll/",
    "official_api_documentation": 'http://api.artic.edu/docs/',
    "official_api_documentation": 'http://developer.vimeo.com/api',
    "official_api_documentation": 'http://wiki.openstreetmap.org/wiki/Nominatim',
    "official_api_documentation": 'http://www.mixcloud.com/developers/',
    "official_api_documentation": 'https://api.base-search.net/',
    "official_api_documentation": 'https://api.imgur.com/',
    "official_api_documentation": 'https://api.openverse.org/v1/',
    "official_api_documentation": 'https://api.stackexchange.com/docs',
    "official_api_documentation": 'https://api.tineye.com/python/docs/',
    "official_api_documentation": 'https://apibay.org/',
    "official_api_documentation": 'https://bandcamp.com/developer',
    "official_api_documentation": 'https://de1.api.radio-browser.info/',
    "official_api_documentation": 'https://developer.ebay.com/',
    "official_api_documentation": 'https://developer.github.com/v3/',
    "official_api_documentation": 'https://developer.spotify.com/web-api/search-item/',
    "official_api_documentation": 'https://developer.yahoo.com/api/',
    "official_api_documentation": 'https://developers.cloudflare.com/workers-ai',
    "official_api_documentation": 'https://developers.deezer.com/',
    "official_api_documentation": 'https://developers.google.com/custom-search',
    "official_api_documentation": 'https://developers.google.com/custom-search/',
    "official_api_documentation": 'https://developers.google.com/youtube/v3/docs/search/list?apix=true',
    "official_api_documentation": 'https://developers.pinterest.com/docs/api/v5/',
    "official_api_documentation": 'https://docs.docker.com/registry/spec/api/',
    "official_api_documentation": 'https://docs.genius.com/',
    "official_api_documentation": 'https://docs.gitea.com/next/development/api-usage',
    "official_api_documentation": 'https://docs.github.com/en/rest/search/search?apiVersion=2022-11-28#search-code',
    "official_api_documentation": 'https://docs.invidious.io/api/',
    "official_api_documentation": 'https://docs.joinmastodon.org/api/',
    "official_api_documentation": 'https://docs.joinpeertube.org/api-rest-reference.html#tag/Search/operation/searchVideos',
    "official_api_documentation": 'https://docs.piped.video/docs/api-documentation/',
    "official_api_documentation": 'https://docs.searxng.org/dev/search_api.html',
    "official_api_documentation": 'https://docs.tubearchivist.com/api/introduction/',
    "official_api_documentation": 'https://en.wikipedia.org/api/',
    "official_api_documentation": 'https://gist.github.com/bagbag/a2888478d27de0e989cf777f81fb33de',
    "official_api_documentation": 'https://github.com/metacpan/metacpan-api/blob/master/docs/API-docs.md',
    "official_api_documentation": 'https://github.com/thedaviddelta/lingva-translate#public-apis',
    "official_api_documentation": 'https://gitlab.com/etke.cc/mrs/api/-/blob/main/openapi.yml?ref_type=heads',
    "official_api_documentation": 'https://help.rumble.com/',
    "official_api_documentation": 'https://libretranslate.com/docs/',
    "official_api_documentation": 'https://mozhi.aryak.me/api/swagger/index.html',
    "official_api_documentation": 'https://mymemory.translated.net/doc/spec.php',
    "official_api_documentation": 'https://photon.komoot.io/',
    "official_api_documentation": 'https://pixabay.com/api/docs/',
    "official_api_documentation": 'https://products.wolframalpha.com/api/',
    "official_api_documentation": 'https://query.wikidata.org/',
    "official_api_documentation": 'https://scanr.enseignementsup-recherche.gouv.fr/opendata',
    "official_api_documentation": 'https://secure.flickr.com/services/api/flickr.photos.search.html',
    "official_api_documentation": 'https://svgapi.com',
    "official_api_documentation": 'https://unsplash.com/developers',
    "official_api_documentation": 'https://wiki.yacy.net/index.php/Dev:API',
    "official_api_documentation": 'https://www.dailymotion.com/developer',
    "official_api_documentation": 'https://www.deepl.com/docs-api',
    "official_api_documentation": 'https://www.deviantart.com/developers/',
    "official_api_documentation": 'https://www.dokuwiki.org/devel:xmlrpc',
    "official_api_documentation": 'https://www.ebi.ac.uk/pdbe/api/doc/search.html',
    "official_api_documentation": 'https://www.loc.gov/api',
    "official_api_documentation": 'https://www.mediawiki.org/w/api.php?action=help&modules=query',
    "official_api_documentation": 'https://www.microsoft.com/en-us/bing/apis/bing-image-search-api',
    "official_api_documentation": 'https://www.microsoft.com/en-us/bing/apis/bing-news-search-api',
    "official_api_documentation": 'https://www.microsoft.com/en-us/bing/apis/bing-video-search-api',
    "official_api_documentation": 'https://www.microsoft.com/en-us/bing/apis/bing-web-search-api',
    "official_api_documentation": 'https://www.opensemanticsearch.org/dev',
    "official_api_documentation": 'https://www.pexels.com/api/',
    "official_api_documentation": 'https://www.reddit.com/dev/api',
    "official_api_documentation": (
    "official_api_documentation": None,
    "official_api_documentation": {
    "official_api_documentation": {'url': 'https://btdig.com/contacts', 'comment': 'on demand'},
    "official_api_documentation": {'url': None, 'comment': 'see https://github.com/MitchellAW/CompuGlobal'},
    "page",
    "page_count",
    "page_range",
    "pap": "pt_BR",
    "PartlyCloudy": "partly cloudy",
    "PE": "pe.search.yahoo.com",  # Peru
    "PH": "ph.search.yahoo.com",  # Philippines
    "pl": "pl",
    "pl": "pl",  # Polish
    "pressure_msl",
    "project source code": "Source code",
    "pt": "pt",  # Portuguese
    "pt": "pt_BR",
    "pt_BR": "lang_region",
    "pub",
    "pubdate",
    "pubnote",
    "Rain": "rain",
    "read_count",
    "regions": {
    "relative_humidity_2m",
    "repository": "Source code",
    "require_api_key": False,
    "require_api_key": True,
    "results": "HTML",
    "results": "JSON (site requires js to get images)",
    "results": "JSON",
    "results": "JSONP",
    "results": "XML",
    "results": "XML-RSS",
    "results": 'empty array',
    "results": 'HTML',
    "results": 'JSON',
    "results": 'RSS',
    "results": 'XML',
    "ro": "ro",  # Romanian
    "ru",  # Russian
    "ru": "ru",  # Russian
    "ScatteredThunderstorms": "heavy rain and thunder",
    "scm": "Source code",
    "SG": "sg.search.yahoo.com",  # Singapore
    "sk": "sk",  # Slovak
    "sl": "sl",  # Slovenian
    "sl-sl": "sl_SI",
    "Sleet": "sleet",
    "Smoky": "fog",
    "Snow": "light snow",
    "sort": [
    "sourcehut": "SourceHut",
    "sources": "Source code",
    "sponsor": "Sponsors",
    "sponsors": "Sponsors",
    "StrongStorms": "heavy rain and thunder",
    "SunFlurries": "light snow",
    "SunShowers": "rain",
    "sv": "sv",
    "sv": "sv",  # Swedish
    "szl": "pl",
    "temperature_2m",
    "th": "th",  # Thai
    "TH": "th.search.yahoo.com",  # Thailand
    "th-en": "th_TH",
    "The image is too simple to find matches. TinEye requires a basic level of"
    "Thunderstorms": "rain and thunder",
    "title",
    "token": "",
    "tokipona_XX": "skip",
    "tr",  # Turkish
    "tr": "tr",  # Turkish
    "TropicalStorm": "rain and thunder",
    "tt",  # Tatar
    "TW": "tw.search.yahoo.com",  # Taiwan
    "tw-tzh": "zh_TW",
    "type": "none",
    "udp://9.rarbg.to:2920/announce",
    "udp://tracker.coppersurfer.tk:6969/announce",
    "udp://tracker.cyberia.is:6969/announce",
    "udp://tracker.internetwarriors.net:1337/announce",
    "udp://tracker.leechers-paradise.org:6969/announce",
    "udp://tracker.opentrackr.org:1337",
    "udp://tracker.pirateparty.gr:6969/announce",
    "uk",  # Ukrainian
    "UK": "uk.search.yahoo.com",
    "use_official_api": False,
    "use_official_api": True,
    "VE": "ve.search.yahoo.com",  # Venezuela
    "video": "video",
    "videos": {
    "vn-en": "vi_VN",
    "volume",
    "weather_code",
    "website": "Homepage",
    "website": "https://annas-archive.org/",
    "website": "https://api.search.brave.com/",
    "website": "https://arxiv.org",
    "website": "https://commons.wikimedia.org/",
    "website": "https://core.ac.uk",
    "website": "https://crates.io/",
    "website": "https://devicon.dev/",
    "website": "https://discourse.org/",
    "website": "https://duckduckgo.com/",
    "website": "https://freesound.org",
    "website": "https://hex.pm/",
    "website": "https://huggingface.co/",
    "website": "https://learn.microsoft.com",
    "website": "https://lite.duckduckgo.com/lite/",
    "website": "https://lucide.dev/",
    "website": "https://marginalia.nu",
    "website": "https://news.ycombinator.com/",
    "website": "https://npms.io/",
    "website": "https://odysee.com/",
    "website": "https://ollama.com",
    "website": "https://open-meteo.com",
    "website": "https://openalex.org/",
    "website": "https://openlibrary.org",
    "website": "https://pic.sogou.com/",
    "website": "https://play.google.com/",
    "website": "https://presearch.io",
    "website": "https://pypi.org",
    "website": "https://quark.sm.cn/",
    "website": "https://scholar.google.com",
    "website": "https://search.brave.com/",
    "website": "https://search.naver.com",
    "website": "https://searchcode.com/",
    "website": "https://soundcloud.com",
    "website": "https://sourcehut.org",
    "website": "https://stock.adobe.com/",
    "website": "https://stract.com/",
    "website": "https://tv.360kan.com/",
    "website": "https://ui.adsabs.harvard.edu/",
    "website": "https://v.sogou.com/",
    "website": "https://weixin.sogou.com/",
    "website": "https://wttr.in",
    "website": "https://www.acfun.cn/",
    "website": "https://www.artic.edu",
    "website": "https://www.ask.com/",
    "website": "https://www.baidu.com",
    "website": "https://www.bilibili.com",
    "website": "https://www.chinaso.com/",
    "website": "https://www.crossref.org/",
    "website": "https://www.ilpost.it",
    "website": "https://www.iqiyi.com/",
    "website": "https://www.ncbi.nlm.nih.gov/pubmed/",
    "website": "https://www.nicovideo.jp/",
    "website": "https://www.portal.azure.com",
    "website": "https://www.reuters.com",
    "website": "https://www.semanticscholar.org/",
    "website": "https://www.seznam.cz/",
    "website": "https://www.so.com/",
    "website": "https://www.sogou.com/",
    "website": "https://www.springernature.com/",
    "website": "https://zlibrary-global.se",
    "website": 'http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion',
    "website": 'https://1337x.to/',
    "website": 'https://1x.com/',
    "website": 'https://9gag.com/',
    "website": 'https://about.gitea.com',
    "website": 'https://ai.cloudflare.com',
    "website": 'https://api.invidious.io/',
    "website": 'https://bandcamp.com/',
    "website": 'https://base-search.net',
    "website": 'https://bitchute.com',
    "website": 'https://bt4gprx.com',
    "website": 'https://btdig.com',
    "website": 'https://cachyos.org',
    "website": 'https://codeberg.org/aryak/mozhi',
    "website": 'https://deepl.com',
    "website": 'https://deezer.com',
    "website": 'https://dictzone.com/',
    "website": 'https://digbt.org',
    "website": 'https://duckduckgo.com/',
    "website": 'https://emojipedia.org',
    "website": 'https://f-droid.org/',
    "website": 'https://findthatmeme.com',
    "website": 'https://fonts.google.com/icons',
    "website": 'https://frinkiac.com',
    "website": 'https://genius.com/',
    "website": 'https://github.com/',
    "website": 'https://github.com/mwmbl/mwmbl',
    "website": 'https://github.com/searxng/searxng',
    "website": 'https://github.com/TeamPiped/Piped/',
    "website": 'https://grokipedia.com',
    "website": 'https://hub.docker.com',
    "website": 'https://images.google.com',
    "website": 'https://imdb.com/',
    "website": 'https://imgur.com/',
    "website": 'https://jisho.org',
    "website": 'https://joinmastodon.org/',
    "website": 'https://joinpeertube.org',
    "website": 'https://kickasstorrents.to',
    "website": 'https://lemmy.ml/',
    "website": 'https://libretranslate.com',
    "website": 'https://lingva.ml',
    "website": 'https://live.space',
    "website": 'https://matrixrooms.info',
    "website": 'https://mediathekviewweb.de/',
    "website": 'https://metacpan.org/',
    "website": 'https://music.yandex.ru',
    "website": 'https://mymemory.translated.net/',
    "website": 'https://news.google.com',
    "website": 'https://news.yahoo.com',
    "website": 'https://nvd.nist.gov',
    "website": 'https://nyaa.si/',
    "website": 'https://openclipart.org/',
    "website": 'https://openverse.org/',
    "website": 'https://pdimagearchive.org',
    "website": 'https://photon.komoot.io',
    "website": 'https://pixabay.com',
    "website": 'https://rumble.com/',
    "website": 'https://scanr.enseignementsup-recherche.gouv.fr',
    "website": 'https://search.yahoo.com/',
    "website": 'https://sepiasearch.org',
    "website": 'https://stackexchange.com',
    "website": 'https://startpage.com',
    "website": 'https://store.steampowered.com/',
    "website": 'https://thepiratebay.org',
    "website": 'https://tineye.com',
    "website": 'https://unsplash.com',
    "website": 'https://uxwing.com',
    "website": 'https://vimeo.com/',
    "website": 'https://wiki.archlinux.org/',
    "website": 'https://wikidata.org/',
    "website": 'https://www.apkmirror.com',
    "website": 'https://www.apple.com/app-store/',
    "website": 'https://www.apple.com/maps/',
    "website": 'https://www.artic.edu',
    "website": 'https://www.artstation.com/',
    "website": 'https://www.bing.com',
    "website": 'https://www.bing.com/images',
    "website": 'https://www.bing.com/news',
    "website": 'https://www.bing.com/videos',
    "website": 'https://www.dailymotion.com',
    "website": 'https://www.deviantart.com/',
    "website": 'https://www.dokuwiki.org/',
    "website": 'https://www.duden.de',
    "website": 'https://www.ebay.com',
    "website": 'https://www.ebi.ac.uk/pdbe',
    "website": 'https://www.flickr.com',
    "website": 'https://www.google.com',
    "website": 'https://www.ina.fr/',
    "website": 'https://www.loc.gov/pictures/',
    "website": 'https://www.mixcloud.com/',
    "website": 'https://www.opensemanticsearch.org/',
    "website": 'https://www.openstreetmap.org/',
    "website": 'https://www.pexels.com',
    "website": 'https://www.pinterest.com/',
    "website": 'https://www.pixiv.net/',
    "website": 'https://www.qwant.com/',
    "website": 'https://www.radio-browser.info/',
    "website": 'https://www.reddit.com/',
    "website": 'https://www.rottentomatoes.com/',
    "website": 'https://www.senscritique.com/',
    "website": 'https://www.solidtorrents.to/',
    "website": 'https://www.spotify.com',
    "website": 'https://www.svgrepo.com',
    "website": 'https://www.tokyotosho.info/',
    "website": 'https://www.tubearchivist.com',
    "website": 'https://www.wikipedia.org/',
    "website": 'https://www.wolframalpha.com',
    "website": 'https://www.wolframalpha.com/',
    "website": 'https://www.wordnik.com',
    "website": 'https://www.youtube.com/',
    "website": 'https://yacy.net/',
    "website": 'https://yandex.com/',
    "website": None,
    "week": "pw",
    "week": 7,
    "week": timedelta(days=7),
    "wikidata_id": "Q102046570",
    "wikidata_id": "Q104863992",
    "wikidata_id": "Q107586666",
    "wikidata_id": "Q10846064",
    "wikidata_id": "Q108943604",
    "wikidata_id": "Q110718454",
    "wikidata_id": "Q115288326",
    "wikidata_id": "Q116777127",
    "wikidata_id": "Q118398",
    "wikidata_id": "Q1201876",
    "wikidata_id": "Q123663245",
    "wikidata_id": "Q124636097",
    "wikidata_id": "Q12805",
    "wikidata_id": "Q130879",
    "wikidata_id": "Q136410803",
    "wikidata_id": "Q14772",
    "wikidata_id": "Q15054354",
    "wikidata_id": "Q1540899",
    "wikidata_id": "Q15735774",
    "wikidata_id": "Q15913890",
    "wikidata_id": "Q21096327",
    "wikidata_id": "Q22661180",
    "wikidata_id": "Q22906900",
    "wikidata_id": "Q22908627",
    "wikidata_id": "Q239303",
    "wikidata_id": "Q2984686",
    "wikidata_id": "Q3077586",
    "wikidata_id": "Q3077675",
    "wikidata_id": "Q3490485",
    "wikidata_id": "Q3792882",
    "wikidata_id": "Q45287179",
    "wikidata_id": "Q485639",
    "wikidata_id": "Q48816502",
    "wikidata_id": "Q494817",
    "wikidata_id": "Q5188229",
    "wikidata_id": "Q565",
    "wikidata_id": "Q568769",
    "wikidata_id": "Q5977430",
    "wikidata_id": "Q686797",
    "wikidata_id": "Q697233",
    "wikidata_id": "Q6979334",
    "wikidata_id": "Q7067518",
    "wikidata_id": "Q725967",
    "wikidata_id": "Q752099",
    "wikidata_id": "Q7554565",
    "wikidata_id": "Q78514485",
    "wikidata_id": "Q79576",
    "wikidata_id": "Q835703",
    "wikidata_id": 'Q100769064',
    "wikidata_id": 'Q101240504',
    "wikidata_id": 'Q101445877',
    "wikidata_id": 'Q103204',
    "wikidata_id": 'Q104765127',
    "wikidata_id": 'Q105584',
    "wikidata_id": 'Q107315222',
    "wikidata_id": 'Q107565255',
    "wikidata_id": 'Q111664849',
    "wikidata_id": 'Q1136',
    "wikidata_id": 'Q12020',
    "wikidata_id": 'Q12805',
    "wikidata_id": 'Q131454',
    "wikidata_id": 'Q1386210',
    "wikidata_id": 'Q14657870',
    "wikidata_id": 'Q156376',
    "wikidata_id": 'Q1665109',
    "wikidata_id": 'Q16676060',
    "wikidata_id": 'Q17062285',
    "wikidata_id": 'Q1746538',
    "wikidata_id": 'Q1759675',
    "wikidata_id": 'Q17639196',
    "wikidata_id": 'Q182496',
    "wikidata_id": 'Q18693938',
    "wikidata_id": 'Q2013',
    "wikidata_id": 'Q207006',
    "wikidata_id": 'Q219885',
    "wikidata_id": 'Q22663',
    "wikidata_id": 'Q22908129',
    "wikidata_id": 'Q2333295',
    "wikidata_id": 'Q2382535',
    "wikidata_id": 'Q239303',
    "wikidata_id": 'Q24568389',
    "wikidata_id": 'Q24882614',
    "wikidata_id": 'Q255381',
    "wikidata_id": 'Q276101',
    "wikidata_id": 'Q277421',
    "wikidata_id": 'Q27877380',
    "wikidata_id": 'Q27986619',
    "wikidata_id": 'Q28134166',
    "wikidata_id": 'Q28233552',
    "wikidata_id": 'Q2878637',
    "wikidata_id": 'Q3044717',
    "wikidata_id": 'Q306956',
    "wikidata_id": 'Q337535',
    "wikidata_id": 'Q3419343',
    "wikidata_id": 'Q3495447',
    "wikidata_id": 'Q355022',
    "wikidata_id": 'Q364',
    "wikidata_id": 'Q368215',
    "wikidata_id": 'Q37312',
    "wikidata_id": 'Q43968444',
    "wikidata_id": 'Q44105684',
    "wikidata_id": 'Q448335',
    "wikidata_id": 'Q4537983',
    "wikidata_id": 'Q46523',
    "wikidata_id": 'Q4836698',
    "wikidata_id": 'Q4914152',
    "wikidata_id": 'Q50938515',
    "wikidata_id": 'Q52',
    "wikidata_id": 'Q521550',
    "wikidata_id": 'Q5281',
    "wikidata_id": 'Q545966',
    "wikidata_id": 'Q55823905',
    "wikidata_id": 'Q58024',
    "wikidata_id": 'Q602243',
    "wikidata_id": 'Q65551500',
    "wikidata_id": 'Q6883832',
    "wikidata_id": 'Q689141',
    "wikidata_id": 'Q73624591',
    "wikidata_id": 'Q769222',
    "wikidata_id": 'Q79343316',
    "wikidata_id": 'Q8034401',
    "wikidata_id": 'Q841507',
    "wikidata_id": 'Q847564',
    "wikidata_id": 'Q84777032',
    "wikidata_id": 'Q851864',
    "wikidata_id": 'Q866',
    "wikidata_id": 'Q936',
    "wikidata_id": 'Q9366',
    "wikidata_id": 'Q979593',
    "wikidata_id": None,
    "wikidiata_id": "Q7240905",
    "Windy": "partly cloudy",
    "wind_direction_10m",
    "wind_speed_10m",
    "WintryMix": "sleet",
    "xa-ar": "ar_SA",
    "year",
    "year": "py",
    "year": 365,
    "year": timedelta(days=365),
    "zh": "zh_chs",  # Chinese (Simplified)
    "zh": (
    "zh-classical": ("zh-classical",),
    "zh-HK": "zh_cht",
    "zh_Hans": "zh_chs",
    "zh_Hant": "zh_cht",  # Chinese (Traditional)
    #
    #          ^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^^
    #     match_json['domains'][0]['backlinks']
    #    data: [{type:"data",data: .... ["q","goggles_id"],route:1,url:1}}]
    #    node_ids: [0, 19],
    #   countries other than the United States.
    #   customers and, even more specifically, for customers in English-speaking
    #   documents written in a particular language.
    #   explicitly.
    #   https://annas-software.org/AnnaArchivist/annas-archive/-/blob/main/allthethings/templates/macros/md5_list.html
    #   https://developers.google.com/custom-search/docs/xml_results#crsp
    #   https://developers.google.com/custom-search/docs/xml_results#glsp
    #   https://developers.google.com/custom-search/docs/xml_results#hlsp
    #   https://developers.google.com/custom-search/docs/xml_results#iesp
    #   https://developers.google.com/custom-search/docs/xml_results#lrsp
    #   https://developers.google.com/custom-search/docs/xml_results#numsp
    #   https://developers.google.com/custom-search/docs/xml_results#oesp
    #   https://developers.google.com/custom-search/docs/xml_results_appendices#interfaceLanguages
    #   https://developers.google.com/custom-search/docs/xml_results_appendices#languageCollections
    #   Language Collection Values:
    #   matches the parameter value. See the Country Codes section for a list of
    #   more than 20 results, only 20 results will be returned.
    #   particular country.
    #   relevance of results. This is particularly true for international
    #   results, the gl parameter boosts search results whose country of origin
    #   search results, you are strongly encouraged to set this parameter
    #   Specifying a gl parameter value in WebSearch requests should improve the
    #   The cr parameter restricts search results to documents originating in a
    #   The default num value is 10, and the maximum value is 20. If you request
    #   The gl parameter value is a two-letter country code. For WebSearch
    #   The hl parameter specifies the interface language (host language) of
    #   The ie parameter sets the character encoding scheme that should be used
    #   The lr (language restrict) parameter restricts search results to
    #   The num parameter identifies the number of search results to return.
    #   The oe parameter sets the character encoding scheme that should be used
    #   to decode the XML result. The default oe value is latin1.
    #   to interpret the query string. The default ie value is latin1.
    #   valid values.
    #   your user interface. To improve the performance and the quality of your
    #  https://dictzone.com/trans/hello%20world/en_de
    # !ddw paris :es-AR --> {'ad': 'es_AR', 'ah': 'ar-es', 'l': 'ar-es'}
    # """The WEB-API "endpoint" is ``api``."""
    # "is_day",
    # "visibility",
    # '!go 日 :zh-CN' --> https://zh.m.wikipedia.org/zh/%E6%97%A5
    # '!go 日 :zh-TW' --> https://zh.m.wiktionary.org/zh-hant/%E6%97%A5
    # '!google natasha allegri' or '!google chris evans'
    # * actor / musician / artist
    # * book / performing art / film / television  / media franchise / concert tour / playwright
    # * company
    # * continent / country / department / location / waterfall
    # * prepared food
    # * website / software / os / programming language / file format / software engineer
    # - app_version=1740727428
    # - b (int): Search offset for pagination
    # - btf (str): Time filter, maps to values like 'd' (day), 'w' (week), 'm' (month)
    # - example: https://www.bing.com/images/async?q=foo&async=content&first=1&count=35
    # - example: one year (525600 minutes) 'qft=+filterui:age-lt525600'
    # - fl (bool): Indicates if a search language is used or not
    # - https://github.com/searxng/searxng/issues/1555
    # - https://github.com/searxng/searxng/pull/1679#issuecomment-1235432746
    # - iscqry (str): Empty string, necessary for results to appear properly on first page
    # - p (str): Search query string
    # - pz (str): Amount of results expected for the page
    # - user_id=451561-497874-703312-310156
    # - vl (str): The search language to use (e.g. lang_fr)
    # - vm (str): SafeSearch filter, maps to values like 'p' (None), 'i' (Moderate), 'r' (Strict)
    # 0	Clear sky
    # 1, 2, 3     Mainly clear, partly cloudy, and overcast
    # 45, 48      Fog and depositing rime fog
    # 51, 53, 55  Drizzle: Light, moderate, and dense intensity
    # 56, 57      Freezing Drizzle: Light and dense intensity
    # 61, 63, 65  Rain: Slight, moderate and heavy intensity
    # 66, 67    Freezing Rain: Light and heavy intensity
    # 71, 73, 75  Snow fall: Slight, moderate, and heavy intensity
    # 77    Snow grains
    # 80, 81, 82  Rain showers: Slight, moderate, and violent
    # 85, 86      Snow showers slight and heavy
    # 95          Thunderstorm: Slight or moderate
    # 96, 99      Thunderstorm with slight and heavy hail
    # ============
    # ================
    # ===========================
    # ?? https://www.startpage.com/sp/cdn/images/filter-chevron.svg
    # ?? ping-back URL: https://www.startpage.com/sp/pb?sc=TLsB0oITjZ8F21
    # ?? x = network.get('https://www.startpage.com/sp/cdn/images/filter-chevron.svg', headers=headers)
    # abstract
    # add answer if there is one
    # add infobox
    # add language tag if specified
    # add rest of adressdata, if something is already found
    # add the wikidata URL at the end
    # adobe_content_types
    # adobe_order
    # after the last page of results, spelling corrections are returned after a HTTP redirect
    # alias languages
    # alias regions
    # all scripts checked, but secret key was not found
    # all, or sending false queries to bing that could raise the suspicion of a
    # API v3 uses different parameters
    # append infobox
    # append link to site
    # append number of results
    # Apply SafeSearch if enabled
    # Apply time filter if specified
    # are enclosed in SGML comments.  These comments are *uncommented* by some
    # attributes
    # auth_key: Bearer XXXXX
    # basic search
    # bing for this market.  Alternatively we use the the market code from Honk
    # bot.
    # Build cookie
    # Build form data
    # Build OpenAlex query using search parameter and paging
    # Build sB cookie (for filters)
    # Build URL parameters
    # build URL query
    # By example: &lr=lang_zh-TW%7Clang_de selects articles written in
    # cache api key for future requests
    # cache counter value for 20sec
    # check for an API error
    # check if search result starts with something like: "2 Sep 2014 ... "
    # check if search result starts with something like: "5 days ago ... "
    # check if there are no results or page layout was changed so we cannot parse it
    # collapse multiple empty tokens
    # construct content body
    # construct url for preview image
    # convert the text to dom
    # Cookies
    # cr parameter:
    # create a new random arc_id every hour
    # create results
    # currently there are two rows for each result, so total count must be even
    # custom_query_json: '{ ... }'
    # Dates
    # DDG search form (POST data)
    # DDG XHTMLRequest
    # definition
    # definitions
    # Detect Baidu Captcha, it will redirect to wappass.baidu.com
    # Different to other google services, Google Scholar supports to select more
    # Don't add localization and country arguments if the user does select a
    # don't use nextpage when user selected to jump back to page 1
    # DTD: https://dtd.nlm.nih.gov/ncbi/pubmed/out/pubmed_250101.dtd
    # DTD: https://eutils.ncbi.nlm.nih.gov/eutils/dtd/20060628/esearch.dtd
    # eng_region = traits.get_region(params['searxng_locale'], 'en_US')
    # Esperanto
    # Example script source containing the data:
    # example: https://www.bing.com/news/infinitescrollajax?q=london&first=1
    # example: https://www.bing.com/videos/asyncv2?q=foo&async=content&first=1&count=35
    # example: one week (10080 minutes) '&qft= filterui:videoage-lt10080'  '&form=VRFLTR'
    # ext (file extensions)
    # extract attributes from XML
    # extracts valid app_js urls from soundcloud.com content
    # fake request to extract api url
    # Fediverse
    # fetch data from wikidata
    # fetch languages
    # fetch regions
    # filter out None and empty values
    # filter out suggested results on further page because they're the same on each page
    # fix market codes not known by bing news:
    # Fixes issue where domain would show up in the title
    # fmt: off
    # fmt: on
    # for better diff; sort the persistence of these traits
    # For example, the ``year:`` filter requires a *Premium Plan* subscription.
    # for live videos
    # form_data: dict[str,str] = {"v": "l", "api": "d.js", "o": "json"}
    # Full text queries
    # get journal and year
    # get name
    # get number of results
    # get number_of_results
    # get results
    # get spelling corrections
    # get the base URL for the language in which request was made
    # get the native name of every language known by babel
    # get token by calling the query page
    # gl parameter: (mandatory by Google News)
    # handle empty page
    # handle errors:  https://newznab.readthedocs.io/en/latest/misc/api/#newznab-error-codes
    # Handle languages
    # handle resp_json['queryresult']['assumptions']?
    # handle the 422 client side errors, and the possible 400 status code error
    # headers required to bypass bot-detection
    # headers["X-Requested-With"] = "XMLHttpRequest"
    # HINT: 'en-hk' is the region code it does not indicate the language en!!
    # hint: all three startpage engines (WEB, Images & News) can/should use the
    # hint: can take up to 40sec!
    # HINT: seems to have no effect (tested in google WEB & Images)
    # HINT: there exists an alternative backlink dict in the domains list / e.g.::
    # hl parameter:
    # http errors are handled manually to be able to reset the api url
    # HTTP headers
    # https://github.com/searxng/searxng/issues/2515#issuecomment-1606294635
    # https://github.com/searxng/searxng/issues/2515#issuecomment-1607150817
    # https://www.deviantart.com/search?q=foo
    # https://www.elastic.co/guide/en/elasticsearch/reference/current/full-text-queries.html
    # https://www.elastic.co/guide/en/elasticsearch/reference/current/term-level-queries.html
    # https://www.google.de/search?q=corona&hl=de&lr=lang_de&start=0&tbs=qdr%3Ad&safe=medium
    # ie parameter:
    # if no geojson is found and osm_type is a node, add geojson Point
    # if there are no results on this page, we get a redirect
    # image
    # img_src
    # import error is ignored because the admin has to install mysql manually to use
    # import error is ignored because the admin has to install postgresql
    # import error is ignored because the admin has to install pymongo manually
    # In bing the market code 'zh-cn' exists, but there is no 'news' category in
    # in some regions where geoblocking is employed (e.g. China),
    # in video items, the title is more or less a "content description", we try
    # include mailto if configured for polite pool (engine module setting)
    # insert alias to map from a script or region to a wikipedia variant
    # is on page 2, on page 3 its PERE1 and on page 4 its PERE2 .. and so forth.
    # journal is optional and may contains some coma
    # JS code, see query of class ".js-scroll-hidden" in Anna's HTML template:
    # js_obj_str = js_obj_str.replace("\xa0", "")  # remove ASCII for &nbsp;
    # js_obj_str = js_obj_str.replace(r"\u003C", "<").replace(r"\u003c", "<")  # fix broken HTML tags in strings
    # just select "web-result" and ignore results of class "result--ad result--ad--small"
    # kit.start(app, element, {
    # Kong.  Even if this is not correct, it is better than having no hits at
    # language (:de, :en, ..)
    # Language filter (expects ISO639-1 like 'fr', 'en')
    # languages
    # languages (UI)
    # list of zh-CN should not be no hant link instead you should find
    # locale --> https://github.com/searxng/searxng/issues/2672
    # lower-case : replacement
    # lr parameter:
    # manually to use the engine.
    # Map
    # map attributes to SearXNG result
    # Media
    # missing attributes: user_id, app_version
    # no results to fetch
    # num parameter:
    # oe parameter:
    # official language of google-country not in google-languages
    # Official website, Wikipedia page
    # OpenAlex may return YYYY, YYYY-MM or YYYY-MM-DD
    # OpenAlex `doi` field is commonly a full URL like https://doi.org/10.1234/abcd
    # Other
    # parse efetch response
    # parse results
    # parse results again if nothing is found yet
    # parse suggestion
    # Places
    # prevent automatic redirects to first page on pagination
    # Process definitions
    # process results
    # processing the results, two rows at a time
    # pylint: disable=bare-except
    # pylint: disable=import-outside-toplevel
    # pylint: disable=import-outside-toplevel, too-many-branches
    # pylint: disable=import-outside-toplevel, too-many-branches, too-many-statements
    # pylint: disable=line-too-long
    # pylint: disable=too-many-branches
    # pylint: disable=too-many-branches, too-many-statements
    # pylint: disable=too-many-branches, too-many-statements, disable=import-outside-toplevel
    # pylint: disable=too-many-locals
    # pylint: disable=too-many-locals, too-many-branches, too-many-statements
    # pylint: disable=too-many-statements
    # pylint: disable=unused-import
    # Query JSON defined by the instance administrator.
    # Quickhits
    # Raise for all other responses
    # raise for other errors
    # Reconstruct by placing tokens at their positions and joining with spaces.
    # regions
    # regions (aka "market codes")
    # regular expression for parsing torrent size strings
    # related topics
    # remove first and last lines to get only json
    # remove the api path
    # replace placeholders with actual content
    # replace shortcuts with API advanced search keywords
    # results sorted by seeder count
    # return empty array if nothing is found
    # return empty array if there are no results
    # return results
    # return results sorted by seeder
    # ret_val['params']['gl'] = country
    # ret_val['params']['num'] = 20
    # reuse the vqd value, the UA header must be static.
    # same sc_code ..
    # Search region/language
    # search regions of brave
    # Search results
    # search_res.get("Entity") possible values (not exhaustive) :
    # see https://github.com/TinEye/pytineye/blob/main/pytineye/api.py
    # see https://torznab.github.io/spec-1.3-draft/torznab/Specification-v1.3.html#predefined-attributes
    # send efetch request with the IDs from esearch response
    # set properties in the cookies
    # set title
    # setting the page number on the first page (i.e. s=0) triggers a rate-limit
    # show images first and text results second
    # simplify the code below: make sure extratags is a dictionary
    # skip these languages
    # Some locales (at least China) do not have a "next page" button and DDG
    # some will be converted to urls
    # sometimes, only adaptive m3u8 streams are available, so video_url is None
    # sourcehut_sort_order: longest-active
    # specify a region (country) only if a region is given in the selected
    # standard user agents are blocked by 'go-away', a foss bot detection tool
    # subdomain is: scholar.google.xy
    # supported domains
    # supported language codes
    # supported region codes
    # table name needs to be quoted to start with digits, so "cache" has been added to avoid sqlite complaining
    # tempC are possible
    # Term-level queries
    # Test zh_hans & zh_hant --> in the topmost links in the result list of list
    # than one language. The languages are separated by a pipe '|' (logical OR).
    # the "autotranslate" of dictzone is loaded by the JS from URL:
    # The 'first' arg should never send on page 1.
    # The abstract is returned as an inverted index {token: [positions...]}
    # the API of tootfinder has an issue that errors on server side are appended to the API response as HTML
    # the engine
    # the format is "{authors} - {journal}, {year} - {publisher}" or "{authors} - {year} - {publisher}"
    # The Interface Language:
    # the naming between different data objects is inconsitent, thus temp_C and
    # The rendering of the WEB page is strange; positions of Anna's result page
    # the useLocation is IP based, we use cookie "country" for the region
    # The vqd value is generated from the query and the UA header. To be able to
    # this ua will allow getting ~50 results instead of 10. #1641
    # thus we're only looking for the line that contains the actual json data and ignore everything else
    # time range
    # title & alt_forms
    # To get correct page, arg first and this arg FORM is needed, the value PERE
    # to merge with wikidata's infobox
    # to reduce the length of the title ..
    # To select 'all' languages an empty 'lr' value is used.
    # to the first page
    # to use the engine
    # TODO : remove result ending with "Meaning" or "Category"  # pylint: disable=fixme
    # traditional chinese OR german language.
    # translate.google.co.inGoogle Translate -> Google Translate
    # trim results so there's not way too many at once
    # Try to load JSON result
    # TW and HK you should a find wiktionary.org zh_hant link.  In the result
    # update server list once in 24h
    # Update the tokens to the newest ones
    # updated from u661.js to u.7669f071a13a7daa57cb / should be updated automatically?
    # URL
    # use ar --> ar_EG (Egypt's arabic)
    # use bn --> bn_BD
    # use de --> de_DE
    # use en --> en_US,
    # use es --> es_ES,
    # use fr --> rf_FR
    # use nl --> nl_NL
    # use pt --> pt_PT
    # user agent: https://www.mediawiki.org/wiki/Wikidata_Query_Service/User_Manual#Query_limits
    # user-friendly keywords
    # using SearXNG User-Agent
    # values that can't be determined by babel's languages names
    # we get html in a JSON container...
    # whatever the page number is
    # wikidata
    # WIKIDATA_PROPERTIES : add property labels
    # WIKIDATA_PROPERTIES : add unit symbols
    # will return a HTTP/2 403 Forbidden for a request of such a page.
    # works only sometimes?
    # write search-language back to params, required in response
    # www.bing.com redirects to the regional version of Bing
    # year
    # years_from
    # years_to
    # Z-library uses English names for languages, so we need to map them to their respective locales
    # zh.m.wikipedia.org/zh somewhere in the top.
    # _fmt:html returns a HTTP 500 when user search for celebrities like
    '&banners=raw'
    '&debuggingdata=false'
    '&format=image,plaintext,imagemap,minput,moutput'
    '&formattimeout=2'
    '&output=JSON'
    '&parsetimeout=2'
    '&proxycode={token}'
    '&scantimeout=0.5'
    '&sponsorcategories=true'
    '&statemethod=deploybutton'
    '&{query}'
    '''
    '''Build request parameters (see :ref:`engine request`).'''
    '''post-response callback
    '''pre-request callback
    '''Scrap *results* from the response (see :ref:`result types`).'''
    './div[contains(@class, "SearchSnippet-infoLabel")]/span/span[@data-test-id="snippet-published"]/strong/text()'
    '/?do=search'
    '?async=false'
    '?{query}&b={offset}'
    'abstract:': 'dcdescription:',
    'Accept-Language': "en-US,en;q=0.5",
    'AE:ar',
    'AL',  # Albanien (sq)
    'allow_embed',
    'any': 'search.yahoo.com',
    'AR:es-419',
    'asc': 'p',
    'AT:de',
    'AU:en',
    'author:': 'dccreator:',
    'AZ',  # Aserbaidschan  (az)
    'BD',  # Bangladesch (bn)
    'BD:bn',
    'BE:fr',
    'BE:nl',
    'bench',
    'bg': 'search.yahoo.com',
    'BG:bg',
    'bin',
    'BN',  # Brunei Darussalam (ms)
    'BR:pt-419',
    'brand:wikidata': partial(value_with_prefix, 'https://wikidata.org/wiki/'),
    'BT',  # Bhutan (dz)
    'building:levels',
    'BW:en',
    'CA:en',
    'CA:fr',
    'ca_ad', 'ca_es', 'ca_fr', 'co_fr', 'de_at', 'de_ch', 'de_de', 'en_au',
    'CH:de',
    'CH:fr',
    'CL:es-419',
    'CN:zh-Hans',
    'CO:es-419',
    'collection:': 'dccollection:',
    'contact:email': partial(value_with_prefix, 'mailto:'),
    'contact:facebook': value_to_https_link,
    'contact:fax': partial(value_with_prefix, 'fax:'),
    'contact:foursquare': value_to_https_link,
    'contact:instagram': value_to_https_link,
    'contact:linkedin': value_to_https_link,
    'contact:mastodon': value_to_https_link,
    'contact:phone': partial(value_with_prefix, 'tel:'),
    'contact:pinterest': value_to_https_link,
    'contact:telegram': value_to_https_link,
    'contact:tripadvisor': value_to_https_link,
    'contact:twitter': value_to_https_link,
    'contact:webcam': value_to_website_link,
    'contact:website': value_to_website_link,
    'contact:yelp': value_to_https_link,
    'contact:youtube': value_to_https_link,
    'contributor:': 'dccontributor:',
    'coverage:': 'dccoverage:',
    'created_time',
    'cs': 'search.yahoo.com',
    'CU:es-419',
    'cuisine',
    'currency:*',
    'custom': _custom_query,
    'CZ:cs',
    'da': 'search.yahoo.com',
    'date:': 'dcdate:',
    'day': 'interval="4"',
    'day': 1,
    'day': 24,
    'day': 60 * 24,
    'day': 60 * 60 * 24,
    'day': relativedelta(),
    'day': timedelta(days=1),
    'DE:de',
    'delivery',
    'delivery:covid19',
    'desc': '-p',
    'description',
    'duration',
    'EG:ar',
    'el': 'search.yahoo.com',
    'email': partial(value_with_prefix, 'mailto:'),
    'en': 'search.yahoo.com',
    'en_ca', 'en_gb', 'en_ie', 'en_my', 'en_nz', 'en_us', 'es_ad', 'es_ar',
    'ES:es',
    'es_cl', 'es_co', 'es_es', 'es_mx', 'es_pe', 'eu_es', 'eu_fr', 'fc_ca',
    'ET',  # Äthiopien (am)
    'et': 'search.yahoo.com',
    'ET:en',
    'ET:en',  # english (ethiopia)
    'facebook': value_to_https_link,
    'fax': partial(value_with_prefix, 'fax:'),
    'fee',
    'filter': {
    'format': 'JSON',
    'format:': 'dcformat:',
    'FR:fr',
    'fr_ad', 'fr_be', 'fr_ca', 'fr_ch', 'fr_fr', 'it_ch', 'it_it', 'nl_be',
    'GB:en',
    'GE',  # Georgien (ka, os)
    'GH:en',
    'GL',  # Grönland (kl)
    'GR:el',
    'hdate:': 'dchdate:',
    'he': 'search.yahoo.com',
    'HK:zh-Hant',
    'hour': timedelta(hours=1),
    'hr': 'search.yahoo.com',
    'https://api.base-search.net/cgi-bin/BaseHttpSearchInterface.fcgi'
    'https://api.flickr.com/services/rest/?method=flickr.photos.search'
    'https://news.search.yahoo.com/search'
    'HU:hu',
    'id',
    'ID:en',
    'ID:en',  # english (indonesia)
    'ID:id',
    'IE:en',
    'IL:en',
    'IL:he',
    'IN:bn',
    'IN:en',
    'IN:hi',
    'IN:ml',
    'IN:mr',
    'IN:ta',
    'IN:te',
    'internet_access:ssid',
    'IT:it',
    'ja': 'search.yahoo.com',
    'JP:ja',
    'KE:en',
    'KH',  # Kambodscha (km)
    'ko': 'search.yahoo.com',
    'KR:ko',
    'LA',  # Laos (lo)
    'language': 'de',
    'language': 'fr',
    'language': 'it',
    'language:': 'dclanguage:',
    'LB:ar',
    'level',
    'LK',  # Sri Lanka (si, ta)
    'LT:lt',
    'LV:en',
    'LV:en',  # english (latvia)
    'LV:lv',
    'MA:fr',
    'match': _match_query,
    'ME',  # Montenegro (sr)
    'minute': timedelta(minutes=1),
    'MK',  # Nordmazedonien (mk, sq)
    'MM',  # Myanmar (my)
    'MN',  # Mongolei (mn)
    'month': 'interval="9"',
    'month': 24 * 30,
    'month': 31,
    'month': 60 * 24 * 31,
    'month': 60 * 60 * 24 * 7 * 4,
    'month': relativedelta(months=-1),
    'month': timedelta(days=30),
    'month': timedelta(days=31),
    'MV',  # Malediven (dv) // dv_MV is unknown by babel
    'MX:es-419',
    'MY',  # Malaysia (ms)
    'MY:en',
    'NA:en',
    'NG:en',
    'NL:nl',
    'nl_nl', 'pt_ad', 'pt_pt',
    'NO:no',
    'NP',  # Nepal (ne)
    'NZ:en',
    'official_api_documentation': "https://wiki.tootfinder.ch/index.php?name=the-tootfinder-rest-api",
    'official_api_documentation': 'https://destatis.api.bund.dev/',
    'official_api_documentation': 'https://docs.developer.yelp.com',
    'official_api_documentation': 'https://github.com/eazyliving/fyyd-api',
    'official_api_documentation': 'https://github.com/voc/voctoweb',
    'official_api_documentation': 'https://repology.org/api/v1',
    'official_api_documentation': 'https://selfh.st/icons-about/',
    'official_api_documentation': 'https://wallhaven.cc/help/api',
    'official_api_documentation': 'https://www.elastic.co/guide/en/elasticsearch/reference/current/search-search.html',
    'official_api_documentation': 'https://www.ipernity.com/help/api',
    'official_api_documentation': 'https://www.mojeek.com/support/api/search/request_parameters.html',
    'official_api_documentation': None,
    'official_api_documentation': None,  # requires an account
    'opening_hours',
    'opening_hours:covid19',
    'organic',
    'outdoor_seating',
    'P11947': 'Lemmy',
    'P12622': 'PeerTube',
    'P1651': 'YouTube',
    'P2002': 'Twitter',
    'P2003': 'Instagram',
    'P2013': 'Facebook',
    'P2397': 'YouTube',
    'P345': 'IMDb',
    'P4033': 'Mastodon',
    'P434': 'MusicBrainz',
    'P435': 'MusicBrainz',
    'P436': 'MusicBrainz',
    'P966': 'MusicBrainz',
    'payment:*',
    'PE:es-419',
    'PH:en',
    'phone': partial(value_with_prefix, 'tel:'),
    'PK:en',
    'PL:pl',
    'price': 'p',
    'PT:pt-150',
    'public_transport',
    'publisher:': 'dcpublisher:',
    'query' term to your local engine to filter out the results.
    'query': {
    'relation:': 'dcrelation:',
    'relevance': None,
    'require_api_key': False,
    'results': "HTML",
    'results': "JSON",
    'results': 'HTML',
    'results': 'JSON',
    'rights:': 'dcrights:',
    'RO:ro',
    'RS:sr',
    'RU:ru',
    'SA:ar',
    'SE:sv',
    'SG:en',
    'SI:sl',
    'simple_query_string': _simple_query_string_query,
    'size': number_of_results,
    'sk': 'search.yahoo.com',
    'SK:sk',
    'sl': 'search.yahoo.com',
    'SN:fr',
    'source:': 'dcsource:',
    'subject:': 'dcsubject:',
    'term': _term_query,
    'terms': _terms_query,
    'TH:th',
    'thumbnail_360_url',
    'title',
    'title:': 'dctitle:',
    'TJ',  # Tadschikistan (tg)
    'TM',  # Turkmenistan (tk)
    'TR:tr',
    'TW:zh-Hant',
    'type:': 'dcdctype:',
    'TZ:en',
    'UA:ru',
    'UA:uk',
    'UG:en',
    'url',
    'urls:': 'dcidentifier:',
    'US:en',
    'US:es-419',
    'use_official_api': False,
    'use_official_api': True,
    'UZ',  # Usbekistan (uz)
    'VE:es-419',
    'VN:vi',
    'website': "https://tagesschau.de",
    'website': "https://www.bpb.de",
    'website': "https://www.chefkoch.de",
    'website': "https://www.moviepilot.de",
    'website': "https://www.tootfinder.ch",
    'website': 'https://fyyd.de',
    'website': 'https://geizhals.de',
    'website': 'https://lib.rs',
    'website': 'https://media.ccc.de',
    'website': 'https://mojeek.com',
    'website': 'https://pkg.go.dev/',
    'website': 'https://podcastindex.org',
    'website': 'https://repology.org',
    'website': 'https://selfh.st/icons/',
    'website': 'https://voidlinux.org/packages/',
    'website': 'https://wallhaven.cc/',
    'website': 'https://www.alpinelinux.org',
    'website': 'https://www.ansa.it',
    'website': 'https://www.destatis.de',
    'website': 'https://www.elastic.co',
    'website': 'https://www.goodreads.com',
    'website': 'https://www.ipernity.com',
    'website': 'https://yep.com/',
    'website': value_to_website_link,
    'week': 'interval="7"',
    'week': 24 * 7,
    'week': 60 * 24 * 7,
    'week': 60 * 60 * 24 * 7,
    'week': 7,
    'week': relativedelta(weeks=-1),
    'week': timedelta(days=7),
    'wheelchair',
    'wikidata': partial(value_with_prefix, 'https://wikidata.org/wiki/'),
    'wikidata_id': "Q703907",
    'wikidata_id': 'Q107409859',
    'wikidata_id': 'Q113486010',
    'wikidata_id': 'Q15977657',
    'wikidata_id': 'Q19310966',
    'wikidata_id': 'Q2359213',
    'wikidata_id': 'Q3050461',
    'wikidata_id': 'Q392934',
    'wikidata_id': 'Q4033826',
    'wikidata_id': 'Q60747299',
    'wikipedia': value_wikipedia_link,
    'year': 24 * 365,
    'year': 365,
    'year': 60 * 24 * 365,
    'year': 60 * 60 * 24 * 7 * 52,
    'year': relativedelta(years=-1),
    'year': timedelta(days=365),
    'ZA:en',
    'zh-CN': "zh_chs",
    'zh-TW': "zh_cht",
    'zh_chs': 'hk.search.yahoo.com',
    'zh_cht': 'tw.search.yahoo.com',
    'ZW:en',
    '_source': ['documentation', "abstract"],
    (But here's the good news: Authenticated requests get a higher rate limit)
    (region) higher than a language (compare :py:obj:`wiki_lc_locale_variants`).
    (template) and therefore also have the same URL.  Results with identical
    (``wiki_netloc``).  Here is a reduced example:
    )
    )  # pyright: ignore[reportAssignmentType]
    ),
    ).encode('utf-8')
    ):
    )[:2]
    * "{authors} - {journal}, {year} - {publisher}"
    * "{authors} - {publisher}"
    * "{authors} - {year} - {publisher}"
    * https://docs.python.org/3/library/sqlite3.html#sqlite3.connect
    * https://www.sqlite.org/uri.html
    *height*.  If no URL can be generated from the *thumbnail data*, an empty
    + '&api_key={api_key}&{text}&sort=relevance'
    + '&extras=description%2C+owner_name%2C+url_o%2C+url_n%2C+url_z'
    + '&per_page={nb_per_page}&format=json&nojsoncallback=1&page={page}'
    + '?func=PerformSearch&{query}&boost=oa&hits={hits}&offset={offset}'
    - Checks whether the tenant_id, client_id and client_secret are set,
    - en_AU
    - en_CA
    - en_GB
    - en_US
    - https://stackoverflow.com/a/33691240
    - in the cookie the **region** is selected
    - in the HTTP POST data the **language** is selected
    - Instantiate a cache for this engine (:py:obj:`CACHE`).
    - `backlinks`, a list of Backlink objects pointing to the original websites
    - `domain`, domain this result was found on.
    - `filesize`, image size in bytes.
    - `format`, image format.
    - `height`, image height in pixels.
    - `image_url`, link to the result image.
    - `overlay`, overlay URL.
    - `score`, a number (0 to 100) that indicates how closely the images match.
    - `size`, image area in pixels.
    - `tags`, whether this match belongs to a collection or stock domain.
    - `width`, image width in pixels.
    - ``arc_id:<...>,use_ac:true,_fmt:prog``
    - ``category``: 'search category'
    - ``data``: {}  # if method == POST
    - ``headers``: {}
    - ``method`` : POST/GET
    - ``pageno``: 1  # number of the requested page
    - ``traits.custom['countrycodes']``: `list of countries API`_
    - ``traits.custom['WIKIPEDIA_LANGUAGES']``: not used in the wikipedia engine
    - ``traits.custom['wiki_netloc']``: wikidata does not have net-locations for
    - ``traits.languages``: `list of languages API`_
    - ``url``: ''
    ..
    .. code: python
    .. code:: python
    .. hint::
    .. _8ed5c729 - Refactor and redesign client:
    .. _ApiSearchResult:
    .. _ApiSearchResults:
    .. _list of countries API: https://de1.api.radio-browser.info/#List_of_countries
    .. _list of languages API: https://de1.api.radio-browser.info/#List_of_languages
    .. _PR1071: https://github.com/searxng/searxng/pull/1071
    .. _REST-API for the images:
    .. _videoLanguages:
    ...
    0: "clear sky",
    0: 'false',
    0: {},
    1: "fair",
    1: 'true',
    1: {'is_created_for_kids': 'true'},
    2025 (current year minus one):
    2: "partly cloudy",
    2: 'true',
    2: {'is_created_for_kids': 'true'},
    3: "cloudy",
    45: "fog",
    48: "fog",
    51: "light rain",
    53: "light rain",
    55: "light rain",
    56: "light sleet showers",
    57: "light sleet",
    61: "light rain",
    63: "rain",
    65: "heavy rain",
    66: "light sleet showers",
    67: "light sleet",
    71: "light sleet",
    73: "sleet",
    75: "heavy sleet",
    77: "snow",
    80: "light rain showers",
    81: "rain showers",
    82: "heavy rain showers",
    85: "snow showers",
    86: "heavy snow showers",
    95: "rain and thunder",
    96: "light snow and thunder",
    99: "heavy snow and thunder",
    :param cookie_params: Dictionary of cookie parameters
    :param dict param: Request parameters of the engine.  At least
    :param eng_traits: Engine's traits fetched from google preferences
    :param params: request parameters
    :param query: the query term
    :py:obj:`api_key` is set, otherwise the engine is inactive.
    :py:obj:`custom field <searx.enginelib.traits.EngineTraits.custom>`
    :py:obj:`editing depth <wikipedia_article_depth>`.
    :py:obj:`list_of_wikipedias` are supported by SearXNG locales, only those
    :py:obj:`sc_code_cache_sec` seconds."""
    :py:obj:`sqlite3.Cursor`.
    :py:obj:`wc_search_type` is valid."""
    :ref:`google videos engine`).
    :returns:
    :returns: Formatted cookie string
    :rtype: dict
    :rtype: str
    :type cookie_params: dict
    </form>
    <form action="/sp/search" method="post">
    <search_form_xpath>`.  Without this argument Startpage considers the request
    <searx.enginelib.traits.EngineTraits.custom>` (``wiki_netloc``).  Depending
    <searx.engines.wikipedia.fetch_wikimedia_traits>` and removes
    ?item rdfs:label ?itemLabel .
    ?wikipediaUrl schema:about ?item;
    ?{name}Node rdf:type wikibase:BestRank ; ps:{name} ?{name} .
    a = dom.xpath("//a")
    aa_content: "magazine"
    aa_ext: "pdf"
    aa_sort: "newest"
    about:
    abstractURL = search_res.get("AbstractURL", "")
    abstract_inverted_index: dict[str, list[int]] | None,
    Accepts multiple terms and the name of the field.
    Accepts one term and the name of the field.
    Accepts query strings, but it is less strict than query_string
    Additionally the arguments form Startpage's search form needs to be set in
    address = {}
    address_name = None
    address_raw = result.get('address')
    add_amount('P2046')  # area
    add_amount('P2048')  # height (building)
    add_amount('P281')  # postal code
    add_date('P577')  # publication date
    add_image('P15', priority=1, url_id='wikimedia_image')  # route map
    add_image('P154', priority=3, url_id='wikimedia_image')  # logo
    add_image('P18', priority=4, url_id='wikimedia_image')  # image
    add_image('P242', priority=2, url_id='wikimedia_image')  # locator map
    add_image('P2716', priority=6, url_id='wikimedia_image')  # collage
    add_image('P2910', priority=7, url_id='wikimedia_image')  # icon
    add_image('P41', priority=5, url_id='wikimedia_image')  # flag
    add_label('P1018')  # language regulatory body
    add_label('P1029')  # crew members (tripulation)
    add_label('P112')  # founded by
    add_label('P1346')  # winner (sports, contests, ...)
    add_label('P136')  # genre (music, film, artistic...)
    add_label('P137')  # operator (service, facility, ...)
    add_label('P1454')  # legal form (company, organization)
    add_label('P169')  # ceo
    add_label('P225')  # taxon name
    add_label('P275')  # copyright license
    add_label('P277')  # programming language
    add_label('P282')  # writing system
    add_label('P364')  # original language
    add_label('P38')  # currency
    add_label('P840')  # narrative location
    add_url('P11947', url_path_prefix='/c/')  # Lemmy community
    add_url('P12622', url_path_prefix='/c/')  # PeerTube channel
    add_url('P1324')  # source code repository
    add_url('P1581')  # blog
    add_url('P1651', url_id='youtube_video')
    add_url('P2002', url_id='twitter_profile')
    add_url('P2003', url_id='instagram_profile')
    add_url('P2013', url_id='facebook_profile')
    add_url('P2397', url_id='youtube_channel')
    add_url('P345', url_id='imdb_id')
    add_url('P4033', url_path_prefix='/@')  # Mastodon user
    add_url('P434', url_id='musicbrainz_artist')
    add_url('P435', url_id='musicbrainz_work')
    add_url('P436', url_id='musicbrainz_release_group')
    add_url('P856', official=True)  # official website
    add_url('P966', url_id='musicbrainz_label')
    add_value('P1082')  # population
    add_value('P1098')  # number of speakers
    add_value('P1120')  # number of deaths
    add_value('P212')  # ISBN-13
    add_value('P218')  # language code (ISO 639-1)
    add_value('P274')  # chemical formula
    add_value('P348')  # version
    add_value('P498')  # currency code (ISO 4217)
    add_value('P957')  # ISBN-10
    adobe_content_types: ["photo", "illustration", "zip_vector", "template", "3d", "image"]
    adobe_content_types: ["video"]
    adobe_order: relevance
    all_results = eval_xpath_list(dom, results_xpath)
    an access token."""
    answer = "{0} {1} = {2} {3} (1 {5} : {4} {6})".format(
    answer: str = search_res.get("Answer", "")
    API rate limit exceeded for <redacted ip>.
    api_key: "..."
    api_key: 'YOUR-API-KEY'  # required
    api_path = hit['result'].get('api_path')
    api_result = resp.json()
    api_url = extr(resp.text, 'const r="', '"', default=None)
    api_url = f"{base_url.rstrip('/')}/{api_path}?".format(language=params['language'])
    API`).
    app_js_urls = [tag.get("src") for tag in script_tags if tag is not None]
    arch_path = ARCH_RE.search(query)
    arc_id = f"arc_id:srp_{_arcid_random[0]}_1{start:02}"
    area_to_osm_zoom,
    args = urlencode(
    args = urlencode({"page": params["pageno"], "per_page": page_size, "sort": sort_criteria, "search": query})
    args = urlencode({"page": params["pageno"], "q": query, "per_page": page_size})
    args = {
    args = {"q": params["query"], "langpair": f"{params['from_lang'][1]}|{params['to_lang'][1]}"}
    args = {"q": query}
    args = {"search": query, "page": params["pageno"], "sort": sourcehut_sort_order}
    args = {'from': params['from_lang'][1], 'to': params['to_lang'][1], 'text': params['query'], 'engine': mozhi_engine}
    args = {'page': params['pageno'] - 1, 'searchKey': query, 'size': results_per_page}
    args = {'q': query, 'limit': page_size, 'sort': sort, 'order': order, 'page': params['pageno']}
    args = {'q': query, 'page': params['pageno']}
    args = {'q': query}
    args = {'query': query, 'limit': number_of_results, 'offset': (params['pageno'] - 1) * number_of_results}
    args = {'query': query}
    args = {'search': query, 'page': params['pageno']}
    args = {'text': query, 'page': params['pageno'] - 1}
    args.update(safesearch_params.get(params['safesearch'], {}))
    args.update(time_range_args(params))
    Args:
    args: dict[str, str | int] = {
    args: dict[str, t.Any] = {
    args["ct"] = "EN"
    argument, but it seems to be a kind of a *timestamp*.
    At Void Linux, several packages sometimes share the same source code
    attributes = get_attributes(language)
    attributes = resp.search_params['attributes']
    attributes = []
    Attributes `(class Match) <https://github.com/TinEye/pytineye/blob/main/pytineye/api.py>`__
    attributes.append(WDArticle(language))  # wikipedia (user language)
    attributes.append(WDGeoAttribute('P625'))
    attributes: list[dict[str, str | dict[str, str]]] = []
    audio_data = item["audio_data"]
    authors: list[str] = s_text[0].split(", ")
    authors: list[str] = []
    author_elements = eval_xpath_list(item, './/div[@class="authors"]//a[@itemprop="author"]')
    autotranslate = http_get(f"{base_url}/trans/{query}/{from_lang}_{to_lang}", timeout=1.0)
    babel.core.UnknownLocaleError: unknown locale 'no_NO'
    babel_reg_list = get_global("territory_languages").keys()
    backend_url:
    backlinks = []
    base_invidious_url = resp.search_params['base_url'] + "/watch?v="
    base_url = 'https://' + netloc + '/index.php?'
    base_url:
    base_url: http://localhost:7700
    base_url: http://localhost:8983
    base_url: http://localhost:9200
    base_url: https://code.forgejo.org
    base_url: https://gitea.com
    base_url: https://gitlab.com
    base_url: https://gitlab.gnome.org
    base_url: https://mrs-host
    bd:serviceParam wikibase:language "%LANGUAGE%,en".
    Beside regions DuckDuckGo also defines its languages by region codes.  By
    biblio: dict[str, str] = item.get("biblio", {})
    bing_ui_lang_map = {
    body = {
    book_comic, magazine, standards_document
    book_fiction, book_unknown, book_nonfiction,
    book_rating = _text(item, './/span[contains(@class, "book-rating-interest-score")]')
    brave_category: goggles
    brave_category: images
    brave_category: news
    brave_category: search
    brave_category: videos
    bt4g_category: 'movie'
    bt4g_order_by: seeders
    btf = time_range_dict.get(params['time_range'])
    CACHE = EngineCache("cache" + engine_settings["name"])
    CACHE = EngineCache("radio_browser")
    CACHE = EngineCache("startpage")
    CACHE = EngineCache(engine_settings["name"])
    CACHE = EngineCache(engine_settings["name"])  # type:ignore
    cache = get_cache()
    CACHE.set("count", count, expire=20)
    CACHE.set('ahmia-tokens', token_str, expire=60 * 60)
    CACHE.set(key="SC_CODE", value=sc_code, expire=sc_code_cache_sec)
    CACHE.set(key="servers", value=servers, expire=60 * 60 * 24)
    CACHE.set(key=COOKIE_CACHE_KEY, value=cookie, expire=COOKIE_CACHE_EXPIRATION_SECONDS)
    CACHE.set(key=CSRF_PRIVATEKEY_CACHE, value=private_token, expire=KEY_EXPIRATION_SECONDS)
    CACHE.set(key=CSRF_PUBLICKEY_CACHE, value=public_token, expire=KEY_EXPIRATION_SECONDS)
    CACHE.set(key=key, value=token, expire=azure_token_expiration_seconds)
    cache.set(key=key, value=value, expire=3600)
    can be ignored::
    catalog_engine2code = {name.lower(): lang_code for lang_code, name in babel.Locale('en').languages.items()}
    catalog_engine2code.update(
    categ = startpage_categ.capitalize()
    categories = ["general", "articles"]
    categories: general
    categories: images
    categories: video
    categories: [general, videos]
    categories: [general, web]
    categories: [images, web]
    categories: [images]
    categories: [news, web]
    categories: [videos]
    category_config = {
    ceid = locales.get_engine_locale(sxng_locale, traits.custom['ceid'], default='US:en')
    ceid_lang, ceid_suffix = (
    ceid_region, ceid_lang = ceid.split(':')
    channel: etree.Element = search_results[0]
    Checks if the user provided path is inside the working directory.  If not,
    check_parsing_options(engine_settings)
    cited_by_count = item.get("cited_by_count")
    client_id = ""
    cmd = []
    cmd = _get_command_to_run(query)
    collection: 'reviews'  # name of the db collection
    collection: my-collection
    command = engine_settings['command']
    command: ['find', '.', '-name', '{{QUERY}}']
    comments = get_attribute(item, 'comments')
    configured.
    connect()
    contains over 7000 *languages* codes (see PR1071_).  We use only those
    content = " | ".join(
    content = ""
    content = """{title} - {authors} {journal} ({volume}) {page} ({year})"""
    content = ''
    content = audio_data.get("description") or ""
    content = html_to_text(result.get('description'))
    content = remove_pua_from_str(html_to_text(result.get('description')))
    content = res.get('name_with_artist', res.get('name', ''))
    content = resp.json()
    content, publishedDate = _parse_published_date(content)
    content: list[str] = []
    content: str = ""
    content_filter = html_to_text if content_html_to_text else identity
    content_list = []
    content_parts = build_content_parts(item, title, original_title)
    content_parts = []
    content_query : summary
    content_xpath : //article[@class="repo-summary"]/p
    cookie = get_cookie(params["url"])
    cookie = headers['set-cookie'].split(";")[0]
    cookie = OrderedDict()
    cookie = {
    cookie. We need to form a request that is identical to the request built by
    Cookie: kl=en-us; df=m
    cookie: str | None = CACHE.get(COOKIE_CACHE_KEY)
    cookie['date_time'] = 'world'
    cookie['disable_family_filter'] = safesearch_dict[params['safesearch']]
    cookie['disable_open_in_new_window'] = '0'
    cookie['enable_post_method'] = '1'  # hint: POST
    cookie['enable_proxy_safety_suggest'] = '1'
    cookie['enable_stay_control'] = '1'
    cookie['instant_answers'] = '1'
    cookie['lang_homepage'] = 's/device/%s/' % lang_homepage
    cookie['num_of_results'] = '10'
    cookie['suggestions'] = '1'
    cookie['wt_unit'] = 'celsius'
    cookie_parts = []
    count = 0
    count: int = CACHE.get("count", 0)
    country = eng_traits.get_region(sxng_locale, eng_traits.all_locale)
    countrycodes = list(countrycodes)
    countrycodes = set()
    countrycodes.sort()
    country_js = country_js[: country_js.index("},k={default")]
    country_js = resp.text[resp.text.index("options:{all") + len("options:") :]
    country_list = get(f'{server}/json/countries').json()  # type: ignore
    country_tags = js_obj_str_to_python(country_js)
    counts: dict[str | None, int] = {}
    custom_query = custom_query_json
    data = json.loads(match.group(1))
    data = json.loads(text, strict=False)
    data = loads(extr(resp.text, 'var data = ', ';\n'))
    data = loads(resp.text)
    data = params["data"]
    data = resp.json()
    data = search_results.get('data', {})
    data = []
    data = _available_query_types[query_type](query)
    data = {
    data = {"offset": start_index, "limit": results_per_page, "query": query, "sensitivity_id": "normal", "sort": "new"}
    data = {"search": query, "offset": start_index}
    data.sort(key=lambda entry: (get_key_rank(entry['key']), entry['label']))
    data.update(args)
    data: dict[str, t.Any] = json.loads(json_str)
    database: 'business'
    database: searx/data/filmliste-v2.db
    data["q"] = query
    data_image_map = parse_data_images(resp.text)
    data_image_map = {}
    data_rows: list[dict[str, str]] = json.loads(_my_offline_engine)
    datetime,
    DATE_FORMAT = {
    db: 0
    DDG defines its languages by a region code (:py:obj:`fetch_traits`).  To
    def add_amount(name):
    def add_date(name):
    def add_image(name, url_id=None, priority=1):
    def add_label(name):
    def add_url(name, url_id=None, url_path_prefix=None, **kwargs):
    def add_value(name):
    def field(k: str) -> str:
    def format_10(self, value, locale):
    def format_11(self, value, locale):
    def format_13(self, value, locale):
    def format_14(self, value, locale):
    def format_8(self, value, locale):  # pylint: disable=unused-argument
    def format_9(self, value, locale):
    def get_geo_url(self, result, osm_zoom=19):
    def get_group_by(self):
    def get_label(self, language):
    def get_select(self):
    def get_str(self, result, language):
    def get_str(self, result, language):  # pylint: disable=unused-argument
    def get_where(self):
    def get_wikibase_label(self):
    def is_result_match(result: dict[str, t.Any]) -> bool:
    def is_result_match(result: tuple[str, list[str]]) -> bool:
    def _field_txt(xml: ElementType, xpath_str: str) -> str:
    def _list(k: str) -> list[str]:
    def _str(k: str) -> str:
    def _use_old_values():
    def __init__(self, language, kwargs=None):
    def __init__(self, name):
    def __init__(self, name, url_id=None, priority=100):
    def __init__(self, name, url_id=None, url_path_prefix=None, kwargs=None):
    def __repr__(self):
    default: str = "en_US",
    definitions = []
    definitionURL = search_res.get("DefinitionURL", "")
    delimiter:
    description: str
    details: str
    Detects if the response contains an Alibaba X5SEC CAPTCHA page.
    detect_google_captcha(dom)
    detect_google_sorry(resp)
    detect_google_sorry,
    disabled: false
    disabled: true
    discovery_filters = []
    doc = fromstring(resp.text)
    doc = html.fromstring(resp.text)
    doc = lxml.html.fromstring(resp.text)
    docs = data.get('response', {}).get('docs', [])
    dom = etree.fromstring(resp.content)
    dom = fromstring(resp.text)
    dom = html.fragment_fromstring(xmlsearchresult.text, create_parent='div')
    dom = html.fromstring(data)
    dom = html.fromstring(htmlResult)
    dom = html.fromstring(resp.content.decode())
    dom = html.fromstring(resp.text)
    dom = html.fromstring(resp.text)  # type: ignore
    dom = html.fromstring(resp.text.replace('<?xml version="1.0" encoding="UTF-8"?>', ''))
    dom = html.fromstring(response_index.text)
    dom = lxml.html.fromstring(resp.text)
    dom = lxml.html.fromstring(resp.text)  # type: ignore
    domain = parsed_url.netloc
    domain = region2domain.get(region)
    domain = resp.search_params['domain']
    DuckDuckGo's language "Browsers preferred language" (``wt_WT``) makes no
    efetch_url = f"{eutils_api}/efetch.fcgi?{args}"
    efetch_xml = etree.XML(resp.content)
    element: etree.Element | None = item.find(
    ElementType,
    elif " " in query:
    elif "," in title:
    elif "infobox" in display_type:
    elif ceid_region.lower() != ceid_lang:
    elif len(title) > 50:
    elif netloc == 'wiki.archlinuxcn.org':
    elif page > 2:
    elif params['pageno'] >= 2:
    elif params['search_urls']['http']:
    elif query_type == 'enum' and len(query_enum) > 0:
    elif qwant_categ == 'images':
    elif re.match(r"^[0-9]+ days? ago \.\.\. ", content):
    elif result['type'] in address_raw:
    elif search_type == 'images':
    elif search_type == 'news':
    elif search_type == 'videos':
    else:
    else:  # web, news, videos
    enable_http: true
    enclosure: etree.Element | None = item.find('enclosure')
    enclosure_url: str | None = None
    enclosure_url: str | None,
    end = min(endpositions)
    end = start + 9
    end = text.rindex("}}]")
    endings = ['/RS', '/RK']
    endpositions = []
    end_tag = '}};'
    engine : json_engine
    engine : xpath
    engine: adobe_stock
    engine: annas_archive
    engine: ansa
    engine: brave
    engine: braveapi
    engine: bt4g
    engine: command
    engine: demo_offline
    engine: demo_online
    engine: elasticsearch
    engine: gitea
    engine: github_code
    engine: gitlab
    engine: google_scholar
    engine: huggingface
    engine: il_post
    engine: meilisearch
    engine: microsoft_learn
    engine: mongodb
    engine: mrs
    engine: openlibrary
    engine: piped
    engine: presearch
    engine: solr
    engine: sourcehut
    engine: sqlite
    engine: startpage
    engine: tubearchivist
    engine: valkey_server
    engine: yacy
    engine_language = traits.get_language(params['searxng_locale'], 'en')
    engine_language = traits.get_language(params['searxng_locale'], 'en')  # type: ignore
    engine_region = traits.get_region(params["searxng_locale"], "all")
    engine_region = traits.get_region(params['searxng_locale'], 'en-US')
    engine_region = traits.get_region(params['searxng_locale'], traits.all_locale)  # type: ignore
    engine_traits.all_locale = ""
    engine_traits.all_locale = 'wt-wt'
    engine_traits.all_locale = regions[0]
    engine_traits.custom["content"] = []
    engine_traits.custom["content"].sort()
    engine_traits.custom["ext"] = l
    engine_traits.custom["ext"] = []
    engine_traits.custom["ext"].sort()
    engine_traits.custom["sort"] = []
    engine_traits.custom["sort"].sort()
    engine_traits.custom["ui_lang"] = {}
    engine_traits.custom["year_from"] = l
    engine_traits.custom["year_to"] = l
    engine_traits.custom['ceid'] = {}
    engine_traits.custom['countrycodes'] = countrycodes
    engine_traits.custom['language_all'] = languages[0]
    engine_traits.custom['lang_region'] = {}
    engine_traits.custom['region_all'] = regions[1]
    engine_traits.custom['supported_domains'] = {}
    engine_traits.custom['title'] = {}
    engine_traits.custom['WIKIPEDIA_LANGUAGES'] = []
    engine_traits.custom['WIKIPEDIA_LANGUAGES'].sort()
    engine_traits.custom['wiki_netloc'] = {}
    engine_traits.languages['en'] = 'English'
    engine_traits.languages['zh'] = 'lang_zh-CN'
    engine_traits.languages['zh'] = 'zh-hans'
    engine_traits.languages['zh_Hans'] = 'zh'
    engine_traits.languages['zh_Hant'] = 'zh'
    engine_traits.regions['zh-CN'] = 'en-hk'
    engine_traits.regions['zh-CN'] = 'HK'
    engine_traits.regions['zh-CN'] = 'zh-cn'
    engine`, :ref:`google images engine`, :ref:`google news engine` and
    eng_lang = eng_traits.get_language(sxng_locale, 'lang_en')
    eng_lang = get_ddg_lang(traits, params['searxng_locale'])
    eng_lang = traits.get_language(params['searxng_locale'], 'en')
    eng_lang = traits.get_language(params['searxng_locale'], None)
    eng_lang: str = get_ddg_lang(traits, params["searxng_locale"]) or "wt-wt"
    eng_region = traits.get_region(params['searxng_locale'], traits.all_locale)
    eng_region: str = traits.get_region(
    eng_region: str = traits.get_region(params['searxng_locale'], 'en_US')  # type: ignore
    eng_tag = eng_traits.get_region(sxng_locale, eng_traits.get_language(sxng_locale, 'en'))
    eng_tag, _wiki_netloc = get_wiki_params(params['searxng_locale'], traits)
    eng_tag_list = set()
    eng_traits: EngineTraits,
    esearch_resp: "SXNG_Response" = get(esearch_url, timeout=3)
    esearch_url = f"{eutils_api}/esearch.fcgi?{args}"
    eval_xpath,
    eval_xpath_getindex,
    eval_xpath_list,
    exact_match_only: false
    example these are the english languages in DuckDuckGo:
    Example:
    Example; when user selects a time range and we find ourselves in the year
    examples = [f"{m['segment']} : {m['translation']}" for m in data['matches'] if m['translation'] != text]
    except (json.JSONDecodeError, AttributeError, TypeError, ValueError):
    except (SearxException, httpx.HTTPError) as exc:
    except (ValueError, TypeError):
    except (ValueError, TypeError, IndexError):
    except babel.core.UnknownLocaleError:
    except Exception as e:
    except IndexError as exc:
    except IndexError:
    except KeyError:
    except parser.ParserError:
    except psycopg2.ProgrammingError:
    except ValueError:
    except:
    except:  # pylint: disable=bare-except
    explicit = "Yes"
    extr,
    extract_text,
    extract_url,
    extratags = result['extratags']
    extra_xpath = results_xpath_filter_recommended if resp.search_params['pageno'] > 1 else ''
    fargs = {
    Fetch languages from Odysee's source code.
    fetch_wikidata(nominatim_json, user_language)
    fetch_wikimedia_traits(engine_traits)
    fetch_wikimedia_traits,
    Fields of the *thumbnail data* (``result.articles.[<int>].thumbnail``):
    filesize = get_attribute(item, 'size')
    filesize = None
    filetype = SEARCH_TYPES[wc_search_type]
    file_quality = _text(item, './/span[contains(@class, "book-rating-quality-score")]')
    filtered_args = dict((k, v) for k, v in args.items() if v)
    filtered_results = filter(is_result_match, resp.json())
    filtered_results = filter(is_result_match, resp.json().items())
    filters: list[str] = []
    filters: []
    filter_mapping,
    first_page = biblio.get("first_page")
    first_result = True
    first_result_index = page_size * (resp.search_params.get('pageno', 1) - 1)
    for a in eval_xpath_list(dom, "//a[@class='interlanguage-link-target']"):
    for app in dom.xpath('//a[@class="package-header"]'):
    for attribute in attributes:
    for attribute in get_attributes('en'):
    for auth in item.get("authorships", []):
    for c in command:
    for c in item.get("concepts", []):
    for ceid in ceid_list:
    for cfg_name in ["base_url", "mount_prefix", "dl_prefix"]:
    for channel_result in json_data['results']['channel_results']:
    for code in languages[1:]:
    for code in regions[2:]:
    for content_type in ADOBE_VALID_TYPES:
    for correction in eval_xpath(dom, "//div[@class='gs_r gs_pda']/a"):
    for correction in eval_xpath_list(dom, correction_xpath):
    for country, v in q_locales.items():
    for current in docs:
    for day in json_data["weather"]:
    for ddg_result in _result_list:
    for definition in info['definitions']:
    for defn_raw in page['senses']:
    for div_result in eval_xpath(doc, '//div[@id="links"]/div[contains(@class, "web-result")]'):
    for doc in json_data["response"]["docs"]:
    for e in eval_xpath_list(dom, '//input[@type="hidden"]'):
    for ending in endings:
    for eng_lang, name in languages.items():
    for eng_tag in eng_tag_list:
    for eng_tag in sp_region_names:
    for eng_tag, name in regions.items():
    for eng_tag, sxng_tag_list in wikipedia_script_variants.items():
    for eng_tag, sxng_tag_list in wiki_lc_locale_variants.items():
    for entry in content["results"]:
    for entry in data:
    for entry in data["data"]["arrRes"]:
    for entry in data["data"]["data"]:
    for entry in data["data"]["documents"]["data"]:
    for entry in data["data"]["list"]:
    for entry in data["data"]["result"]:
    for entry in data["data"]["templates"]:
    for entry in data["feed"]["entry"]:
    for entry in eval_xpath_list(dom, '/html/body/main/div/div/div/form/div/ul/li/a[@class="package-snippet"]'):
    for entry in eval_xpath_list(dom, xpath_entry):
    for entry in search_results.xpath('./channel/item'):
    for entry in search_results.xpath('./result/doc'):
    for entry in suggestions.get('d', []):
    for ext in eval_xpath_list(dom, "//div[@id='advSearch-noJS']//select[@id='sf_extensions']/option"):
    for fmt in ("%Y-%m-%d", "%Y-%m", "%Y"):
    for forecast in json_data['forecastHourly']['hours']:
    for hit in data["hits"]:
    for href in eval_xpath(dom, '//div[@id="language-section-content"]//div[@class="languageItem"]/a/@href'):
    for href in eval_xpath(dom, '//div[@id="region-section-content"]//div[@class="regionItem"]/a/@href'):
    for i in ('answers', 'infoboxes'):
    for i in range(0, len(rows), 2):
    for i, match in enumerate(code_matches):
    for icon_name, tags in filtered_results:
    for img_id, data_image in RE_DATA_IMAGE.findall(text):
    for index, time in enumerate(json_data["hourly"]["time"]):
    for index, value in items:
    for ip_tuple in ips:
    for item in channel.iterfind('item'):
    for item in data.get("results", []):
    for item in data.get('feed', []):
    for item in data.get('hit3', []):
    for item in data.get('image', []):
    for item in data.get('list', []):
    for item in data:
    for item in data["body"]["illust"]["data"]:
    for item in dom.xpath('//div[@id="searchResultBox"]//div[contains(@class, "resItemBox")]'):
    for item in dom.xpath('//div[contains(@class, "vrwrap")]'):
    for item in dom.xpath('//li[contains(@class, "res-list")]'):
    for item in dom.xpath('//li[contains(@id, "sogou_vr_")]'):
    for item in eval_xpath_list(
    for item in eval_xpath_list(doc, "(//div[@class='event-list'])[1]/div[contains(@class, 'event')]"):
    for item in eval_xpath_list(dom, "//li[contains(@class, 'video_item')]"):
    for item in eval_xpath_list(dom, "//main//div[contains(@class, 'js-aarecord-list-outer')]/div"):
    for item in eval_xpath_list(dom, "//ul[contains(@class, 'lst_total')]/li[contains(@class, 'bx')]"):
    for item in eval_xpath_list(dom, '//section/article'):
    for item in eval_xpath_list(dom, results_xpath):
    for item in items:
    for item in json['searchResults']:
    for item in json_data.get("docs", []):
    for item in json_data["ischj"].get("metadata", []):
    for item in json_data['topics']:
    for item in json_resp.get("results", []):
    for item in json_resp["items"].values():
    for item in json_resp['search']['webResults']['results']:
    for item in json_results.get('specialSections', {}).get('topStoriesCompact', {}).get('data', []):
    for item in json_results.get('standardResults', []):
    for item in mwv_result_list:
    for item in pages:
    for item in resp.json().get('data', []):
    for item in resp.json().get('items', []):
    for item in resp.json():
    for item in resp.json()['events']:
    for item in resp.json()['list']:  # type: ignore
    for item in resp_json.get("results", []):
    for item in search_res.get("data", {}).get("result", []):
    for item in search_res.get('videos', []):
    for item in search_res:
    for item in search_results.get('items', []):
    for item in search_res["data"]:
    for item in search_res["packages"]:
    for item in search_res["results"]:
    for item in search_res['response'][0]['grid']['vulnerabilities']:
    for k in key_name.split(':') + ['*']:
    for k, mapping_function in VALUE_TO_LINK.items():
    for k, v in country_tags.items():
    for k, v in pua_chars.items():
    for k, v in result['extratags'].items():
    for k, v in WIKIDATA_UNITS.items():
    for key in _valkey_client.scan_iter(match='*{}*'.format(query)):
    for key, val in shortcut_dict.items():
    for key, value in cookie_params.items():
    for key, value in iterate(data):
    for lang in language_list:
    for lang in re.finditer(r"\{ id: '([a-z]+)', label:", js_lang.group(1)):
    for lang_code in filter(lambda lang_code: lang_code.find('_') == -1, babel.localedata.locale_identifiers()):
    for line in resp.text.split("\n"):
    for line in resp.text.split("\n")[1:-4]:
    for link in eval_xpath_list(dom, '//a'):
    for locale in babel.core.localedata.locale_identifiers():
    for match in matches:
    for match_json in json_data['matches']:
    For more details see :py:obj:`searx.enginelib.Engine.init`.
    For more details see :py:obj:`searx.enginelib.Engine.init`."""
    For more details see :py:obj:`searx.enginelib.Engine.setup`.
    For more details see :py:obj:`searx.enginelib.Engine.setup`."""
    for newsitem in eval_xpath_list(dom, '//div[contains(@class, "newsitem")]'):
    for opt in ("azure_tenant_id", "azure_client_id", "azure_client_secret"):
    for option in dom.xpath("//section//option[@value='en-us']/../option"):
    for option in dom.xpath('//form[@name="settings"]//select[@name="language"]/option'):
    for option in dom.xpath('//form[@name="settings"]//select[@name="search_results_region"]/option'):
    for p in [
    for package in resp.json():
    for package in resp.json()["crates"]:
    for page in search_results.get('data', []):
    for part in query.split('/'):
    for photo in photos:
    for pkgname, repositories in resp_json.items():
    for pkg_url, pkg_list in packages.items():
    for pod in pods:
    for pod in resp_json['queryresult']['pods']:
    for pos, engdef, extra in definitions:
    for post in json_data['posts']:
    for post in posts:
    for prefix in WIKIDATA_PREFIX:
    for pubmed_article in eval_xpath_list(efetch_xml, "//PubmedArticle"):
    for query_key, query_value in custom_query.items():
    for query_part in query_parts:
    for r in eval_xpath(doc, '//div[@class="search_quickresult"]/ul/li'):
    for r in eval_xpath(doc, '//dl[@class="search_results"]/*'):
    for r in json.get('features', {}):
    for record in json_data["message"]["items"]:
    for record in json_data["records"]:
    for region in country_list:
    for region in regions[1:]:
    for res in search_res.get('list', []):
    for result in data.get("collection", []):
    for result in data.get("web", {}).get("results", []):
    for result in data['filtered']['data']:
    for result in dom.xpath('//div[@class="dg_u"]//div[contains(@id, "mc_vtvc_video")]'):
    for result in dom.xpath('//ul[contains(@class, "dgControl_list")]/li'):
    for result in dom.xpath(xpath_results):
    for result in eval_xpath(dom, ".//table[@id='r']//tr"):
    for result in eval_xpath(dom, '//li[contains(@class, "search-result")]'):
    for result in eval_xpath_list(doc, "//article[starts-with(@id, 'post')]"):
    for result in eval_xpath_list(doc, "//div[@class='article']"):
    for result in eval_xpath_list(doc, "//table/tbody/tr"):
    for result in eval_xpath_list(doc, '//script[@type="text/javascript"]'):
    for result in eval_xpath_list(doc, results_xpath):
    for result in eval_xpath_list(dom, "//article[contains(@class, 'listview__item')]"):
    for result in eval_xpath_list(dom, "//div[@data-rp]"):
    for result in eval_xpath_list(dom, "//div[@id='content']//div[@class='listWidget']/div/div[@class='appRow']"):
    for result in eval_xpath_list(dom, "//div[contains(@class, 'gallery')]/div[contains(@class, 'artwork')]"):
    for result in eval_xpath_list(dom, "//div[contains(@class, 'results')]//div[@data-type='news']"):
    for result in eval_xpath_list(dom, "//div[contains(@class, 'snippet ')]"):
    for result in eval_xpath_list(dom, './/div[contains(@class, "MjjYud")]'):
    for result in eval_xpath_list(dom, '//div[@class="xrnccd"]'):
    for result in eval_xpath_list(dom, '//div[contains(@class,"algo-sr")]'):
    for result in eval_xpath_list(dom, '//div[starts-with(@class, "EmojisList")]/a'):
    for result in eval_xpath_list(dom, '//li[contains(@class, "searchresult")]'):
    for result in eval_xpath_list(dom, '//ol[@id="b_results"]/li[contains(@class, "b_algo")]'):
    for result in eval_xpath_list(dom, '//ol[contains(@class,"searchCenterMiddle")]//li'):
    for result in eval_xpath_list(dom, '//section[not(contains(@class, "essay"))]'):
    for result in eval_xpath_list(dom, '//table[contains(@class, "table-list")]/tbody//tr'):
    for result in eval_xpath_list(dom, '//ul[@class="mw-search-results"]/li'):
    for result in eval_xpath_list(dom, image_results_xpath):
    for result in eval_xpath_list(dom, news_results_xpath):
    for result in eval_xpath_list(dom, results_xpath):
    for result in eval_xpath_list(dom, res_xpath):
    for result in filtered_results:
    for result in json.get('result', []):
    for result in json:
    for result in jsonresponse.get('results', {}).get('bindings', []):
    for result in jsonresponse.get('results', {}).get('bindings', {}):
    for result in json["comments"]:
    for result in json["communities"]:
    for result in json["items"]:
    for result in json["posts"]:
    for result in json["users"]:
    for result in json['data']:
    for result in json['feeds']:
    for result in json['results']:
    for result in json[mastodon_type]:
    for result in json_data.get("results", []):
    for result in json_data.get('data', []):
    for result in json_data.get('page', {}).get('results', []):
    for result in json_data["data"]:
    for result in json_data["docs"]:
    for result in json_data["responses"]:
    for result in json_data["results"]:
    for result in json_data['data']:
    for result in json_data['items']:
    for result in json_data['results']:
    for result in json_data['results'][0]['hits']:
    for result in json_resp:
    for result in json_resp["results"]:
    for result in json_resp['resource_response']['data']['results']:
    for result in json_resp['teaser']:
    for result in json_results:
    for result in json_results["icons"]:
    for result in json_results['posts']:
    for result in json_result['results']:
    for result in loads(json_str):
    for result in more:
    for result in nominatim_json:
    for result in resp.json().get("results", []):
    for result in resp.json():
    for result in resp.json()["webpages"]:
    for result in resp.json()['data']:
    for result in resp.json()[1]['results']:
    for result in response_data:
    for result in resp_json["hits"]["hits"]:
    for result in resp_json["response"]["docs"]:
    for result in resp_json["result"].get("articles", []):
    for result in resp_json['results']:
    for result in result_divs:
    for result in res_json['results']:
    for result in rs:
    for result in search_res.get("results", []):
    for result in search_res.get('data', []):
    for result in search_res.get('tracks', {}).get('items', []):
    for result in search_res.get('tracks', {}).get('items', {}):
    for result in search_res:
    for result in search_results:
    for result in search_results['items']:
    for result in search_results['query']['search']:
    for result in search_results[0].get('items', []):
    for result in search_res['results']:
    for result in trimmed_results:
    for results_categ in results_obj.get('mainline', []):
    for result_dom in results_dom:
    for result_element in eval_xpath_list(
    for row in data_rows:
    for row in dom.xpath('//table[contains(@class,"sortable")]//tbody/tr'):
    for row in mainline:
    for row in query:
    for row in resp_json['hits']:
    for script_src in eval_xpath_list(doc, "//script/@src"):
    for section in (
    for section in data.get('list_container', []):
    for section in eval_xpath(dom, '//c-wiz/section/header/..'):
    for section in resp.json()['response']['sections']:
    for section in sections:
    for src in dom.xpath('//*[@id="define"]//h3[@class="source"]'):
    for suggestion in eval_xpath(dom, "//div[contains(@class, 'gs_qsuggest_wrap')]//li//a"):
    for suggestion in eval_xpath(dom, suggestion_xpath):
    for suggestion in eval_xpath_list(dom, "//a[contains(@class, 'related-query')]"):
    for suggestion in eval_xpath_list(dom, '//c-wiz[@jsrenderer="qyd4Kb"]//div[@class="ULeU3b neq64b"]'):
    for suggestion in eval_xpath_list(dom, '//div[contains(@class, "AlsoTry")]//table//a'):
    for suggestion in eval_xpath_list(dom, suggestion_xpath):
    for suggestion in query(json, suggestion_query):
    for tag in search_res[1:]:
    for token, positions in abstract_inverted_index.items():
    for translation in info["extraTranslations"]:
    for url in app_js_urls[::-1]:
    for val in re.split(r"(\s+)", query):
    for video_result in json_data['results']['video_results']:
    for x in eval_xpath_list(dom, "//div[@id='advSearch-noJS']//select[@id='sf_languages']/option"):
    for x in eval_xpath_list(dom, "//form//input[@name='content']"):
    for x in eval_xpath_list(dom, "//form//input[@name='ext']"):
    for x in eval_xpath_list(dom, "//form//input[@name='lang']"):
    for x in eval_xpath_list(dom, "//form//select[@name='sort']//option"):
    for x in eval_xpath_list(dom, "//select[@name='gl']/option"):
    for x in eval_xpath_list(dom, "//select[@name='hl']/option"):
    for x in ["./i", "./span", "./button"]:
    for x in ['wikipedia', 'google']:
    for x, index in enumerate(legend):
    for year in eval_xpath_list(dom, "//div[@id='advSearch-noJS']//select[@id='sf_yearFrom']/option"):
    for year in eval_xpath_list(dom, "//div[@id='advSearch-noJS']//select[@id='sf_yearTo']/option"):
    form = eval_xpath(doc, '//input[@name="vqd"]/..')
    format: str
    form_data = {
    fp = {  # pylint: disable=invalid-name
    from babel import Locale, languages
    from babel import Locale, UnknownLocaleError
    from babel.core import get_global
    from extended_types import SXNG_Response
    from pymongo import MongoClient  # type: ignore
    from search.processors.online import OnlineParams
    from searx import network
    from searx.engines import engines  # pylint: disable=import-outside-toplevel
    from searx.engines.bing import fetch_traits as _f
    from searx.extended_types import SXNG_Response
    from searx.locales import get_official_locales, region_tag
    from searx.locales import language_tag
    from searx.locales import region_tag
    from searx.locales import region_tag, language_tag
    from searx.network import get  # see https://github.com/searxng/searxng/issues/762
    from searx.search.processors import OnlineCurrenciesParams
    from searx.search.processors import OnlineParams
    from searx.search.processors import RequestParams
    from searx.utils import extr
    from searx.utils import gen_useragent
    from searx.utils import js_obj_str_to_python
    from your local engine.
    from_lang = params["from_lang"][2]  # "english"
    from_lang = resp.search_params["from_lang"][1]  # "en"
    from_to_prefix = "%s-%s " % (resp.search_params['from_lang'][1], resp.search_params['to_lang'][1])
    frontend_url: https://..
    function scrapes a new sc-code from Startpage's home page every
    function.
    gen_useragent,
    geojson = result.get('geojson')
    geoloc = weather.GeoLocation.by_query(resp.search_params["query"])
    get region and language of a DDG service use:
    get_earth_coordinates_url,
    get_embeded_stream_url,
    get_external_url,
    get_google_info,
    get_sc_url = base_url + "/"
    get_wiki_params,
    ghc_auth:
    ghc_highlight_matching_lines: true
    ghc_strip_new_lines: true
    ghc_strip_whitespace: true
    Github additionally sends context for _word_ highlights; pygments supports
    global CACHE  # pylint: disable=global-statement
    global command, working_dir, delimiter, parse_regex, environment_variables  # pylint: disable=global-statement
    global instance_index  # pylint: disable=global-statement
    global _arcid_random  # pylint: disable=global-statement
    global _CACHE
    global _client  # pylint: disable=global-statement
    global _connection  # pylint: disable=global-statement
    global _my_offline_engine, CACHE  # pylint: disable=global-statement
    global _my_online_engine  # pylint: disable=global-statement
    global _search_url
    global _valkey_client  # pylint: disable=global-statement
    global __CACHED_API_URL  # pylint:disable=global-statement
    Google Scholar supports a detailed search by year.  Searching by *last
    google_info = get_google_info(params, traits)
    google_info["subdomain"] = google_info["subdomain"].replace("www.", "scholar.")
    google_info['params']['gl'] = ceid_region
    google_info['params']['hl'] = ceid_lang
    google_info['params']['lr'] = 'lang_' + ceid_lang.split('-')[0]
    google_info['subdomain'] = 'news.google.com'  # google news has only one domain
    group_by = list(filter(lambda s: len(s) > 0, [a.get_group_by() for a in attributes]))
    guest_client_id = CACHE.get("guest_client_id")
    guid = get_attribute(item, 'guid')
    guid: str | None,
    header).
    headers = params["headers"]
    headers = resp.headers
    headers = {
    headers = {**params['headers']}
    headers["Accept"] = "*/*"
    headers["Host"] = "duckduckgo.com"
    headers["Referer"] = "https://duckduckgo.com/"
    headers["Referer"] = "https://html.duckduckgo.com/"
    headers["User-Agent"] = _HTTP_User_Agent
    heading: str = search_res.get("Heading", "")
    height of, for example, 80 points, only a few KB remain!
    height: default is *unset* (``0``)
    highlighted_lines_index: set[int] = set()
    highlighting lines, as such we calculate which lines to highlight while
    highlights = hit['highlights']
    hint:Prior hint:rangeSafe true;""".replace(
    host: '127.0.0.1'
    host_venue: dict[str, str] = item.get("host_venue", {})
    HTML POST data / compare ``<input>`` elements: :py:obj:`search_form_xpath`.
    html_to_text,
    html_url: str = landing_page_url
    http = 'http://'
    https://dev.springernature.com/docs/advanced-querying/pagination-limits/
    https://docs.github.com/en/rest/search/search?apiVersion=2022-11-28#constructing-a-search-query
    https://docs.github.com/en/rest/search/search?apiVersion=2022-11-28#search-code
    https://docs.github.com/en/rest/search/search?apiVersion=2022-11-28#search-code--fine-grained-access-tokens
    https://docs.github.com/en/search-github/getting-started-with-searching-on-github/understanding-the-search-syntax
    https://example.org/?search={query}&page={pageno}{time_range}{safe_search}
    https://learn.microsoft.com/en-us/entra/identity-platform/quickstart-register-app
    https://{base_url}/w/api.php?action=query&list=search&format=json
    http_response.raise_for_status()
    HTTP_WIKIMEDIA_IMAGE = 'http://commons.wikimedia.org/wiki/Special:FilePath/'
    huggingface_endpoint: datasets
    huggingface_endpoint: spaces
    if "." in title.strip()[:-1]:
    if "Abstract" in search_res:
    if "backlinks" in match_json:
    if "data" in data:
    if "data" not in data or "result" not in data["data"]:
    if "data" not in data or "templates" not in data["data"]:
    if "Definition" in search_res:
    if "error" in json_data:
    if "error" in resp_json:
    if "infobox" in display_type:
    if "Infobox" in search_res:
    if "list" in display_type or api_result.get('type') != 'standard':
    if "searchList" in data and "searchList" in data["searchList"]:
    if "typo" in info:
    if "_vapp.mxf" in title:
    if 'command' not in engine_settings:
    if 'countrycode' in station_filters:
    if 'data' not in json_data:
    if 'data' not in search_results:
    if 'definitions' in info:  # pylint: disable=too-many-nested-blocks
    if 'delimiter' in engine_settings and 'parse_regex' in engine_settings:
    if 'delimiter' in engine_settings:
    if 'delimiter' not in engine_settings and 'parse_regex' not in engine_settings:
    if 'environment_variables' in engine_settings:
    if 'error' in json:
    if 'error' in resp_json:
    if 'error' in search_res:
    if 'error' in search_results and 'message' in search_results['error']:
    if 'items' not in search_results:
    if 'language' in station_filters:
    if 'legend' not in model_export:
    if 'osm_id' not in result:
    if 'pageno' in params:
    if 'parse_regex' in engine_settings:
    if 'photo' not in search_results['photos']:
    if 'photos' not in search_results:
    if 'query_str' not in engine_settings:
    if 'query_type' in engine_settings and engine_settings['query_type'] not in _available_query_types:
    if 'result' in json:
    if 'results' in json_data:
    if 'results' not in json_data:
    if 'runs' in element:
    if 'safesearch' in params:
    if 'tags' in json_results:
    if 'wappass.baidu.com/static/captcha' in resp.headers.get('Location', ''):
    if 'wikidata' in result:
    if 'working_dir' in engine_settings:
    if (
    if ('topics' or 'posts') not in json_data.keys():
    if aa_content and aa_content not in traits.custom["content"]:
    if aa_ext and aa_ext not in traits.custom["ext"]:
    if aa_sort and aa_sort not in traits.custom["sort"]:
    if abstractURL != "":
    if address_name:
    if add_domains:
    if adobe_order not in ["relevance", "featured", "creation", "nb_downloads"]:
    if ads_sort:
    if answer:
    if api_key != '':
    if api_key:
    if api_path:
    if api_url is None:
    if api_username != '':
    if arch_path:
    if audio_data.get("album"):
    if auth_key != '':
    if autotranslate.ok and autotranslate.text:
    if baidu_category == 'images':
    if baidu_category not in ('general', 'images', 'it'):
    if book_rating and float(book_rating):
    if brave_category == "goggles":
    if brave_category == 'images':
    if brave_category == 'videos':
    if brave_category in ("search", "goggles"):
    if brave_category in ('news'):
    if brave_category in ('search', 'goggles'):
    if brave_spellcheck:
    if btf:
    if ceid_suffix and ceid_suffix not in ['Hans', 'Hant']:
    if chinaso_category == 'news' and chinaso_news_source not in t.get_args(ChinasoNewsSourceType):
    if chinaso_category not in ('news', 'videos', 'images'):
    if chinaso_news_source != 'all':
    if client_id:
    if collection == '':
    if comments and comments.startswith('http'):
    if cookie:
    if default_fields != '':
    if definitionURL != "":
    if delimiter:
    if dom is None:
    if domain == "search.yahoo.com":
    if domain_is_seized(dom):
    if element is not None:
    if enclosure is not None:
    if enclosure_url and enclosure_url.startswith('http'):
    if enclosure_url and enclosure_url.startswith('magnet'):
    if engine_language:
    if engine_region:
    if engine_settings.get("wc_search_type") not in SEARCH_TYPES:
    if engine_settings["base_url"].endswith("/"):
    if engine_settings["ddg_category"] not in ["images", "videos", "news"]:
    if eng_lang is not None:
    if eng_region == "wt-wt":
    if eval_xpath(dom, "//form[@id='gs_captcha_f']"):
    if eval_xpath(dom, '//div[@class="v6DsQb"]'):
    if exact_match_only:
    if field_list != '':
    if file_quality and float(file_quality):
    if filters:
    if first_page and last_page:
    if first_page:
    if ghc_auth['type'] == "bearer":
    if ghc_auth['type'] == "none":
    if ghc_auth['type'] == "personal_access_token":
    if guest_client_id is None:
    if guid and guid.startswith('http'):
    if guid and guid.startswith('magnet'):
    if highlights:
    if http_digest_auth_user and http_digest_auth_pass:
    if http_response.status_code != 200:
    if huggingface_endpoint not in ('datasets', 'models', 'spaces'):
    if image is not None and urlparse(image).netloc == "":
    if index == '':
    if info:
    if infobox_id:
    if int(height) > 0:
    if int(width) > 0:
    if isinstance(adobe_content_types, list):
    if isinstance(base_url, list):
    if isinstance(cited_by_count, int):
    if isinstance(iterable, dict):
    if isinstance(json_resp["items"], list):
    if isinstance(language, str) and language != "all":
    if isinstance(mailto, str) and mailto != "":
    if isinstance(obj, str):
    if isinstance(url, list):
    if is_alibaba_captcha(text):
    if is_ddg_captcha(doc):
    if item.get('category'):
    if item.get('countries'):
    if item.get('directors'):
    if item.get('duration'):
    if item.get('genresInfos'):
    if item.get('medias', {}) and item['medias'].get('picture'):
    if item.get('rating') and item.get('stats', {}).get('ratingCount'):
    if item_list:
    if key and key not in ("unset", "unknown", "..."):
    if key_name.startswith('currency:'):
    if key_rank is None:
    if kvmap:
    if l:
    if lang in yandex_supported_langs:
    if lang is not None:
    if language:
    if last:
    if last_page:
    if leechers:
    if lemmy_type == "Comments":
    if lemmy_type == "Communities":
    if lemmy_type == "Posts":
    if lemmy_type == "Users":
    if len(a) >= 1:
    if len(alt_forms) > 1:
    if len(api_key) > 0:
    if len(base_url) < 1:
    if len(counts) == 0:
    if len(facet_filters) > 0:
    if len(form):
    if len(heading) > 0:
    if len(journal_year) > 1:
    if len(params['searxng_locale'].split('-')) > 1:
    if len(query) <= 2:
    if len(query) >= 500:
    if len(rows) == 0 or len(rows) % 2 != 0:
    if len(search_results) == 0:
    if len(sxng_locale.split('-')) > 1:
    if len(s_text) != 3:
    if len(torznab_categories) > 0:
    if length.tm_hour:
    if link and link.startswith('http'):
    if link and link.startswith('magnet'):
    if magneturl and magneturl.startswith('magnet'):
    if match:
    if matches is None:
    if max_index < 0:
    if method == 'GET':
    if missing:
    if missing_opts:
    if name is None:
    if naver_category in naver_category_dict:
    if naver_category not in ('general', 'images', 'news', 'videos'):
    if netloc == main_wiki:
    if nextpage_url:
    if not abstract_inverted_index:
    if not adobe_content_types:
    if not adobe_order:
    if not api_key:
    if not base_url:
    if not categories:
    if not cmd:
    if not data and result['translation']:
    if not data.get("data", {}).get("arrRes"):
    if not data.get("data", {}).get("data"):
    if not data.get("data", {}).get("documents", {}).get("data"):
    if not data.get("data", {}).get("list"):
    if not data.get("feed", {}).get("entry"):
    if not data.get('translations'):
    if not date:
    if not doi_value:
    if not domain:
    if not engine_settings['query_str'].lower().startswith('select '):
    if not exact_match_only:
    if not extratags:
    if not filesize and enclosure:
    if not geojson and osm_type == 'node':
    if not headers.get("Accept-Language"):
    if not img_src and extratags.get('image'):
    if not img_src and extratags.get('wikimedia_commons'):
    if not img_src is None and _IMG_SRC_DEFAULT_URL_PREFIX in img_src.split()[0]:
    if not items:
    if not json_data:
    if not json_results:
    if not js_lang:
    if not l:
    if not language.startswith('en'):
    if not legend or not legend[0]:
    if not mainline:
    if not match:
    if not matches:
    if not offset:
    if not params['engine_data'].get('next_page_token'):
    if not published_date_raw:
    if not q:
    if not query:
    if not query_type:
    if not raw_search_results:
    if not raw_value:
    if not resp.ok or not resp.text:
    if not resp.ok:
    if not resp.ok:  # type: ignore
    if not resp.text:
    if not resp_json.get("result"):
    if not resp_json['queryresult']['success']:
    if not results_dom:
    if not result_chunks:
    if not ret_val:
    if not search_res:
    if not search_results.get('query', {}).get('search'):
    if not secret_key:
    if not servers:
    if not suggestion_query:
    if not ta_token:
    if not text:
    if not title:
    if not token_str:
    if not url:
    if not value:
    if not _api_key:
    if not _arcid_random or (int(time.time()) - _arcid_random[1]) > 3600:
    if no_result_for_http_status and resp.status_code in no_result_for_http_status:
    if number_of_results:
    if number_of_results_element is not None:
    if offset == 0:
    if original_title and original_title != title:
    if page == 2:
    if page > 1:
    if params.get("time_range") in time_range_dict:
    if params.get('time_range'):
    if params["language"] != "all":
    if params["pageno"] == 1:
    if params["pageno"] > 1:
    if params["safesearch"]:
    if params["searxng_locale"] != "all":
    if params["searxng_locale"] == "all":
    if params["time_range"] in time_range_dict:
    if params["time_range"]:
    if params['discovery']:
    if params['language'] != 'all':
    if params['language'] == 'all':
    if params['pageno'] == 1:
    if params['pageno'] > 1 and nextpage_url is not None:
    if params['pageno'] > 1:
    if params['safesearch'] > 0:
    if params['safesearch']:
    if params['search_urls']['data:image']:
    if params['searxng_locale'] != 'all':
    if params['time_range'] and search_type != 'images':
    if params['time_range'] in time_range_dict:
    if params['time_range'] in time_range_table:
    if params['time_range']:
    if parse_regex:
    if password:
    if play_categ == 'apps':
    if play_categ == 'movies':
    if play_categ not in ('movies', 'apps'):
    if property_element is not None:
    if pubDate is not None:
    if public_token and private_token:
    if published_date_raw is None:
    if quark_category == 'general':
    if quark_category == 'images':
    if quark_category not in ('general', 'images'):
    if query.islower():
    if query_arch:
    if query_fields != '':
    if query_type == 'path':
    if query_type not in _available_query_types:
    if qwant_categ == 'web':
    if qwant_categ == 'web-lite':
    if re.match(r"^([1-9]|[1-2][0-9]|3[0-1]) [A-Z][a-z]{2} [0-9]{4} \.\.\. ", content):
    if release_time:
    if remove_ai_images is True:
    if request_body:
    if request_url.startswith("https://libretranslate.com") and not api_key:
    if resp.headers.get('Location', '').startswith("https://www.startpage.com/sp/captcha"):
    if resp.headers.get('x-yandex-captcha') == 'captcha':
    if resp.search_params.get('engine_data'):
    if resp.search_params['discovery']:
    if resp.status_code != 200:
    if resp.status_code == 200:
    if resp.status_code == 302:
    if resp.status_code == 303:
    if resp.status_code == 400:
    if resp.status_code == 403:
    if resp.status_code == 404:
    if resp.status_code == 422:
    if resp.status_code in (301, 302, 303, 307, 308) and "Location" in resp.headers:
    if resp.status_code in (400, 422):
    if resp.text.strip() == "ddg_spice_forecast();":
    if resp.url.host == 'sorry.google.com' or resp.url.path.startswith('/sorry'):
    if resp.url.path.startswith('/verify'):
    if result and not is_broken_text(result):
    if result.get('date'):
    if result.get('filesize'):
    if result.get('thumbnailUrl'):
    if result.get('width') and result.get('height'):
    if results:
    if results_query:
    if results_xpath:
    if safe_search is not None:
    if sc_code:
    if search_res is None:
    if search_res.get('total', 0) < 1:
    if search_results.get('status') != 'success':
    if search_results.tag == "error":
    if search_results.xpath(failure_xpath):
    if search_res[0]["name"] == "No results returned":
    if search_type == '' and params['pageno'] > 1:
    if search_type == '':
    if search_type == 'images':
    if search_type == 'news':
    if search_type == 'search':
    if search_type == 'web':
    if search_type not in ('', 'images', 'news'):
    if search_type not in valid_types:
    if search_type not in ['search', 'images', 'videos', 'news']:
    if search_type:
    if seeders and peers:
    if servers:
    if show_magnet_links:
    if show_torrent_files:
    if sort != '':
    if sort_order_path:
    if spot is not None:
    if srenablerewrites:
    if start == 0 or len(endpositions) == 0:
    if str(resp.url).startswith('https://www.startpage.com/sp/captcha'):
    if suggestion_xpath:
    if sxng_locale == 'all':
    if tag_label is None and lang != 'en':
    if tag_label is None and len(labels.values()) > 0:
    if tag_label is None:
    if text is None or text == "":
    if thumbnail and not thumbnail.startswith('/'):
    if time() - (token['last_updated'] or 0) > 1800:
    if timestamp:
    if time_delta:
    if time_range in time_range_dict:
    if time_range:
    if time_range_dict.get(params['time_range']) and quark_category == 'general':
    if time_range_dict.get(params['time_range']):
    if token is None:
    if token:
    if translation['target_transliteration'] and not re.match(
    if translation['word_choices']:
    if t_range:
    if unit:
    if url.endswith("/"):
    if username and password:
    if username:
    if value.startswith(http):
    if value:
    if wikidata_ids:
    if x:
    if year:
    if zero_click and (
    if zlib_ext and zlib_ext not in traits.custom["ext"]:
    if zlib_year_from and zlib_year_from not in traits.custom["year_from"]:
    if zlib_year_to and zlib_year_to not in traits.custom["year_to"]:
    if _api_key == "public":
    if _CACHE is None:  # pyright: ignore[reportUnnecessaryComparison]
    if __CACHED_API_URL:
    image = None if image == "" else image
    image = search_res.get("Image")
    images = eval_xpath_list(doc, '//a[starts-with(@href, "/doc")]//img')
    Images from commons.wikimedia.org are (HTTP) redirected to
    images.
    img_results = []
    img_src = None
    img_src_priority = 0
    import babel
    import babel.core
    import babel.languages
    import contextlib
    import httpx
    import mariadb  # pyright: ignore [reportMissingImports]
    import mysql.connector  # type: ignore
    import psycopg2  # type: ignore
    in OSM_KEYS_TAGS, labels have key == '*'
    In this demo engine we ignore the 'query' term, usual you would pass the
    inactive: false
    index: my-index
    info = json_results.get('infoSection', {}).get('data')
    info = result["info"]
    infobox_attributes = []
    infobox_content = attribute_result.get('itemDescription', [])
    infobox_content = []
    infobox_content.append(
    infobox_content.append('</ul>')
    infobox_id = attribute_result['item']
    infobox_id = None
    infobox_id_lang = None
    infobox_title = ""
    infobox_title = alt_forms[0]
    infobox_title = attribute_result.get('itemLabel')
    infobox_urls = []
    infobox_urls.append({'title': 'Wikidata', 'url': attribute_result['item']})
    init(None)
    instance_index += 1
    int_or_zero,
    ips = socket.getaddrinfo("all.api.radio-browser.info", 80, 0, 0, socket.IPPROTO_TCP)
    Is a list of allowed search terms.  If the user submits something which is
    is from a bot.  We do not know what is encoded in the value of the ``sc``
    is set an empty dictionary of arguments is returned.
    is set, otherwise the engine is inactive.
    is ``wt-wt`` (the region).
    is_onion = 'onions' in categories
    It might confuse, but the ``l`` value of the cookie is what SearXNG calls
    item = res.types.Translations.Item(text=translation['translated-text'])
    item = results.types.Translations.Item(text=text, examples=examples)
    item = results.types.Translations.Item(text=text, examples=json_resp.get('alternatives', []))
    item.synonyms = translation.get('source_synonyms', [])
    item: dict[str, t.Any],
    item: etree.Element
    items = response_data.get('data', {}).get('searchProductExplorer', {}).get('items', [])
    items {
    item_list = []
    Iterate over multiple possible matches, for each extract a code fragment.
    j = loads(r.text)
    journal: str = host_venue.get("display_name", "")
    journal_year = s_text[1].split(", ")
    json = loads(resp.text)
    json = loads(resp.text)['response']['docs']
    json = network.get(base_url + "/docs/openapi.json").json()
    json = resp.json()
    jsonresponse = loads(resp.content.decode())
    jsonresponse = send_wikidata_query(query, timeout=20)
    json_data = loads(resp.text)
    json_data = loads(resp.text[json_start:])
    json_data = loads(resp.text[resp.text.find('\n') + 1 : resp.text.rfind('\n') - 2])
    json_data = resp.json()
    json_data: dict[str, dict[str, t.Any]] = resp.json()
    json_data: dict[str, t.Any] = extract_json_data(resp.text)
    json_resp = resp.json()
    json_resp = resp.text[resp.text.find("\n") + 1 : resp.text.rfind("\n") - 2]
    json_resp = utils.js_obj_str_to_python(script)
    json_resp: dict[str, t.Any] = json_data['data'][1]["data"]['body']['response']
    json_result = loads(resp.text)
    json_results = json_data.get('results')
    json_results = loads(resp.text)['data']
    json_results = loads(resp.text[5:])
    json_results = resp.json()
    json_results = resp.json()['data']
    json_results = []
    json_start = resp.text.find('{"ischj":')
    json_str = ""
    json_str = js_obj_str_to_json_str(js_obj_str)
    json_string = extr(resp.text, 'INITIAL_PROPS = ', '</script>')
    js_code = extr(resp.text, 'languages:', ',regions')
    js_code = extr(resp.text, 'regions:', ',snippetLengths')
    js_lang = re.search(r"videoLanguages \(\)[^\n]+(.*?)\]", resp.text, re.DOTALL)  # type: ignore
    js_obj_str = "{" + js_obj_str + "}}]}"
    js_obj_str = text[start:end]
    js_obj_str_to_json_str,
    js_obj_str_to_python,
    key = cache.secret_hash(f"{query}//{params['headers']['User-Agent']}")
    key = f"azure_tenant_id: {t_id:}, azure_client_id: {c_id}, azure_client_secret: {c_secret}"
    key, value = query.split(':')
    key: 'name'            # key in the collection to search for
    key: str = engine_settings.get("api_key", "")
    key_rank = KEY_RANKS.get(k)
    known from :py:obj:`searx.locales.LOCALE_NAMES` or those with a minimal
    kvmap: dict[str, str] = _valkey_client.hgetall(query)
    kwargs: dict[str, str | int] = {'port': port}
    l = re.findall(r"from\s+(.*)\s+to\s+(.+)", resp.search_params["query"])
    l = []
    l: list[str]
    labels = OSM_KEYS_TAGS['keys']
    landing_page_url: str = primary_location.get("landing_page_url") or ""
    lang = lang_all
    lang = params["language"].split("-")[0]
    lang = traits.get_language(params["searxng_locale"], traits.all_locale)
    lang = traits.get_language(params['searxng_locale'], None)
    lang = yahoo_languages.get(lang, "any")
    lang, region = (params["language"].split("-") + [None])[:2]
    lang: str | None = eng_traits.get_language(sxng_locale, default)
    lang: str | None = traits.get_language(params["searxng_locale"], traits.all_locale)
    language = params.get("language")
    language = resp.search_params['language']
    language = _text(item, './/div[contains(@class, "property_language")]//div[contains(@class, "property_value")]')
    language codes that are used in the locales.
    language is mapped in a :py:obj:`custom field
    languages = eval_xpath_list(dom, f'//select[@name="{language_param}"]/option/@value')
    languages: dict[str, str] = js_obj_str_to_python(js_code)
    language_list = get(f'{server}/json/languages').json()  # type: ignore
    language_name_locale_map: dict[str, babel.Locale] = {}
    lang_code = eng_lang.split('_')[-1]  # lang_zh-TW --> zh-TW / lang_en --> en
    lang_homepage = 'en'
    lang_map = {'no': 'nb'}
    lang_map = {'no': 'nb'}  # norway
    lang_map: dict[str, str] = {}
    last = RE_DATA_IMAGE_end.search(text)
    last_page = biblio.get("last_page")
    leechers = get_torznab_attribute(item, 'leechers')
    leftover = ''
    legend = model_export['legend']
    lemmy_type: Comments
    lemmy_type: Communities
    lemmy_type: Posts
    lemmy_type: Users
    length = time.gmtime(seconds)
    length = timedelta(milliseconds=video.get("duration", 0))
    license: str
    limit: $limit
    lines: list[str] = []
    link = f"{web_url}/search.php?{urllib.parse.urlencode(args)}"
    link = get_attribute(item, 'link')
    link: str | None,
    links = []
    link_keys = set()
    Locales fetched from `api/locales <https://api.dailymotion.com/locales>`_.
    locale_lang_list = [x.split('_')[0] for x in engine_traits.regions.values()]
    location = weather.GeoLocation.by_query(resp.search_params["query"])
    logger.debug("ARC_ID: %s", str_async)
    logger.debug("bing cookies: %s", params['cookies'])
    logger.debug("cookies %s", params["cookies"])
    logger.debug("cookies: %s", params['cookies'])
    logger.debug("data: %s", args)
    logger.debug("fetch_vqd: request value from from duckduckgo.com")
    logger.debug("found %s results", len(results))
    logger.debug("get_sc_code: new value is: %s", sc_code)
    logger.debug("get_sc_code: querying new sc timestamp @ %s", get_sc_url)
    logger.debug("get_sc_code: request headers: %s", headers)
    logger.debug("obtained cookie: %s", cookie)
    logger.debug("param cookies: %s", params["cookies"])
    logger.debug("param data: %s", params["data"])
    logger.debug("param headers: %s", params["headers"])
    logger.debug("query URL: %s", query)
    logger.debug("querying url: %s", params["url"])
    logger.debug("query_url --> %s", params["url"])
    logger.debug("query_url --> %s", params['url'])
    logger.debug("request --> language %s // len(attributes): %s", eng_tag, len(attributes))
    logger.debug("request --> language %s // len(attributes): %s", language, len(attributes))
    logger.debug("REST API: %s", params["url"])
    logger.debug("SQL Query: %s", query_to_run)
    logger.debug("url %s", params["url"])
    logger.debug('cookie preferences: %s', params['cookies']['preferences'])
    logger.debug('data:image objects --> %s', list(data_image_map.keys()))
    logger.debug('get_thumbnail(): %s', img_src)
    logger.debug('request time %s', str(http_response.elapsed))
    logger.debug(f"query_url --> {params['url']}")
    logger.debug(f'cookies: {params["cookies"]}')
    logger.debug(f'domain selected: {domain}')
    logger.error("Astrophysics Data System (ADS) API key is not set or invalid.")
    logger.error("CORE's API key is not set or invalid.")
    magneturl: str | None,
    map_lang = {'prs': 'fa-AF', 'en': 'en-us'}
    map_market_codes = {
    match = extr(data, '<script>var imageSearchTabData=', '</script>')
    match = matches.group(1)
    match = re.search(r'window\.__INITIAL_STATE__\s*=\s*({.*?});', resp.text, re.S)
    matches = modelexport_re.search(resp.text)
    matches = re.findall(r'bigPipe\.onPageletArrive\((\{.*?\})\);', resp.text, re.DOTALL)
    max_index = -1
    missing: list[str] = []
    missing_opts: list[str] = []
    mode creates an empty file on the file system.  See:
    model_export = json.loads(match)
    month* or *last week* (as offered by SearXNG) is uncommon for scientific
    more = eval_xpath_list(dom, '//c-wiz[@jsrenderer="RBsfwb"]//div[@role="listitem"]', min_len=1)
    Must be call after get_links
    mwv_result = resp['result']
    mwv_result_list = mwv_result['results']
    name = WIKIDATA_PROPERTIES.get(entity_id)
    name_token = extract_text(dom.xpath(name_token_xpath))
    name_token, value_token = token_str.split(":")
    need to do a total of 2 requests (over tor, might be ridiculously slow).
    netloc: str = traits.custom['wiki_netloc'].get(sxng_lang, main_wiki)  # type: ignore
    network: adobe stock
    network: piped
    newest, oldest, largest, smallest
    nextpage_url = extract_text(eval_xpath(dom, cursor_xpath))
    nextpage_url = params['engine_data'].get('nextpage')
    nominatim_json = resp.json()
    not included in the list, the query returns an error.
    not redirected to ``sorry.google.com``.
    Note: the values are not translated
    now = datetime.now()
    no_result_for_http_status: []
    number = biblio.get("issue", "")
    number_of_results = eval_xpath(dom, number_of_results_xpath)
    number_of_results = json_data.get('num_matches')
    number_of_results_element = eval_xpath_getindex(
    object.
    offset = (params['pageno'] - 1) * 10
    offset = (params['pageno'] - 1) * 10 + 1
    offset = (params['pageno'] - 1) * 20
    offset = (params['pageno'] - 1) * 25
    offset = (params['pageno'] - 1) * number_of_results
    offset = (params['pageno'] - 1) * page_size
    offset = params['pageno'] - 1
    offset = _s2i.get(time_range, 0)
    offset: $offset
    on the location, the ``title`` argument in the request is translated.
    Open database in read only mode: if the database doesn't exist.  The default
    open_access: dict[str, str] = item.get("open_access", {})
    OPTIONAL { ?item rdfs:label ?name. }
    OPTIONAL { ?{name}Node psv:{name}/wikibase:quantityUnit ?{name}Unit. } }""".replace(
    ordered_tokens = [position_to_token.get(i, "") for i in range(0, max_index + 1)]
    order_by = bt4g_order_by
    original_title = item.get('originalTitle')
    osm_type = result.get('osm_type', result.get('type'))
    outlined = not re.findall(filled_regex, query)
    packages = {}
    page = int(params.get('pageno', 1)) - 1
    page = params.get('pageno', 1)
    pages = json_data.get("query", {}).get("pages", {}).values()
    pages: str = _stringify_pages(biblio)
    page_num = params["pageno"]
    paging : True
    paging: true
    params = resp.search_params
    params = shlex_split(query)
    params: "OnlineParams",
    params: OnlineCurrenciesParams = resp.search_params  # pyright: ignore[reportAssignmentType]
    params<dict>:
    params["allow_redirects"] = False
    params["base_url"] = get_base_url_choice()
    params["cookies"] = cookie
    params["cookies"] = cookies
    params["cookies"] = google_info["cookies"]
    params["cookies"] = {"PRIVATE-CSRF-TOKEN": private_token}
    params["cookies"] = {'cookie': "yp=1716337604.sp.family%3A0#1685406411.szm.1:1920x1080:1920x999"}
    params["cookies"]["ad"] = eng_lang  # zh_CN
    params["cookies"]["ah"] = eng_region  # "us-en,de-de"
    params["cookies"]["country"] = engine_region.split("-")[-1].lower()  # type: ignore
    params["cookies"]["l"] = eng_region  # "hk-tzh"
    params["cookies"]["safesearch"] = safesearch_map.get(params["safesearch"], "off")
    params["cookies"]["summarizer"] = "0"
    params["cookies"]["ui_lang"] = ui_lang
    params["cookies"]["useLocation"] = "0"
    params["headers"] = {"Authorization": f"Bearer {api_key}"}
    params["headers"].update(
    params["headers"].update(google_info["headers"])
    params["headers"]["Accept"] = "application/json"
    params["headers"]["Accept-Encoding"] = "gzip, deflate"
    params["headers"]["API-Key"] = api_key
    params["headers"]["Authorization"] = f"Bearer {api_key}"
    params["headers"]["Authorization"] = f"Bearer {token}"
    params["headers"]["Content-Type"] = "application/json"
    params["headers"]["Content-Type"] = "application/x-www-form-urlencoded"
    params["headers"]["Referer"] = "https://www.bilibili.com"
    params["headers"]["Referer"] = ddg_url
    params["headers"]["secret-key"] = CACHE.get(SECRET_KEY_DB_KEY)
    params["headers"]["User-Agent"] = searxng_useragent()
    params["headers"]["X-Subscription-Token"] = api_key
    params["json"] = query_data
    params["json"] = {
    params["method"] = "POST"
    params["method"] = 'POST'
    params["raise_for_httperror"] = False
    params["url"] = "https://" + google_info["subdomain"] + "/scholar?" + urlencode(args)
    params["url"] = azure_batch_endpoint
    params["url"] = base_url
    params["url"] = base_url % params
    params["url"] = base_url + "?" + urlencode(search_params)
    params["url"] = base_url.format(query=quote(query), lang=eng_lang.split('_')[0])
    params["url"] = ddg_url
    params["url"] = efetch_url
    params["url"] = f"https://duckduckgo.com/{search_path_map[ddg_category]}.js?{urlencode(args)}"
    params["url"] = f"{api_url}/v1/forecast?{urlencode(args)}"
    params["url"] = f"{base_url}/api/video/shortVideoV2?{urlencode(query_params)}"
    params["url"] = f"{base_url}/api/{huggingface_endpoint}?{urlencode(query_params)}"
    params["url"] = f"{base_url}/de/Ajax/Search?{urlencode(args)}"
    params["url"] = f"{base_url}/en-us/api/v3/search/photos?{urlencode(args)}"
    params["url"] = f"{base_url}/json?{urlencode(args)}"
    params["url"] = f"{base_url}/pf/api/v3/content/fetch/articles-by-search-v2?query={quote_plus(dumps(args))}"
    params["url"] = f"{base_url}/pics?{urlencode(query_params)}"
    params["url"] = f"{base_url}/portal/lw/search/homePageV3?{urlencode(query_params)}"
    params["url"] = f"{base_url}/results?id={request_id}"
    params["url"] = f"{base_url}/s?{urlencode(query_params)}"
    params["url"] = f"{base_url}/search.naver?{urlencode(query_params)}"
    params["url"] = f"{base_url}/search?search={quote_plus(query)}"
    params["url"] = f"{base_url}/search?{urlencode(query_params)}"
    params["url"] = f"{base_url}/v1/video/list?{urlencode(query_params)}"
    params["url"] = f"{base_url}/web?{urlencode(query_params)}"
    params["url"] = f"{base_url}/weixin?{urlencode(query_params)}"
    params["url"] = f"{base_url}/{from_lang}-{to_lang}-dictionary/{urllib.parse.quote_plus(query)}"
    params["url"] = f"{base_url}/{query}?{urlencode(query_params)}"
    params["url"] = f"{base_url}/{search_type}?{urlencode(query_params)}"
    params["url"] = f"{base_url}?{urlencode(args)}"
    params["url"] = f"{base_url}?{urlencode(query_params)}"
    params["url"] = f"{base_url}?{urlencode(search_args)}"
    params["url"] = f"{base_url}{brave_category}?{urlencode(args)}"
    params["url"] = f"{base_url}{category_config[chinaso_category]['endpoint']}?{urlencode(query_params)}"
    params["url"] = f"{cdn_base_url}/devicon.json"
    params["url"] = f"{cdn_base_url}/tags.json"
    params["url"] = f"{params['base_url']}/search?{urlencode(filtered_args)}"
    params["url"] = f"{query_url}?{urlencode(query_params)}"
    params["url"] = f"{search_api}?{args}"
    params["url"] = f"{search_api}?{urlencode(args)}"
    params["url"] = f"{search_url}?{args}"
    params["url"] = f"{search_url}?{urlencode(args)}"
    params["url"] = f"{wc_api_url}?{urlencode(args, safe=':|')}"
    params["url"] = f"{_base_url()}/yacysearch.json?{urlencode(args)}"
    params["url"] = search_api + urlencode(args)
    params["url"] = search_api + urlencode(query_params)
    params["url"] = search_url
    params["url"] = search_url.format(
    params["url"] = search_url.format(query=quote_plus(query)) + "&page={pageno}".format(pageno=params["pageno"])
    params["url"] = search_url.format(search_term=quote(query), search_type=search_type)
    params["url"] = url.format(query=quote(query), lang=params["language"])
    params["url"] = URL.format(query=urlencode({"q": query}))
    params["url"] = _backend_url() + f"{path}?" + urlencode(args)
    params["verify"] = False
    params['allow_redirects'] = False
    params['allow_redirects'] = True
    params['attributes'] = attributes
    params['base_url'] = random.choice(base_url) if isinstance(base_url, list) else base_url
    params['cookies'] = google_info['cookies']
    params['cookies'] = response_index.cookies
    params['cookies'].update(cookies)
    params['cookies']['ad'] = eng_lang
    params['cookies']['ah'] = eng_region
    params['cookies']['CONSENT'] = "YES+"
    params['cookies']['g_rated'] = safesearch_map[params['safesearch']]
    params['cookies']['l'] = eng_region
    params['cookies']['preferences'] = 'N1N'.join(["%sEEE%s" % x for x in cookie.items()])
    params['cookies']['sB'] = build_sb_cookie(sbcookie_params)
    params['cookies']['_EDGE_CD'] = f'm={engine_region}&u={engine_language}'
    params['cookies']['_EDGE_S'] = f'mkt={engine_region}&ui={engine_language}'
    params['data'] = args
    params['data'] = dumps(
    params['data'] = dumps(data)
    params['data'] = dumps(form_data)
    params['data'] = dumps(request_data)
    params['data'] = json.dumps(args)
    params['data'] = {
    params['data'] = {'auth_key': api_key, 'text': params['query'], 'target_lang': params['to_lang'][1]}
    params['data'] = {'q': query, 'wt': "json"}  # request response in parsable format
    params['data'] = {'query': query}
    params['discovery'] = len(discovery_filters) != 0
    params['domain'] = domain
    params['headers'] = get_headers()
    params['headers'] = {
    params['headers'] = {'Accept': 'application/json', 'Content-Type': 'application/json'}
    params['headers'] = {'Authorization': 'Bearer ' + token['value']}
    params['headers'] = {'Authorization': 'Bearer {}'.format(j.get('access_token'))}
    params['headers'] = {'Content-Type': 'application/json'}
    params['headers'] = {'Cookie': cookie}
    params['headers'].update(
    params['headers'].update(google_info['headers'])
    params['headers'].update(headers)
    params['headers']['Accept'] = accept_header
    params['headers']['Authorization'] = f'Bearer {cf_ai_api}'
    params['headers']['Authorization'] = f'Token {ta_token}'
    params['headers']['content-type'] = "application/json"
    params['headers']['Content-Type'] = 'application/json'
    params['headers']['Content-type'] = 'text/plain'
    params['headers']['Origin'] = 'https://yep.com'
    params['headers']['Origin'] = base_url
    params['headers']['PUBLIC-CSRF-TOKEN'] = public_token
    params['headers']['Referer'] = "https://nvd.nist.gov/vuln/search"
    params['headers']['Referer'] = 'https://yep.com/'
    params['headers']['Referer'] = base_url + '/'
    params['headers']['Referer'] = referer_url.format(query=urlencode({'i': query}))
    params['headers']['Referer'] = site_url.format(query=urlencode({'i': query}))
    params['headers']['User-Agent'] = (
    params['headers']['User-Agent'] = searxng_useragent()
    params['headers']['X-GitHub-Api-Version'] = ghc_api_version
    params['language'] = eng_tag
    params['method'] = "POST"
    params['method'] = 'GET'
    params['method'] = 'POST'
    params['method'] = method
    params['query'] = query
    params['raise_for_httperror'] = False
    params['req_url'] = request_url
    params['soft_max_redirects'] = 1
    params['soft_max_redirects'] = 2
    params['soft_max_redirects'] = soft_max_redirects
    params['url'] = 'https://mediathekviewweb.de/api/query'
    params['url'] = (
    params['url'] = api_url + urlencode(args)
    params['url'] = base_url + '?' + args  #
    params['url'] = base_url + '?' + urlencode(query_params)
    params['url'] = base_url + '?' + urlencode(url_params)
    params['url'] = base_url + search_path
    params['url'] = base_url + search_string.format(query=query, page=params['pageno'])
    params['url'] = base_url + search_string.format(query=urlencode({'q': query}), limit=number_of_results)
    params['url'] = base_url + search_string.format(query=urllib.parse.urlencode({'q': query}))
    params['url'] = base_url + search_url.format(query=urlencode({'id': query}))
    params['url'] = base_url + urlencode(args)
    params['url'] = base_url.format(**string_args)
    params['url'] = f"https://www.wordnik.com/words/{query}"
    params['url'] = f"{api_url}/get?{urllib.parse.urlencode(args)}"
    params['url'] = f"{api_url}/public/events/search?{urlencode(args)}"
    params['url'] = f"{api_url}/search/?{urlencode({'s': query})}"
    params['url'] = f"{base_url.rstrip('/')}/api/search?{urlencode(args)}"
    params['url'] = f"{base_url}/0.2/search/podcast?{urlencode(args)}"
    params['url'] = f"{base_url}/?s={quote_plus(query)}"
    params['url'] = f"{base_url}/?{urlencode(args)}"
    params['url'] = f"{base_url}/api/search/byterm?q={quote_plus(query)}"
    params['url'] = f"{base_url}/api/search/v3/catalog/search?{urlencode(args)}"
    params['url'] = f"{base_url}/api/v1/projects/?{urlencode(args)}"
    params['url'] = f"{base_url}/api/v1/repos/search?{urlencode(args)}"
    params['url'] = f"{base_url}/api/v1/search?{urlencode(args)}"
    params['url'] = f"{base_url}/api/v2/search?{urlencode(args)}"
    params['url'] = f"{base_url}/api2u/search?{urlencode(args)}"
    params['url'] = f"{base_url}/bpbapi/filter/search?{urlencode(args)}"
    params['url'] = f"{base_url}/fs/2/search?{urlencode(args)}"
    params['url'] = f"{base_url}/packages?{urlencode(args)}"
    params['url'] = f"{base_url}/resource/BaseSearchResource/get/?data={dumps(args)}"
    params['url'] = f"{base_url}/rest/api/search/{query}"
    params['url'] = f"{base_url}/search/?{urlencode(args)}"
    params['url'] = f"{base_url}/search/photo/@/page:{params['pageno']}:{page_size}?q={quote_plus(query)}"
    params['url'] = f"{base_url}/search/public/stream?{urlencode(args)}"
    params['url'] = f"{base_url}/search/score/{time_range}?{urlencode(args)}"
    params['url'] = f"{base_url}/search/{query}?{urlencode(query_params)}"
    params['url'] = f"{base_url}/search/{quote_plus(query)}/{page_size}/{(params['pageno']-1)*page_size}"
    params['url'] = f"{base_url}/search?q={quote_plus(query)}"
    params['url'] = f"{base_url}/search?{urlencode(args)}"
    params['url'] = f"{base_url}/search?{urlencode(query_params)}"
    params['url'] = f"{base_url}/v1/query/{arch_path}?q={quote_plus(query)}"
    params['url'] = f"{base_url}/v2/search-gateway/recipes?{urlencode(args)}"
    params['url'] = f"{base_url}/vectors/{query}/{params['pageno']}/"
    params['url'] = f"{base_url}/{api_path}?{urlencode(args)}"
    params['url'] = f"{base_url}/{index}/_search"
    params['url'] = f"{base_url}/{pixabay_type}/search/{quote_plus(query)}/?{urlencode(args)}"
    params['url'] = f"{base_url}api/v3/search?{urlencode(args)}"
    params['url'] = f"{cdn_base_url}/index.json"
    params['url'] = f"{request_url}/api/translate?{urllib.parse.urlencode(args)}"
    params['url'] = f"{request_url}/translate"
    params['url'] = f"{search_url}?{urlencode(args)}"
    params['url'] = f"{server}/json/stations/search?{urlencode(args)}"
    params['url'] = f"{url}/api/v1/{params['from_lang'][1]}/{params['to_lang'][1]}/{params['query']}"
    params['url'] = f'https://gateway.ai.cloudflare.com/v1/{cf_account_id}/{cf_ai_gateway}/workers-ai/{cf_ai_model}'
    params['url'] = f'https://{domain}/search?{urlencode(url_params)}'
    params['url'] = f'{base_url}' + search_url.format(query=quote(query), pageno=params['pageno'])
    params['url'] = f'{base_url}/api/storesearch/?{urlencode(query_params)}'
    params['url'] = f'{base_url}?{urlencode(query_params)}'
    params['url'] = f'{base_url}search/video?{urlencode(args)}'
    params['url'] = f'{base_url}{search_endpoint}?{urlencode(args)}'
    params['url'] = graphql_url
    params['url'] = instance_urls[instance_index % len(instance_urls)]
    params['url'] = params['base_url'] + f'/usearch/{quote(query)}/{params["pageno"]}/'
    params['url'] = pdbe_solr_url
    params['url'] = query_url
    params['url'] = rest_v1_summary_url.format(wiki_netloc=wiki_netloc, title=title)
    params['url'] = search_api + args
    params['url'] = search_api + urlencode(query_params)
    params['url'] = search_url
    params['url'] = search_url + '?' + urlencode(args)
    params['url'] = search_url + query_str
    params['url'] = search_url + urlencode({'query': query, 'page': params['pageno'], 'per_page': page_size})
    params['url'] = search_url.format(
    params['url'] = search_url.format(**fargs)
    params['url'] = search_url.format(**fp)
    params['url'] = SEARCH_URL.format(base=BASE, query=urlencode({'q': query}))
    params['url'] = search_url.format(offset=offset, query=urlencode({'p': query}))
    params['url'] = search_url.format(pageno=params['pageno'], query=urlencode({'q': query}))
    params['url'] = search_url.format(query=query)
    params['url'] = SEARCH_URL.format(query=query, pageno=params['pageno'])
    params['url'] = search_url.format(query=urlencode(args))
    params['url'] = search_url.format(query=urlencode({'input': query}), api_key=api_key)
    params['url'] = search_url.format(query=urlencode({'input': query}), token=token)
    params['url'] = search_url.format(query=urlencode({'q': query, 'lang': params['language']}))
    params['url'] = search_url.format(query=urlencode({'q': query, name_token: value_token}))
    params['url'] = search_url.format(query=urlencode({'q': query}))
    params['url'] = search_url.format(query=urlencode({'q': query}), api_key=api_key)
    params['url'] = search_url.format(query=urlencode({'q': query}), offset=offset)
    params['url'] = search_url.format(query=urlencode({'q': query}), page=urlencode({'page': params['pageno']}))
    params['url'] = search_url.format(query=urlencode({'term': query, 'media': 'software', 'explicit': explicit}))
    params['url'] = search_url.format(query=urlencode({'text': query}), page=params['pageno']) + _get_time_range_url(
    params['url'] = search_url.format(search_term=quote(query), pageno=params['pageno'] - 1)
    params['url'] = search_url.format(search_term=quote(query), pageno=params['pageno'])
    params['url'] = search_url.format(start=params['pageno'] * page_size, query=urlencode({'q': query}))
    params['url'] = SPARQL_ENDPOINT_URL
    params['url'] = suggestion_url.format(letter=query[0], query=query)
    params['url'] = url
    params['url'] = url + urlencode(args)
    params['url'] = url.format(
    params['url'] = _get_algolia_api_url()
    params['url'] = _search_url
    params['url'] = _search_url.format(params=urlencode(query_params))
    parsed = urlparse(url)
    parsed_url = urlparse(url)
    parsers = {'general': parse_general, 'images': parse_images, 'it': parse_it}
    parsers = {'general': parse_general, 'images': parse_images, 'news': parse_news, 'videos': parse_videos}
    parsers = {'news': parse_news, 'images': parse_images, 'videos': parse_videos}
    parse_data_images,
    parse_duration_string,
    pass
    password: ''
    password: changeme
    path = "/search"
    pdf_url: str = primary_location.get("pdf_url") or open_access.get("oa_url") or ""
    pdia_config_filepart = extr(resp.text, pdia_config_start, pdia_config_end)
    pdia_config_url = pdia_base_url + pdia_config_start + pdia_config_filepart + pdia_config_end
    peers = get_torznab_attribute(item, 'peers')
    photos = search_results['photos']['photo']
    piped_filter: music_songs
    piped_filter: videos
    pmids: list[str] = [i.text for i in pmids_results.xpath("//eSearchResult/IdList/Id")]
    pmids_results = etree.XML(esearch_resp.content)
    pods = search_results.xpath(pods_xpath)
    port: 27017
    port: 6379
    pos = script.index(end_tag) + len(end_tag) - 1
    pos = script.index(start_tag) + len(start_tag) - 1
    position_to_token: dict[int, str] = {}
    Possible formats:
    posts = search_results.get('data', {}).get('children', [])
    primary_location: dict[str, str] = item.get("primary_location", {})
    print("WIKIPEDIA_LANGUAGES: %s" % len(engine_traits.custom['WIKIPEDIA_LANGUAGES']))
    private_token = resp.cookies["PRIVATE-CSRF-TOKEN"]
    private_token: str | None = CACHE.get(CSRF_PRIVATEKEY_CACHE)
    property_element: etree.Element | None = item.find(property_name)
    pua_chars = {
    pubDate = get_attribute(item, 'pubDate')
    publications and is not supported by Google Scholar.
    public_token = resp.json()["public_csrf_token"]
    public_token, private_token = fetch_csrf_tokens()
    public_token: str | None = CACHE.get(CSRF_PUBLICKEY_CACHE)
    publishedDate = None
    published_date = None
    published_date = _parse_date(item.get("publication_date"))
    publisher: str = host_venue.get("publisher", "")
    publisher: str = s_text[-1]
    q = parse(query_string)
    q = [query, f'order:{api_order}']
    q = []  # pylint: disable=invalid-name
    qkey = q[0]
    quality: float
    Quark may return a CAPTCHA challenge after 9 requests in a short period.
    query = (
    query = params["query"]
    query = query.replace(" ", "_").lower()
    query = QUERY_PROPERTY_NAMES.replace('%ATTRIBUTES%', " ".join(wikidata_property_names))
    query = quote_ddg_bangs(query)
    query = re.sub(filled_regex, "", query).strip()
    query = resp.search_params["query"]
    query = resp.search_params["query"].lower()
    query = urlencode({'keyword': query})
    query = urlencode({'page': params['pageno'], 'terms': query})
    query = urlencode({'q': query, 'limit': page_size})
    query = urlencode({'q': query, 'page': params['pageno'], 'lang': ''})
    query = urlencode({'q': query, 'page': params['pageno']})
    query = urlencode({'query': query, 'c': (params['pageno'] - 1) * page_size})
    query = urlencode({'url': query})
    query = [(k, v) for (k, v) in parse_qsl(parsed.query) if k != 'ixid']
    query = [(k, v) for (k, v) in parse_qsl(parsed.query) if k not in ['ixid', 's']]
    query = _client.find({key: q}).skip((params['pageno'] - 1) * results_per_page).limit(results_per_page)
    query, attributes = get_query(query, eng_tag)
    query: $query
    query: str
    query: str,
    query_arch = ARCH_RE.search(query)
    query_data = query_data_template
    query_data["from"] = (params["pageno"] - 1) * number_of_results
    query_data["query"]["multi_match"]["query"] = query
    query_params = category_config[baidu_category]['params']
    query_params = category_config[quark_category]['params']
    query_params = [
    query_params = {
    query_params = {"count": 10, "q": query, "start": params["pageno"] * 10}
    query_params = {"count": results_per_page, "nsfw": min(params["safesearch"], 1), "query": query}
    query_params = {"key": query, "pageNum": params["pageno"], "pageSize": 25}
    query_params = {"keyword": query, "pCursor": params["pageno"]}
    query_params = {"page": params['pageno']}
    query_params = {"q": query}
    query_params = {"term": query, "cc": "us", "l": "en"}
    query_params = {'q': query, 'rows': rows}
    query_params = {'query': query}
    query_params.update(category_config[chinaso_category]['params'])
    query_params_images = {
    query_params_web = {
    query_parts = query.split(" ")
    query_parts = resp.search_params["query"].lower().split(" ")
    query_parts = resp.search_params['query'].lower().split(' ')
    query_str = urlencode(args)
    query_str: >-
    query_to_run = query_str + ' LIMIT :limit OFFSET :offset'
    query_to_run = query_str + ' LIMIT {0} OFFSET {1}'.format(limit, (params['pageno'] - 1) * limit)
    query_type: match
    query_type: path
    query_url = (
    query_url = category_config[baidu_category]['endpoint']
    query_url = category_config[quark_category]['endpoint']
    quoted."""
    qwant_categ: images
    qwant_categ: news
    qwant_categ: videos
    qwant_categ: web-lite  # alternatively use 'web'
    q_initial_props = loads(json_string)
    q_locale = traits.get_region(params["searxng_locale"], default='en_US')
    q_locales = q_initial_props.get('locales')
    r = http_post(
    raise RuntimeError("Couldn't find any request id for presearch")
    raise SearxEngineAPIException("failed to obtain secret key")
    raise ValueError(f"Invalid search type {search_type}")
    raise ValueError(f"Unsupported brave category: {brave_category}")
    raise ValueError(f"Unsupported lemmy type: {lemmy_type}")
    raise ValueError(f"Unsupported play category: {play_categ}")
    raise_for_httperror(resp)
    ranges (*day*, *week*, *month*, *year*) are mapped to *year*.  If no range
    raw_search_results = loads(resp.text)
    reader_thread = Thread(target=_get_results_from_process, args=(res, cmd, params['pageno']))
    reader_thread.join(timeout=timeout)
    reader_thread.start()
    redirected to a CAPTCHA page."""
    REF: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-match-query.html
    REF: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-simple-query-string-query.html
    REF: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-term-query.html
    REF: https://www.elastic.co/guide/en/elasticsearch/reference/current/query-dsl-terms-query.html
    Referer: https://html.duckduckgo.com/
    region = traits.get_region(params["searxng_locale"], default=traits.all_locale)
    regions = eval_xpath_list(dom, f'//select[@name="{region_param}"]/option/@value')
    regions = json.loads(js_code)
    regions = json['components']['schemas']['Region']['enum']
    regions>` from Brave."""
    regions>` from Startpage."""
    relatedTopics: list[dict[str, str | list[str]]] = []
    release_time = album_info.get("releaseTime", {}).get("value")
    request_body: >-
    request_data = {
    request_id, cookies = _get_request_id(query, params)
    request_url = random.choice(base_url) if isinstance(base_url, list) else base_url
    res = EngineResults()
    res = hit['result']
    res.add(
    res.add(res.types.Answer(answer=answer, url=url))
    res.add(res.types.LegacyResult(number_of_results=count))
    res.add(res.types.LegacyResult(number_of_results=query.count()))
    res.add(res.types.Translations(translations=translations))
    res.add(res.types.Translations(translations=[item], url=url))
    res.add(weather_answer)
    resolution = None
    resp = get(
    resp = get("https://cdn.search.brave.com/serp/v2/_app/immutable/chunks/parameters.734c106a.js")
    resp = get("https://www.bing.com/account/general", headers=headers)
    resp = get('https://api.dailymotion.com/languages')
    resp = get('https://api.dailymotion.com/locales')
    resp = get('https://duckduckgo.com/dist/util/u.7669f071a13a7daa57cb.js')
    resp = get('https://search.brave.com/settings')
    resp = get('https://wiki.archlinux.org/', timeout=3)
    resp = get('https://www.google.com/preferences')
    resp = get('https://www.startpage.com/do/settings', headers=headers)
    resp = get(f"{pdia_base_url}/search/?q=", timeout=3)
    resp = get(get_base_url_choice() + "/search")
    resp = get(get_sc_url, headers=headers)
    resp = get(pdia_config_url)
    resp = get(url, headers=headers, timeout=5)
    resp = http_get(url, timeout=3)
    resp = loads(resp.text)
    resp = network.get(about['website'])
    resp = network.get(base_url + "/preferences", headers={'Accept-Language': 'en-US,en;q=0.5'})
    resp = post("https://www.artstation.com/api/v2/csrf_protection/token.json")
    resp = scraper.get(base_url)
    resp = _network.get(list_of_wikipedias)
    resp.raise_for_status()
    resp: requests response object
    resp: SXNG_Response = http_get(url, timeout=10, allow_redirects=False)
    resp: SXNG_Response = http_post(url, body, timeout=5)
    response from `api.artic.edu <https://artic.edu>`__ and filter out all
    response_data = loads(resp.text)
    response_index = get(base_url, headers=params['headers'], raise_for_httperror=True, timeout=3)
    response_json = loads(resp.text)
    resp_json = loads(resp.text)
    resp_json = resp.json()
    resp_json: ApiSearchResults = resp.json()  # type: ignore
    rest/api/azure-resourcegraph/?view=rest-azureresourcegraph-resourcegraph-2024-04-01",
    result = ""
    result = resp.json()
    result = {
    result = {}
    result: dict[str, t.Any] = {
    results = EngineResults()
    results = response_json['results']
    results = []
    results.add(
    results.add(results.types.Translations(translations=[item]))
    results.add(results.types.Translations(translations=[item], url=link))
    results.append(
    results.append({'number_of_results': len(json_data['topics'])})
    results.append({'number_of_results': response_json['number_of_results']})
    results.append({'number_of_results': result_len})
    results.append({'url': resp.request.headers['Referer'], 'title': title, 'content': result_content})
    results.extend({'suggestion': s} for s in response_json['suggestions'])
    results: list[ApiSearchResult]
    results_data = extr(response_text, 'ytInitialData = ', ';</script>')
    results_dom = dom.xpath('//li[contains(@class, "video-listing-entry")]')
    results_dom = dom.xpath(results_xpath)
    results_json = loads(results_data) if results_data else {}
    results_json = loads(results_raw)
    results_obj = results_json.get('render', {}).get('presenter', {}).get('regions', {})
    results_per_page: 20
    results_per_page: 20     # optional
    results_query : documents
    results_raw = '{' + extr(resp.text, f"React.createElement(UIStartpage.AppSerp{categ}, {{", '}})') + '}}'
    result["content"] = " | ".join(content)
    result_chunks = []
    result_content = ""
    result_divs = eval_xpath_list(dom, '//div[contains(@class, "MjjYud")]')
    result_index = 0
    result_json = loads(response_text)
    result_len = 0
    result_type: MainResult
    res_json = resp.json()
    res_xpath = results_xpath.format(extra=extra_xpath)
    ret = []
    return " ".join(_q)
    return ""
    return "&".join(cookie_parts)
    return ",".join([arc_id, use_ac, _fmt])
    return "{} {}".format(area.get("amount", ""), area.get("unit", ""))
    return ''
    return ('https://{0}.wikipedia.org/wiki/{1}'.format(*value), '{1} ({0})'.format(*value))
    return (date.today() - timedelta(days=offset)).isoformat()
    return (int(pageno) - 1) * 10 + 1
    return (prefix + value, value)
    return (value, value)
    return api_url
    return arg
    return attributes
    return authors
    return authors, journal, publisher, publishedDate
    return base_url
    return bool(dom.xpath('//title') and "seized" in dom.xpath('//title')[0].text.lower())
    return bool(eval_xpath(dom, "//form[@id='challenge-form']"))
    return bool(re.search(CAPTCHA_PATTERN, html))
    return client_id or None
    return cmd
    return content, published_date
    return content_parts
    return cookie
    return custom_query
    return data
    return data_image_map
    return definitions
    return doi_value.removeprefix("https://doi.org/")
    return do_query(data, q)
    return element.get('simpleText', '')
    return EngineResults.types.WeatherAnswer.Item(
    return eng_tag, wiki_netloc
    return eng_traits.custom["lang_region"].get(sxng_locale, lang) or None
    return extract_text(eval_xpath(item, selector))
    return f"{name_token}:{value_token}"
    return f'{base_url.rstrip("/")}{relative_url}'
    return False
    return frontend_url or engines["piped"].frontend_url  # type: ignore
    return get_external_url('wikimedia_image', raw_value)
    return get_label(labels, lang)
    return get_label(tag_labels, lang)
    return http_response.content
    return img_results + text_results
    return img_src
    return isinstance(obj, Iterable)
    return journal, publisher, pages, volume, number, published_date
    return key_rank
    return lines, highlighted_lines_index
    return links, link_keys
    return loads(http_response.content.decode())
    return MainResult(
    return max(counts, key=counts.get)
    return name
    return None
    return params
    return parsers[baidu_category](data)
    return parsers[chinaso_category](data)
    return parsers[naver_category](resp.text)
    return parse_first_page_response(resp.text)
    return parse_web_api(resp)
    return photo_url.format(userid=user_id, photoid=photo_id)
    return public_token, private_token
    return q
    return query, attributes
    return res
    return resp.json()["access_token"]
    return result
    return results
    return ret
    return ret_val
    return sc_code
    return servers
    return sorted(results, key=itemgetter("seed"), reverse=True)
    return sorted(results, key=itemgetter('seed'), reverse=True)
    return start, end
    return tags
    return tag_label
    return text
    return text if text != "" else None
    return text.startswith("http") and " " in text
    return text.strip()
    return time.strftime("%M:%S", length)
    return title
    return title, address
    return tmp_result
    return token
    return True
    return unit
    return unquote(url_string[start:end])
    return url
    return url, html_url, pdf_url
    return url, osm, geojson
    return urlunparse((parsed.scheme, parsed.netloc, parsed.path, parsed.params, urlencode(query), parsed.fragment))
    return value
    return video_response(resp)
    return WeatherAnswer.Item(
    return [
    return [title, content, thumbnail]
    return [x for xs in xss for x in xs]
    return []
    return _CACHE
    return {
    return {"query": {"match": {key: {'query': value}}}}
    return {"title": html_to_text(data.get('title')), "url": data.get('url'), "content": html_to_text(data.get('desc'))}
    return {'Accept': 'application/sparql-results+json', 'User-Agent': searxng_useragent()}
    return {'query': {'simple_query_string': {'query': query}}}
    return {'query': {'term': {key: value}}}
    return {'query': {'terms': {key: values.split(',')}}}
    Returns:
    ret_val: dict[str, int] = {}
    ret_val: dict[str, t.Any] = {
    ret_val: str = CACHE.get("X-S2-UI-Version")
    ret_val['cookies']['CONSENT'] = "YES+"
    ret_val['country'] = country
    ret_val['headers']['Accept'] = '*/*'
    ret_val['headers']['User-Agent'] = gen_gsa_useragent()
    ret_val['language'] = eng_lang
    ret_val['locale'] = locale
    ret_val['params']['cr'] = ''
    ret_val['params']['hl'] = f'{lang_code}-{country}'
    ret_val['params']['ie'] = 'utf8'
    ret_val['params']['lr'] = eng_lang
    ret_val['params']['oe'] = 'utf8'
    ret_val['subdomain'] = eng_traits.custom['supported_domains'].get(country.upper(), 'www.google.com')
    rows = dom.xpath('//table[@class="listing"]//tr[contains(@class, "category_0")]')
    safesearch: true
    safesearch_table,
    safes_search_map:
    safe_search = ''
    safe_search = safesearch_cookies.get(params["safesearch"])
    sbcookie_params = {
    scraper = cloudscraper.create_scraper()
    script = script[:pos]
    script = script[pos:]
    script = utils.eval_xpath_getindex(dom, '//script', 0, default=None).text
    script_tags = tree.xpath("//script[contains(@src, '/assets/')]")
    sc_code = CACHE.get("SC_CODE")
    sc_code = str(sc_code)
    search={query}&page={pageno}{time_range}{safe_search}
    search_args: dict[str, str | int | None] = {
    search_params = {
    search_path = search_string.format(
    search_path = search_string.format(query=urlencode({'q': query}), nb_per_page=nb_per_page, page=params['pageno'])
    search_path = search_string.format(query=urlencode({'q': query}), page=params['pageno'])
    search_res = dom.xpath('.//td[@class="x-item"]')
    search_res = dom.xpath('//div[@class="one_result"]')
    search_res = eval_xpath_list(dom, '//table[contains(@class, "data")]//tr[descendant::a]', None)
    search_res = loads(resp.text)
    search_res = resp.json()
    search_res: dict[str, str] = resp.json()
    search_results = etree.XML(resp.content)
    search_results = json.loads(resp.text)
    search_results = loads(resp.text)
    search_results = lxml.etree.XML(resp.content)
    search_results = raw_search_results.get('channels', [])
    search_results = resp.json()
    search_results = resp.json()["hits"]["hits"]
    search_type = 'search'
    search_type = search_types.get(params["category"], "0")
    search_type: image
    search_type: images
    search_type: news
    search_type: search
    search_type: text
    search_type: videos
    search_url : https://bitbucket.org/repo/all/{pageno}?name={query}
    search_url : https://developer.mozilla.org/api/v1/search?q={query}&page={pageno}
    search_url = params["base_url"] + "/api/v1/search?q={query}"
    search_url = params['base_url'] + '/search?{query}'
    search_url: str = (
    search_url: str = base_url + '?t=search&q={search_query}'
    searx format: "key:value" e.g. city:berlin
    searx format: "key:value1,value2" e.g. city:berlin,paris
    SearxEngineAccessDeniedException,
    SearxEngineAPIException,
    SearxEngineCaptchaException,
    SearxEngineTooManyRequestsException,
    SearXNG format: "key:value" e.g. city:berlin
    SearXNG's locale.
    SearXNG's ``all`` locale maps DuckDuckGo's "All regions" (``wt-wt``).
    seconds = int(duration)
    secret_key = CACHE.get(SECRET_KEY_DB_KEY)
    sections = (
    See videoLanguages_ in commit `8ed5c729 - Refactor and redesign client`_
    seeders = get_torznab_attribute(item, 'seeders')
    seen_entities = set()
    select = [a.get_select() for a in attributes]
    sense in a SearXNG request since SearXNG's ``all`` will not add a
    server = random.choice(servers)
    server = server_list()[0]
    servers = CACHE.get("servers", [])
    servers = server_list()
    server_list()
    set, a :py:obj:`ValueError` is raised during initialization.
    set_bing_cookies(params, engine_language, engine_region)
    shortcut : rds
    shortcut: aaa
    shortcut: ans
    shortcut: asi
    shortcut: bt4gv
    shortcut: demo
    shortcut: els
    shortcut: fnd
    shortcut: forgejo
    shortcut: ghc
    shortcut: gitea
    shortcut: gl
    shortcut: gn
    shortcut: gos
    shortcut: hf
    shortcut: hfd
    shortcut: hfs
    shortcut: md
    shortcut: mediathekview
    shortcut: mes
    shortcut: msl
    shortcut: ol
    shortcut: ppdm
    shortcut: pst
    shortcut: slr
    shortcut: srht
    shortcut: tuba
    shortcut: ya
    shortcut: yai
    size_re = re.compile(r'[\d.]+(T|G|M)?B', re.IGNORECASE)
    skip_eng_tags = {
    sort = None
    sort: asc
    sortBy: $sortBy
    sort_order_path = SORT_RE.search(query)
    spot = eval_xpath_getindex(dom, '//div[@class="ipRz4"]', 0, None)
    sp_region_names = []
    start = (pageno - 1) * 10
    start = (params['pageno'] - 1) * 10
    start = text.index("data: [{")
    start = url_string.find('http', url_string.find('/RU=') + 1)
    start, end = __get_results_limits(pageno)
    Startpage puts a ``sc`` argument on every HTML :py:obj:`search form
    Startpage's search form generates a new sc-code on each request.  This
    Startpage's search form:
    startpage_categ: web
    start_index = (params["pageno"] - 1) * 50
    start_index = (params["pageno"] - 1) * results_per_page
    start_tag = 'window.MESON.initialState = {'
    streams = item['streams']
    string will be returned.
    string_args = {
    str_async = ui_async(start)
    suggestions = json.loads(resp.text)
    suggestion_xpath,
    svg_type = "fill1" if not outlined else "default"
    sxng_lang = params['searxng_locale'].split('-')[0]
    sxng_lang = resp.search_params['searxng_locale'].split('-')[0]
    sxng_locale = params.get('searxng_locale', 'all')
    sxng_locale = params.get('searxng_locale', 'en-US')
    sxng_locale: str,
    s_text = text.split(" - ")
    tags: list[str] = []
    tag_label = labels.get(lang.lower())
    tag_labels = OSM_KEYS_TAGS['tags'].get(tag_category, {}).get(tag_name, {})
    tag_name = '' if tag_name is None else tag_name
    ta_link_to_mp4: true
    ta_token:
    tempC: float = data.get("temp_C") or data.get("tempC")  # type: ignore
    territories = Locale("en").territories
    text = " ".join(t for t in ordered_tokens if t != "")
    text = data['responseData']['translatedText']
    text = json_resp.get('translatedText')
    text = resp.text
    text = text[text.index("<script") : text.index("</script")]
    text_results = []
    the *region*:
    The arc_id is random generated every hour.
    The best solution seems to ignore these results.
    The field used can be specified in index.query.default_field in Elasticsearch.
    The file size of a full-size image is usually several MB; when reduced to a
    The function :py:obj:`get_ddg_lang` evaluates DuckDuckGo's language from
    The href URL is broken, the "Related website" may contains some HTML.
    The language list `api/languages <https://api.dailymotion.com/languages>`_
    The location of the Wikipedia address of a language is mapped in a
    The origin of this demo engine is a simple json string which is loaded in
    the query is not executed.
    The rank defines in which order the key are displayed in the HTML result
    The standard for full text queries.
    The tokens are hidden in a hidden input field.
    There are duplications in the locale codes returned from Dailymotion which
    They update every minute, but allow up to 1 hour old tokens to be used.
    this example while the engine is initialized.
    This function is called by the various google engines (:ref:`google web
    This function removes these 2 issues.
    thumbnail = _text(item, './/img[contains(@class, "cover")]/@data-src')
    thumbnail.resizer_url:
    thumbnail.url:
    thumbnail.width & .height:
    thumbnail: str = ""
    thumbnailUrl = None
    timedelta,
    timeout: 10
    timestamp = hit['result']['lyrics_updated_at']
    time_delta = time_delta_dict.get(params["time_range"])
    time_range = ''
    time_range = params.get('time_range')
    time_range = params['time_range'] or 'all'
    time_range_dict = {
    time_range_dict,
    time_range_map:
    time_range_support: true
    time_range_table,
    time_range_url : '&days={time_range_val}'
    title = "Wolfram Alpha (%s)" % infobox_title
    title = html_to_text(title)
    title = item.get('title', '')
    title = item["title"]
    title = item['title']
    title = remove_pua_from_str(html_to_text(result['title']))
    title = result['title']
    title = urllib.parse.quote(query)
    title = utils.html_to_text(api_result.get('titles', {}).get('display') or api_result.get('title'))
    title may be None
    title: str
    title: str = traits.custom['title'].get(sxng_lang, 'Special:Search')  # type: ignore
    Titles from Presearch shows domain + title without spacing, and HTML
    title_filter = html_to_text if title_html_to_text else identity
    title_map = {
    title_query : title
    title_xpath : //article[@class="repo-summary"]//a[@class="repo-link"]
    title_xpath = './/h3//a/@aria-label'
    tmp_result = {}
    To avoid CAPTCHAs we need to send a well formed HTTP POST request with a
    To limit the result list when the users selects a range, all the SearXNG
    To spend the least amount of requests, it is best to always get the newest
    token = authenticate(t_id, c_id, c_secret)
    token = CACHE.get(key="token")
    token = get_auth_token(azure_tenant_id, azure_client_id, azure_client_secret)
    token = obtain_token()
    token: str | None = CACHE.get(key)
    tokens from each request. In worst case if it has expired, it would
    token_str = _get_tokens(dom)
    token_str: str | None = CACHE.get('ahmia-tokens')
    topics = {}
    to_lang = params["to_lang"][2]  # "german"
    to_lang = resp.search_params["to_lang"][1]  # "de"
    traits = EngineTraits(**ENGINE_TRAITS["annas archive"])
    traits: EngineTraits = EngineTraits(**ENGINE_TRAITS["z-library"])
    Transforming "translate.google.co.in<em>Google</em> Translate" into "Google Translate"
    translation = resp.json()
    translations = [res.types.Translations.Item(text=t['text']) for t in data['translations']]
    traversing the text.
    tree = html.fromstring(resp.content)
    trimmed_results = all_results[first_result_index : first_result_index + page_size]
    try:
    Typically, the ban duration is around 15 minutes.
    t_range: str = time_range_dict.get(str(params["time_range"]), "")
    ui_async,
    ui_lang = locales.get_engine_locale(params["searxng_locale"], traits.custom["ui_lang"], "en-us")
    ui_lang = params["searxng_locale"]
    unit = unit_to_str(area.get("unit", ""))
    unquote,
    update_time = time() - (time() % 1800)
    upload.wikimedia.org.  The redirected URL can be calculated by this
    uri = 'file:' + database + '?mode=ro'
    url + "search/text/?query={query}&page={page}&fields=name,url,download,created,description,type&token={api_key}"
    url + 'input/json.jsp'
    url = "https://soundcloud.com"
    url = api_url + f'{qwant_categ}?'
    url = ddg_link_url % params
    url = f"https://login.microsoftonline.com/{t_id}/oauth2/v2.0/token"
    url = f"https://www.senscritique.com{item['url']}"
    url = f"{base_url}/{search_type}?{urlencode(args)}"
    url = result.get('altClickUrl')
    url = thumbnail.get("resizer_url")
    url = url._replace(path="", fragment="").geturl()
    url = urllib.parse.urlparse(resp.search_params["url"])
    url = video_url or f"{base_url}/multimedia/video/{item['sophoraId']}.html"
    URL to fetch images from `artic.edu <https://artic.edu>`__."""
    url: list[str] | str = backend_url or engines["piped"].backend_url  # type: ignore
    url: list[str] | str = base_url or engines["yacy"].base_url  # type: ignore
    url: str
    url: str = landing_page_url or work_url
    urlencode,
    urlparse,
    URLs are merged as one result for SearXNG.
    urls: list[dict[str, str | bool]] = []
    url_params = {
    url_params = {'p': query}
    url_prefix : https://developer.mozilla.org
    url_query : mdn_url
    url_xpath : //article[@class="repo-summary"]//a[@class="repo-link"]/@href
    url_xpath = './/div[contains(@class,"compTitle")]/h3/a/@href'
    uselang: str = "en"
    username: elastic
    user_language = resp.search_params['language']
    use_ac = "use_ac:true"
    valid_types = [
    value = ""
    value = value.split(':', 1)
    value = value.split(';')[0]
    value: str = cache.get(key=key) or ""
    value_token = extract_text(dom.xpath(value_token_xpath))
    video_response(resp, results)
    video_response,
    video_url = streams.get('h264s') or streams.get('h264m') or streams.get('h264l') or streams.get('h264xl')
    volume = biblio.get("volume", "")
    vqd = get_vqd(query=query, params=params) or fetch_vqd(query=query, params=params)
    wd_to_results = {}
    weather_answer = EngineResults.types.WeatherAnswer(
    weather_answer = WeatherAnswer(
    where = list(filter(lambda s: len(s) > 0, [a.get_where() for a in attributes]))
    width: default is *unset* (``0``)
    wikibase:geoLatitude ?{name}Lat ;
    wikibase:geoLongitude ?{name}Long ] }""".replace(
    wikibase:timeCalendarModel ?{name}timeCalendar ] . }
    wikibase:timePrecision ?{name}timePrecision ;
    wikibase:timeTimezone ?{name}timeZone ;
    wikibase:timeValue ?{name} ;
    wikibase_label = list(filter(lambda s: len(s) > 0, [a.get_wikibase_label() for a in attributes]))
    wikidata_ids = []
    wikidata_property_names = []
    wikipedia_link = api_result['content_urls']['desktop']['page']
    wiki_netloc = eng_traits.custom['wiki_netloc'].get(eng_tag, 'en.wikipedia.org')
    with contextlib.closing(sqlite3.connect(uri, uri=True)) as connect:
    with Popen(cmd, stdout=PIPE, stderr=PIPE, env=environment_variables) as process:
    with sqlite_cursor() as cur:
    with _connection.cursor() as cur:
    with _connection:
    work_url: str = item.get("id", "")
    x = res.get('release_date_components')
    xmldom = etree.fromstring(resp.content)
    xmlsearchresult = eval_xpath_getindex(xmldom, '//data', 0)
    year = item.get('yearOfProduction')
    year = journal_year[-1]
    year = _text(item, './/div[contains(@class, "property_year")]//div[contains(@class, "property_value")]')
    zero_click = extract_text(eval_xpath(doc, zero_click_info_xpath)).strip()  # type: ignore
    zero_click_info_xpath = '//div[@id="zero_click_abstract"]'
    [
    ]
    ],
    ]:  # composer
    ]:  # date of spacecraft landing
    ]:  # headquarters location
    ]:  # official language
    _api_key = engine_settings.get("api_key")
    _client = MongoClient(host, **kwargs)[database][collection]
    _connection = mariadb.connect(database=database, user=username, password=password, host=host, port=port)
    _connection = mysql.connector.connect(
    _connection = psycopg2.connect(
    _eng_tag, wiki_netloc = get_wiki_params(params['searxng_locale'], traits)
    _f(engine_traits)
    _fetch_traits(engine_traits, add_domains=False)
    _fmt = "_fmt:prog"
    _my_offline_engine = (
    _my_online_engine = engine_settings.get("name")
    _network.raise_for_httperror(resp)
    _q: list[str] = []
    _result_list = search_res.get("RelatedTopics", [])  # pyright: ignore[reportAssignmentType]
    _result_list: list[dict[str, str]] = search_res.get("Results", [])  # pyright: ignore[reportAssignmentType]
    _search_url = base_url + '/indexes/' + index + '/search'
    _search_url = base_url + '/solr/' + collection + '/select?{params}'
    _valkey_client = valkey.StrictValkey(
    __CACHED_API_URL = api_url
    __CACHED_API_URL = None
    __check_query_params(params)
    __slots__ = 'language', 'kwargs'
    __slots__ = 'url_id', 'url_path_prefix', 'kwargs'
    __slots__ = ('name',)
    __slots__ = ('priority',)
    ``Accept-Language`` HTTP header.  The value in ``engine_traits.all_locale``
    ``params["time_range"]``.
    ``sxng_locale``.  To support LanguageConverter_ this function rates a locale
    {
    }
    }  # equals sign
    } UNION {
    },
   (`format=json`_) from a search query (`list=search`_).  If you need other
   - :origin:`elasticsearch.py <searx/engines/elasticsearch.py>`
   - :origin:`meilisearch.py <searx/engines/meilisearch.py>`
   - :origin:`searx/engines/recoll.py`
   - :origin:`solr.py <searx/engines/solr.py>`
   - name: azure
   - name: chinaso images
   - name: chinaso news
   - name: chinaso videos
   - name: marginalia
   - name: my_database
   - name: openalex
   - name: paddling
   - name: pubmed
   - name: reuters
   - name: semantic scholar
   - name: wikicommons.audio
   - name: wikicommons.files
   - name: wikicommons.images
   - name: wikicommons.videos
   - name: z-library 2010s epub
   - `Elasticsearch <https://www.elastic.co/elasticsearch/>`_
   - `Elasticsearch Guide
   - `Install Elasticsearch
   - `Install MeiliSearch
   - `Install Solr <https://solr.apache.org/guide/installing-solr.html>`_
   - `MeiliSearch <https://www.meilisearch.com>`_
   - `MeiliSearch Documentation <https://docs.meilisearch.com/>`_
   - `Recoll <https://www.lesbonscomptes.com/recoll/>`_
   - `recoll-webui <https://framagit.org/medoc92/recollwebui.git>`_
   - `Solr <https://solr.apache.org>`_
   - `Solr Resources <https://solr.apache.org/resources.html>`_
   :py:obj:`babel` and note that when searching for radio stations, users are
   <https://www.iana.org/assignments/language-subtag-registry/language-subtag-registry>`_ ::
   <https://www.w3.org/International/questions/qa-choosing-language-tags#langsubtag>`_
   Access to music is limited to a few countries: `Countries where Yandex.Music
   Add a `...` between each code fragment before merging them.
   Anna's Archive is a beta release: Filter results by file extension does not
   As long as the problem has not been resolved, these engines are
   At the moment it is little more than an idea together with a proof of concept
   base_url: https://recoll.example.org
   Beside the ``photos`` endpoint_ there are more endpoints available / we are
   Bing News is *different* in some ways!
   By default Presearch's video category is intentionally placed into::
   ChinaSo engine does not return real URL, the links from these search
   Control the highlighting of the matched text (turns off/on).
   dl_prefix: https://download.example.org
   engines violate the privacy of the users!!
   For WEB searches and to determine the ``vqd`` value, DDG-html (no-JS) is
   https://about.marginalia-search.com/
   https://about.marginalia-search.com/article/api/
   https://de1.api.radio-browser.info/#Advanced_station_search
   https://developer.mozilla.org/de/docs/Web/HTTP/Reference/Headers/Referer
   https://developer.mozilla.org/docs/Web/HTTP/Reference/Headers/Referrer-Policy#directives
   https://developer.mozilla.org/en-US/docs/Web/HTTP/Basics_of_HTTP/Data_URIs
   https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Accept-Language
   https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Sec-Fetch-Mode>
   https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/User-Agent
   https://docs.github.com/en/rest/search/search?apiVersion=2022-11-28#search-code--fine-grained-access-tokens
   https://en.wikipedia.org/api/rest_v1/#/Page%20content/get_page_summary__title_
   https://en.wikipedia.org/wiki/Chinese_Wikipedia#Automatic_conversion_between_traditional_and_simplified_Chinese_characters
   https://github.com/ggfevans/searxng/blob/mod-sidecar-harvester/docs/ddg-bot-detection-research.md
   https://github.com/Jackett/Jackett
   https://github.com/Jackett/Jackett/wiki/Jackett-Categories
   https://github.com/Prowlarr/Prowlarr
   https://info.arxiv.org/help/api/user-manual.html#51-details-of-query-construction
   https://meta.wikimedia.org/wiki/Wikipedias_in_multiple_writing_systems
   https://meta.wikimedia.org/wiki/Wikipedias_in_multiple_writing_systems#Chinese
   https://torznab.github.io/spec-1.3-draft/index.html
   https://wallhaven.cc/faq#What-are-SFW-Sketchy-and-NSFW-all-about
   https://wiki.servarr.com/en/prowlarr/cardigann-yml-definition#categories
   https://www.mediawiki.org/wiki/Writing_systems#LanguageConverter
   https://zh.wikipedia.org/wiki/%E5%87%BA%E7%A7%9F%E8%BB%8A
   implementation according to your needs.
   implementation of the web front-end and search technology on a small index.
   In its actual state, this engine is implemented to parse JSON result
   is available`_
   languages.  The search results for languages like Chinese or Arabic are of
   looking forward for contributions implementing more endpoints.
   low quality.
   macrolanguage, i.e. this primary language subtag encompasses a number of more
   more likely to search by name than by region or language.
   mount_prefix: /export/documents
   must be careful to avoid leaking private data.
   Mwmbl_ does not support regions, languages, safe-search or time range.
   not active in a standard setup (``inactive: true``).
   not displayed in the Tagesschau.
   out the line ``auth_request /api/ping/;`` to ``# auth_request /api/ping/;``.
   RadioBrowser has registered a lot of languages and countrycodes unknown to
   really work on Anna's Archive.
   run``, this would be::
   search.
   search_dir: ""
   specific primary language subtags in the registry.  ...  As we recommended for
   Strip any whitespace at the start or end of each code fragment.
   Strip new lines at the start or end of each code fragment.
   TA docker container
   The actual source may contain additional content, such as commentary, that is
   The ADS_ engine requires an :py:obj:`API key <api_key>`.
   the collection subtags mentioned above, in most cases you should try to use
   The CORE engine requires an :py:obj:`API key <api_key>`.
   The language (aka region) support of Brave's index is limited to very basic
   the more specific subtags ... `W3: The primary language subtag
   the official API `[api.tineye.com] <https://api.tineye.com/python/docs/>`_.
   The Springer engine requires an API key, which can be obtained via the
   This engine needs to allow images from the `data URLs`_ (prefixed with the
   This engine only search for **live streams**!
   This SearXNG engine only supports *'searching by URL'* and it does not use
   Use macrolanguages with care.  Some language subtags have a Scope field set to
   used.
   We try to find a solution for this problem, please follow `issue #4694`_.
   When creating and enabling a ``command`` engine on a public instance, you
   With ``docker compose``, this would be::
   `Springer subscription`_.
   ``action`` and ``list`` types ask SearXNG developers to extend the
   ``data:`` scheme)::
  # MongoDB engine
  # Required dependency: pymongo
  # Required dependency: valkey
  %WHERE%
  (concepts), PDF/HTML links, pages, volume, issue, published date, and a short
  (no news, videos etc).  DDG-lite and DDG-html can be used by clients that do
  (reconstructed from inverted index), journal/venue, publisher, DOI, tags
  (`https://zh.wikipedia.org/wiki/出租車`_) but if your browser's
  ) {
  - name : bitbucket
  - name : mdn
  - name: adobe stock
  - name: adobe stock video
  - name: annas articles
  - name: ansa
  - name: astrophysics data system
  - name: brave
  - name: brave.goggles
  - name: brave.images
  - name: brave.news
  - name: brave.videos
  - name: braveapi
  - name: bt4g.movie
  - name: core.ac.uk
  - name: elasticsearch
  - name: find
  - name: forgejo.com
  - name: gitea.com
  - name: github code
  - name: gitlab
  - name: gnome
  - name: google scholar
  - name: huggingface
  - name: huggingface datasets
  - name: huggingface spaces
  - name: il post
  - name: lemmy comments
  - name: lemmy communities
  - name: lemmy posts
  - name: lemmy users
  - name: mediathekview
  - name: meilisearch
  - name: microsoft learn
  - name: MRS
  - name: my offline engine
  - name: my online engine
  - name: mymongo
  - name: myvalkey
  - name: openlibrary
  - name: piped
  - name: piped.music
  - name: presearch
  - name: presearch images
  - name: presearch news
  - name: presearch videos
  - name: qwant
  - name: qwant images
  - name: qwant news
  - name: qwant videos
  - name: repology
  - name: solr
  - name: sourcehut
  - name: springer nature
  - name: startpage
  - name: tubearchivist
  - name: yacy
  - name: yacy images
  - You can test the API here: https://reqbin.com/gesg2kvx
  - `builtwith.com Discourse <https://trends.builtwith.com/websitelist/Discourse>`_
  .. code:: yaml
  /collections/the-history-of-four-footed-beasts-and-serpents-1658/
  /etc/nginx/sites-available/default
  /shop/nov-2023-prints-00043.jpg
  100/110/111 <-- Bits stands for: SFW, Sketchy and NSFW
  8616383182_5740fa7851_o.jpg
  :menuselection:`Settings --> User --> Admin Interface`.
  :py:obj:`supports safe-search <safe_search_support>`.  The ``{safe_search}``
  :py:obj:`time_range_map`.
  :py:obj:`URL parameter <time_range_url>` if engine :py:obj:`supports time
  ?fit=clip&w=310&h=800&auto=format,compress
  ?fit=clip&w=970&h=800&auto=format,compress
  ?fit=max&h=360&w=360
  A comma separated list of the elements of the command.  A special token
  A dict containing the regular expressions for each result key.
  A list containing allowed search terms if ``query_type`` is set to ``enum``.
  A mapping containing a delimiter ``char`` and the *titles* of each element in
  about 1 hour without requests from this IP)
  access link is available, it is exposed via the ``PDF`` and/or ``HTML`` links
  API through which additional content for a query can be requested (vqd
  apps.
  assumption).
  be loaded for the query while scrolling).
  Change the authentication method used when using the API, defaults to none.
  checked by DDG's bot blocker.
  citations comment
  comes from the region the user selected.  For instance ``:de-AU`` will filter
  DDG query in a real web browser over the blocked IP (at least that's my
  DDG-Lite and DDG-HTML TRY to guess user's preferred language from the HTTP
  DDG-lite works a bit differently: here, ``d.js`` is not an endpoint but a
  different to the UI language) and a region filter.
  does not exist in the process flows provided by DDG (and is a clear indication
  en-GB_GB --> en_GB
  enabled.
  engine adds ``filter=language:<iso2>`` (e.g. ``language:fr``). If OpenAlex has
  Ensure the selected language has sufficient coverage at OpenAlex, or set the
  executed on a JS-capable client.
  few results for that language, you may see fewer items.
  field (``api=d.js``) in a form that is sent to DDG-lite.
  filter category after a ``-``.
  Filter stations by selected country.  The 2-digit countrycode of the station
  Filter stations by selected language.  For instance the ``de`` from ``:de-AU``
  fil_PH --> fil_PH
  header today
  headers (such as curl) are filtered.
  hint:Prior hint:runFirst "true".
  hint:Query hint:optimizer "None".
  How exactly the blocking mechanism currently works is not fully known, and
  https://consent.google.com/m?continue=
  If not supported, the URL parameter is an empty string.
  if you are querying a Torznab endpoint without authentication or if the
  In :py:obj:`fetch_traits` we use::
  In the past, Sec-Fetch-Mode had to be set to 'navigate', otherwise there were
  in the result footer.
  instance is private.  Be aware that private trackers may ban you if you share
  instance settings so signed cookies work; see :ref:`settings server`.
  IP when ``Accept-Language`` HTTP header is unset.
  ISO 639-1 language code (en, de, fr ..)
  ISO 639-1 language code (en, de, fr ..) of the search language.
  language).
  Make sure your ``server.secret_key`` and ``server.base_url`` are set in your
  not support JS, aka *no-JS*.
  nov-2023-prints-00043.jpg
  of a bot).
  only required when additional pages are accessed (or when new content needs to
  OpenAlex is a DOI resolver, the engine will use that stable link. When an open
  OPTIONAL {
  OPTIONAL { ?item wdt:P1766|wdt:P8505|wdt:P8667 ?sign }
  OPTIONAL { ?item wdt:P18|wdt:P8517|wdt:P4291|wdt:P5252|wdt:P3451|wdt:P4640|wdt:P5775|wdt:P2716|wdt:P1801|wdt:P4896 ?image }
  OPTIONAL { ?item wdt:P41|wdt:P94|wdt:P154|wdt:P158|wdt:P2910|wdt:P4004|wdt:P5962|wdt:P8972 ?symbol }
  OPTIONAL { ?item wdt:P856 ?website }
  or .. Wikipedia's LC automatically returns the desired script in their
  or to the video category)
  out all stations not in ``AU``.
  Page number if engine supports paging :py:obj:`paging`
  past, the IP blocking was implemented as a 'sliding window' (unblock after
  Peer-to-Peer search
  plain text and HTML as well as the type of page. This is useful for page
  previews (fka. Hovercards, aka. Popups) on the web and link previews in the
  Privacy or Stealth mode, restricts the search to local yacy instance.
  problems with the bot blocker.. I don't know if DDG still evaluates this
  protection and is used by all request to DDG.
  Prowlarr-categories_ or Jackett-categories_ for more information.
  pt-BR_BR --> pt_BR
  range <time_range_support>`.  The value for the parameter is taken from
  replacement is taken from the :py:obj:`safes_search_map`.  Filter results::
  requesting the first result page.
  required)
  results for a query term are in the first response.
  Safe-search :py:obj:`URL parameter <safe_search_map>` if engine
  Search terms from user.
  search trending items by the provided filters, which are appended to the
  searchProductExplorer(
  second page is requested, this field is not set!
  SERVICE wikibase:label {
  SERVICE wikibase:mwapi {
  services when the category is changed (e.g., from web search to news, images,
  services.
  Startpage tries to guess user's language and territory from the HTTP
  Supported categories are ``web``, ``news`` and ``images``.
  The API key to use for authentication.
  The categories to use for searching. This is a list of category IDs.  See
  The character that separates results. Default: ``\\n``.
  The desired language variant code for wikis where LanguageConverter_ is
  The directory where the command has to be executed.  Default: ``./``.
  The displayed name in Startpage's settings page depend on the location of the
  The expected type of user search terms.  Possible values: ``path`` and
  The HTTP User-Agent is also involved in the formation of the vqd value, read
  The HTTP User-Agent_ (see below) is generated by the WEB-client and are
  The service at https://lite.duckduckgo.com/lite offers general WEB searches
  The summary response includes an extract of the first paragraph of the page in
  the torrent file.  Defaults to ``false``.
  The Wikipedia link returned by the API is still the same in all cases
  The ``*.js`` endpoints return a JSON response and can therefore only be
  the-history-of-four-footed-beasts-and-serpents-1658/8616383182_5740fa7851_o.jpg
  there were also changes to the bot blocker in the period of Q3/Q4 2025: in the
  this with Prowlarr_ or Jackett_ leaks the API key.  This should be used only
  to get uniform names independent from the IP).
  To sort by price in ascending order.
  To sort by price in descending order.
  Torznab endpoint URL.
  trackers may ban you if you share the magnet link.  Defaults to ``true``.
  typically empty string.  Opposite of ``b``; this field is not set when
  UI language to English and retry.
  values ?item { %WIKIDATA_IDS% }
  web-page.
  When *time_range* is activate, the results always ordered by ``time``.
  whether the UA is a known header. However, it is possible that certain UA
  Whether to show the magnet link in the search results.  Be aware that private
  Whether to show the torrent file in the search results.  Be careful as using
  will be translated to `german` and used in the argument ``language=``.
  zh-CN_CN --> zh_Hans_CN
  zh-TW_HK --> zh_Hant_HK
  zh-TW_TW --> zh_Hant_TW
  `DuckDuckGo Bot Detection Research & Solution`_.  However, it is not checked
  ``Accept-Language`` is set to any of ``zh``, ``zh-CN``, ``zh-TW``, ``zh-HK``
  ``Accept-Language``.  Optional the user can select a region filter (but not a
  ``Accept-Language``.  Optional the user can select a search-language (can be
  ``enum``.
  ``enum``:
  ``jahr``, ``jahrzent``, ``land``, ``online``, ``stimmung`` will be used to
  ``keys``.
  ``path``:
  ``vqd``, as this would lead to an immediate blocking, since such a use-case
  ``{{QUERY}}`` tells where to put the search terms of the user. Example:
  }
"""
""".. sidebar:: info
"""1337x"""
"""1x (Images)"""
"""200 pages maximum (``&first=1991``)"""
"""360Search search engine for searxng"""
"""360Search-Videos: A search engine for retrieving videos from 360Search."""
"""5 pages maximum (``&p=5``): Trying to do more just results in an improper
"""9GAG (social media)"""
""":py:obj:`searx.search.processors.online_url_search`"""
"""A list of display types composed from ``infobox`` and ``list``.  The latter
"""A list of filters to be applied to the search of radio stations.  By default
"""A list of of content types.  The following content types are offered:
"""A string '4' means *last hour*.  We use *last hour* for ``day`` here since the
"""Acfun search engine for searxng"""
"""An alternative privacy-friendly YouTube frontend which is efficient by
"""An enhanced endpoint with additional metadata fields and optimized queries
"""Anan's search form field **Content** / possible values::
"""Any of ``Communities``, ``Users``, ``Posts``, ``Comments``"""
"""API key for Brave Search API (required)."""
"""API key of the Discourse forum."""
"""API username of the Discourse forum."""
"""APKMirror"""
"""Apple App Store"""
"""Apple Maps"""
"""ARD: `Tagesschau API`_
"""Artstation (images)"""
"""arXiv is a free distribution service and an open-access archive for nearly
"""Ask.com has at max 5 pages."""
"""Ask.com"""
"""Baidu_
"""Bandcamp (Music)
"""BASE (Scholar publications)"""
"""Base URL for the Brave Search API."""
"""Base URL of the GitLab host."""
"""Base URL of the Peertube instance.  A list of instances is available at:
"""Base URL of the Tube Archivist instance.  Fill this in with your own
"""Base URL of the Wikimedia wiki.
"""Bilibili is a Chinese video sharing website.
"""Bing (Images) search URL"""
"""Bing (News) search URL"""
"""Bing (Videos) async search URL."""
"""Bing (Web) search URL"""
"""Bing results are always SFW.  To get NSFW links from bing some age
"""Bing-Images: description see :py:obj:`searx.engines.bing`."""
"""Bing-News: description see :py:obj:`searx.engines.bing`.
"""Bing-Videos: description see :py:obj:`searx.engines.bing`."""
"""bitchute (Videos)"""
"""BPB refers to ``Bundeszentrale für poltische Bildung``, which is a German
"""Brave only supports paging in :py:obj:`brave_category` ``search`` (UI
"""Brave only supports time-range in :py:obj:`brave_category` ``search`` (UI
"""Brave supports common web-search, videos, images, news, and goggles search.
"""Brave supports some kind of spell checking.  When activated, Brave tries to
"""Brave supports the categories listed in :py:obj:`brave_category` (General,
"""Browse one of the largest collections of copyleft icons
"""Browse public projects."""
"""BT$G offers categories: ``all`` (default), ``audio``, ``movie``, ``doc``,
"""BT4G_ (bt4g.com) is not a tracker and doesn't store any content and only
"""By default, https://lemmy.ml is used for providing the results.  If you want
"""By default, the family filter is turned on. Setting this parameter to
"""Cache to store the secret API key for the engine."""
"""CachyOS (packages, it)"""
"""Cargo search on crates.io"""
"""Change the method of authenticating to the github API.
"""Chefkoch is a German database of recipes."""
"""ChinaSo supports news, videos, images search.
"""ChinaSo_, a search engine for the chinese language area.
"""Cloudflare AI engine"""
"""Configure ChinaSo category (:py:obj:`ChinasoCategoryType`)."""
"""Configure ChinaSo-News type (:py:obj:`ChinasoNewsSourceType`)."""
"""Content filter ``music_songs`` or ``videos``"""
"""CORE_ (COnnecting REpositories) provides a comprehensive bibliographic
"""Crossref_ is the sustainable source of community-owned scholarly metadata and
"""Currency convert (DuckDuckGo)"""
"""DDG-lite: user can't select but the results are filtered."""
"""Deepl translation engine"""
"""Default architecture to search for.  For valid values see :py:obj:`ARCH_RE`"""
"""DeStatis"""
"""Deviantart (Images)"""
"""Devicons (icons)"""
"""Docker Hub (IT)"""
"""Duden"""
"""Dummy Offline"""
"""Dummy"""
"""Each file possibly consists of more than one code block that matches the
"""Emojipedia
"""Enable internal query rewriting (Type: boolean).  Some search backends can
"""Engine for Ansa, Italy's oldest news agency.
"""Engine for Azure resources.  This engine mimics the standard search bar in Azure
"""Engine for Il Post, a largely independent online Italian newspaper.
"""Engine for Microsoft Learn, Microsoft's technical knowledge base.
"""Engine to search in collaborative software platforms based on Gitea_ or Forgejo_.
"""Engine to search in collaborative software platforms based on GitLab_ with
"""Engine to search in the collaborative software platform SourceHut_.
"""Engine to search using the Brave (WEB) Search API.
"""Filename of the SQLite DB."""
"""Filter Anna's results by a file ending.  Common filters for example are
"""Filter z-library's results by a file ending. Common filters for example are
"""Filter z-library's results by year from. E.g '2010'.
"""Filter z-library's results by year to. E.g. '2010'.
"""Filtering ChinaSo-News results by source:
"""FindThatMeme (Images)"""
"""Flickr (Images)"""
"""For an API key register at https://core.ac.uk/services/api and insert
"""Fyyd (podcasts)"""
"""Geizhals is a German website to compare the price of a product on the
"""Genius"""
"""Get an API token as described in https://ui.adsabs.harvard.edu/help/api"""
"""Github (IT)"""
"""GitHub code search with `search syntax`_ as described in `Constructing a
"""Goodreads (books)"""
"""Google Play Apps & Google Play Movies"""
"""Google Scholar is a freely accessible web search engine that indexes the full
"""Grokipedia (general)"""
"""Hackernews"""
"""hex.pm"""
"""Highlight the matching code lines."""
"""Hostname of the DB connector"""
"""How many records to return for the ADS request."""
"""HTTP digest password for the local YACY instance"""
"""HTTP digest user for the local YACY instance"""
"""Hugging Face supports datasets, models, spaces as search endpoint.
"""If go through the pages and there are actually no new results for another
"""If you own an API key you can add it here, further read `Rate Limiting and
"""IMDB - Internet Movie Database
"""Imgur (images)"""
"""Invidious (Videos)
"""Ipernity (images)"""
"""iQiyi: A search engine for retrieving videos from iQiyi."""
"""Kernel architecture: ``x86_64``, ``x86``, ``aarch64``, ``armhf``,
"""Key used for the Meta-API_.  Get your API key from: `Springer subscription`_"""
"""Kickass Torrent (Videos, Music, Files)"""
"""lib.rs (packages)"""
"""Library of Congress: query Photo, Print and Drawing from API endpoint_
"""LibreTranslate (Free and Open Source Machine Translation API)"""
"""Lingva (alternative Google Translate frontend)"""
"""List of Anna's archive domains or a single domain (as string)."""
"""List of region/language combinations supported by Google News.  Values of the
"""LiveSpace (Videos)
"""Location where recoll-webui can be reached."""
"""Location where the file hierarchy as indexed by recoll can be reached."""
"""Location where the file hierarchy is mounted on your *local* filesystem."""
"""Map language to domain"""
"""Map regions to domain"""
"""Mapping of SearXNG time ranges to Brave API time ranges."""
"""Mapping rule of the LanguageConverter_ to map a language and its variants to
"""MariaDB is a community driven fork of MySQL. Before enabling MariaDB engine,
"""Mastodon_ is an open source alternative to large social media platforms like
"""Material Icons (icons)"""
"""Matrix Rooms Search - a fully-featured, standalone, matrix rooms search service.
"""Maximum number of results per page (default 10)."""
"""Maximum number of results per page (default 20)."""
"""media.ccc.de"""
"""MediathekViewWeb (API)"""
"""metacpan"""
"""Mixcloud (Music)"""
"""Mojeek (general, images, news)"""
"""MongoDB_ is a document based database program that handles JSON like data.
"""Moviepilot is a German movie database, similar to IMDB or TMDB.  It doesn't
"""Mozhi (alternative frontend for popular translation engines)"""
"""must be any of ``search``, ``images``, ``videos``, ``news``"""
"""Mwmbl_ is a non-profit, ad-free, free-libre and free-lunch search engine with
"""MyMemory Translated"""
"""MySQL is said to be the most popular open source database.  Before enabling
"""Name of the database."""
"""National Vulnerability Database (it)"""
"""Naver for SearXNG"""
"""Naver supports general, images, news, videos search.
"""Niconico search engine for searxng"""
"""npms.io"""
"""Number of results to return in the request, see `Pagination and Limits`_ for
"""Nyaa.si (Anime Bittorrent tracker)"""
"""Odysee_ is a decentralized video hosting platform.
"""Ollama model search engine for searxng"""
"""One of ``text``, ``image`` / The search-types ``app``, ``audio`` and
"""One of ``web-lite`` (or ``web``), ``news``, ``images`` or ``videos``"""
"""Open Meteo (weather)"""
"""Open Semantic Search"""
"""OpenClipArt (images)"""
"""OpenStreetMap (Map)"""
"""Optional, if true SearXNG will link directly to the mp4 of the video to play
"""Order method, valid values are: ``latest``, ``likes``, ``views``, ``latest_topic``"""
"""Part of the indexed file hierarchy to be search, if empty the full domain is
"""Password for the DB connection."""
"""Peertube and :py:obj:`SepiaSearch <searx.engines.sepiasearch>` do share
"""Persistent (SQLite) key/value cache that deletes its values after ``expire``
"""Pexels (images)"""
"""Pinterest (images)"""
"""Piped-Backend_: The core component behind Piped.  The value is an URL or a
"""Piped-Frontend_: URL to use as link and for embeds.
"""Pixabay provides royalty-free media (images, videos)"""
"""Pixiv (images)"""
"""pkg.go.dev (packages)"""
"""Podcast Index"""
"""Port of the DB connector"""
"""PostgreSQL is a powerful and robust open source database.  Before configuring
"""Presearch supports the search types listed in :py:obj:`search_type` (general,
"""Public domain image archive"""
"""PubMed_ comprises more than 39 million citations for biomedical literature
"""pypi.org"""
"""Quark (Shenma) search engine for searxng"""
"""Reddit"""
"""Regular expression that match a architecture in the query string."""
"""Regular expression to match supported architectures in the query string."""
"""Repology_ monitors a huge number of package repositories and other sources
"""Result list can be ordered by ``relevance`` (default), ``size``, ``seeders``
"""Returns a list of all works (journal articles, conference proceedings, books,
"""Reuters_ (news) is an international news agency.
"""RottenTomatoes (movies)"""
"""Rumble (Videos)"""
"""Search fields, for more details see, `Details of Query Construction`_.
"""Search radio stations from RadioBrowser by `Advanced station search API`_.
"""Searchcode (IT)"""
"""SearXNG engine for `Void Linux binary packages`_.  Void is a general purpose
"""selfh.st/icons - A collection of logos for self-hosted dashboards and
"""SensCritique (movies)"""
"""SepiaSearch uses the same languages as :py:obj:`Peertube
"""Set of fields to return in the response from ADS."""
"""Set the sort order of returned results.  One of the following values:
"""Seznam"""
"""Show avatar of the user who send the post."""
"""Sogou search engine for searxng"""
"""Sogou-Images: A search engine for retrieving images from Sogou."""
"""Sogou-Videos: A search engine for retrieving videos from Sogou."""
"""Sogou-WeChat search engine for retrieving WeChat Article from Sogou"""
"""SolidTorrents"""
"""Sort Anna's results, possible values::
"""Sort criteria, possible values:
"""Sort order, can be one of:
"""Sort order, one of ``relevance``, ``display_date:desc`` or ``display_data:asc``."""
"""Sort order, possible values:
"""SoundCloud is a German audio streaming service."""
"""Spotify (Music)"""
"""SQL query that returns the result items."""
"""SQLite is a small, fast and reliable SQL database engine.  It does not require
"""Stack Exchange API v2.3
"""Startpage's category, visit :ref:`startpage categories`.
"""Startpage's language & region selectors are a mess ..
"""Steam (store) for SearXNG."""
"""Stores cookies from 360search to avoid re-fetching them on every request."""
"""Stract is an independent open source search engine.  At this state, it's
"""Strip all leading and trailing whitespace for each returned fragment.
"""Strip leading and trailing newlines for each returned fragment.
"""Svgrepo (icons)"""
"""Tested 18 pages maximum (argument ``page``), to be save max is set to 20."""
"""Tested 9 pages maximum (``&offset=8``), to be save max is set to 10.  Trying
"""The *editing depth* of Wikipedia is one of several possible rough indicators
"""The API key to use for Authorization_ header.  Can be found under:
"""The Art Institute of Chicago
"""The Astrophysics Data System (ADS_) is a digital library portal for
"""The category must be any of ``images``, ``videos`` and ``news``
"""The engine uses the API at the endpoint search.json_.
"""The format is 'field' + 'direction' where direction is one of 'asc' or 'desc'
"""The JSON engine is a *generic* engine with which it is possible to configure
"""The longhand version of MediaWiki time strings."""
"""The MediaWiki engine is a *generic* engine to **query** Wikimedia wikis by
"""The OpenAlex engine integrates the `OpenAlex`_ Works API to return scientific
"""The path the PHP api is listening on.
"""The path the `project API <https://docs.gitlab.com/ee/api/projects.html>`_.
"""The process flow for determining the ``vqd`` values was implemented for the
"""The result type can be :py:obj:`MainResult` or :py:obj:`KeyValue`."""
"""The sort order of the results.  Possible values:
"""The value is an URL or a list of URLs.  In the latter case instance will be
"""The version of the GitHub REST API.
"""The XPath engine is a *generic* engine with which it is possible to configure
"""This engine implements *Tineye - reverse image search*
"""This engine uses the Lemmy API (https://lemmy.ml/api/v3/search), which is
"""This engine uses the Qwant API (https://api.qwant.com/v3) to implement Qwant
"""This is not the official (developer) url, it is the API which is used by the
"""This is the implementation of the Bing-WEB engine. Some of this
"""This is the implementation of the Google Images engine using the internal
"""This is the implementation of the Google News engine.
"""This is the implementation of the Google Videos engine.
"""This is the implementation of the Google WEB engine.  Some of this
"""This module implements the Wikidata engine.  Some implementations are shared
"""This module implements the Wikipedia engine.  Some of this implementations
"""This should be a URL ending in ``.goggle``"""
"""Time for which an auth token is valid (sec.)"""
"""Time in seconds the sc-code is cached in memory :py:obj:`get_sc_code`."""
"""TinEye error message"""
"""To get an API key, please follow the instructions from `Key and license`_
"""Tokyo Toshokan (A BitTorrent Library for Japanese Media)"""
"""Tootfinder (social media)"""
"""Torznab_ is an API specification that provides a standardized way to query
"""True if this video is "Created for Kids" / intends to target an audience
"""Turn purities on(1) or off(0) NSFW requires a valid API key.
"""Unsplash"""
"""URL of Qwant's API (JSON)"""
"""URL of Qwant-Lite (HTML)"""
"""URL of the Discourse forum."""
"""URL of the Gitea_ instance."""
"""URL path of the `search endpoint`_.
"""URL template to embed video in SearXNG's result list."""
"""URL to retrieve a list of videos.
"""Username for the DB connection."""
"""UXwing (images)"""
"""Valkey is an open source (BSD licensed), in-memory data structure (key value
"""Wallhaven_ is a site created by and for people who like wallpapers.
"""When set to false, display URLs from Tagesschau, and not the actual source
"""Which properties to return.
"""Which type of search to perform.  One of the following values: ``nearmatch``,
"""With *command engines* administrators can run engines to integrate arbitrary
"""Within this module we implement a *demo offline engine*.  Do not look to
"""Within this module we implement a *demo online engine*.  Do not look to
"""Wolfram|Alpha (Science)"""
"""Wordnik (general)"""
"""wttr.in (weather forecast service)"""
"""XPath of Startpage's origin search form
"""Yacy search mode ``global`` or ``local``.  By default, Yacy operates in ``global``
"""YaCy_ is a free distributed search engine, built on the principles of
"""Yahoo (News)
"""Yahoo Search (Web)
"""Yandex (Web, images)"""
"""Yandex Music
"""Yep (general, images, news)"""
"""Youtube (Videos)"""
"""`Adobe Stock`_ is a service that gives access to millions of royalty-free
"""`Alpine Linux binary packages`_.  `Alpine Linux`_ is a Linux-based operation
"""`Anna's Archive`_ is a free non-profit online shadow library metasearch
"""`arXiv API`_ URL, for more details see Query-Interface_
"""`Fields selection`_, by default, a few fields are returned. To request more
"""`Google max 50 pages`_
"""`Hugging Face`_ search engine for SearXNG.
"""`List of all wikipedias <https://meta.wikimedia.org/wiki/List_of_Wikipedias>`_
"""`Marginalia Search`_ is an independent open source Internet search engine
"""`Open Library`_ is an open, editable library catalog, building towards a web
"""`Semantic Scholar`_ provides free, AI-driven search and discovery tools, and
"""`Springer Nature`_ is a global publisher dedicated to providing service to
"""`Tube Archivist`_ - *Your self hosted YouTube media server.*
"""`Wikimedia Commons`_ is a collection of more than 120 millions freely usable
"""`Z-Library`_ (abbreviated as z-lib, formerly BookFinder) is a shadow library
"Hace N time" (Spanish). Because of this, the dates are simply joined together
#
#  safesearch : results are identical for safesearch=0 and safesearch=2
# '&maximumRecords={limit}'
# '&startRecord={offset}'
# 'mapillary': P1947
# * https://github.com/blazegraph/database/wiki/QueryHints
# * https://www.wikidata.org/wiki/Wikidata:SPARQL_query_service/query_optimization
# ------------------------
# ...6T+9Nl4cnD+gr9OK8I56/tX3l86nWYw//2Q==26;
# =26;[3,"dimg_ZNMiZPCqE4apxc8P3a2tuAQ_137"]a87;data:image/jpeg;base64,/9j/4AAQSkZJRgABA
# about
# adapted from https://gist.github.com/mikesprague/048a93b832e2862050356ca233ef4dc1
# adapted from https://github.com/chubin/wttr.in/blob/master/lib/constants.py
# ads xpath //div[@id="results"]/div[@id="sponsored"]//div[@class="result"]
# Assistant messages hint to the AI about the desired output format. Not all models support this role.
# Base URL
# base url for results
# base_url can be overwritten by a list of URLs in the settings.yml
# but https://github.com/kartaview/openstreetcam.org/issues/60
# but https://taginfo.openstreetmap.org/keys/kartaview ...
# but we don't use it here (may we can learn from).
# Cache keys & expiration
# default trackers provided by thepiratebay
# Default values, should be overridden in settings.yml
# defined in settings.yml
# do search-request
# Do we need support for "free_collection" and "include_stock_enterprise"?
# Doku is OpenSearch compatible
# engine config
# Engine Configuration
# engine dependent config
# Engine metadata
# engine settings
# engine specific variables
# engines: - name: openalex; engine: openalex; mailto: "[email protected]"
# example (Jackett): "http://localhost:9117/api/v2.0/indexers/all/results/torznab"
# False here.
# FileType = t.Literal["bitmap", "drawing", "video", "audio", "multimedia", "office", "archive", "3d"]
# FILE_TYPES = list(t.get_args(FileType))
# Filter results. 0: None, 1: Moderate, 2: Strict
# fmt: off
# fmt: on
# Format the video duration
# from the links not the links itself.
# get response from search request
# get response from search-request
# get supported languages from their site
# Get the calendar names and the property names
# Google-News results are always *SafeSearch*. Option 'safesearch' is set to
# hard coded here to avoid to an additional SPARQL request when the server starts
# have the desired local TLD.
# HTTP2 requests immediately get blocked by a CAPTCHA
# https://api.stackexchange.com/docs/advanced-search
# https://docs.github.com/en/rest/search/search?apiVersion=2022-11-28#text-match-metadata
# https://en.wikibooks.org/wiki/SPARQL/WIKIDATA_Precision,_Units_and_Coordinates
# https://github.com/searxng/searxng/pull/2857#issuecomment-1741713999
# https://lists.w3.org/Archives/Public/public-rdf-dawg/2011OctDec/0175.html
# https://newznab.readthedocs.io/en/latest/misc/api/#predefined-categories
# https://nrkno.github.io/yr-weather-symbols/
# https://open-meteo.com/en/docs#weather_variable_documentation
# https://www.ansa.it/ricerca/ansait/search.shtml?start=0&any=houthi&periodo=&sort=data%3Adesc
# https://www.mediawiki.org/wiki/Wikibase/Indexing/RDF_Dump_Format#Data_model
# https://www.w3.org/TR/sparql11-query/#rSTRING_LITERAL1
# if there is a need for globals, use a leading underline
# key value that are link: mapping functions
# link to preview image of structure
# list of supported languages
# Metadata
# mongodb connection variables
# Naver cannot set the number of results on one page, set default value for paging
# Nonstandard language codes
# not ads: div[@class="result"] are the direct children of div[@id="results"]
# optimization:
# Optional: include your email for OpenAlex polite pool. Can be set from settings.yml
# paging = True
# paging is broken in searchcode.com's API .. not sure it will ever been fixed
# piratebay specific type-definitions
# pods to display as image in infobox
# Possible categories: sfw, sketchy, nsfw
# pylint: disable-next=line-too-long
# pylint: disable=fixme
# pylint: disable=global-statement
# pylint: disable=invalid-name
# pylint: disable=line-too-long
# pylint: disable=missing-class-docstring
# pylint: disable=too-many-branches, invalid-name
# pylint: disable=too-many-statements
# pylint: disable=use-dict-literal
# replace private user area characters to make text legible
# safe_search_map = {0: '&safesearch=0', 1: '&safesearch=1', 2: '&safesearch=2'}
# search request
# search url
# Search URL (Note: lighthouse.lbry.com/search works too, and may be faster at times)
# search-url
# see the property "dummy value" of https://www.wikidata.org/wiki/Q2013 (Wikidata)
# send_accept_language_header = False
# SERVICE wikibase:label: https://en.wikibooks.org/wiki/SPARQL/SERVICE_-_Label#Manual_Label_SERVICE
# SERVICE wikibase:mwapi : https://www.mediawiki.org/wiki/Wikidata_Query_Service/User_Manual/MWAPI
# Set base_url in settings.yml in order to
# shortcuts for advanced search
# SPARQL
# SPDX-License-Identifier: AGPL-3.0-or-later
# SPDX-License-Identifier: AGPL-3.0-or-lat_er
# specific xpath variables
# standard (which is how wiki subdomains are chosen nowadays).
# status codes of unpublished entries
# Suggestions are links placed in a *card-section*, we extract only the text
# suggestion_url = "https://sg.media-imdb.com/suggestion/{letter}/{query}.json"
# Supported languages
# System messages define the AI's personality. You can use them to set rules and how you expect the AI to behave.
# the key seems to be constant
# There exits a https://github.com/ohblue/baidu-serp-api/
# These Wikipedias use language codes that do not conform to the ISO 639
# this pods do return a plaintext, but they look better and are more useful as images
# url for api query
# using http2 returns forbidden errors
# Valid values: name inserted_at updated_at total_downloads recent_downloads
# valkey connection variables
# xpath queries
# xpath variables
# xpaths
## 1337x
## 360search
## 360search_videos
## 9gag
## acfun
## adobe_stock
## ahmia
## alpinelinux
## annas_archive
## ansa
## apkmirror
## apple_app_store
## apple_maps
## archlinux
## artic
## artstation
## arxiv
## ask
## astrophysics_data_system
## azure
## baidu
## bandcamp
## base
## bilibili
## bing
## bing_images
## bing_news
## bing_videos
## bitchute
## bpb
## brave
## braveapi
## bt4g
## btdigg
## cachy_os
## ccc_media
## chefkoch
## chinaso
## cloudflareai
## command
## core
## crates
## crossref
## currency_convert
## dailymotion
## deepl
## deezer
## demo_offline
## demo_online
## destatis
## deviantart
## devicons
## dictzone
## digbt
## discourse
## docker_hub
## doku
## duckduckgo
## duckduckgo_definitions
## duckduckgo_extra
## duckduckgo_weather
## duden
## dummy
## dummy-offline
## ebay
## elasticsearch
## emojipedia
## fdroid
## findthatmeme
## flickr
## flickr_noapi
## freesound
## frinkiac
## fyyd
## geizhals
## genius
## gitea
## github
## github_code
## gitlab
## goodreads
## google
## google_images
## google_news
## google_play
## google_scholar
## google_videos
## grokipedia
## hackernews
## hex
## huggingface
## il_post
## imdb
## imgur
## ina
## invidious
## ipernity
## iqiyi
## jisho
## json_engine
## kickass
## lemmy
## libretranslate
## lib_rs
## lingva
## livespace
## loc
## lucide
## marginalia
## mariadb_server
## mastodon
## material_icons
## mediathekviewweb
## mediawiki
## meilisearch
## metacpan
## microsoft_learn
## mixcloud
## mojeek
## mongodb
## moviepilot
## mozhi
## mrs
## mwmbl
## mysql_server
## naver
## niconico
## npm
## nvd
## nyaa
## odysee
## ollama
## openalex
## openclipart
## openlibrary
## opensemantic
## openstreetmap
## openverse
## open_meteo
## pdbe
## peertube
## pexels
## photon
## pinterest
## piped
## piratebay
## pixabay
## pixiv
## pkg_go_dev
## podcastindex
## postgresql
## presearch
## public_domain_image_archive
## pubmed
## pypi
## quark
## qwant
## radio_browser
## recoll
## reddit
## repology
## reuters
## rottentomatoes
## rumble
## scanr_structures
## searchcode_code
## searx_engine
## selfhst
## semantic_scholar
## senscritique
## sepiasearch
## seznam
## sogou
## sogou_images
## sogou_videos
## sogou_wechat
## solidtorrents
## solr
## soundcloud
## sourcehut
## spotify
## springer
## sqlite
## stackexchange
## startpage
## steam
## stract
## svgrepo
## tagesschau
## tineye
## tokyotoshokan
## tootfinder
## torznab
## translated
## tubearchivist
## unsplash
## uxwing
## valkey_server
## vimeo
## voidlinux
## wallhaven
## wikicommons
## wikidata
## wikipedia
## wolframalpha_api
## wolframalpha_noapi
## wordnik
## wttr
## www1x
## xpath
## yacy
## yahoo
## yahoo_news
## yandex
## yandex_music
## yep
## youtube_api
## youtube_noapi
## zlibrary
## 📊 Statistics
## 📑 Table of Contents
### Source Code
#. Copy the contents of the file ``/etc/nginx/sites-available/default`` in the
#. Edit ``location /cache/videos`` and ``location /cache/channels``.  Comment
#. Mount this new configuration over the default configuration.  With ``docker
#. Perform any backups of TA before editing core configurations.
#. Recoll indexes a local filesystem mounted in ``/export/documents/reference``,
#. Save the file to wherever you normally store your docker configuration.
#. Start the TA container.
#. the contents of this filesystem can be reached though https://download.example.org/reference
#. the Recoll search interface can be reached at https://recoll.example.org/ and
'''
'''Engine supports paging [True or False].'''
'''Engine supports safe-search.'''
'''Engine supports search time range.'''
'''Extract text from a HTML content string'''
'''Extract text from a HTML title string'''
'''JSON query for the list of result items.
'''JSON query of result's ``content``. For the query string documentation see :py:obj:`results_query`'''
'''JSON query of result's ``suggestion``. For the query string documentation see :py:obj:`results_query`'''
'''JSON query of result's ``thumbnail``. For the query string documentation see :py:obj:`results_query`'''
'''JSON query of result's ``title``. For the query string documentation see :py:obj:`results_query`'''
'''JSON query of result's ``url``. For the query string documentation see :py:obj:`results_query`'''
'''Maps safe-search value to ``{safe_search}`` in :py:obj:`search_url`.
'''Maps time range value from user to ``{time_range_val}`` in
'''Maximum redirects, soft limit. Record an error but don't stop the engine'''
'''Number of results on each page.  Only needed if the site requires not a page
'''Number of the first page (usually 0 or 1).'''
'''Replacement ``{lang}`` in :py:obj:`search_url` if language ``all`` is
'''Return empty result for these HTTP status codes instead of throwing an error.
'''Some engines might offer different result based headers.  Possible use-case:
'''Some engines might offer different result based on cookies or headers.
'''Some engines might offer different result based on cookies.
'''Some engines might require to do POST requests for search.'''
'''String to prepend to the result's ``thumbnail``.'''
'''String to prepend to the result's ``url``.'''
'''The body of the request.  This can only be used if different :py:obj:`method`
'''Time range URL parameter in the in :py:obj:`search_url`.  If no time range is
'''`XPath selector`_ for the list of result items'''
'''`XPath selector`_ of result's ``content``.'''
'''`XPath selector`_ of result's ``suggestion``.'''
'''`XPath selector`_ of result's ``thumbnail``.'''
'''`XPath selector`_ of result's ``title``.'''
'''`XPath selector`_ of result's ``url``.'''
'Reader' role in your subscription.
(:py:obj:`rest_v1_summary_url`).
(e.g. NDR, WDR, SWR, HR, ...)
(more or less) the same REST API and the schema of the JSON result is identical.
(torrent identifier).
(`Goggles Quickstart`_).
)
) -> str | None:
) -> str:
) -> tuple[str, str, str, str, str, datetime | None]:
):
* https://api.stackexchange.com/
* https://artic.edu
**Generated:** 2026-02-28 00:52:21
**Total Engines:** 219
*Generated by SearXNG Engine Catalog Generator*
*next page* form fields:
- **Average Lines per Engine:** 126
- **Average Size per Engine:** 3.87 KB
- **Total Engines:** 219
- **Total Lines of Code:** 27,759
- **Total Size:** 0.83 MB
- 3D: ``3d``
- :py:obj:`aa_content`
- :py:obj:`aa_ext`
- :py:obj:`aa_sort`
- :py:obj:`ads_sort`
- :py:obj:`api_key`
- :py:obj:`api_order`
- :py:obj:`api_path`
- :py:obj:`api_username`
- :py:obj:`backend <backend_url>`
- :py:obj:`base_url`
- :py:obj:`bt4g_category`
- :py:obj:`bt4g_order_by`
- :py:obj:`chinaso_category` (:py:obj:`ChinasoCategoryType`)
- :py:obj:`chinaso_news_source` (:py:obj:`ChinasoNewsSourceType`)
- :py:obj:`content_html_to_text`
- :py:obj:`content_query`
- :py:obj:`content_xpath`
- :py:obj:`cookies`
- :py:obj:`dl_prefix`
- :py:obj:`first_page_num`
- :py:obj:`frontend <frontend_url>`
- :py:obj:`ghc_auth`
- :py:obj:`ghc_highlight_matching_lines`
- :py:obj:`ghc_insert_block_separator`
- :py:obj:`ghc_strip_new_lines`
- :py:obj:`ghc_strip_whitespace`
- :py:obj:`headers`
- :py:obj:`http_digest_auth_pass`
- :py:obj:`http_digest_auth_user`
- :py:obj:`huggingface_endpoint`
- :py:obj:`lang_all`
- :py:obj:`lemmy_type`
- :py:obj:`method`
- :py:obj:`mount_prefix`
- :py:obj:`no_result_for_http_status`
- :py:obj:`order`
- :py:obj:`page_size`
- :py:obj:`paging`
- :py:obj:`qwant_categ`
- :py:obj:`request_body`
- :py:obj:`results_per_page`
- :py:obj:`results_query`
- :py:obj:`results_xpath`
- :py:obj:`result_type`
- :py:obj:`safe_search_map`
- :py:obj:`safe_search_support`
- :py:obj:`search_dir`
- :py:obj:`search_endpoint`
- :py:obj:`search_mode`
- :py:obj:`search_type`
- :py:obj:`search_url`
- :py:obj:`show_avatar`
- :py:obj:`soft_max_redirects`
- :py:obj:`sort_order`
- :py:obj:`sort`
- :py:obj:`sourcehut_sort_order`
- :py:obj:`srenablerewrites`
- :py:obj:`srprop`
- :py:obj:`srsort`
- :py:obj:`suggestion_query`
- :py:obj:`suggestion_xpath`
- :py:obj:`ta_link_to_mp4`
- :py:obj:`ta_token`
- :py:obj:`thumbnail_prefix`
- :py:obj:`thumbnail_query`
- :py:obj:`thumbnail_xpath`
- :py:obj:`time_range_map`
- :py:obj:`time_range_support`
- :py:obj:`time_range_url`
- :py:obj:`title_html_to_text`
- :py:obj:`title_query`
- :py:obj:`title_xpath`
- :py:obj:`url_prefix`
- :py:obj:`url_query`
- :py:obj:`url_xpath`
- :py:obj:`zlib_ext`
- :py:obj:`zlib_year_from`
- :py:obj:`zlib_year_to`
- :ref:`bing images engine`
- :ref:`bing news engine`
- :ref:`bing videos engine`
- :ref:`google autocomplete`
- :ref:`google images engine`
- :ref:`google news engine`
- :ref:`google scholar engine`
- :ref:`google videos engine`
- :ref:`wikidata engine`
- A query containing one of the category identifiers ``fsk``, ``genre``,
- Adobe-Stock's :py:obj:`adobe_content_types`
- Adobe-Stock's :py:obj:`adobe_order`
- Any normal search query -> Fetch search results by the query
- As far as is known, it is possible to remove a un-blocked an IP by executing a
- Audio ``audio``
- By filter: ``!mp fsk-0 land-deutschland genre-actionfilm``
- By filter: ``!mp jahrzehnt-2020er online-netflix``
- By filter: ``!mp person-Ryan-Gosling``
- DDG does not accept queries with more than 499 chars
- DuckDuckGo Images   : ``https://duckduckgo.com/i.js??q=...&vqd=...``
- DuckDuckGo News     : ``https://duckduckgo.com/news.js??q=...&vqd=...``
- DuckDuckGo Videos   : ``https://duckduckgo.com/v.js??q=...&vqd=...``
- DuckDuckGo WEB      : ``https://links.duckduckgo.com/d.js?q=..``  (HTTP GET)
- DuckDuckGo WEB html : ``https://html.duckduckgo.com/html``        (HTTP POST no-JS / form data)
- DuckDuckGo WEB lite : ``https://lite.duckduckgo.com/lite``        (HTTP POST no-JS / form data)
- DuckDuckGo WEB no-AI: ``https://noai.duckduckgo.com/``            (HTTP GET)
- Except ``Cookie: kl=..; df=..`` DDG does not use cookies in any of its
- Few or no results in a non-English UI language:
- HTML (``_fmt:html``)
- https://download.yacy.net/
- https://github.com/searxng/searxng/issues/2722#issuecomment-2884993248
- https://github.com/yacy/yacy_search_server/tags
- https://instances.joinpeertube.org/instances
- https://liste.mediathekview.de/filmliste-v2.db.bz2
- https://rapidapi.com/blog/how-to-use-imdb-api/
- https://stackoverflow.com/questions/1966503/does-imdb-provide-an-api
- https://the-public-domain-review.imgix.net
- https://the-public-domain-review.imgix.net/collections/
- https://the-public-domain-review.imgix.net/shop/
- https://www.moviepilot.de/filme/beste.
- Illustrations: ``illustration``
- Images: ``image``
- JSON (``_fmt:json``)
- Language filter support (maps SearXNG language to ``filter=language:<iso2>``)
- Language is inherited from the user's UI language; when it is not ``all``, the
- Maps fields commonly used in scholarly results: title, authors, abstract
- Normal: ``!mp Tom Cruise``
- NSFW = "Not safe for work". *Grandma isn't sure who you are anymore.*
- num_ : the number of search results is ignored / there is no paging all
- Paging support via ``page`` and ``per-page``
- Photos: ``photo``
- Preference changes fail while testing locally:
- Protobuf_ (``_fmt:pb``)
- Protobuf_ compressed? (``_fmt:pc``)
- Protobuf_ encoded in JSON (``_fmt:jspb``).
- proxy
- Relevance sorting (``sort=relevance_score:desc``)
- Results typically include a main link. When the primary landing page from
- save_ : is ignored / Google-News results are always *SafeSearch*
- SearXNG's :ref:`engine categories`
- SFW = "Safe for work" wallpapers.  *Grandma approves.*
- Sketchy = Not quite SFW not quite NSFW.  *Grandma might be uncomfortable.*
- Supports OpenAlex "polite pool" by adding a ``mailto`` parameter
- Templates: ``template``
- the gl_ argument is mandatory
- the hl_ argument has to be set correctly (and different to Google WEB)
- The no-JS clients receive a form that contains all the controlling parameters.
- The request argument ``origin=funnel_home_website`` is often seen in the DDG
- The second page (additional content) for a query cannot be requested without
- the ``ceid`` argument has to be set (:py:obj:`ceid_list`)
- The ``mailto`` key is optional but recommended by OpenAlex for better service.
- The ``vqd`` value ("Validation query digest") is needed to pass DDG's bot
- The ``vqd`` value is generally not needed for the first query (intro); it is
- Uses the official Works endpoint (JSON)
- Vectors: ``zip_vector`` (Vectors),
- Videos: ``video``
- When the form data is submitted, a real WEB browser sets the HTTP _Sec-Fetch_ headers.
- `Automatic conversion between traditional and simplified Chinese characters`_
- `Global API Parameters`_
- `REST GET`_
- `Video filters API`_
- `Wikipedias in multiple writing systems`_
- ``!springer (doi:10.1007/s10948-025-07019-1 OR doi:10.1007/s10948-025-07035-1)``
- ``!springer keyword:ybco year:2024``
- ``!springer keyword:ybco``
- ``!wp 出租車 :zh-CN`` should show 出租车
- ``!wp 出租車 :zh-HK`` should show 的士
- ``!wp 出租車 :zh-SG`` should show 德士
- ``!wp 出租車 :zh-TW`` should show 計程車
- ``!wp 出租車 :zh``    should show 出租車
- ``all``: all sources
- ``alpha``
- ``api`` (str): API endpoint identifier, typically 'd.js'
- ``asc``
- ``authors`` (display names)
- ``BUSINESS``: business publication
- ``b`` (str): Beginning parameter - empty string for first page requests.  If a
- ``CENTRAL``: central publication
- ``created``
- ``creation`` (most recent) or
- ``d.js``, ``i.js``, ``v.js``, ``news.js`` are the endpoints of the DDG's web
- ``datasets``: search for datasets
- ``dc`` (int): Display count - value equal to offset (s) + 1
- ``desc`` (default)
- ``df`` (str): Time filter, maps to values like 'd' (day), 'w' (week), 'm' (month), 'y' (year)
- ``doi`` (normalized to the plain DOI, without the ``https://doi.org/`` prefix)
- ``EPAPER``: E-Paper
- ``featured`` or
- ``general``: search for general
- ``goggles``: Common WEB search with custom rules, requires a :py:obj:`Goggles` URL.
- ``id``
- ``images``: search for images
- ``journal`` (host venue display name) and ``publisher``
- ``kl`` (str): Keyboard language/region code (e.g. 'en-us' default: 'wt-wt')
- ``LOCAL``: local publication
- ``longest-active``
- ``match``,
- ``models``: search for models
- ``nb_downloads`` (number of downloads)
- ``news``: search for news
- ``nextParams`` (str): Continuation parameters from previous page response,
- ``o`` (str): Output format, typically ``json``
- ``pages``, ``volume``, ``number`` (issue)
- ``pdf_url`` (Open access PDF if available) and ``html_url`` (landing page)
- ``publishedDate`` (parsed from ``publication_date``)
- ``q`` (str): Search query string
- ``recently-updated``
- ``relevance`` or
- ``search``: Common WEB search
- ``simple_query_string``,
- ``size``
- ``spaces``: search for spaces
- ``s`` (int): Search offset for pagination
- ``tags`` (OpenAlex concepts display names)
- ``terms``.
- ``term`` and
- ``title`` and ``content`` (abstract; reconstructed from the inverted index)
- ``type`` and a brief ``comments`` string with citation count
- ``updated`` (default)
- ``videos``: search for videos
- ``vqd`` (str): Validation query digest
- ``v`` (str): Typically ``l`` for subsequent pages
- ``web-lite``: uses the :py:obj:`web_lite_url` which returns a HTML page
- ``web``: uses the :py:obj:`api_url` which returns a JSON structure
---
-----
------------
-Web, -News, -Images and -Videos.  The API is undocumented but can be reverse
.. admonition:: Content-Security-Policy (CSP)
.. Adobe Stock: https://stock.adobe.com/
.. attention::
.. code: html
.. code:: python
.. code:: text
.. code:: yaml
.. hint::
.. note::
.. [1] `iana: language-subtag-registry
.. [2]
.. _/api2u/search: http://tagesschau.api.bund.dev/
.. _Accept-Language:
.. _ADS: https://ui.adsabs.harvard.edu
.. _Advanced station search API:
.. _Alpine Linux binary packages: https://pkgs.alpinelinux.org
.. _Alpine Linux: https://www.alpinelinux.org
.. _Anna's Archive: https://annas-archive.org/
.. _AnnaArchivist: https://annas-software.org/AnnaArchivist/annas-archive
.. _API-Playground: https://dev.springernature.com/docs/live-documentation/
.. _API: https://docs.joinmastodon.org/api/
.. _arXiv API: https://info.arxiv.org/help/api/user-manual.html
.. _Authorization: https://developer.mozilla.org/en-US/docs/Web/HTTP/Reference/Headers/Authorization
.. _auth_key: https://www.meilisearch.com/docs/reference/api/overview#authorization
.. _Automatic conversion between traditional and simplified Chinese characters:
.. _Baidu: https://www.baidu.com
.. _Bilibili: https://www.bilibili.com
.. _brave googles:
.. _brave languages:
.. _brave regions:
.. _Brave Search API: https://api-dashboard.search.brave.com/documentation
.. _BT4G: https://bt4g.com/
.. _bundDEV: https://bund.dev/apis
.. _Bundesstelle für Open Data: https://github.com/bundesAPI
.. _ChinaSo: https://www.chinaso.com/
.. _Constructing a search query:
.. _core engine config:
.. _CORE: https://core.ac.uk/about
.. _Countries where Yandex.Music is available: https://yandex.com/support/music/access.html
.. _Crossref: https://www.crossref.org/documentation/retrieve-metadata/
.. _data URLs:
.. _Details of Query Construction:
.. _Dev:APIyacysearch: https://wiki.yacy.net/index.php/Dev:APIyacysearch
.. _DuckDuckGo Bot Detection Research & Solution:
.. _endpoint: https://www.loc.gov/apis/json-and-yaml/requests/endpoints/
.. _Fields selection: https://developers.dailymotion.com/api/#fields-selection
.. _Forgejo: https://forgejo.org/
.. _Gitea: https://about.gitea.com/
.. _Github REST API auth for code search:
.. _Github REST API for code search:
.. _GitHub settings:
.. _GitLab REST API: https://docs.gitlab.com/ee/api/
.. _GitLab: https://about.gitlab.com/install/
.. _gl: https://developers.google.com/custom-search/docs/xml_results#glsp
.. _Global API Parameters: https://developers.dailymotion.com/api/#global-parameters
.. _Goggles Quickstart: https://github.com/brave/goggles-quickstart
.. _Goggles Whitepaper: https://brave.com/static-assets/files/goggles.pdf
.. _Google max 50 pages: https://github.com/searxng/searxng/issues/2982
.. _hl: https://developers.google.com/custom-search/docs/xml_results#hlsp
.. _https://zh.wikipedia.org/wiki/出租車:
.. _Hugging Face: https://huggingface.co
.. _Interface Search: https://join-lemmy.org/api/interfaces/Search.html
.. _issue #4694: https://github.com/searxng/searxng/issues/4694
.. _Jackett-categories:
.. _Jackett:
.. _Key and license:
.. _LanguageConverter:
.. _LC Chinese:
.. _lemmy-js-client: https://join-lemmy.org/api/modules.html
.. _list of Goggles: https://search.brave.com/goggles/discover
.. _Marginalia Search:
.. _Mastodon: https://mastodon.social
.. _MediathekView: https://mediathekview.de/
.. _MediaWiki Action API: https://www.mediawiki.org/wiki/API:Main_page
.. _MediaWiki query API: https://commons.wikimedia.org/w/api.php?action=help&modules=query
.. _Meta-API: https://dev.springernature.com/docs/api-endpoints/meta-api/
.. _Mwmbl: https://github.com/mwmbl/mwmbl
.. _num: https://developers.google.com/custom-search/docs/xml_results#numsp
.. _Odysee: https://github.com/OdyseeTeam/odysee-frontend
.. _Open Library: https://openlibrary.org
.. _OpenAlex API overview: https://docs.openalex.org/how-to-use-the-api/api-overview
.. _OpenAlex: https://openalex.org
.. _OpenAPI: https://swagger.io/specification/
.. _Pagination and Limits:
.. _Piped-Backend: https://github.com/TeamPiped/Piped-Backend
.. _Piped-Frontend: https://github.com/TeamPiped/Piped
.. _Piped-Instances: https://github.com/TeamPiped/Piped/wiki/Instances
.. _Piped’s architecture: https://docs.piped.video/docs/architecture/
.. _PR-2554: https://github.com/searx/searx/pull/2554
.. _preference page: https://www.bing.com/account/general
.. _Protobuf: https://en.wikipedia.org/wiki/Protocol_Buffers
.. _Prowlarr-categories:
.. _Prowlarr:
.. _PubMed: https://pubmed.ncbi.nlm.nih.gov/
.. _Python API Wrapper: https://pypi.org/project/springernature-api-client/
.. _query action: https://www.mediawiki.org/w/api.php?action=help&modules=query
.. _Query-Interface: https://info.arxiv.org/help/api/user-manual.html#_query_interface
.. _Rate Limiting and Errors: https://wallhaven.cc/help/api#limits
.. _Referer:
.. _Referrer-Policy:
.. _register an application in Microsoft Entra ID:
.. _Repology: https://repology.org/docs/about
.. _REST GET: https://developers.dailymotion.com/tools/
.. _Reuters: https://www.reuters.com
.. _save: https://developers.google.com/custom-search/docs/xml_results#safesp
.. _search endpoint: https://docs.discourse.org/#tag/Search
.. _search syntax:
.. _search syntax: https://ui.adsabs.harvard.edu/help/search/search-syntax
.. _search-APIs: https://learn.microsoft.com/en-us/bing/search-apis/
.. _search.json: https://openlibrary.org/dev/docs/api/search
.. _search/query: https://ui.adsabs.harvard.edu/help/api/api-docs.html#get-/search/query
.. _Sec-Fetch-Mode:
.. _Semantic Scholar: https://www.semanticscholar.org/about
.. _SourceHut: https://sourcehut.org/
.. _Springer Nature: https://www.springernature.com/
.. _Springer subscription:  https://dev.springernature.com/subscription/
.. _Springer-API: https://dev.springernature.com/docs/introduction/
.. _startpage categories:
.. _startpage languages:
.. _startpage regions:
.. _Supported Query Parameters: https://dev.springernature.com/docs/supported-query-params/
.. _Tagesschau API: https://github.com/AndreasFischer1985/tagesschau-api/blob/main/README_en.md
.. _Torznab:
.. _Tube Archivist: https://www.tubearchivist.com
.. _User-Agent:
.. _Video filters API: https://developers.dailymotion.com/api/#video-filters
.. _Void Linux binary packages: https://voidlinux.org/packages/
.. _Wallhaven: https://wallhaven.cc/about#Copyright
.. _Website: https://lucide.dev
.. _What are SFW, Sketchy and NSFW all about?:
.. _Wikimedia Commons: https://commons.wikimedia.org/
.. _wikipedia rest_v1 summary API:
.. _Wikipedias in multiple writing systems:
.. _Works/get_works: https://api.crossref.org/swagger-ui/index.html#/Works/get_works
.. _XPath selector: https://quickref.me/xpath.html#xpath-selectors
.. _Yacy: https://yacy.net/
.. _Z-Library: https://zlibrary-global.se/
.. __IMDPro: https://pro.imdb.com/login
.. _`format=json`: https://www.mediawiki.org/w/api.php?action=help&modules=json
.. _`list=search`: https://www.mediawiki.org/w/api.php?action=help&modules=query%2Bsearch
1. [1337x](#1337x)
10. [ansa](#ansa)
100. [json_engine](#json-engine)
101. [kickass](#kickass)
102. [lemmy](#lemmy)
103. [lib_rs](#lib-rs)
104. [libretranslate](#libretranslate)
105. [lingva](#lingva)
106. [livespace](#livespace)
107. [loc](#loc)
108. [lucide](#lucide)
109. [marginalia](#marginalia)
11. [apkmirror](#apkmirror)
110. [mariadb_server](#mariadb-server)
111. [mastodon](#mastodon)
112. [material_icons](#material-icons)
113. [mediathekviewweb](#mediathekviewweb)
114. [mediawiki](#mediawiki)
115. [meilisearch](#meilisearch)
116. [metacpan](#metacpan)
117. [microsoft_learn](#microsoft-learn)
118. [mixcloud](#mixcloud)
119. [mojeek](#mojeek)
12. [apple_app_store](#apple-app-store)
120. [mongodb](#mongodb)
121. [moviepilot](#moviepilot)
122. [mozhi](#mozhi)
123. [mrs](#mrs)
124. [mwmbl](#mwmbl)
125. [mysql_server](#mysql-server)
126. [naver](#naver)
127. [niconico](#niconico)
128. [npm](#npm)
129. [nvd](#nvd)
13. [apple_maps](#apple-maps)
130. [nyaa](#nyaa)
131. [odysee](#odysee)
132. [ollama](#ollama)
133. [open_meteo](#open-meteo)
134. [openalex](#openalex)
135. [openclipart](#openclipart)
136. [openlibrary](#openlibrary)
137. [opensemantic](#opensemantic)
138. [openstreetmap](#openstreetmap)
139. [openverse](#openverse)
14. [archlinux](#archlinux)
140. [pdbe](#pdbe)
141. [peertube](#peertube)
142. [pexels](#pexels)
143. [photon](#photon)
144. [pinterest](#pinterest)
145. [piped](#piped)
146. [piratebay](#piratebay)
147. [pixabay](#pixabay)
148. [pixiv](#pixiv)
149. [pkg_go_dev](#pkg-go-dev)
15. [artic](#artic)
150. [podcastindex](#podcastindex)
151. [postgresql](#postgresql)
152. [presearch](#presearch)
153. [public_domain_image_archive](#public-domain-image-archive)
154. [pubmed](#pubmed)
155. [pypi](#pypi)
156. [quark](#quark)
157. [qwant](#qwant)
158. [radio_browser](#radio-browser)
159. [recoll](#recoll)
16. [artstation](#artstation)
160. [reddit](#reddit)
161. [repology](#repology)
162. [reuters](#reuters)
163. [rottentomatoes](#rottentomatoes)
164. [rumble](#rumble)
165. [scanr_structures](#scanr-structures)
166. [searchcode_code](#searchcode-code)
167. [searx_engine](#searx-engine)
168. [selfhst](#selfhst)
169. [semantic_scholar](#semantic-scholar)
17. [arxiv](#arxiv)
170. [senscritique](#senscritique)
171. [sepiasearch](#sepiasearch)
172. [seznam](#seznam)
173. [sogou](#sogou)
174. [sogou_images](#sogou-images)
175. [sogou_videos](#sogou-videos)
176. [sogou_wechat](#sogou-wechat)
177. [solidtorrents](#solidtorrents)
178. [solr](#solr)
179. [soundcloud](#soundcloud)
18. [ask](#ask)
180. [sourcehut](#sourcehut)
181. [spotify](#spotify)
182. [springer](#springer)
183. [sqlite](#sqlite)
184. [stackexchange](#stackexchange)
185. [startpage](#startpage)
186. [steam](#steam)
187. [stract](#stract)
188. [svgrepo](#svgrepo)
189. [tagesschau](#tagesschau)
19. [astrophysics_data_system](#astrophysics-data-system)
190. [tineye](#tineye)
191. [tokyotoshokan](#tokyotoshokan)
192. [tootfinder](#tootfinder)
193. [torznab](#torznab)
194. [translated](#translated)
195. [tubearchivist](#tubearchivist)
196. [unsplash](#unsplash)
197. [uxwing](#uxwing)
198. [valkey_server](#valkey-server)
199. [vimeo](#vimeo)
2. [360search](#360search)
2.4 million scholarly articles in the fields of physics, mathematics, computer
20. [azure](#azure)
200. [voidlinux](#voidlinux)
201. [wallhaven](#wallhaven)
202. [wikicommons](#wikicommons)
203. [wikidata](#wikidata)
204. [wikipedia](#wikipedia)
205. [wolframalpha_api](#wolframalpha-api)
206. [wolframalpha_noapi](#wolframalpha-noapi)
207. [wordnik](#wordnik)
208. [wttr](#wttr)
209. [www1x](#www1x)
21. [baidu](#baidu)
210. [xpath](#xpath)
211. [yacy](#yacy)
212. [yahoo](#yahoo)
213. [yahoo_news](#yahoo-news)
214. [yandex](#yandex)
215. [yandex_music](#yandex-music)
216. [yep](#yep)
217. [youtube_api](#youtube-api)
218. [youtube_noapi](#youtube-noapi)
219. [zlibrary](#zlibrary)
22. [bandcamp](#bandcamp)
23. [base](#base)
24. [bilibili](#bilibili)
25. [bing](#bing)
26. [bing_images](#bing-images)
27. [bing_news](#bing-news)
28. [bing_videos](#bing-videos)
29. [bitchute](#bitchute)
3. [360search_videos](#360search-videos)
30. [bpb](#bpb)
31. [brave](#brave)
32. [braveapi](#braveapi)
33. [bt4g](#bt4g)
34. [btdigg](#btdigg)
35. [cachy_os](#cachy-os)
36. [ccc_media](#ccc-media)
37. [chefkoch](#chefkoch)
38. [chinaso](#chinaso)
39. [cloudflareai](#cloudflareai)
4. [9gag](#9gag)
40. [command](#command)
41. [core](#core)
42. [crates](#crates)
43. [crossref](#crossref)
44. [currency_convert](#currency-convert)
45. [dailymotion](#dailymotion)
46. [deepl](#deepl)
47. [deezer](#deezer)
48. [demo_offline](#demo-offline)
49. [demo_online](#demo-online)
5. [acfun](#acfun)
50. [destatis](#destatis)
51. [deviantart](#deviantart)
52. [devicons](#devicons)
53. [dictzone](#dictzone)
54. [digbt](#digbt)
55. [discourse](#discourse)
56. [docker_hub](#docker-hub)
57. [doku](#doku)
58. [duckduckgo](#duckduckgo)
59. [duckduckgo_definitions](#duckduckgo-definitions)
6. [adobe_stock](#adobe-stock)
60. [duckduckgo_extra](#duckduckgo-extra)
61. [duckduckgo_weather](#duckduckgo-weather)
62. [duden](#duden)
63. [dummy-offline](#dummy-offline)
64. [dummy](#dummy)
65. [ebay](#ebay)
66. [elasticsearch](#elasticsearch)
67. [emojipedia](#emojipedia)
68. [fdroid](#fdroid)
69. [findthatmeme](#findthatmeme)
7. [ahmia](#ahmia)
70. [flickr](#flickr)
71. [flickr_noapi](#flickr-noapi)
72. [freesound](#freesound)
73. [frinkiac](#frinkiac)
74. [fyyd](#fyyd)
75. [geizhals](#geizhals)
76. [genius](#genius)
77. [gitea](#gitea)
78. [github](#github)
79. [github_code](#github-code)
8. [alpinelinux](#alpinelinux)
80. [gitlab](#gitlab)
81. [goodreads](#goodreads)
82. [google](#google)
83. [google_images](#google-images)
84. [google_news](#google-news)
85. [google_play](#google-play)
86. [google_scholar](#google-scholar)
87. [google_videos](#google-videos)
88. [grokipedia](#grokipedia)
89. [hackernews](#hackernews)
9. [annas_archive](#annas-archive)
90. [hex](#hex)
91. [huggingface](#huggingface)
92. [il_post](#il-post)
93. [imdb](#imdb)
94. [imgur](#imgur)
95. [ina](#ina)
96. [invidious](#invidious)
97. [ipernity](#ipernity)
98. [iqiyi](#iqiyi)
99. [jisho](#jisho)
:py:obj:`chinaso_news_source`.
:py:obj:`fetch_wikimedia_traits` function.
:py:obj:`get_wiki_params` and :py:obj:`wiki_lc_locale_variants' in the
:py:obj:`startpage_categ` in  settings.yml::
:py:obj:`time_range_url`.
:ref:`infinite_scroll <settings plugins>` plugin in SearXNG but it does not
:ref:`settings <settings engines>` and set the ``base_url``.
<babel.languages.get_official_languages>`):
<engine network>` from the ``chinaso news`` engine.
<https://developer.imdb.com/documentation>`_
<https://mariadb.com/docs/server/connect/programming-languages/c/install/>`_
<https://piped-instances.kavin.rocks/>`__)
<searx.engines.peertube>` and the response is identical to the response from the
<settings engines>`:
<time_range_support>` is limited (see remarks).
=======
============
=============
==============
===============
================
=================
===================
====================
=====================
=========================
@contextlib.contextmanager
@parse       url, title, content, publishedDate, iframe_src, thumbnail
@provide-api no
@results     HTML
@website     https://bandcamp.com/
a focus on useability and speed.
a huge number of torrent sites which are not directly supported.
A LanguageConverter_ (LC) is a system based on language variants that
a Locale (used in the HTTP ``Accept-Language`` header). For example see `LC
aa_content: str = ""
aa_ext: str = ""
aa_sort: str = ""
about = {
about politics and history.
about: dict[str, str | bool] = {
about: dict[str, t.Any] = {
Accept-Language_:
accept_header = 'application/vnd.github.preview.text-match+json'
accept_header = 'application/vnd.github.text-match+json'
Additional subcategories:
adobe_content_types: list = []
adobe_order: str = ""
ADOBE_VALID_TYPES = ["photo", "illustration", "zip_vector", "video", "template", "3d", "audio", "image"]
ads_field_list = [
ads_rows = 10
ads_sort = "read_count desc"
After these steps, double check that TA works as normal (nothing should be
AGO_RE = re.compile(r'([0-9]+)\s*(year|month|week|day|minute|hour)')
AGO_TIMEDELTA = {
Ahmia (Onions)
all languages, but there is one Wikipedia for each supported language. Some of
all yacy engines (unless an individual value for ``base_url`` is configured for
allows you to quickly install, update and remove software; software is provided
alpine_arch = 'x86_64'
An alternative that needs IMDPro_ is `IMDb and Box Office Mojo
and disciplines.
and field is any of the valid indexes."""
and general-interest books.  It began as a mirror of Library Genesis, from which
and improved service quality, include a contact email in each request via
and provide the JSON payload to submit to Elasticsearch in
and there does not exists ``async`` API.  Even though the API slightly vintage
and there is at least one tag with a three letter language tag (ISO 639-2)::
and unpack into ``searx/data/filmliste-v2.db``.  To search the database use e.g
any extra dependency.
API <frontend_url>`.  This feature is *next page driven* and plays well with the
API returns on such calls::
API Wrapper`_.
API: Dev:APIyacysearch_
api_client_id = None
api_client_secret = None
api_key = ""
api_key = "unset"
api_key = ''
api_key = ''  # defined in settings.yml
api_key = None
api_key: str = ""
api_key: str = ''
api_order = 'desc'
api_order = 'likes'
api_path: str = 'api/v4/projects'
api_path: str = 'w/api.php'
api_site = 'stackoverflow'
api_sort = 'activity'
api_url = "https://api.media.ccc.de"
api_url = "https://api.mwmbl.org/api/v1"
api_url = "https://api.mymemory.translated.net"
api_url = "https://api.open-meteo.com"
api_url = 'https://api.qwant.com/v3/search/'
api_username = ''
app_locale_map = {
Arch Linux Wiki
Arch Wiki blocks access to it.
ARCH_RE = re.compile("x86_64|x86|aarch64|armhf|ppc64le|s390x|armv7|riscv64")
ARCH_RE = re.compile('aarch64-musl|armv6l-musl|armv7l-musl|x86_64-musl|aarch64|armv6l|armv7l|i686|x86_64')
are shared by other engines:
are updated.  The measurement of depth was introduced after some limitations of
Array entries can be specified using the index or can be omitted entirely,
arxiv_max_results = 10
arxiv_namespaces = {
arxiv_search_prefix = "all"
As far we can say the *instant answers* API does not support languages, or at
assets, videos, motion graphics templates and audio tracks.
assets. Assets types include photos, vectors, illustrations, templates, 3D
attributes.  By default it is set to ``caching_sha2_password``.
authentication configured to read from ``my-index`` index.
authentication.  What this means is that by default, SearXNG will have no way to
author_xpath = ".//a[contains(@class, 'authorName')]"
author_xpath = './/div[@class="ellipsis-1"]'
auth_key = ''
auth_plugin = 'caching_sha2_password'
automatically converts the content of a page into a different variant. A variant
azure_batch_endpoint = "https://management.azure.com/batch?api-version=2020-06-01"
azure_client_id = ""
azure_client_secret = ""
azure_tenant_id = ""
azure_token_expiration_seconds = 5000
backend_url: list[str] | str = []
baidu_category = 'general'
BASE = 'https://frinkiac.com/'
based) store.  Before configuring the ``valkey_server`` engine, you must install
base_url = ""
base_url = "https://api.adsabs.harvard.edu/v1/search/query"
base_url = "https://api.bilibili.com/x/web-interface/search/type"
base_url = "https://api.bitchute.com/api/beta/search/videos"
base_url = "https://api.chefkoch.de"
base_url = "https://api.core.ac.uk/v3/search/works/"
base_url = "https://api.fyyd.de"
base_url = "https://api.search.brave.com/res/v1/web/search"
base_url = "https://api.springernature.com/meta/v2/json"
base_url = "https://api.yep.com"
base_url = "https://api2.marginalia-search.com"
base_url = "https://bandcamp.com/"
base_url = "https://dictzone.com"
base_url = "https://duckduckgo.com/js/spice/currency/1/%(from_iso4217)s/%(to_iso4217)s"
base_url = "https://duckduckgo.com/js/spice/forecast/{query}/{lang}"
base_url = "https://export.arxiv.org/api/query"
base_url = "https://findthatmeme.com/api/v1/search"
base_url = "https://geizhals.de"
base_url = "https://grokipedia.com/api/full-text-search"
base_url = "https://hn.algolia.com/api/v1"
base_url = "https://hub.docker.com"
base_url = "https://huggingface.co"
base_url = "https://imgur.com"
base_url = "https://lemmy.ml/"
base_url = "https://libretranslate.com/translate"
base_url = "https://lighthouse.odysee.tv/search"
base_url = "https://mastodon.social"
base_url = "https://mesh.if.iqiyi.com"
base_url = "https://mozhi.aryak.me"
base_url = "https://nvd.nist.gov/extensions/nudp/services/json/nvd/cve/search/results"
base_url = "https://ollama.com"
base_url = "https://openclipart.org"
base_url = "https://openlibrary.org"
base_url = "https://packages.cachyos.org/api/search"
base_url = "https://peer.tube"
base_url = "https://pic.sogou.com"
base_url = "https://pkg.go.dev"
base_url = "https://pkgs.alpinelinux.org"
base_url = "https://podcastindex.org"
base_url = "https://presearch.com"
base_url = "https://pypi.org"
base_url = "https://search.brave.com/"
base_url = "https://search.naver.com"
base_url = "https://stock.adobe.com"
base_url = "https://store.steampowered.com"
base_url = "https://stract.com/beta/api"
base_url = "https://tv.360kan.com"
base_url = "https://uxwing.com"
base_url = "https://v.sogou.com"
base_url = "https://wallhaven.cc"
base_url = "https://weixin.sogou.com"
base_url = "https://www.acfun.cn"
base_url = "https://www.artstation.com/api/v2/search/projects.json"
base_url = "https://www.ask.com/web"
base_url = "https://www.bpb.de"
base_url = "https://www.chinaso.com"
base_url = "https://www.destatis.de"
base_url = "https://www.goodreads.com"
base_url = "https://www.mojeek.com"
base_url = "https://www.moviepilot.de"
base_url = "https://www.nicovideo.jp"
base_url = "https://www.pixiv.net/ajax/search/illustrations"
base_url = "https://www.reuters.com"
base_url = "https://www.rottentomatoes.com"
base_url = "https://www.semanticscholar.org"
base_url = "https://www.so.com"
base_url = "https://www.sogou.com"
base_url = "https://www.svgrepo.com"
base_url = "https://www.tagesschau.de"
base_url = "https://www.tootfinder.ch"
base_url = "https://xq-api.voidlinux.org"
base_url = 'http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion'
base_url = 'http://localhost:7700'
base_url = 'http://localhost:8090'
base_url = 'http://localhost:8983'
base_url = 'http://localhost:8983/solr/opensemanticsearch/'
base_url = 'http://localhost:9200'
base_url = 'https://1x.com'
base_url = 'https://api.openverse.org/v1/images/'
base_url = 'https://backend.live.space'
base_url = 'https://emojipedia.org'
BASE_URL = 'https://jisho.org/word/'
base_url = 'https://kickasstorrents.to'
base_url = 'https://lib.rs'
base_url = 'https://nominatim.openstreetmap.org/'
base_url = 'https://nyaa.si/'
base_url = 'https://photon.komoot.io/'
base_url = 'https://pixabay.com'
base_url = 'https://play.google.com'
base_url = 'https://rumble.com/'
base_url = 'https://search.f-droid.org/'
base_url = 'https://search.seznam.cz/'
base_url = 'https://sepiasearch.org'
base_url = 'https://solidtorrents.to'
base_url = 'https://tineye.com'
base_url = 'https://unsplash.com/'
base_url = 'https://vimeo.com/'
base_url = 'https://www.ansa.it'
base_url = 'https://www.apkmirror.com'
base_url = 'https://www.bing.com/images/async'
base_url = 'https://www.bing.com/news/infinitescrollajax'
base_url = 'https://www.bing.com/search'
base_url = 'https://www.bing.com/videos/asyncv2'
base_url = 'https://www.deviantart.com'
base_url = 'https://www.duden.de/'
base_url = 'https://www.googleapis.com/youtube/v3/search'
base_url = 'https://www.ina.fr'
base_url = 'https://www.ipernity.com'
base_url = 'https://www.loc.gov'
base_url = 'https://www.pexels.com'
base_url = 'https://www.pinterest.com'
base_url = 'https://www.reddit.com/'
base_url = 'https://www.startpage.com'
base_url = 'https://www.tokyotosho.info/'
base_url = 'https://www.youtube.com/results'
base_url = (
base_url = None
base_url: list[str] | str = []
base_url: str = ""
base_url: str = "https://sr.ht/projects"
base_url: str = "https://zlibrary-global.se"
base_url: str = ''
base_url: str = 'https://repology.org'
base_url: str = 'https://{language}.wikipedia.org/'
base_url: str = None  # type: ignore
base_url_images = 'https://yandex.com/images/search'
base_url_web = 'https://yandex.com/search/site/'
base_youtube_url = 'https://www.youtube.com/watch?v='
be tested.
Before configuring the ``mongodb`` engine, you must install the dependency
Below is an example configuration for using a MongoDB collection:
Below is an example configuration:
billion images `[tineye.com] <https://tineye.com/how>`_.
Bing does not have news range ``year`` / we use ``month`` instead."""
Brave Goggles
Brave languages
Brave regions
Brave uses two-digit tags for the regions like ``ca`` while SearXNG deals with
Brave's language support is limited to the UI (menus, area local notations,
brave_category: t.Literal["search", "videos", "images", "news", "goggles"] = "search"
brave_spellcheck = False
bt4g_category = 'all'
bt4g_order_by = 'relevance'
BTDigg (Videos, Music, Files)
by a team of anonymous archivists (AnnaArchivist_).
bypassing auth for images in TA by altering the default TA nginx file.
CACHE: EngineCache
cached_url = ''
cached_xpath = ''
CAPTCHA_PATTERN = r'\{[^{]*?"action"\s*:\s*"captcha"\s*,\s*"url"\s*:\s*"([^"]+)"[^{]*?\}'
cast_xpath = "concat('Starring ', string(./@cast))"
categories = searx_categories.keys()
categories = ["currency", "general"]
categories = ["files", "books"]
categories = ["files"]
categories = ["general", "books"]
categories = ["general", "web"]
categories = ["general", "web"]  # general, images, videos, news
categories = ["general"]
categories = ["images", "icons"]
categories = ["images"]
categories = ["it", "cloud"]
categories = ["it", "packages", "cargo"]
categories = ["it", "packages"]
categories = ["it", "repos"]
categories = ["it"]
categories = ["movies"]
categories = ["music"]
categories = ["news"]
categories = ["packages", "it"]
categories = ["science", "scientific publications"]
categories = ["videos", "music"]
categories = ["videos"]
categories = ["weather"]
categories = ['code']
categories = ['dictionaries', 'define']
categories = ['dictionaries']
categories = ['files', 'apps']
categories = ['files']
categories = ['general', 'news']
categories = ['general', 'translate']
categories = ['general', 'web']
categories = ['general']
categories = ['general']  # 'images', 'music', 'videos', 'files'
categories = ['images', 'icons']
categories = ['images', 'web']
categories = ['images']
categories = ['it', 'packages']
categories = ['it', 'packages']  # optional
categories = ['it', 'repos']
categories = ['it', 'software wikis']
categories = ['it']
categories = ['map']
categories = ['movies']
categories = ['music', 'lyrics']
categories = ['music', 'radio']
categories = ['music']
categories = ['news']
categories = ['onions']
categories = ['packages', 'it']
categories = ['science']
categories = ['shopping']
categories = ['social media']
categories = ['videos', 'music', 'files']
categories = ['videos', 'music']
categories = ['videos', 'web']
categories = ['videos']
categories = []
categories: list[str] = ["files", "books"]
categories: list[str] = ["general", "web"]
categories: list[str] = ['files']
categories: list[str] = ['packages', 'it']
categories: list[str] = []
category All) and in the goggles category."""
category for the Chinese market.
category.
cdn_base_url = "https://cdn.jsdelivr.net/gh/devicons/devicon@latest"
cdn_base_url = "https://cdn.jsdelivr.net/npm/lucide-static"
cdn_base_url = 'https://cdn.jsdelivr.net/gh/selfhst/icons'
ceid_list = [
cf_account_id = ''
cf_ai_api = ''
cf_ai_gateway = ''
cf_ai_model = ''
cf_ai_model_assistant = 'Keep your answers as short and effective as possible.'
cf_ai_model_display_name = 'Cloudflare AI'
cf_ai_model_system = 'You are a self-aware language model who is honest and direct about any question from the user.'
change.  This will limit any debugging to only images, rather than
ChinasoCategoryType = t.Literal['news', 'videos', 'images']
ChinasoNewsSourceType = t.Literal['CENTRAL', 'LOCAL', 'BUSINESS', 'EPAPER', 'all']
chinaso_category = 'news'
chinaso_news_source: ChinasoNewsSourceType = 'all'
Chinese`_.
cid_re = re.compile(r'client_id:"([^"]*)"', re.I | re.U)
class ApiSearchResult(t.TypedDict):
class ApiSearchResults(t.TypedDict):
class WDAmountAttribute(WDAttribute):
class WDArticle(WDAttribute):
class WDAttribute:
class WDDateAttribute(WDAttribute):
class WDGeoAttribute(WDAttribute):
class WDImageAttribute(WDURLAttribute):
class WDLabelAttribute(WDAttribute):
class WDURLAttribute(WDAttribute):
close to the implementation, its just a simple example which queries `The Art
close to the implementation, its just a simple example.
code blocks in a single file might be returned from the API).
code indentation.
code lines are just relabeled (starting from 1) and appended (a disjoint set of
collection = ''
collection = None
collects torrent metadata (such as file names and file sizes) and a magnet link
command = []
common usage of emoji characters in the Unicode Standard.  It is owned by Zedge
Compared to other Google services the Scholar engine has a simple GET REST-API
comparing packages versions across them and gathering other information.
complete list of official instances see Piped-Instances (`JSON
components, etc), 20 per page (`Works/get_works`_).
Configuration
Configurations for endpoints:
configured working directory:
Configured ``brave`` engines:
Configured ``presarch`` engines:
CONSENT dialog::
content will change way less than the HTML page.
content_html_to_text = False
content_query = None
content_xpath = '..//p[@class="s"]'
content_xpath = './/div[@class="b-serp-item__content"]//div[@class="b-serp-item__text"]'
content_xpath = './/div[@span="SECONDARY_INFO"]'
content_xpath = './/div[contains(@class, "column")]/p/text()'
content_xpath = './/div[contains(@class,"sous-titre-fonction")]'
content_xpath = './/p[1]'
content_xpath = './/p[@class="itemDescription"]/@title'
content_xpath = './/p[@class="max-w-lg break-words text-neutral-800 text-md"]/text()'
content_xpath = './div[@class="h"]/p'
content_xpath = './p[@class="SearchSnippet-synopsis"]'
content_xpath = None
contexts (``family_filter`` in `Global API Parameters`_ ).
cookie = {
cookies = {}
COOKIE_CACHE_EXPIRATION_SECONDS = 3600
COOKIE_CACHE_KEY = "cookie"
correction_xpath = '//*[@id="didYouMean"]//a'
CSRF_PRIVATEKEY_CACHE = "private_csrf_token"
CSRF_PUBLICKEY_CACHE = "public_csrf_token"
cursor_xpath = '(//a[@class="vQ2brP"]/@href)[last()]'
custom_query_json = {}
Dailymotion (Videos)
database = ""
database = None
database of the world’s scholarly literature, collecting and indexing
data_of_interest = (
Data`_ offers a `OpenAPI`_ portal at bundDEV_ where APIs are documented an can
date_xpath = './/a/span[contains(@class, "c-result__date")]'
db = 0
DDG's content search / see engine ``duckduckgo_extra.py``
DDG's WEB search:
ddg_category = ""
ddg_lang_map: dict[str, str] = {
ddg_link_url = "https://duckduckgo.com/?q=%(from_iso4217)s+to+%(to_iso4217)s"
ddg_reg_map: dict[str, str] = {
ddg_url: str = "https://html.duckduckgo.com/html/"
Deezer (Music)
def absolute_url(relative_url):
def area_to_str(area: dict[str, str]) -> str:
def authenticate(t_id: str, c_id: str, c_secret: str) -> str:
def build_content_parts(item: dict[str, t.Any], title: str, original_title: str | None) -> list[str]:
def build_flickr_url(user_id, photo_id):
def build_result(item: etree.Element) -> dict[str, t.Any]:
def build_sb_cookie(cookie_params):
def catch_bad_response(resp):
def check_parsing_options(engine_settings):
def clean_url(url):
def connect():
def construct_body(result):
def debug_explain_wikidata_query(query, method='GET'):
def detect_google_captcha(dom: ElementType):
def detect_google_sorry(resp):
def domain_is_seized(dom: ElementType):
def do_query(data, q):  # pylint: disable=invalid-name
def extract_code(code_matches: list[dict[str, t.Any]]) -> tuple[list[str], set[int]]:
def extract_json_data(text: str) -> dict[str, t.Any]:
def extract_response_info(result):
def extract_video_data(video_block):
def fetch_csrf_tokens():
def fetch_traits(engine_traits: EngineTraits) -> None:
def fetch_traits(engine_traits: EngineTraits):
def fetch_traits(engine_traits: EngineTraits, add_domains: bool = True):
def fetch_vqd(
def fetch_wikidata(nominatim_json, user_language):
def fetch_wikimedia_traits(engine_traits: EngineTraits):
def format_duration(duration):
def get_attribute(item: etree.Element, property_name: str) -> str | None:
def get_attributes(language):
def get_auth_token(t_id: str, c_id: str, c_secret: str) -> str:
def get_base_url_choice() -> str:
def get_cache() -> EngineCache:
def get_client_id() -> str | None:
def get_cookie(url: str) -> str:
def get_data(result, user_language, ignore_keys):
def get_ddg_lang(
def get_definitions(page):
def get_google_info(params: "OnlineParams", eng_traits: EngineTraits) -> dict[str, t.Any]:
def get_headers():
def get_img_src(result):
def get_infobox(alt_forms, result_url, definitions):
def get_key_label(key_name, lang):
def get_key_rank(k):
def get_label(labels, lang):
def get_label_for_entity(entity_id, language):
def get_links(result, user_language):
def get_query(query, language):
def get_results(attribute_result, attributes, language):
def get_sc_code(params):
def get_tag_label(tag_category, tag_name, lang):
def get_text_from_json(element):
def get_thumbnail(img_src):
def get_title_address(result):
def get_torznab_attribute(item: etree.Element, attribute_name: str) -> str | None:
def get_ui_version() -> str:
def get_url_osm_geojson(result):
def get_vqd(
def get_wikipedia_image(raw_value):
def get_wiki_params(sxng_locale, eng_traits):
def identity(arg):
def init(engine_settings):
def init(engine_settings):  # pylint: disable=unused-argument
def init(engine_settings: dict[str, t.Any]) -> bool:  # pylint: disable=unused-argument
def init(engine_settings: dict[str, t.Any]):
def init(engine_settings=None):  # pylint: disable=unused-argument
def init(_):
def init(_engine_settings):
def is_alibaba_captcha(html):
def is_broken_text(text: str) -> bool:
def is_ddg_captcha(dom: ElementType):
def is_iterable(obj):
def iterate(iterable):
def obtain_token() -> str:
def obtain_token():
def parse(query):  # pylint: disable=redefined-outer-name
def parse_addition(data):
def parse_ai_page(data):
def parse_album(hit):
def parse_artist(hit):
def parse_audio_item(item):
def parse_baike_sc(data):
def parse_data_images(text: str):
def parse_finance_shuidi(data):
def parse_first_page_response(response_text):
def parse_general(data):
def parse_gs_a(text: str | None) -> tuple[list[str], str, str, datetime | None]:
def parse_images(data):
def parse_image_item(item):
def parse_it(data):
def parse_item(item: dict[str, t.Any]) -> MainResult | None:
def parse_kk_yidian_all(data):
def parse_life_show_general_image(data):
def parse_lyric(hit):
def parse_med_struct(data):
def parse_music_new_song(data):
def parse_nature_result(data):
def parse_news(data):
def parse_news_uchq(data):
def parse_next_page_response(response_text):
def parse_search_query(json_results):
def parse_ss_doc(data):
def parse_ss_note(data):
def parse_tineye_match(match_json):
def parse_travel_dest_overview(data):
def parse_travel_ranking_list(data):
def parse_url(url_string):
def parse_videos(data):
def parse_video_item(item):
def parse_web_api(resp):
def parse_web_lite(resp):
def query(data, query_string):
def quote_ddg_bangs(query: str) -> str:
def replace_pua_chars(text):
def request(query, params):
def request(query, params):  # pylint: disable=redefined-outer-name
def request(query, params):  # pylint: disable=unused-argument
def request(query: str, params):
def request(query: str, params: "OnlineCurrenciesParams") -> None:  # pylint: disable=unused-argument
def request(query: str, params: "OnlineParams") -> None:
def request(query: str, params: "OnlineParams"):
def request(query: str, params: dict[str, t.Any]) -> dict[str, t.Any]:
def request(query: str, params: dict[str, t.Any]) -> None:
def request(query: str, params: dict[str, t.Any]):
def request(_query, params):
def resize_url(thumbnail: dict[str, str], width: int = 0, height: int = 0) -> str:
def response(resp) -> EngineResults:
def response(resp) -> EngineResults:  # pylint: disable=too-many-branches
def response(resp):
def response(resp):  # pylint: disable=too-many-branches
def response(resp):  # pylint: disable=unused-argument
def response(resp: "SXNG_Response") -> EngineResults:
def response(resp: "SXNG_Response") -> EngineResults:  # pylint: disable=too-many-locals
def response(resp: "SXNG_Response") -> list[dict[str, t.Any]]:
def response(resp: "SXNG_Response"):
def response(resp: 'SXNG_Response') -> EngineResults:
def response(resp: SXNG_Response) -> EngineResults:
def response(resp: SXNG_Response):
def response_apps(resp):
def response_movies(resp):
def result_to_text(text: str, htmlResult: str) -> str | None:
def search(query, params) -> EngineResults:
def search(query, request_params):  # pylint: disable=unused-argument
def search(query, _params) -> EngineResults:
def search(query: str, params: "RequestParams") -> EngineResults:
def search_after(time_range: str | None) -> str:
def search_keys(query) -> list[dict]:
def send_wikidata_query(query, method='GET', **kwargs):
def server_list() -> list[str]:
def setup(engine_settings: "OnlineParams") -> bool:
def setup(engine_settings: dict[str, t.Any]) -> bool:
def setup(engine_settings: dict[str, t.Any]) -> bool:  # pylint: disable=unused-argument
def set_bing_cookies(params, engine_language, engine_region):
def set_vqd(query: str | int, value: str, params: "OnlineParams") -> None:
def sqlite_cursor():
def time_range_args(params: "OnlineParams") -> dict[str, int]:
def ui_async(start: int) -> str:
def unit_to_str(unit: str) -> str:
def value_to_https_link(value):
def value_to_website_link(value):
def value_wikipedia_link(value):
def value_with_prefix(prefix, value):
def video_response(resp):
def video_response(resp, results: EngineResults) -> None:
def _backend_url() -> str:
def _base_url() -> str:
def _clean_up_node(node):
def _clean_url(url):
def _clear_cached_api_url():
def _custom_query(query):
def _doi_to_plain(doi_value: str | None) -> str:
def _extract_authors(item: dict[str, t.Any]) -> list[str]:
def _extract_biblio(
def _extract_comments(item: dict[str, t.Any]) -> str:
def _extract_links(item: dict[str, t.Any]) -> tuple[str, str, str]:
def _extract_published_date(published_date_raw: str | None):
def _extract_published_date(published_date_raw: str):
def _extract_tags(item: dict[str, t.Any]) -> list[str]:
def _fetch_results(cur) -> EngineResults:
def _fix_title(title, url):
def _flatten(xss):
def _frontend_url() -> str:
def _general_results(dom):
def _get_algolia_api_url():
def _get_command_to_run(query):
def _get_comments(json):
def _get_communities(json):
def _get_image_result(result) -> dict[str, t.Any] | None:
def _get_most_common(items: list[str | None]) -> str | None:
def _get_news_result(result):
def _get_posts(json):
def _get_request_id(query, params):
def _get_result(item: ElementBase, base_url_choice) -> dict[str, t.Any]:
def _get_results_from_process(res: EngineResults, cmd, pageno):
def _get_secret_key():
def _get_time_range_url(time_range):
def _get_tokens(dom: ElementType | None = None) -> str:
def _get_users(json):
def _get_web_result(result):
def _images_result(result):
def _image_result(result):
def _image_results(dom):
def _map_leechers(leechers: str | None, seeders: str | None, peers: str | None) -> str | None:
def _map_magnet_link(
def _map_published_date(pubDate: str | None) -> datetime | None:
def _map_result_url(guid: str | None, comments: str | None) -> str | None:
def _map_torrent_file(link: str | None, enclosure_url: str | None) -> str | None:
def _match_query(query):
def _news_result(result):
def _news_results(dom):
def _page_offset(pageno):
def _parse_date(date: str) -> datetime | None:
def _parse_date(value: str | None) -> datetime | None:
def _parse_images(json_resp: dict[str, t.Any]) -> EngineResults:
def _parse_news(resp: SXNG_Response) -> EngineResults:
def _parse_published_date(content: str) -> tuple[str, datetime | None]:
def _parse_result(item: ElementType) -> dict[str, t.Any]:
def _parse_search(resp: SXNG_Response) -> EngineResults:
def _parse_videos(json_resp: dict[str, t.Any]) -> EngineResults:
def _reconstruct_abstract(
def _result(video: dict[str, typing.Any], album_info: dict[str, typing.Any]):
def _simple_query_string_query(query):
def _story(item):
def _stringify_pages(biblio: dict[str, t.Any]) -> str:
def _strip_leading_strings(text):
def _terms_query(query):
def _term_query(query):
def _text(item: ElementType, selector: str) -> str | None:
def _video(item):
def _video_result(result):
def _weather_data(location: weather.GeoLocation, data: dict[str, t.Any]):
def _web_result(result):
def __check_query_params(params):
def __get_results_limits(pageno):
def __parse_single_result(raw_result):
default_fields = ''  # default field to query
delimiter = {}
design.  `Piped’s architecture`_ consists of 3 components:
Dictzone
difference of *last day* and *last week* in the result list is just marginally.
different on the TA side).  Searching again should now show images.
DigBT (Videos, Music, Files)
directly."""
disabled = True
Discourse is an open source Internet forum system.  To search in a forum this
display_type = ["infobox"]
distributions, it uses musl, BusyBox and OpenRC.  Alpine is mostly used on
dl_prefix: str = ""
doctype_xpath = './/div[contains(@class, "c-result__doctype")]/p'
documentation"""
documented at `lemmy-js-client`_ / `Interface Search`_.  Since Lemmy is
does not offer WEB or API access, this can be achieved using recoll-webui_
Doku Wiki
download_count_xpath = './div[@class="meta"]/span[@class="downloads"]'
DOWNLOAD_ERROR = gettext("The image could not be downloaded.")
DuckDuckGo Extra (images, videos, news)
DuckDuckGo Instant Answer API
DuckDuckGo Weather
DuckDuckGo WEB
DUMMY_ENTITY_URLS = set(
Each result uses the :ref:`result_types.paper` class and may include:
earned_xpath = './/span[@class="video-item--meta video-item--earned"]/@data-value'
Ebay (Videos, Music, Files)
either look for exact matches or use partial keywords to find what you are
Elasticsearch_ supports numerous ways to query the data it is storing.  At the
embed_url = "https://embed.nicovideo.jp"
Emojipedia is an emoji reference website which documents the meaning and
enable_http2 = False
endpoint = 'photos'
endpoints that follow this pattern::
ends.
engine offers some additional settings:
engine providing access to a variety of book resources (also via IPFS), created
engine that uses the official public API and does not require an API key.
engineered by reading the network log of https://www.qwant.com/ queries.
engineering and systems science, and economics.
engines for specific searches in Anna's Archive.  For example a engine to search
engines for specific searches in Z-Library.  For example a engine to search
engines for specific torrent searches.  For example a engine to search only for
engines in the settings.
engines:
engines` section, further read :ref:`engines-dev`.
engine_type = "offline"
engine_type = "online"
engine_type = "online_currency"
engine_type = 'offline'
engine_type = 'online'
engine_type = 'online_dictionary'
engine_type = 'online_url_search'
environment_variables = {}
Errors`_.
etc).  Brave's index only seems to support a locale, but it does not seem to
eutils_api = "https://eutils.ncbi.nlm.nih.gov/entrez/eutils"
evaluate additional headers.  For example, in the response from DDG, the
exact_match_only = False
exact_match_only = True
Example
Example full image urls (from html)
Example scenario:
Example thumbnail urls (from requests & html):
example::
except ImportError:
Explore thousands of artworks from The Art Institute of Chicago.
extent).
F-Droid (a repository of FOSS applications for Android)
facet_filters = []
failure_xpath = '/queryresult[attribute::success="false"]'
family_filter_map = {
federated, results are from many different, independent lemmy instances, and not
field_list = 'name'  # list of field names to display on the UI
FILESIZE = 3
FILESIZE_MULTIPLIER = 4
filled_regex = r"(fill)(ed)?"
filter_mapping = {0: 'images', 1: 'active', 2: 'active'}
filter_mapping = {0: 'off', 1: 'medium', 2: 'high'}
filter_types = ["fsk", "genre", "jahr", "jahrzehnt", "land", "online", "stimmung", "person"]
first_page_num = 1
fix typos, e.g. it searches for ``food`` when the user queries for ``fooh``.  In
Flickr (Images)
for *newest* articles and journals (PDF) / by shortcut ``!aaa <search-term>``.
For a list of all public filters, observe the url path when browsing
For example, the ``year:`` filter requires a *Premium Plan* subscription.
for issues):
for more efficient and comprehensive retrieval (Meta-API_ `v2`).
For Qwant's *web-search* two alternatives are implemented:
For reference see languages-subtag at iana; ``no`` is the macrolanguage [1]_ and
for searching images, videos, audio, and other files in the Wikimedia.
for storing web pages you have visited and searching in the contents later.
for the ``paddling.com`` forum:
for users, accounts or other types of content on Mastodon however.
for ``backend_url`` and ``frontend_url`` are configured for the engine).
forbids pagination without OAuth.
FORMAT_NOT_SUPPORTED = gettext(
found in :py:obj:`lang2domain` URL ``<lang>.search.yahoo.com`` is used.
Freesound (Sound)
Frinkiac (Images)
from :ref:`wikipedia engine`.
from babel import dates
from babel.dates import format_datetime, format_date, format_time, get_datetime_format
from collections import OrderedDict
from collections.abc import Iterable
from datetime import (
from datetime import date, timedelta
from datetime import datetime
from datetime import datetime, timedelta
from datetime import timedelta
from dateutil import parser
from dateutil import parser as date_parser
from dateutil.parser import isoparse
from dateutil.parser import parse
from dateutil.parser import parse as dateparse
from dateutil.relativedelta import relativedelta
from en-us to en-ca or en-gb).
from facebook (aka Meta) or Twitter (aka X).  Since these are not real links to
from flask_babel import gettext
from flask_babel import gettext  # pyright: ignore[reportUnknownVariableType]
from functools import partial
from functools import reduce
from hashlib import md5
from html import unescape
from httpx import DigestAuth
from json import dumps
from json import dumps, loads
from json import loads
from json import loads, dumps
from json import loads, JSONDecodeError
from lxml import etree
from lxml import etree  # type: ignore
from lxml import html
from lxml import html, etree
from lxml.etree import ElementBase
from lxml.etree import XPath
from lxml.html import fromstring
from MEDLINE, life science journals, and online books. Citations may include
from operator import itemgetter
from os.path import expanduser, isabs, realpath, commonprefix
from searx import locales
from searx import logger
from searx import network as _network
from searx import utils
from searx import weather
from searx.data import ENGINE_TRAITS
from searx.data import OSM_KEYS_TAGS, CURRENCIES
from searx.data import WIKIDATA_UNITS
from searx.enginelib import EngineCache
from searx.enginelib.traits import EngineTraits
from searx.engines import categories as searx_categories
from searx.engines.bing import fetch_traits  # pylint: disable=unused-import
from searx.engines.bing import set_bing_cookies
from searx.engines.bing_images import time_map
from searx.engines.duckduckgo import fetch_traits  # pylint: disable=unused-import
from searx.engines.duckduckgo import get_ddg_lang
from searx.engines.duckduckgo import get_ddg_lang, get_vqd, set_vqd
from searx.engines.google import (
from searx.engines.google import fetch_traits  # pylint: disable=unused-import
from searx.engines.google import fetch_traits as _fetch_traits  # pylint: disable=unused-import
from searx.engines.openstreetmap import get_key_label
from searx.engines.peertube import (
from searx.engines.peertube import fetch_traits  # pylint: disable=unused-import
from searx.engines.wikidata import send_wikidata_query, sparql_string_escape, get_thumbnail
from searx.engines.wikipedia import (
from searx.engines.xpath import extract_text
from searx.engines.xpath import extract_url, extract_text, eval_xpath_list, eval_xpath
from searx.engines.yahoo import parse_url
from searx.exceptions import (
from searx.exceptions import SearxEngineAccessDeniedException
from searx.exceptions import SearxEngineAccessDeniedException, SearxEngineException
from searx.exceptions import SearxEngineAPIException
from searx.exceptions import SearxEngineAPIException, SearxEngineCaptchaException
from searx.exceptions import SearxEngineAPIException, SearxEngineXPathException
from searx.exceptions import SearxEngineCaptchaException
from searx.exceptions import SearxEngineCaptchaException, SearxEngineAccessDeniedException
from searx.exceptions import SearxEngineXPathException
from searx.exceptions import SearxException
from searx.extended_types import SXNG_Response
from searx.external_bang import EXTERNAL_BANGS, get_node  # type: ignore
from searx.external_urls import (
from searx.external_urls import get_external_url
from searx.external_urls import get_external_url, get_earth_coordinates_url, area_to_osm_zoom
from searx.locales import language_tag
from searx.locales import language_tag, region_tag
from searx.locales import language_tag, region_tag, get_official_locales
from searx.locales import region_tag
from searx.locales import region_tag, language_tag
from searx.network import get
from searx.network import get  # see https://github.com/searxng/searxng/issues/762
from searx.network import get as http_get
from searx.network import get as http_get  # https://github.com/searxng/searxng/issues/762
from searx.network import get, raise_for_httperror  # see https://github.com/searxng/searxng/issues/762
from searx.network import post
from searx.network import post as http_post
from searx.network import post, get
from searx.network import raise_for_httperror
from searx.result_types import EngineResults
from searx.result_types import EngineResults, MainResult
from searx.result_types import EngineResults, WeatherAnswer
from searx.result_types import MainResult, KeyValue
from searx.utils import (
from searx.utils import ecma_unescape, html_to_text
from searx.utils import eval_xpath, eval_xpath_list, eval_xpath_getindex
from searx.utils import eval_xpath, eval_xpath_list, extract_text
from searx.utils import eval_xpath, eval_xpath_list, extract_text, searxng_useragent
from searx.utils import eval_xpath, extract_text
from searx.utils import eval_xpath, extract_text, eval_xpath_list, eval_xpath_getindex
from searx.utils import eval_xpath_getindex, eval_xpath_list, eval_xpath, extract_text
from searx.utils import eval_xpath_getindex, html_to_text
from searx.utils import eval_xpath_list
from searx.utils import eval_xpath_list, eval_xpath_getindex, eval_xpath, extract_text
from searx.utils import extr
from searx.utils import extr, extract_text, eval_xpath, eval_xpath_list
from searx.utils import extr, extract_text, eval_xpath, gen_useragent, html_to_text, humanize_bytes, remove_pua_from_str
from searx.utils import extract_text
from searx.utils import extract_text, eval_xpath
from searx.utils import extract_text, eval_xpath, eval_xpath_getindex, eval_xpath_list
from searx.utils import extract_text, eval_xpath, eval_xpath_list
from searx.utils import extract_text, eval_xpath, eval_xpath_list, ElementType
from searx.utils import extract_text, eval_xpath, eval_xpath_list, eval_xpath_getindex
from searx.utils import extract_text, eval_xpath, eval_xpath_list, eval_xpath_getindex, gen_gsa_useragent
from searx.utils import extract_text, eval_xpath_list, eval_xpath_getindex
from searx.utils import extract_text, eval_xpath_list, eval_xpath_getindex, searxng_useragent
from searx.utils import extract_text, extract_url, eval_xpath, eval_xpath_list
from searx.utils import extract_text, html_to_text, get_string_replaces_function
from searx.utils import extract_text, int_or_zero
from searx.utils import gen_useragent
from searx.utils import gen_useragent, ElementType
from searx.utils import gen_useragent, html_to_text, parse_duration_string
from searx.utils import get_embeded_stream_url
from searx.utils import get_embeded_stream_url, html_to_text, gen_useragent, extr
from searx.utils import html_to_text
from searx.utils import html_to_text, get_embeded_stream_url
from searx.utils import html_to_text, humanize_bytes
from searx.utils import html_to_text, humanize_number
from searx.utils import humanize_bytes
from searx.utils import humanize_bytes, eval_xpath, eval_xpath_list, extract_text, extr
from searx.utils import humanize_number
from searx.utils import markdown_to_text
from searx.utils import searxng_useragent
from searx.utils import searxng_useragent, get_string_replaces_function
from searx.utils import to_string, html_to_text
from shlex import split as shlex_split
from subprocess import Popen, PIPE
from threading import Thread
from time import time
from unicodedata import normalize, combining
from urllib.parse import (
from urllib.parse import parse_qs, urlencode, urlparse
from urllib.parse import quote
from urllib.parse import quote, urljoin
from urllib.parse import quote_plus
from urllib.parse import quote_plus, urlencode
from urllib.parse import quote_plus, urlparse
from urllib.parse import urlencode
from urllib.parse import urlencode, quote
from urllib.parse import urlencode, unquote
from urllib.parse import urlencode, urljoin
from urllib.parse import urlencode, urljoin, urlparse
from urllib.parse import urlencode, urlparse
from urllib.parse import urlencode, urlparse, parse_qs
from urllib.parse import urlencode, urlparse, urljoin
from urllib.parse import urlencode, urlparse, urlunparse, parse_qsl
from urllib.parse import urljoin
from urllib.parse import urlunparse
from __future__ import annotations
frontend_url: str | None = None
future.
gallery_url = 'https://gallery.1x.com/'
General assumptions regarding DDG's bot blocker:
geo_url = "https://geocoding-api.open-meteo.com"
ghc_api_version = "2022-11-28"
ghc_auth = {
ghc_highlight_matching_lines = True
ghc_insert_block_separator = False
ghc_strip_new_lines = True
ghc_strip_whitespace = False
GitHub does not return the code line indices alongside the code fragment in the
globe.
Goggles allow you to choose, alter, or extend the ranking of Brave Search
Goggles: str = ""
Google API used by the Google Go Android app.
Google News has a different region handling compared to Google WEB.
governmental institution aiming to reduce misinformation by providing resources
graphql_query = """query SearchProductExplorer($query: String, $offset: Int, $limit: Int,
graphql_url = 'https://apollo.senscritique.com/'
GROUP BY ?item ?itemLabel ?itemDescription ?lat ?long %GROUP_BY%
have any official API, but it uses JSON requests internally to fetch search
have to set these values in both requests we send to Presearch; in the first
headers = {}
Here is a simple example of a JSON engine configure in the :ref:`settings
Here is a simple example of a XPath engine configured in the :ref:`settings
Here is a simple example to query a Meilisearch instance:
hide_obsolete = False
host = "127.0.0.1"
host = '127.0.0.1'
However, please note that the available options depend on the subscription type.
href_base = 'https://imdb.com/{category}/{entry_id}'
HTML frontend of the common WEB site.
HTML ``<form>``, HTTP-Headers & DDG's bot Blocker:
HTTP ``Accept-Language`` header (``send_accept_language_header``):
http_digest_auth_pass = ""
http_digest_auth_user = ""
huggingface_endpoint = 'models'
i18n_book_rating = gettext("Book rating")
i18n_file_quality = gettext("File quality")
i18n_language = gettext("Language")
If none of the methods fit your use case, you can select ``custom`` query type
If one of this argument is not set correctly, the request is redirected to
if t.TYPE_CHECKING:
If the forum is private, you need to add an API key and username for the search:
If there is any privacy concerns about generating a token, one can use the API
If you want to offer the engine, the ``inactive`` flag must be set to ``false``.
If you want to use invidious with SearXNG you should setup one locally.
If you would like to use additional instances, just configure new engines in the
iframe_src = "https://bandcamp.com/EmbeddedPlayer/{type}={result_id}/size=large/bgcol=000/linkcol=fff/artwork=small"
iframe_src = "https://www.dailymotion.com/embed/video/{video_id}"
iframe_src = "https://www.deezer.com/plugins/player?type=tracks&id={audioid}"
iframe_src = "https://www.mixcloud.com/widget/iframe/?feed={url}"
images, videos, news).
images.  The news area has also been harmonized with the other categories.  Only
image_api = "https://www.artic.edu/iiif/2/"
image_api = 'https://www.artic.edu/iiif/2/'
image_img_src_xpath = './a/img/@src'
image_pods = {'VisualRepresentation', 'Illustration', 'Symbol'}
image_pods = {'VisualRepresentation', 'Illustration'}
image_results_xpath = '//div[@id="results"]/div[contains(@class, "image")]'
image_sizes = ('o', 'k', 'h', 'b', 'c', 'z', 'm', 'n', 't', 'q', 's')
image_title_xpath = './a/@data-title'
image_url = "https://assets.cdn.moviepilot.de/files/{image_id}/fill/155/223/{filename}"
IMAGE_URL = '{base}img/{episode}/{timestamp}.jpg'
image_url_xpath = './a/@href'
image_xpath = './img'
imagination can limit the power of this engine (and maybe security concerns).
img_alt_xpath = './@alt'
img_src_url = "https://fonts.gstatic.com/s/i/short-term/release/materialsymbolsoutlined/{icon_name}/{svg_type}/24px.svg"
img_src_xpath = ".//img/@src"
img_src_xpath = './@src'
img_src_xpath = './div/img/@srcset'
Implementation
Implementations
implementations (manly the :py:obj:`get_google_info`) are shared by other
implementations are shared by other engines:
import babel
import babel.core
import babel.languages
import babel.localedata
import base64
import cloudscraper
import contextlib
import datetime
import dateutil
import dateutil.parser
import flask_babel
import html
import httpx
import isodate
import json
import lxml
import lxml.etree
import lxml.html
import pathlib
import random
import re
import secrets
import socket
import sqlite3
import string
import time
import typing
import typing as t
import urllib.parse
import valkey  # pylint: disable=import-error
In  DDG's bot blocker, the IP will be blocked (DDG does not have a client session!)
in binary packages or can be built directly from sources with the help of the
In order to query MongoDB_, you have to select a ``database`` and a
In Presearch there are languages for the UI and regions for narrowing down the
in SearXNG to get the translations of data such as *"published last week"*.
in SearXNG, the spellchecking is disabled by default.
in section :ref:`private engines`.  The engine base is flexible.  Only your
In the :py:obj:`EngineTraits object <searx.enginelib.traits.EngineTraits>` the
in the API to get a better picture of bing, but the value specifications like
in the browser.  The default behavior is to link into TubeArchivist's interface
In the category ``news`` you can additionally filter by option
In the example below, all three ChinaSO engines are using the :ref:`network
In the list of regions there are tags we need to map to common region tags::
In the meantime while work is done on the TA side, this can be worked around by
in the torrent file.  It's a tradeoff for a "stable" engine as the XML from RSS
in use from the DDG search engine.
in which case each entry is considered -
in ``settings.yml``:
INA (Videos)
including Prowlarr_ and Jackett_.
index = ''
info box.  Both values can be set, or one of the two can be set."""
info_text_xpath = ".//span[contains(@class, 'uitext')]"
input_xpath = '//pod[starts-with(attribute::id, "Input")]/subpod/plaintext'
instance by correcting spelling errors.
instances, which can be chosen instead by modifying ``base_url``.
instance_index = 0
instance_urls = []
instead of searching in indices, you can search in collections.
Institute of Chicago <https://www.artic.edu>`_
Intro page: https://html.duckduckgo.com/html/
is based on the *nextpage* method of Piped's REST API / the :py:obj:`frontend
is mostly the same language in a different script.
is passed through, the `search syntax`_ of ADS can be used (at least to some
is relied upon by thousands of systems across the research ecosystem and the
is set, e.g. ``POST``.  For formatting see the documentation of :py:obj:`search_url`::
is set, e.g. ``POST``. For formatting see the documentation of :py:obj:`search_url`.
it as you wish.
it doesn't support formats such as "N time ago", "vor N time" (German),
It is **strongly** recommended first setting up the intial connection and
Jisho (the Japanese-English dictionary)
JSON query:
key = None
Key features
KEY_EXPIRATION_SECONDS = 3600
KEY_ORDER = [
KEY_RANKS = {k: i for i, k in enumerate(KEY_ORDER)}
Known Quirks
lang2domain = {
LANGUAGE and COUNTRY/REGION).  The Language is the language of the UI, we need
language and the region) from the ``Accept-Language`` header.
Languages & Regions
Languages are supported by mapping the language to a domain.  If domain is not
language_param = 'lb'
language_support = False
language_support = True
lang_all = 'en'
lang_map = locales.LOCALE_BEST_MATCH.copy()
lang_map.update(
least we could not find out how language support should work.  It seems that
lemmy_type = "Communities"
length_xpath = './/span[@class="video-item--duration"]/@data-value'
license_name_xpath = './div[contains(@class, "SearchSnippet-infoLabel")]/span[@data-test-id="snippet-license"]/a/text()'
license_url_xpath = './div[contains(@class, "SearchSnippet-infoLabel")]/span[@data-test-id="snippet-license"]/a/@href'
limit = 10
linked_terms = OrderedDict(
linked_terms = {
links to full text content from PubMed Central and publisher web sites.
list in ``settings.yml``:
list of URLs.  In the latter case instance will be selected randomly.  For a
list_of_wikipedias = 'https://meta.wikimedia.org/wiki/List_of_Wikipedias'
locales.  To get a mapping, all *officiat de-facto* languages of the Brave
Lofgren .
looking for by configuring ``exact_match_only``.
mailto = ""
main_wiki = 'wiki.archlinux.org'
mastodon_type = "accounts"
matrix_url = "https://matrix.to"
max_page = 10
max_page = 18
max_page = 200
max_page = 5
max_page = 50
max_result_count = 50
media files to which anyone can contribute.
MeiliSearch_ is aimed at individuals and small companies.  It is designed for
method = 'GET'
Minimal example for :origin:`settings.yml <searx/settings.yml>`:
mode.
modelexport_re = re.compile(r"^\s*modelExport:\s*({.*}),$", re.M)
moment the engine supports the most popular search methods (``query_type``):
more details.
More info on api-key : https://www.flickr.com/services/apps/create/
most common German shopping sites and find the lowest price.
most implementations will default to the first entry in this case.
most of its books originate.
most of the features are based on English terms.
mount_prefix: str = ""
Moviepilot additionally allows to discover movies by certain categories
Movies and sort the result list by the count of seeders.
mozhi_engine = "google"
Multiple different formats can be supported by using ``dateutil`` parser, but
music_player = 'https://genius.com{api_path}/apple_music_player'
must be escaped by doubling each ``{`` and ``}``.
MySQL engine, you must install the package ``mysql-connector-python``.
named `piped` and are used by all ``piped`` engines (unless an individual values
name_token_xpath = '//form[@id="searchForm"]/input[@type="hidden"]/@name'
naver_category = "general"
naver_category_dict = {
nb_per_page = 10
nb_per_page = 15
nb_per_page = 20
need to be adjusted.
news and media reports are available in JSON format.  The `Bundesstelle für Open
news, videos, images).  The support of :py:obj:`paging` and :py:obj:`time range
news_content_xpath = './/p[@class="s"]'
news_results_xpath = '//section[contains(@class, "news-search-result")]//article'
news_title_xpath = './/h2/a'
news_url_xpath = './/h2/a/@href'
next_page_url = 'https://www.youtube.com/youtubei/v1/search?key=AIzaSyAO_FJ2SlqU8Q4STEHLGCilw_Y9_11qcW8'
No public instance offer a public API now
no-JS variant (DDG-html)"""
none filters are applied. Valid filters are:
Note: Curly brackets which aren't encapsulating a replacement placeholder
Notes
no_result_for_http_status = []
NO_SIGNATURE_ERROR = gettext(
number, but an offset.'''
number_of_results = 10
number_of_results = 20
number_of_results = 20  # Don't put this over 5000
number_of_results = 5
number_of_results_xpath = '//*[@id="totalResults"]'
Observatory (SAO) under a NASA grant.  The ADS_ is a solr instance, but not with
of Brave Search users.
of the collection.  Furthermore, you can search in MeiliSearch_ instances that
of the encyclopedia's collaborative quality, showing how frequently its articles
On the `preference page`_ Bing offers a lot of languages an regions (see section
one will add a hit to the result list.  The first one will show a hit in the
only for EPUB from 2010 to 2020.
only the official one.
open resources for the global research community.  `Semantic Scholar`_ index
OpenAlex offers a free public API with generous daily limits. For extra courtesy
Openverse (formerly known as: Creative Commons search engine) [Images]
operating out of Sweden. It is principally developed and operated by Viktor
operating system, based on the monolithic Linux kernel. Its package system
Optional settings are:
Optional settings:
options:
or filters, hence we provide the following syntax:
or ``time``.
order = "desc"
ORDER by ?item
other related information.
over 200 million academic papers sourced from publisher partnerships, data
package_name_xpath = './div[@class="SearchSnippet-headerContainer"]/h2/a/span/text()'
page for every book ever published.
page, then bing returns the results from the last page again."""
pagesize = 10
page_size = 1
page_size = 10
page_size = 12
page_size = 16
page_size = 20
page_size = 25
page_size = 40
page_size = 5
page_size: int = 10
paging = False
paging = True
paging = True  # paging is only supported for general search
Paging:
paging: bool = False
paging: bool = True
paper results using the :ref:`result_types.paper` class.  It is an "online" JSON
parse = {'lyric': parse_lyric, 'song': parse_lyric, 'artist': parse_artist, 'album': parse_album}
parse_regex = {}
password = ""
password = ''
PDBe (Protein Data Bank in Europe)
pdbe_entry_url = 'https://www.ebi.ac.uk/pdbe/entry/pdb/{pdb_id}'
pdbe_preview_url = 'https://www.ebi.ac.uk/pdbe/static/entry/{pdb_id}_deposited_chain_front_image-200x200.png'
pdbe_solr_url = 'https://www.ebi.ac.uk/pdbe/search/pdb/select?'
pdb_unpublished_codes = ['HPUB', 'HOLD', 'PROC', 'WAIT', 'AUTH', 'AUCO', 'REPL', 'POLC', 'REFI', 'TRSF', 'WDRN']
pdia_base_url = 'https://pdimagearchive.org'
pdia_config_end = ".js"
pdia_config_start = "/_astro/InfiniteSearch."
peer-to-peer (P2P) networks.
peertube engines.
Photon (Map)
photo_url = 'https://www.flickr.com/photos/{userid}/{photoid}'
piped_filter = 'all'
Piratebay (Videos, Music, Files)
pixabay_type = "images"  # alternative: 'videos'
pixiv_image_proxies: list[str] = []
pkg_repo_url = "https://github.com/void-linux/void-packages"
plaintext_xpath = './plaintext'
play_categ = None  # apps|movies
pods_xpath = '//pod'
pod_id_xpath = './@id'
pod_primary_xpath = './@primary'
pod_title_xpath = './@title'
political adjustments still seem to be made -- for example, there is no news
popularity_xpath = './div[contains(@class, "SearchSnippet-infoLabel")]/a/strong/text()'
port = "5432"
port = 27017
port = 3306
port = 6379
Portal (for resources and resource groups).
Possible use-case: To set safesearch cookie or header to moderate.'''
Possible use-case: To set safesearch cookie.'''
PR-2554_:
premium_keytext = 'Watch the artist to view this deviation'
premium_xpath = '../div/div/div/text()'
prerequities.
price_xpath = './/div[contains(@class, "s-item__detail")]/span[@class="s-item__price"][1]/text()'
project for file-sharing access to scholarly journal articles, academic texts
providers, and web crawls.
publishedDate_xpath = './/div[contains(@class,"dateAgenda")]'
published_date = './/time[@class="video-item--meta video-item--time"]/@datetime'
publish_date_xpath = './/span[contains(@class, "flex items-center")]/@title'
pubmed_url = "https://www.ncbi.nlm.nih.gov/pubmed/"
pull images from TA (as there is no way to pass cookies in a URL string only).
pymongo_.
quark_category = 'general'
Query field:
Query to test: ``!mediathekview concert``
query_data_template = {
query_enum = []
query_fields = ''  # query fields
QUERY_PROPERTY_NAMES = """
query_str = ""
QUERY_TEMPLATE = """
query_type = ''
query_type = 'match'
qwant_categ = None
qwant_news_locales = [
Rate limits & polite pool
really fit into SearXNG's UI to select a page by number.
Recoll_ is a desktop full-text search tool based on Xapian.  By itself Recoll_
redirect"""
referer_url = url + 'input/?{query}'
Referrer-Policy_ is always set to ``origin``.  A real browser would then include
region are mapped to regions in SearXNG (see :py:obj:`babel
region2domain = {
region_param = 'arc'
Releases:
release_year_xpath = "concat('From ', string(./@releaseyear))"
remove_ai_images = False
Replacements are:
replace_http_by_https = get_string_replaces_function({"http:": "https:"})
replace_http_by_https = get_string_replaces_function({'http:': 'https:'})
Repology_ shows you in which repositories a given project is packaged, which
request to get the request-ID from Presearch and in the final request to get the
request.
Request:
requested by the user, the URL parameter is an empty string.  The
request_body = ''
require authentication by setting `auth_key`_.
research community with official Springer-API_ (API-Playground_).
research from repositories and journals.
researchers in astronomy and physics, operated by the Smithsonian Astrophysical
Response:
rest_v1_summary_url = 'https://{wiki_netloc}/api/rest_v1/page/summary/{title}'
result list.
results (`Goggles Whitepaper`_).  Goggles are openly developed by the community
results and suggestions, that's being used in this implementation.
results_per_page = 10
results_per_page = 15
results_per_page = 20
results_per_page = 30
results_per_page: int = 20
results_query = ''
results_xpath = "//div[@class='style_nodeListing__7Nmro']/div"
results_xpath = "//div[contains(@class, 'cards')]/div[contains(@class, 'post')]"
results_xpath = "//search-page-media-row"
results_xpath = "//table//tr"
results_xpath = ''
results_xpath = '//div[@class="V_S0t_"]/div/div/a'
results_xpath = '//div[@id="searchHits"]/div'
results_xpath = '//div[contains(@class, "l-content-wrapper")]/div[contains(@class, "row")]/div[contains(@class, "column")]/div[contains(@class, "c-result"){extra}]'
results_xpath = '//li[@class="result"]'
results_xpath = '//li[@data-video-item]'
results_xpath = '//li[@x-test-model]'
results_xpath = '//li[contains(@class, "s-item")]'
results_xpath = '//li[contains(@class, "serp-item")]'
results_xpath = '//ul[@class="results-standard"]/li/a[@class="ob"]'
results_xpath = '/html/body/main/div/ol/li/a'
results_xpath = '/html/body/main/div[contains(@class,"SearchResults")]/div[not(@class)]/div[@class="SearchSnippet"]'
results_xpath_filter_recommended = " and not(contains(@class, 'c-result--recommended'))"
result_base_url = 'https://openstreetmap.org/{osm_type}/{osm_id}'
result_fields = [
result_id_url = 'https://openstreetmap.org/{osm_type}/{osm_id}'
result_lat_lon_url = 'https://www.openstreetmap.org/?mlat={lat}&mlon={lon}&zoom={zoom}&layers=M'
result_separator = '\n'
result_type: t.Literal["MainResult", "KeyValue"] = "KeyValue"
result_url = "https://fonts.google.com/icons?icon.query={query}&selected=Material+Symbols+Outlined:{icon_name}:FILL@0{fill};wght@400;GRAD@0;opsz@24"  # pylint: disable=line-too-long
RESULT_URL = '{base}?{query}'
Retrieves results from a basic search.  Advanced search options are not
reverse engineering we can see that some services (e.g. instant answers) still
rewrite the query into another which is thought to provide better results, for
RE_DATA_IMAGE = re.compile(r'"(dimg_[^"]*)"[^;]*;(data:image[^;]*;[^;]*);')
RE_DATA_IMAGE_end = re.compile(r'"(dimg_[^"]*)"[^;]*;(data:image[^;]*;[^;]*)$')
re_transliteration_unsupported = "Direction '.*' is not supported"
route_url = 'https://graphhopper.com/maps'
rows = 10
rumbles_xpath = './/span[@class="video-item--meta video-item--rumbles"]/@data-value'
Safe-Search:
safesearch = False
safesearch = True
safesearch: bool = True
safesearch_args = {0: "1", 1: None, 2: "1"}
safesearch_cookies = {0: "-2", 1: None, 2: "1"}
safesearch_dict = {0: '1', 1: '0', 2: '0'}
safesearch_dict = {0: 'p', 1: 'i', 2: 'r'}
safesearch_map = {0: '111', 1: '110', 2: '100'}
safesearch_map = {0: 'false', 1: 'true', 2: 'true'}
safesearch_map = {0: 'off', 1: '1', 2: '1'}
safesearch_map = {0: 'off', 1: 'moderate', 2: 'strict'}
safesearch_map = {2: "strict", 1: "moderate", 0: "off"}  # cookie: safesearch=off
safesearch_params = {
safesearch_table = {0: 'both', 1: 'false', 2: 'false'}
safe_search_map = {0: '&filter=none', 1: '&filter=moderate', 2: '&filter=strict'}
safe_search_support = False
ScanR Structures (Science)
science, quantitative biology, quantitative finance, statistics, electrical
score_xpath = "concat('Score: ', string(./@tomatometerscore))"
sc_code_cache_sec = 3600
search API. Since these are not super important for the user experience all the
Search examples:
Search options:
search query`_ in the documentation of GitHub's REST API.
Search type ``video``
Search URL of the engine.  Example::
search, if this is set to true, the blocks will be separated with ``...`` line.
search.  If we set "auto" for the region in the WEB-UI of Presearch and cookie
search.  You can do that by uploading an image or searching by URL. You can also
searched.  MongoDB_ also supports the option ``exact_match_only``, so configure
searched."""
searching for your hosted videos.
search_api = "https://api.artic.edu/api/v1/artworks/search"
search_api = "https://api.ilpost.org/search/api/site_search/?"
search_api = "https://api.npms.io/v2/search?"
search_api = "https://learn.microsoft.com/api/search?"
search_api = "https://openlibrary.org/search.json"
search_api = "https://searchcode.com/api/codesearch_I/?"
search_api = 'https://api.artic.edu/api/v1/artworks/search?'
search_api = 'https://api.stackexchange.com/2.3/search/advanced?'
search_api = 'https://www.ansa.it/ricerca/ansait/search.shtml?'
search_categories = {"nm": "name", "tt": "title", "kw": "keyword", "co": "company", "ep": "episode"}
search_dir: str = ""
search_endpoint = '/search.json'
search_form_xpath = '//form[@id="search"]'
search_mode = 'global'
search_path_map = {"images": "i", "videos": "v", "news": "news"}
search_string = "/{endpoint}/?sp={page}&{query}&fo=json"
search_string = '/api/v1/result_json/?page={page}&{query}'
search_string = '?page={page}&page_size={nb_per_page}&format=json&{query}'
search_string = 'api/?{query}&limit={limit}'
search_string = 'query?q={query}'
search_string = 'search?{query}&page={page}'
search_string = 'search?{query}&polygon_geojson=1&format=jsonv2&addressdetails=1&extratags=1&dedupe=1'
search_type = ""
search_type = ""  # leave blank for general, other possible values: images, news
search_type = "search"
search_type = "web"  # 'web', 'images', 'news'
search_type = 'text'
search_type: str = 'nearmatch'
search_types = {"files": "0", "music": "100", "videos": "200"}
SEARCH_TYPES: dict[str, str] = {
search_url = "https://9gag.com/v1/search-posts?{query}"
search_url = "https://api-v2.soundcloud.com/search"
search_url = "https://api.apple-mapkit.com/v1/search?{query}&mkjsVersion=5.72.53"
search_url = "https://api.crossref.org/works"
search_url = "https://api.openalex.org/works"
search_url = "https://apibay.org/q.php?q={search_term}&cat={search_type}"
search_url = "https://crates.io/api/v1/crates"
search_url = "https://fonts.google.com/metadata/icons?key=material_symbols&incomplete=true"
search_url = "https://hex.pm/api/packages/"
search_url = "https://www.semanticscholar.org/api/1/search"
search_url = '/sch/i.html?_nkw={query}&_sacat={pageno}'
search_url = 'http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion/search/?{query}'
search_url = 'https://api.dailymotion.com/videos?'
search_url = 'https://api.github.com/search/code?sort=indexed&{query}&{page}'
search_url = 'https://api.github.com/search/repositories?sort=stars&order=desc&{query}'
search_url = 'https://api.wolframalpha.com/v2/query?appid={api_key}&{query}'
search_url = 'https://itunes.apple.com/search?{query}'
search_url = 'https://www.flickr.com/search?{query}&page={page}'
SEARCH_URL = '{base}api/search?{query}'
search_url = (
search_url = base_url + "/search"
search_url = base_url + "/store/search?{query}&c={play_categ}"
search_url = base_url + '/?post_type=app_release&searchtype=apk&page={pageno}&{query}'
search_url = base_url + '/ajax/recherche?{query}&espace=1&sort=pertinence&order=desc&offset={start}&modified=size'
search_url = base_url + '/backend/search.php?{query}'
search_url = base_url + '/search/?{query}'
search_url = base_url + '/search?{query}'
search_url = base_url + '/sp/search'
search_url = base_url + '?part=snippet&{query}&maxResults=20&key={api_key}'
search_url = base_url + '?search_query={query}&page={page}'
search_url = base_url + '?{query}'
search_url = base_url + 'napi/search/photos?'
search_url = base_url + 'search.json?{query}'
search_url = base_url + 'search.php?{query}'
search_url = base_url + 'search/page:{pageno}?{query}'
search_url = base_url + 'suchen/dudenonline/{query}?search_api_fulltext=&page={offset}'
search_url = f"{base_url}/SiteGlobals/Forms/Suche/Expertensuche_Formular.html"
search_url = None
SEARCH_URL = URL + '/api/v1/search/words?{query}'
search_url = url + '/handlers/music-search.jsx'
SEARCH_URL = URL + '/search/{query}-time-{pageno}'
search_url = url + '/search?q={search_term}&orderby={order_by}&category={category}&p={pageno}&page=rss'
search_url = url + '/search?q={search_term}&p={pageno}'
search_url = url + 'api/structures/search'
search_url = url + 'search/?{query}&type=cloudcast&limit=10&offset={offset}'
search_url = url + 'search/{index}?{query}&page={pageno}&per_page={page_size}'
search_url = url + 'search/{search_term}/{pageno}/'
search_url = url + 'search?{query}&index={offset}'
search_url = url + 'v1/search?{query}&type=track&offset={offset}'
search_url = urlunparse(["https", "fastapi.metacpan.org", "/v1/file/_search", "", "", ""])
Searx (all)
SearXNG needs in the response to build a video result list.
Sec-Fetch-Mode_:
seconds."""
SECRET_KEY_DB_KEY = "secret-key"
SECRET_KEY_RE = re.compile('"secret-key":\b*"(.*?)"')
See ``srenablerewrites`` argument in `list=search`_ documentation.
See ``srprop`` argument in `list=search`_ documentation.
See ``srwhat`` argument in `list=search`_ documentation.
See: `OpenAlex API overview`_.
select ?item ?itemLabel ?image ?sign ?symbol ?website ?wikipediaName
SELECT ?item ?itemLabel ?itemDescription ?lat ?long %SELECT%
SELECT ?item ?name
Select a database to search in and set its index in the option ``db``.  You can
Select from the `list of Goggles`_ people have published, or create your own
selected randomly.
selected.
servers and for Docker images.
shell commands.
shipping_xpath = './/span[contains(@class, "s-item__shipping")]/text()'
shortcut = "cpan"
shortcut_dict = {
show_avatar = False
show_magnet_links: bool = True
show_metadata = False
show_torrent_files: bool = False
simply drag and drop your images to start your search.  TinEye constantly crawls
since 2021. Emojipedia is a voting member of The Unicode Consortium.[1]
Since it's federated and self-hostable, there's a large amount of available
Since the region is already "auto" by default, we only need to set the
Since the search term is passed 1:1 to the API, SearXNG users can use the
Single file might return multiple code fragments.
Single file might return multiple code fragments. Enabling this might break
site_url = 'https://www.wolframalpha.com/input/?{query}'
skip_countries = [
skip_premium = True
small (and its not clear to me where the difference in UI is when switching
small-scale (less than 10 million documents) data collections.  E.g. it is great
soft_max_redirects = 0
Solr_ is a popular search engine based on Lucene, just like Elasticsearch_.  But
sort = ''  # sorting: asc or desc
sort: str = "updated"
sort_criteria = "recent_downloads"
sort_order = "relevance"
sort_order = 'relevance'
sort_order_map = {
SORT_RE = re.compile(r"sort:(\w+)")
soundcloud_facet = "model"
sourcehut_sort_order: str = "recently-updated"
source_country_xpath = './/span[contains(@class, "s-item__location")]/text()'
SPARQL_ENDPOINT_URL = 'https://query.wikidata.org/sparql'
SPARQL_EXPLAIN_URL = 'https://query.wikidata.org/bigdata/namespace/wdq/sparql?explain'
sparql_string_escape = get_string_replaces_function(
Special features of the no-JS services (DDG-lite & DDG-html):
specific fields, the ``fields`` parameter is used with the list of fields
SQLite engine download the database:
srenablerewrites: bool = True
srprop: str = 'sectiontitle|snippet|timestamp|categorysnippet'
srsort: str = 'relevance'
Startpage categories
Startpage languages
Startpage regions
Startpage's category (for Web-search, News, Videos, ..) is set by
startpage_categ = 'web'
station_filters = []  # ['countrycode', 'language']
Steps to enable **unauthenticated** metadata access for channels and videos:
still in beta and hence this implementation will need to be updated once beta
subject to additional bot detection mechanisms and breaking changes in the
subpods_xpath = './subpod'
suggestion_query = ''
suggestion_url = "https://v2.sg.media-imdb.com/suggestion/{letter}/{query}.json"
suggestion_xpath = ''
suggestion_xpath = '//div[@class="top-info"]/p[@class="top-info spell"]/em/a'
suggestion_xpath = '//div[contains(@class, "ouy7Mc")]//a'
support any languages in its index.  The choice of available languages is very
supported.  IMDB's API is undocumented, here are some posts about:
supported_languages = ['de', 'en', 'fr', 'it']
system designed to be small, simple and secure.  Contrary to many other Linux
tags_xpath = './div[@class="meta"]/span[contains(@class, "k")]/text()'
ta_link_to_mp4: bool = False
ta_token: str = ""
Terms / phrases that you keep coming across:
text or metadata of scholarly literature across an array of publishing formats
that can be used for own projects (e.g. apps, websites).
That's why we use tootfinder.ch for finding posts, which doesn't support searching
The :py:obj:`backend_url` and :py:obj:`frontend_url` has to be set in the engine
The :py:obj:`base_url` has to be set in the engine named `yacy` and is used by
the API key in the engine :ref:`core engine config`."""
The API supports paging and time filters.
the API we can use or that bing itself would use.  You can look up some things
the article linked by :py:obj:`list_of_wikipedias`.
The authentication plugin is configurable by setting ``auth_plugin`` in the
the classic measurement of article count were realized.
The cleaned thumbnail url will have THUMBNAIL_SUFFIX added to them, based on the original thumbnail parameters
The default path should work fine usually.
the dependency valkey_.
The easiest solution is to limit the access by setting ``tokens`` as described
The engine has the following (additional) settings:
The engine has the following additional settings:
The engine has the following mandatory setting:
The engine has the following mandatory settings:
The engine has the following required settings:
The engine has the following settings:
The engine is inactive by default, meaning it is not available in the service.
The engine supports faceted search, so you can search in a subset of documents
The engine uses the REST Meta-API_ `v2` endpoint, but there is also a `Python
The engine uses the `arXiv API`_.
the engine).
The example engine below can be used to find files with a specific name in the
The fields of the html-form are reverse-engineered from DDG-html and may be
the following header in the next request::
The following HTTP headers are being evaluated (and may possibly be responsible
The following is an example configuration for an Elasticsearch_ instance with
The following options are available:
The google news API ignores some parameters from the common :ref:`google API`:
The implementation to support :py:obj:`paging <searx.enginelib.Engine.paging>`
The key/value pairs ``df`` and ``kl`` are additional saved in the cookies,
The list of supported languages is :py:obj:`fetched <fetch_wikimedia_traits>` from
The locale code ``no_NO`` from Startpage does not exists and is mapped to
the market codes are usually outdated or at least no longer used by bing itself.
The market codes have been harmonized and are identical for web, video and
The personal access token or a bearer for an org or a group can be generated [in
the PostgreSQL engine, you must install the dependency ``psychopg2``.
The query string is a slash `/` separated path of JSON key names.
The results in the video category are most often links to pages that contain a
The RSS feed provides fewer data like amount of seeders/leechers and the files
The service sometimes takes a very long time to respond, the ``timeout`` may
The sorting of the search results can be influenced by the following additions
the standard API paths.
The Tagesschau is a news program of the ARD.  Via the `Tagesschau API`_, current
The thumbnail url from the request will be cleaned for the full image link
The time format returned by Presearch varies depending on the language set.
the UI of Brave the user gets warned about this, since we can not warn the user
the web and adds images to its index.  Today, the TinEye index is over 50.2
The `DDG-API <https://duckduckgo.com/api>`__ is no longer documented but from
the `GitHub settings`_.
the `GitLab REST API`_.
the `MediaWiki Action API`_.  For a `query action`_ all Wikimedia wikis have
There is a description of the official search-APIs_, unfortunately this is not
these Wikipedias have a LanguageConverter_ enabled
This engine connects with a self-hosted instance of `Tube Archivist`_ to allow
This engine does not parse the HTML page because there is an API in XML (RSS).
This engine uses the `MediaWiki query API`_, with which engines can be configured
This engine uses the `search/query`_ API endpoint.  Since the user's search term
This implementation does not use a official API: Mediawiki provides API, but
This implementation is used by different lemmy engines in the :ref:`settings.yml
This implementation is used by different qwant engines in the :ref:`settings.yml
This internal API offer results in
This is an example configuration for querying a MariaDB server:
This is an example configuration for querying a MySQL server:
This is an example configuration for searching in the collection
This is located in the main tubearchivist docker container at::
This might break the lexer and thus result in the lack of code highlighting.
This SearXNG engine uses the `/api2u/search`_ API.
thumbnail_format = "crop-240x300"
thumbnail_prefix = ''
thumbnail_query = False
thumbnail_src_xpath = './div/img/@src'
THUMBNAIL_SUFFIX = "?fit=max&h=360&w=360"
thumbnail_xpath = ".//img[contains(@class, 'bookCover')]/@src"
thumbnail_xpath = "./a/img/@src"
thumbnail_xpath = './/img/@data-src'
thumbnail_xpath = './/img[@class="s-item__image-img"]/@src'
thumbnail_xpath = './/img[@class="thumb"]/@src'
thumbnail_xpath = './/img[@class="video-item--img"]/@src'
thumbnail_xpath = False
THUMB_URL = '{base}img/{episode}/{timestamp}/medium.jpg'
Time Range:
timeout = 2.0
timeout = 4.0
timestamp_format = '%Y-%m-%dT%H:%M:%SZ'
time_delta_dict = {
time_map = {
time_range_args = {
time_range_args = {"month": "pub_date:ultimi_30_giorni", "year": "pub_date:ultimo_anno"}
time_range_dict = {
time_range_dict = {"day": "1d", "week": "1w", "month": "1m", "year": "1y"}
time_range_dict = {"day": 1, "week": 7, "month": 30, "year": 365}
time_range_dict = {"day": 86400, "week": 604800, "month": 2592000, "year": 31536000}
time_range_dict = {'day': '1', 'week': '2', 'month': '3'}
time_range_dict = {'day': '24h', 'week': '1w', 'month': '1m', 'year': '1y'}
time_range_dict = {'day': '4', 'week': '3', 'month': '2', 'year': '1'}
time_range_dict = {'day': 'Ag', 'week': 'Aw', 'month': 'BA', 'year': 'BQ'}
time_range_dict = {'day': 'd', 'week': 'w', 'month': 'm', 'year': 'y'}
time_range_dict = {'day': 'd', 'week': 'w', 'month': 'm'}
time_range_dict = {'day': 'inttime_day', 'week': 'inttime_week', 'month': 'inttime_month', 'year': 'inttime_year'}
time_range_dict = {'day': 1, 'week': 7, 'month': 30}
time_range_dict: dict[str, str] = {"day": "d", "week": "w", "month": "m", "year": "y"}
time_range_duration_map = {
time_range_map = {
time_range_map = {"day": "past_day", "week": "past_week", "month": "past_month", "year": "past_year"}
time_range_map = {'day': '1d', 'week': '1w', 'month': '1m', 'year': '1y'}
time_range_map = {'day': 'last_24_hours', 'week': 'last_week', 'month': 'last_month', 'year': 'last_year'}
time_range_map: dict[str, str] = {
time_range_support = False
time_range_support = True
time_range_support = True  # time range search is supported for general and news
time_range_support: bool = False
time_range_support: bool = True
time_range_table = {
time_range_url = '&hours={time_range_val}'
time_range_url = '&min_upload_date={start}&max_upload_date={end}'
time_range_url = '&sp=EgII{time_range}%253D%253D'
title_html_to_text = False
title_query = None
title_xpath = ".//a/@title"
title_xpath = ".//a[contains(@class, 'bookTitle')]"
title_xpath = "./a/img/@alt"
title_xpath = '../h2/a'
title_xpath = './/a/text()'
title_xpath = './/div[contains(@class,"title-bloc-small")]'
title_xpath = './/h3[@class="b-serp-item__title"]/a[@class="b-serp-item__title-link"]/span'
title_xpath = './/h3[@class="s-item__title"]'
title_xpath = './/h3[@class="video-item--title"]'
title_xpath = './/p[@class="itemTitle"]/a'
title_xpath = './/span[@x-test-search-response-title]/text()'
title_xpath = './@aria-label'
title_xpath = './div[@class="h"]/h4'
title_xpath = './div[@class="SearchSnippet-headerContainer"]/h2/a/text()'
title_xpath = './h4/a[1]'
title_xpath = None
To demonstrate the power of database engines, here is a more complex example
to do more won't return any result and you will most likely be flagged as a bot.
To get in use of this *demo* engine add the following entry to your engines
To get in use of this *demo* engine add the following entry to your engines list
To get in use of this engine add the following entry to your engines list in
To not filter use an empty string (default).
To search in your favorite Discourse forum, add a configuration like shown here
To set header to moderate.'''
To simulate the behavior of a real browser session, it might be necessary to
To sort by *most relevant* use an empty string (default)."""
To support Wikipedia's LanguageConverter_, a SearXNG request to Wikipedia uses
To test in SearXNG, query for ``!wp 出租車`` with each of the available Chinese
to the search term:
to use a different lemmy instance, you can specify ``base_url``.
To use this engine add the following entry to your engines
To use this engine add the following entry to your engines list
To use this engine, add an entry similar to the following to your engine list in
token = {'value': '', 'last_updated': None}
tokens/networking.
torrent site for content. It is used by a number of torrent applications,
torznab_categories: list[str] = []
trackers = [
Troubleshooting
try:
Tube Archivist URL (``http://your-instance:port``)."""
Twitter/X, Facebook, ...
UI languages are stored in a custom field named ``ui_lang``:
under the age of 16 (``is_created_for_kids`` in `Video filters API`_ )
Unlike traditional search engines, wikipedia does not support one Wikipedia for
updated_xpath = (
upload_time_xpath = './/p[@class="itemTime"]//span[@class="time"]/text()'
URL = "https://api.duckduckgo.com/" + "?{query}&format=json&pretty=0&no_redirect=1&d=1"
url = "https://freesound.org/apiv2/"
url = "https://lingva.thedaviddelta.com"
url = "https://thepiratebay.org/"
url = "https://wttr.in/{query}?format=j1&lang={lang}"
url = 'https://1337x.to/'
url = 'https://api-free.deepl.com/v2/translate'
url = 'https://api.deezer.com/'
url = 'https://api.mixcloud.com/'
url = 'https://api.spotify.com/'
url = 'https://bt4gprx.com'
url = 'https://btdig.com'
URL = 'https://digbt.org'
url = 'https://genius.com/api/'
URL = 'https://jisho.org'
url = 'https://music.yandex.ru'
url = 'https://scanr.enseignementsup-recherche.gouv.fr/'
url = 'https://www.wolframalpha.com/'
url = (
url_prefix = ""
url_query = None
url_xpath = ".//a/@href"
url_xpath = ".//a[contains(@class, 'bookTitle')]/@href"
url_xpath = "./a/@href"
url_xpath = "./a[1]/@href"
url_xpath = './/a/@href'
url_xpath = './/a[@class="b-serp-item__title-link"]/@href'
url_xpath = './/a[@class="itemThumbWrap"]/@href'
url_xpath = './/a[@class="s-item__link"]/@href'
url_xpath = './/a[@class="video-item--a"]/@href'
url_xpath = './@href'
url_xpath = './a/@href'
url_xpath = './div[@class="SearchSnippet-headerContainer"]/h2/a/@href'
url_xpath = './h4/a/@href'
url_xpath = None
use this template, then the user doesn't want to see these hits in the videos
User-Agent_:
username = ""
username = ''
use_source_url = True
Using this engine together with Prowlarr_ or Jackett_ allows you to search
Using TinEye, you can search by image or perform what we call a reverse image
value_token_xpath = '//form[@id="searchForm"]/input[@type="hidden"]/@value'
VALUE_TO_LINK = {
verification by a cookie is needed / thats not possible in SearXNG.
version is the latest and which needs updating, who maintains the package, and
version_xpath = './div[@class="meta"]/span[contains(@class, "version")]'
version_xpath = './div[contains(@class, "SearchSnippet-infoLabel")]/span/strong[1]/text()'
verying searching works first with broken images, and then attempting this
video streams SearXNG can't use the video template for this and if SearXNG can't
video, for instance many links from Preasearch's video category link content
video_length_xpath = './/span[@class="videoLength"]'
views_xpath = './/span[@class="video-item--meta video-item--views"]/@data-value'
void_arch = 'x86_64'
W3C recommends subtag over macrolanguage [2]_.
wc_api_url = "https://commons.wikimedia.org/w/api.php"
wc_search_type: str = ""
we can make use of the :ref:`google API` to assemble the arguments of the GET
We use their official API_ for searching, but unfortunately, their Search API_
WEATHERKIT_TO_CONDITION: dict[str, weather.WeatherConditionType] = {
web_lite_url = 'https://lite.qwant.com/'
web_url = "https://mymemory.translated.net"
weight = 100
What is returned
When type is not `none` a token is expected to be passed as well in
WHERE
where {
which reads from a MediathekView_ (DE) movie database.  For this example of the
wikidata_image_sparql = """
WIKIDATA_PREFIX = ["http://www.wikidata.org/entity/", "https://www.wikidata.org/entity/"]
WIKIDATA_PROPERTIES = {
Wikipedia (Web
wikipedia_article_depth = 'https://meta.wikimedia.org/wiki/Wikipedia_article_depth'
wikipedia_script_variants = {
wiki_lc_locale_variants = {
with the rest of other metadata.
With this options a SearXNG maintainer is able to configure **additional**
without authentication.  The calls will be heavily rate limited, this is what the
WMO_TO_CONDITION: dict[int, weather.WeatherConditionType] = {
Wolfram|Alpha (Science)
working_dir = realpath('.')
WWO_TO_CONDITION: dict[str, weather.WeatherConditionType] = {
XBPS source packages collection.
xpath_author_name = XPath(".//atom:author/atom:name", namespaces=arxiv_namespaces)
xpath_category = './/td[1]/a[1]'
xpath_category = XPath(".//atom:category/@term", namespaces=arxiv_namespaces)
xpath_comment = XPath("./arxiv:comment", namespaces=arxiv_namespaces)
xpath_doi = XPath(".//arxiv:doi", namespaces=arxiv_namespaces)
xpath_downloads = './/td[8]/text()'
xpath_entry = XPath("//atom:entry", namespaces=arxiv_namespaces)
xpath_filesize = './/td[4]/text()'
xpath_id = XPath(".//atom:id", namespaces=arxiv_namespaces)
xpath_journal = XPath(".//arxiv:journal_ref", namespaces=arxiv_namespaces)
xpath_leeches = './/td[7]/text()'
xpath_pdf = XPath(".//atom:link[@title='pdf']", namespaces=arxiv_namespaces)
xpath_published = XPath(".//atom:published", namespaces=arxiv_namespaces)
xpath_results = '//table[contains(@class, "torrent-list")]//tr[not(th)]'
xpath_seeds = './/td[6]/text()'
xpath_summary = XPath(".//atom:summary", namespaces=arxiv_namespaces)
xpath_title = './/td[2]/a[last()]'
xpath_title = XPath(".//atom:title", namespaces=arxiv_namespaces)
xpath_torrent_links = './/td[3]/a'
Yahoo News is "English only" and do not offer localized nor language queries.
yahoo_languages = {
yandex_supported_langs = [
You can configure a Marginalia engine by:
You can configure the following setting:
You must configure the following settings:
you must the install the pip package ``mariadb`` along with the necessary
You must `register an application in Microsoft Entra ID`_ and assign it the
Youtube (Videos)
zlib_ext: str = ""
zlib_year_from: str = ""
zlib_year_to: str = ""
[1] https://en.wikipedia.org/wiki/Emojipedia
]
_arcid_random: tuple[str, int] | None = None
_arcid_range = string.ascii_letters + string.digits + "_-"
_available_query_types = {
_CACHE: EngineCache = None  # pyright: ignore[reportAssignmentType]
_ceid_locale_map = {'NO:no': 'nb-NO'}
_client = None
_command_logger = logger.getChild('command')
_compiled_parse_regex = {}
_connection = None
_delta_kwargs = {'day': 'days', 'week': 'weeks', 'month': 'months', 'year': 'years'}
_HTTP_User_Agent: str = gen_useragent()
_IMG_SRC_DEFAULT_URL_PREFIX = "https://commons.wikimedia.org/wiki/Special:FilePath/"
_IMG_SRC_NEW_URL_PREFIX = "https://upload.wikimedia.org/wikipedia/commons/thumb/"
_my_offline_engine: str = ""
_my_online_engine = None
_s2i: dict[str | None, int] = {"day": 1, "week": 7, "month": 30, "year": 365}
_search_url = ''
_skip_values = [
_valkey_client = None
__CACHED_API_URL = None
`See the following documentation for more details
`Supported Query Parameters`_.
`Tube Archivist`_ (TA) requires authentication for all image loads via cookie
`What are SFW, Sketchy and NSFW all about?`_:
`wikipedia rest_v1 summary API`_:
`XPath selector`_:
``api_key``:
``app`` and `` other``.
``asc`` or ``price``
``auth.token``.
``base_url``:
``ceid`` argument of the Google News REST API."""
``collection``.  Furthermore, you have to select a ``key`` that is going to be
``command``:
``countrycode``
``create_timestamp_asc``, ``create_timestamp_desc``, ``incoming_links_asc``,
``custom_query_json``.
``delimiter``:
``desc``
``false`` will stop filtering-out explicit content from searches and global
``global``
``incoming_links_desc``, ``just_match``, ``last_edit_asc``, ``last_edit_desc``,
``language``
``local``
``mailto``. You can set it directly in the engine configuration as shown above.
``my-collection`` and get the results in ascending order.
``nb-NO``::
``none``, ``random``, ``relevance``, ``user_random``.
``parse_regex``:
``pdf`` and ``epub``.
``photos``.
``ppc64le``, ``s390x``, ``armv7`` or ``riscv64``"""
``query_enum``:
``query_type``:
``result_separator``:
``settings.yml``:
``show_magnet_links``:
``show_torrent_files``:
``text`` or ``title``.
``torznab_categories``:
``type`` needs to be one of ``none``, ``personal_access_token``, or ``bearer``.
``use_local_search_results=false``, then the defaults are set for both (the
``use_local_search_results`` cookie and send the ``Accept-Language`` header.  We
``video`` are not yet implemented (Pull-Requests are welcome).
``working_dir``:
```
```python
``{language}``:
``{lang}``:
``{pageno}``:
``{query}``:
``{safe_search}``:
``{time_range_val}`` replacement is taken from the :py:obj:`time_range_map`.
``{time_range}``:
{
| **Categories** | code |
| **Categories** | currency, general |
| **Categories** | dictionaries |
| **Categories** | dictionaries, define |
| **Categories** | files |
| **Categories** | files, apps |
| **Categories** | general |
| **Categories** | general, articles |
| **Categories** | general, books |
| **Categories** | general, news |
| **Categories** | general, translate |
| **Categories** | general, web |
| **Categories** | images |
| **Categories** | images, icons |
| **Categories** | images, web |
| **Categories** | it |
| **Categories** | it, cloud |
| **Categories** | it, packages |
| **Categories** | it, packages, cargo |
| **Categories** | it, repos |
| **Categories** | it, software wikis |
| **Categories** | map |
| **Categories** | movies |
| **Categories** | music |
| **Categories** | music, lyrics |
| **Categories** | music, radio |
| **Categories** | news |
| **Categories** | onions |
| **Categories** | packages, it |
| **Categories** | science |
| **Categories** | science, scientific publications |
| **Categories** | shopping |
| **Categories** | social media |
| **Categories** | videos |
| **Categories** | videos, music |
| **Categories** | videos, music, files |
| **Categories** | videos, web |
| **Categories** | weather |
| **Features** | Paging |
| **Features** | Paging, SafeSearch |
| **Features** | Paging, SafeSearch, Time Range |
| **Features** | Paging, Time Range |
| **Features** | SafeSearch |
| **File** | `1337x.py` |
| **File** | `360search.py` |
| **File** | `360search_videos.py` |
| **File** | `9gag.py` |
| **File** | `acfun.py` |
| **File** | `adobe_stock.py` |
| **File** | `ahmia.py` |
| **File** | `alpinelinux.py` |
| **File** | `annas_archive.py` |
| **File** | `ansa.py` |
| **File** | `apkmirror.py` |
| **File** | `apple_app_store.py` |
| **File** | `apple_maps.py` |
| **File** | `archlinux.py` |
| **File** | `artic.py` |
| **File** | `artstation.py` |
| **File** | `arxiv.py` |
| **File** | `ask.py` |
| **File** | `astrophysics_data_system.py` |
| **File** | `azure.py` |
| **File** | `baidu.py` |
| **File** | `bandcamp.py` |
| **File** | `base.py` |
| **File** | `bilibili.py` |
| **File** | `bing.py` |
| **File** | `bing_images.py` |
| **File** | `bing_news.py` |
| **File** | `bing_videos.py` |
| **File** | `bitchute.py` |
| **File** | `bpb.py` |
| **File** | `brave.py` |
| **File** | `braveapi.py` |
| **File** | `bt4g.py` |
| **File** | `btdigg.py` |
| **File** | `cachy_os.py` |
| **File** | `ccc_media.py` |
| **File** | `chefkoch.py` |
| **File** | `chinaso.py` |
| **File** | `cloudflareai.py` |
| **File** | `command.py` |
| **File** | `core.py` |
| **File** | `crates.py` |
| **File** | `crossref.py` |
| **File** | `currency_convert.py` |
| **File** | `dailymotion.py` |
| **File** | `deepl.py` |
| **File** | `deezer.py` |
| **File** | `demo_offline.py` |
| **File** | `demo_online.py` |
| **File** | `destatis.py` |
| **File** | `deviantart.py` |
| **File** | `devicons.py` |
| **File** | `dictzone.py` |
| **File** | `digbt.py` |
| **File** | `discourse.py` |
| **File** | `docker_hub.py` |
| **File** | `doku.py` |
| **File** | `duckduckgo.py` |
| **File** | `duckduckgo_definitions.py` |
| **File** | `duckduckgo_extra.py` |
| **File** | `duckduckgo_weather.py` |
| **File** | `duden.py` |
| **File** | `dummy-offline.py` |
| **File** | `dummy.py` |
| **File** | `ebay.py` |
| **File** | `elasticsearch.py` |
| **File** | `emojipedia.py` |
| **File** | `fdroid.py` |
| **File** | `findthatmeme.py` |
| **File** | `flickr.py` |
| **File** | `flickr_noapi.py` |
| **File** | `freesound.py` |
| **File** | `frinkiac.py` |
| **File** | `fyyd.py` |
| **File** | `geizhals.py` |
| **File** | `genius.py` |
| **File** | `gitea.py` |
| **File** | `github.py` |
| **File** | `github_code.py` |
| **File** | `gitlab.py` |
| **File** | `goodreads.py` |
| **File** | `google.py` |
| **File** | `google_images.py` |
| **File** | `google_news.py` |
| **File** | `google_play.py` |
| **File** | `google_scholar.py` |
| **File** | `google_videos.py` |
| **File** | `grokipedia.py` |
| **File** | `hackernews.py` |
| **File** | `hex.py` |
| **File** | `huggingface.py` |
| **File** | `il_post.py` |
| **File** | `imdb.py` |
| **File** | `imgur.py` |
| **File** | `ina.py` |
| **File** | `invidious.py` |
| **File** | `ipernity.py` |
| **File** | `iqiyi.py` |
| **File** | `jisho.py` |
| **File** | `json_engine.py` |
| **File** | `kickass.py` |
| **File** | `lemmy.py` |
| **File** | `libretranslate.py` |
| **File** | `lib_rs.py` |
| **File** | `lingva.py` |
| **File** | `livespace.py` |
| **File** | `loc.py` |
| **File** | `lucide.py` |
| **File** | `marginalia.py` |
| **File** | `mariadb_server.py` |
| **File** | `mastodon.py` |
| **File** | `material_icons.py` |
| **File** | `mediathekviewweb.py` |
| **File** | `mediawiki.py` |
| **File** | `meilisearch.py` |
| **File** | `metacpan.py` |
| **File** | `microsoft_learn.py` |
| **File** | `mixcloud.py` |
| **File** | `mojeek.py` |
| **File** | `mongodb.py` |
| **File** | `moviepilot.py` |
| **File** | `mozhi.py` |
| **File** | `mrs.py` |
| **File** | `mwmbl.py` |
| **File** | `mysql_server.py` |
| **File** | `naver.py` |
| **File** | `niconico.py` |
| **File** | `npm.py` |
| **File** | `nvd.py` |
| **File** | `nyaa.py` |
| **File** | `odysee.py` |
| **File** | `ollama.py` |
| **File** | `openalex.py` |
| **File** | `openclipart.py` |
| **File** | `openlibrary.py` |
| **File** | `opensemantic.py` |
| **File** | `openstreetmap.py` |
| **File** | `openverse.py` |
| **File** | `open_meteo.py` |
| **File** | `pdbe.py` |
| **File** | `peertube.py` |
| **File** | `pexels.py` |
| **File** | `photon.py` |
| **File** | `pinterest.py` |
| **File** | `piped.py` |
| **File** | `piratebay.py` |
| **File** | `pixabay.py` |
| **File** | `pixiv.py` |
| **File** | `pkg_go_dev.py` |
| **File** | `podcastindex.py` |
| **File** | `postgresql.py` |
| **File** | `presearch.py` |
| **File** | `public_domain_image_archive.py` |
| **File** | `pubmed.py` |
| **File** | `pypi.py` |
| **File** | `quark.py` |
| **File** | `qwant.py` |
| **File** | `radio_browser.py` |
| **File** | `recoll.py` |
| **File** | `reddit.py` |
| **File** | `repology.py` |
| **File** | `reuters.py` |
| **File** | `rottentomatoes.py` |
| **File** | `rumble.py` |
| **File** | `scanr_structures.py` |
| **File** | `searchcode_code.py` |
| **File** | `searx_engine.py` |
| **File** | `selfhst.py` |
| **File** | `semantic_scholar.py` |
| **File** | `senscritique.py` |
| **File** | `sepiasearch.py` |
| **File** | `seznam.py` |
| **File** | `sogou.py` |
| **File** | `sogou_images.py` |
| **File** | `sogou_videos.py` |
| **File** | `sogou_wechat.py` |
| **File** | `solidtorrents.py` |
| **File** | `solr.py` |
| **File** | `soundcloud.py` |
| **File** | `sourcehut.py` |
| **File** | `spotify.py` |
| **File** | `springer.py` |
| **File** | `sqlite.py` |
| **File** | `stackexchange.py` |
| **File** | `startpage.py` |
| **File** | `steam.py` |
| **File** | `stract.py` |
| **File** | `svgrepo.py` |
| **File** | `tagesschau.py` |
| **File** | `tineye.py` |
| **File** | `tokyotoshokan.py` |
| **File** | `tootfinder.py` |
| **File** | `torznab.py` |
| **File** | `translated.py` |
| **File** | `tubearchivist.py` |
| **File** | `unsplash.py` |
| **File** | `uxwing.py` |
| **File** | `valkey_server.py` |
| **File** | `vimeo.py` |
| **File** | `voidlinux.py` |
| **File** | `wallhaven.py` |
| **File** | `wikicommons.py` |
| **File** | `wikidata.py` |
| **File** | `wikipedia.py` |
| **File** | `wolframalpha_api.py` |
| **File** | `wolframalpha_noapi.py` |
| **File** | `wordnik.py` |
| **File** | `wttr.py` |
| **File** | `www1x.py` |
| **File** | `xpath.py` |
| **File** | `yacy.py` |
| **File** | `yahoo.py` |
| **File** | `yahoo_news.py` |
| **File** | `yandex.py` |
| **File** | `yandex_music.py` |
| **File** | `yep.py` |
| **File** | `youtube_api.py` |
| **File** | `youtube_noapi.py` |
| **File** | `zlibrary.py` |
| **Lines** | 100 |
| **Lines** | 102 |
| **Lines** | 103 |
| **Lines** | 104 |
| **Lines** | 105 |
| **Lines** | 109 |
| **Lines** | 110 |
| **Lines** | 112 |
| **Lines** | 115 |
| **Lines** | 116 |
| **Lines** | 117 |
| **Lines** | 118 |
| **Lines** | 119 |
| **Lines** | 121 |
| **Lines** | 122 |
| **Lines** | 125 |
| **Lines** | 126 |
| **Lines** | 129 |
| **Lines** | 132 |
| **Lines** | 133 |
| **Lines** | 137 |
| **Lines** | 139 |
| **Lines** | 142 |
| **Lines** | 143 |
| **Lines** | 146 |
| **Lines** | 150 |
| **Lines** | 151 |
| **Lines** | 153 |
| **Lines** | 156 |
| **Lines** | 157 |
| **Lines** | 160 |
| **Lines** | 161 |
| **Lines** | 163 |
| **Lines** | 170 |
| **Lines** | 172 |
| **Lines** | 175 |
| **Lines** | 181 |
| **Lines** | 182 |
| **Lines** | 186 |
| **Lines** | 190 |
| **Lines** | 194 |
| **Lines** | 196 |
| **Lines** | 198 |
| **Lines** | 203 |
| **Lines** | 208 |
| **Lines** | 21 |
| **Lines** | 214 |
| **Lines** | 218 |
| **Lines** | 22 |
| **Lines** | 221 |
| **Lines** | 229 |
| **Lines** | 237 |
| **Lines** | 240 |
| **Lines** | 242 |
| **Lines** | 243 |
| **Lines** | 249 |
| **Lines** | 256 |
| **Lines** | 267 |
| **Lines** | 279 |
| **Lines** | 294 |
| **Lines** | 296 |
| **Lines** | 305 |
| **Lines** | 320 |
| **Lines** | 333 |
| **Lines** | 362 |
| **Lines** | 380 |
| **Lines** | 41 |
| **Lines** | 423 |
| **Lines** | 44 |
| **Lines** | 458 |
| **Lines** | 47 |
| **Lines** | 48 |
| **Lines** | 484 |
| **Lines** | 50 |
| **Lines** | 51 |
| **Lines** | 515 |
| **Lines** | 517 |
| **Lines** | 52 |
| **Lines** | 53 |
| **Lines** | 54 |
| **Lines** | 55 |
| **Lines** | 56 |
| **Lines** | 58 |
| **Lines** | 59 |
| **Lines** | 60 |
| **Lines** | 61 |
| **Lines** | 614 |
| **Lines** | 62 |
| **Lines** | 63 |
| **Lines** | 64 |
| **Lines** | 65 |
| **Lines** | 66 |
| **Lines** | 67 |
| **Lines** | 68 |
| **Lines** | 69 |
| **Lines** | 70 |
| **Lines** | 71 |
| **Lines** | 73 |
| **Lines** | 74 |
| **Lines** | 75 |
| **Lines** | 76 |
| **Lines** | 77 |
| **Lines** | 79 |
| **Lines** | 80 |
| **Lines** | 81 |
| **Lines** | 82 |
| **Lines** | 824 |
| **Lines** | 83 |
| **Lines** | 84 |
| **Lines** | 85 |
| **Lines** | 86 |
| **Lines** | 87 |
| **Lines** | 88 |
| **Lines** | 89 |
| **Lines** | 90 |
| **Lines** | 92 |
| **Lines** | 93 |
| **Lines** | 94 |
| **Lines** | 95 |
| **Lines** | 96 |
| **Lines** | 97 |
| **Lines** | 98 |
| **Lines** | 99 |
| **Path** | `engines_consolidated/1337x.py` |
| **Path** | `engines_consolidated/360search.py` |
| **Path** | `engines_consolidated/360search_videos.py` |
| **Path** | `engines_consolidated/9gag.py` |
| **Path** | `engines_consolidated/acfun.py` |
| **Path** | `engines_consolidated/adobe_stock.py` |
| **Path** | `engines_consolidated/ahmia.py` |
| **Path** | `engines_consolidated/alpinelinux.py` |
| **Path** | `engines_consolidated/annas_archive.py` |
| **Path** | `engines_consolidated/ansa.py` |
| **Path** | `engines_consolidated/apkmirror.py` |
| **Path** | `engines_consolidated/apple_app_store.py` |
| **Path** | `engines_consolidated/apple_maps.py` |
| **Path** | `engines_consolidated/archlinux.py` |
| **Path** | `engines_consolidated/artic.py` |
| **Path** | `engines_consolidated/artstation.py` |
| **Path** | `engines_consolidated/arxiv.py` |
| **Path** | `engines_consolidated/ask.py` |
| **Path** | `engines_consolidated/astrophysics_data_system.py` |
| **Path** | `engines_consolidated/azure.py` |
| **Path** | `engines_consolidated/baidu.py` |
| **Path** | `engines_consolidated/bandcamp.py` |
| **Path** | `engines_consolidated/base.py` |
| **Path** | `engines_consolidated/bilibili.py` |
| **Path** | `engines_consolidated/bing.py` |
| **Path** | `engines_consolidated/bing_images.py` |
| **Path** | `engines_consolidated/bing_news.py` |
| **Path** | `engines_consolidated/bing_videos.py` |
| **Path** | `engines_consolidated/bitchute.py` |
| **Path** | `engines_consolidated/bpb.py` |
| **Path** | `engines_consolidated/brave.py` |
| **Path** | `engines_consolidated/braveapi.py` |
| **Path** | `engines_consolidated/bt4g.py` |
| **Path** | `engines_consolidated/btdigg.py` |
| **Path** | `engines_consolidated/cachy_os.py` |
| **Path** | `engines_consolidated/ccc_media.py` |
| **Path** | `engines_consolidated/chefkoch.py` |
| **Path** | `engines_consolidated/chinaso.py` |
| **Path** | `engines_consolidated/cloudflareai.py` |
| **Path** | `engines_consolidated/command.py` |
| **Path** | `engines_consolidated/core.py` |
| **Path** | `engines_consolidated/crates.py` |
| **Path** | `engines_consolidated/crossref.py` |
| **Path** | `engines_consolidated/currency_convert.py` |
| **Path** | `engines_consolidated/dailymotion.py` |
| **Path** | `engines_consolidated/deepl.py` |
| **Path** | `engines_consolidated/deezer.py` |
| **Path** | `engines_consolidated/demo_offline.py` |
| **Path** | `engines_consolidated/demo_online.py` |
| **Path** | `engines_consolidated/destatis.py` |
| **Path** | `engines_consolidated/deviantart.py` |
| **Path** | `engines_consolidated/devicons.py` |
| **Path** | `engines_consolidated/dictzone.py` |
| **Path** | `engines_consolidated/digbt.py` |
| **Path** | `engines_consolidated/discourse.py` |
| **Path** | `engines_consolidated/docker_hub.py` |
| **Path** | `engines_consolidated/doku.py` |
| **Path** | `engines_consolidated/duckduckgo.py` |
| **Path** | `engines_consolidated/duckduckgo_definitions.py` |
| **Path** | `engines_consolidated/duckduckgo_extra.py` |
| **Path** | `engines_consolidated/duckduckgo_weather.py` |
| **Path** | `engines_consolidated/duden.py` |
| **Path** | `engines_consolidated/dummy-offline.py` |
| **Path** | `engines_consolidated/dummy.py` |
| **Path** | `engines_consolidated/ebay.py` |
| **Path** | `engines_consolidated/elasticsearch.py` |
| **Path** | `engines_consolidated/emojipedia.py` |
| **Path** | `engines_consolidated/fdroid.py` |
| **Path** | `engines_consolidated/findthatmeme.py` |
| **Path** | `engines_consolidated/flickr.py` |
| **Path** | `engines_consolidated/flickr_noapi.py` |
| **Path** | `engines_consolidated/freesound.py` |
| **Path** | `engines_consolidated/frinkiac.py` |
| **Path** | `engines_consolidated/fyyd.py` |
| **Path** | `engines_consolidated/geizhals.py` |
| **Path** | `engines_consolidated/genius.py` |
| **Path** | `engines_consolidated/gitea.py` |
| **Path** | `engines_consolidated/github.py` |
| **Path** | `engines_consolidated/github_code.py` |
| **Path** | `engines_consolidated/gitlab.py` |
| **Path** | `engines_consolidated/goodreads.py` |
| **Path** | `engines_consolidated/google.py` |
| **Path** | `engines_consolidated/google_images.py` |
| **Path** | `engines_consolidated/google_news.py` |
| **Path** | `engines_consolidated/google_play.py` |
| **Path** | `engines_consolidated/google_scholar.py` |
| **Path** | `engines_consolidated/google_videos.py` |
| **Path** | `engines_consolidated/grokipedia.py` |
| **Path** | `engines_consolidated/hackernews.py` |
| **Path** | `engines_consolidated/hex.py` |
| **Path** | `engines_consolidated/huggingface.py` |
| **Path** | `engines_consolidated/il_post.py` |
| **Path** | `engines_consolidated/imdb.py` |
| **Path** | `engines_consolidated/imgur.py` |
| **Path** | `engines_consolidated/ina.py` |
| **Path** | `engines_consolidated/invidious.py` |
| **Path** | `engines_consolidated/ipernity.py` |
| **Path** | `engines_consolidated/iqiyi.py` |
| **Path** | `engines_consolidated/jisho.py` |
| **Path** | `engines_consolidated/json_engine.py` |
| **Path** | `engines_consolidated/kickass.py` |
| **Path** | `engines_consolidated/lemmy.py` |
| **Path** | `engines_consolidated/libretranslate.py` |
| **Path** | `engines_consolidated/lib_rs.py` |
| **Path** | `engines_consolidated/lingva.py` |
| **Path** | `engines_consolidated/livespace.py` |
| **Path** | `engines_consolidated/loc.py` |
| **Path** | `engines_consolidated/lucide.py` |
| **Path** | `engines_consolidated/marginalia.py` |
| **Path** | `engines_consolidated/mariadb_server.py` |
| **Path** | `engines_consolidated/mastodon.py` |
| **Path** | `engines_consolidated/material_icons.py` |
| **Path** | `engines_consolidated/mediathekviewweb.py` |
| **Path** | `engines_consolidated/mediawiki.py` |
| **Path** | `engines_consolidated/meilisearch.py` |
| **Path** | `engines_consolidated/metacpan.py` |
| **Path** | `engines_consolidated/microsoft_learn.py` |
| **Path** | `engines_consolidated/mixcloud.py` |
| **Path** | `engines_consolidated/mojeek.py` |
| **Path** | `engines_consolidated/mongodb.py` |
| **Path** | `engines_consolidated/moviepilot.py` |
| **Path** | `engines_consolidated/mozhi.py` |
| **Path** | `engines_consolidated/mrs.py` |
| **Path** | `engines_consolidated/mwmbl.py` |
| **Path** | `engines_consolidated/mysql_server.py` |
| **Path** | `engines_consolidated/naver.py` |
| **Path** | `engines_consolidated/niconico.py` |
| **Path** | `engines_consolidated/npm.py` |
| **Path** | `engines_consolidated/nvd.py` |
| **Path** | `engines_consolidated/nyaa.py` |
| **Path** | `engines_consolidated/odysee.py` |
| **Path** | `engines_consolidated/ollama.py` |
| **Path** | `engines_consolidated/openalex.py` |
| **Path** | `engines_consolidated/openclipart.py` |
| **Path** | `engines_consolidated/openlibrary.py` |
| **Path** | `engines_consolidated/opensemantic.py` |
| **Path** | `engines_consolidated/openstreetmap.py` |
| **Path** | `engines_consolidated/openverse.py` |
| **Path** | `engines_consolidated/open_meteo.py` |
| **Path** | `engines_consolidated/pdbe.py` |
| **Path** | `engines_consolidated/peertube.py` |
| **Path** | `engines_consolidated/pexels.py` |
| **Path** | `engines_consolidated/photon.py` |
| **Path** | `engines_consolidated/pinterest.py` |
| **Path** | `engines_consolidated/piped.py` |
| **Path** | `engines_consolidated/piratebay.py` |
| **Path** | `engines_consolidated/pixabay.py` |
| **Path** | `engines_consolidated/pixiv.py` |
| **Path** | `engines_consolidated/pkg_go_dev.py` |
| **Path** | `engines_consolidated/podcastindex.py` |
| **Path** | `engines_consolidated/postgresql.py` |
| **Path** | `engines_consolidated/presearch.py` |
| **Path** | `engines_consolidated/public_domain_image_archive.py` |
| **Path** | `engines_consolidated/pubmed.py` |
| **Path** | `engines_consolidated/pypi.py` |
| **Path** | `engines_consolidated/quark.py` |
| **Path** | `engines_consolidated/qwant.py` |
| **Path** | `engines_consolidated/radio_browser.py` |
| **Path** | `engines_consolidated/recoll.py` |
| **Path** | `engines_consolidated/reddit.py` |
| **Path** | `engines_consolidated/repology.py` |
| **Path** | `engines_consolidated/reuters.py` |
| **Path** | `engines_consolidated/rottentomatoes.py` |
| **Path** | `engines_consolidated/rumble.py` |
| **Path** | `engines_consolidated/scanr_structures.py` |
| **Path** | `engines_consolidated/searchcode_code.py` |
| **Path** | `engines_consolidated/searx_engine.py` |
| **Path** | `engines_consolidated/selfhst.py` |
| **Path** | `engines_consolidated/semantic_scholar.py` |
| **Path** | `engines_consolidated/senscritique.py` |
| **Path** | `engines_consolidated/sepiasearch.py` |
| **Path** | `engines_consolidated/seznam.py` |
| **Path** | `engines_consolidated/sogou.py` |
| **Path** | `engines_consolidated/sogou_images.py` |
| **Path** | `engines_consolidated/sogou_videos.py` |
| **Path** | `engines_consolidated/sogou_wechat.py` |
| **Path** | `engines_consolidated/solidtorrents.py` |
| **Path** | `engines_consolidated/solr.py` |
| **Path** | `engines_consolidated/soundcloud.py` |
| **Path** | `engines_consolidated/sourcehut.py` |
| **Path** | `engines_consolidated/spotify.py` |
| **Path** | `engines_consolidated/springer.py` |
| **Path** | `engines_consolidated/sqlite.py` |
| **Path** | `engines_consolidated/stackexchange.py` |
| **Path** | `engines_consolidated/startpage.py` |
| **Path** | `engines_consolidated/steam.py` |
| **Path** | `engines_consolidated/stract.py` |
| **Path** | `engines_consolidated/svgrepo.py` |
| **Path** | `engines_consolidated/tagesschau.py` |
| **Path** | `engines_consolidated/tineye.py` |
| **Path** | `engines_consolidated/tokyotoshokan.py` |
| **Path** | `engines_consolidated/tootfinder.py` |
| **Path** | `engines_consolidated/torznab.py` |
| **Path** | `engines_consolidated/translated.py` |
| **Path** | `engines_consolidated/tubearchivist.py` |
| **Path** | `engines_consolidated/unsplash.py` |
| **Path** | `engines_consolidated/uxwing.py` |
| **Path** | `engines_consolidated/valkey_server.py` |
| **Path** | `engines_consolidated/vimeo.py` |
| **Path** | `engines_consolidated/voidlinux.py` |
| **Path** | `engines_consolidated/wallhaven.py` |
| **Path** | `engines_consolidated/wikicommons.py` |
| **Path** | `engines_consolidated/wikidata.py` |
| **Path** | `engines_consolidated/wikipedia.py` |
| **Path** | `engines_consolidated/wolframalpha_api.py` |
| **Path** | `engines_consolidated/wolframalpha_noapi.py` |
| **Path** | `engines_consolidated/wordnik.py` |
| **Path** | `engines_consolidated/wttr.py` |
| **Path** | `engines_consolidated/www1x.py` |
| **Path** | `engines_consolidated/xpath.py` |
| **Path** | `engines_consolidated/yacy.py` |
| **Path** | `engines_consolidated/yahoo.py` |
| **Path** | `engines_consolidated/yahoo_news.py` |
| **Path** | `engines_consolidated/yandex.py` |
| **Path** | `engines_consolidated/yandex_music.py` |
| **Path** | `engines_consolidated/yep.py` |
| **Path** | `engines_consolidated/youtube_api.py` |
| **Path** | `engines_consolidated/youtube_noapi.py` |
| **Path** | `engines_consolidated/zlibrary.py` |
| **Size** | 0.43 KB |
| **Size** | 0.47 KB |
| **Size** | 1.06 KB |
| **Size** | 1.16 KB |
| **Size** | 1.21 KB |
| **Size** | 1.23 KB |
| **Size** | 1.24 KB |
| **Size** | 1.25 KB |
| **Size** | 1.26 KB |
| **Size** | 1.29 KB |
| **Size** | 1.35 KB |
| **Size** | 1.38 KB |
| **Size** | 1.42 KB |
| **Size** | 1.45 KB |
| **Size** | 1.46 KB |
| **Size** | 1.48 KB |
| **Size** | 1.49 KB |
| **Size** | 1.52 KB |
| **Size** | 1.54 KB |
| **Size** | 1.58 KB |
| **Size** | 1.59 KB |
| **Size** | 1.60 KB |
| **Size** | 1.62 KB |
| **Size** | 1.63 KB |
| **Size** | 1.64 KB |
| **Size** | 1.65 KB |
| **Size** | 1.67 KB |
| **Size** | 1.70 KB |
| **Size** | 1.71 KB |
| **Size** | 1.72 KB |
| **Size** | 1.73 KB |
| **Size** | 1.74 KB |
| **Size** | 1.75 KB |
| **Size** | 1.76 KB |
| **Size** | 1.78 KB |
| **Size** | 1.80 KB |
| **Size** | 1.83 KB |
| **Size** | 1.84 KB |
| **Size** | 1.85 KB |
| **Size** | 1.87 KB |
| **Size** | 1.88 KB |
| **Size** | 1.89 KB |
| **Size** | 1.92 KB |
| **Size** | 1.95 KB |
| **Size** | 10.82 KB |
| **Size** | 10.90 KB |
| **Size** | 11.00 KB |
| **Size** | 11.32 KB |
| **Size** | 11.36 KB |
| **Size** | 12.32 KB |
| **Size** | 15.77 KB |
| **Size** | 15.85 KB |
| **Size** | 17.22 KB |
| **Size** | 18.93 KB |
| **Size** | 2.00 KB |
| **Size** | 2.01 KB |
| **Size** | 2.02 KB |
| **Size** | 2.04 KB |
| **Size** | 2.05 KB |
| **Size** | 2.08 KB |
| **Size** | 2.10 KB |
| **Size** | 2.12 KB |
| **Size** | 2.13 KB |
| **Size** | 2.16 KB |
| **Size** | 2.19 KB |
| **Size** | 2.20 KB |
| **Size** | 2.25 KB |
| **Size** | 2.27 KB |
| **Size** | 2.29 KB |
| **Size** | 2.31 KB |
| **Size** | 2.32 KB |
| **Size** | 2.33 KB |
| **Size** | 2.34 KB |
| **Size** | 2.35 KB |
| **Size** | 2.36 KB |
| **Size** | 2.37 KB |
| **Size** | 2.40 KB |
| **Size** | 2.42 KB |
| **Size** | 2.45 KB |
| **Size** | 2.46 KB |
| **Size** | 2.47 KB |
| **Size** | 2.48 KB |
| **Size** | 2.49 KB |
| **Size** | 2.51 KB |
| **Size** | 2.52 KB |
| **Size** | 2.53 KB |
| **Size** | 2.54 KB |
| **Size** | 2.55 KB |
| **Size** | 2.61 KB |
| **Size** | 2.67 KB |
| **Size** | 2.75 KB |
| **Size** | 2.81 KB |
| **Size** | 2.82 KB |
| **Size** | 2.83 KB |
| **Size** | 2.88 KB |
| **Size** | 2.90 KB |
| **Size** | 2.91 KB |
| **Size** | 2.92 KB |
| **Size** | 2.93 KB |
| **Size** | 2.95 KB |
| **Size** | 2.98 KB |
| **Size** | 20.46 KB |
| **Size** | 29.39 KB |
| **Size** | 3.00 KB |
| **Size** | 3.03 KB |
| **Size** | 3.05 KB |
| **Size** | 3.06 KB |
| **Size** | 3.07 KB |
| **Size** | 3.08 KB |
| **Size** | 3.13 KB |
| **Size** | 3.16 KB |
| **Size** | 3.17 KB |
| **Size** | 3.19 KB |
| **Size** | 3.22 KB |
| **Size** | 3.23 KB |
| **Size** | 3.25 KB |
| **Size** | 3.27 KB |
| **Size** | 3.29 KB |
| **Size** | 3.34 KB |
| **Size** | 3.40 KB |
| **Size** | 3.41 KB |
| **Size** | 3.45 KB |
| **Size** | 3.51 KB |
| **Size** | 3.59 KB |
| **Size** | 3.60 KB |
| **Size** | 3.67 KB |
| **Size** | 3.74 KB |
| **Size** | 3.81 KB |
| **Size** | 3.85 KB |
| **Size** | 3.94 KB |
| **Size** | 4.04 KB |
| **Size** | 4.09 KB |
| **Size** | 4.11 KB |
| **Size** | 4.20 KB |
| **Size** | 4.26 KB |
| **Size** | 4.28 KB |
| **Size** | 4.50 KB |
| **Size** | 4.54 KB |
| **Size** | 4.55 KB |
| **Size** | 4.57 KB |
| **Size** | 4.59 KB |
| **Size** | 4.64 KB |
| **Size** | 4.65 KB |
| **Size** | 4.66 KB |
| **Size** | 4.76 KB |
| **Size** | 4.92 KB |
| **Size** | 4.94 KB |
| **Size** | 5.00 KB |
| **Size** | 5.06 KB |
| **Size** | 5.18 KB |
| **Size** | 5.19 KB |
| **Size** | 5.32 KB |
| **Size** | 5.43 KB |
| **Size** | 5.53 KB |
| **Size** | 5.61 KB |
| **Size** | 5.87 KB |
| **Size** | 5.90 KB |
| **Size** | 5.99 KB |
| **Size** | 6.00 KB |
| **Size** | 6.02 KB |
| **Size** | 6.08 KB |
| **Size** | 6.36 KB |
| **Size** | 6.50 KB |
| **Size** | 6.83 KB |
| **Size** | 6.88 KB |
| **Size** | 6.98 KB |
| **Size** | 7.46 KB |
| **Size** | 7.47 KB |
| **Size** | 7.71 KB |
| **Size** | 7.73 KB |
| **Size** | 7.74 KB |
| **Size** | 7.81 KB |
| **Size** | 7.83 KB |
| **Size** | 8.84 KB |
| **Size** | 8.98 KB |
| **Size** | 9.34 KB |
| **Size** | 9.85 KB |
| **Size** | 9.99 KB |
| **Website** | http://juhanurmihxlp77nkq76byazcldy2hlmovfu2epvl5ankdibsot4csyd.onion |
| **Website** | https://1337x.to/ |
| **Website** | https://1x.com/ |
| **Website** | https://9gag.com/ |
| **Website** | https://about.gitea.com |
| **Website** | https://ai.cloudflare.com |
| **Website** | https://api.invidious.io/ |
| **Website** | https://api.search.brave.com/ |
| **Website** | https://arxiv.org |
| **Website** | https://bandcamp.com/ |
| **Website** | https://base-search.net |
| **Website** | https://bitchute.com |
| **Website** | https://bt4gprx.com |
| **Website** | https://btdig.com |
| **Website** | https://cachyos.org |
| **Website** | https://codeberg.org/aryak/mozhi |
| **Website** | https://commons.wikimedia.org/ |
| **Website** | https://core.ac.uk |
| **Website** | https://crates.io/ |
| **Website** | https://deepl.com |
| **Website** | https://deezer.com |
| **Website** | https://devicon.dev/ |
| **Website** | https://dictzone.com/ |
| **Website** | https://digbt.org |
| **Website** | https://discourse.org/ |
| **Website** | https://duckduckgo.com/ |
| **Website** | https://emojipedia.org |
| **Website** | https://f-droid.org/ |
| **Website** | https://findthatmeme.com |
| **Website** | https://fonts.google.com/icons |
| **Website** | https://freesound.org |
| **Website** | https://frinkiac.com |
| **Website** | https://fyyd.de |
| **Website** | https://geizhals.de |
| **Website** | https://genius.com/ |
| **Website** | https://github.com/ |
| **Website** | https://github.com/mwmbl/mwmbl |
| **Website** | https://github.com/searxng/searxng |
| **Website** | https://github.com/TeamPiped/Piped/ |
| **Website** | https://grokipedia.com |
| **Website** | https://hex.pm/ |
| **Website** | https://hub.docker.com |
| **Website** | https://huggingface.co/ |
| **Website** | https://images.google.com |
| **Website** | https://imdb.com/ |
| **Website** | https://imgur.com/ |
| **Website** | https://jisho.org |
| **Website** | https://joinmastodon.org/ |
| **Website** | https://joinpeertube.org |
| **Website** | https://kickasstorrents.to |
| **Website** | https://learn.microsoft.com |
| **Website** | https://lemmy.ml/ |
| **Website** | https://lib.rs |
| **Website** | https://libretranslate.com |
| **Website** | https://lingva.ml |
| **Website** | https://live.space |
| **Website** | https://lucide.dev/ |
| **Website** | https://marginalia.nu |
| **Website** | https://matrixrooms.info |
| **Website** | https://media.ccc.de |
| **Website** | https://mediathekviewweb.de/ |
| **Website** | https://metacpan.org/ |
| **Website** | https://mojeek.com |
| **Website** | https://music.yandex.ru |
| **Website** | https://mymemory.translated.net/ |
| **Website** | https://news.google.com |
| **Website** | https://news.yahoo.com |
| **Website** | https://news.ycombinator.com/ |
| **Website** | https://npms.io/ |
| **Website** | https://nvd.nist.gov |
| **Website** | https://nyaa.si/ |
| **Website** | https://odysee.com/ |
| **Website** | https://ollama.com |
| **Website** | https://open-meteo.com |
| **Website** | https://openalex.org/ |
| **Website** | https://openclipart.org/ |
| **Website** | https://openlibrary.org |
| **Website** | https://openverse.org/ |
| **Website** | https://pdimagearchive.org |
| **Website** | https://photon.komoot.io |
| **Website** | https://pic.sogou.com/ |
| **Website** | https://pixabay.com |
| **Website** | https://pkg.go.dev/ |
| **Website** | https://play.google.com/ |
| **Website** | https://podcastindex.org |
| **Website** | https://presearch.io |
| **Website** | https://pypi.org |
| **Website** | https://quark.sm.cn/ |
| **Website** | https://repology.org |
| **Website** | https://rumble.com/ |
| **Website** | https://scanr.enseignementsup-recherche.gouv.fr |
| **Website** | https://scholar.google.com |
| **Website** | https://search.brave.com/ |
| **Website** | https://search.naver.com |
| **Website** | https://search.yahoo.com/ |
| **Website** | https://searchcode.com/ |
| **Website** | https://selfh.st/icons/ |
| **Website** | https://sepiasearch.org |
| **Website** | https://soundcloud.com |
| **Website** | https://sourcehut.org |
| **Website** | https://stackexchange.com |
| **Website** | https://startpage.com |
| **Website** | https://stock.adobe.com/ |
| **Website** | https://store.steampowered.com/ |
| **Website** | https://stract.com/ |
| **Website** | https://tagesschau.de |
| **Website** | https://thepiratebay.org |
| **Website** | https://tineye.com |
| **Website** | https://tv.360kan.com/ |
| **Website** | https://ui.adsabs.harvard.edu/ |
| **Website** | https://unsplash.com |
| **Website** | https://uxwing.com |
| **Website** | https://v.sogou.com/ |
| **Website** | https://vimeo.com/ |
| **Website** | https://voidlinux.org/packages/ |
| **Website** | https://wallhaven.cc/ |
| **Website** | https://weixin.sogou.com/ |
| **Website** | https://wiki.archlinux.org/ |
| **Website** | https://wikidata.org/ |
| **Website** | https://wttr.in |
| **Website** | https://www.acfun.cn/ |
| **Website** | https://www.alpinelinux.org |
| **Website** | https://www.ansa.it |
| **Website** | https://www.apkmirror.com |
| **Website** | https://www.apple.com/app-store/ |
| **Website** | https://www.apple.com/maps/ |
| **Website** | https://www.artic.edu |
| **Website** | https://www.artstation.com/ |
| **Website** | https://www.ask.com/ |
| **Website** | https://www.baidu.com |
| **Website** | https://www.bilibili.com |
| **Website** | https://www.bing.com |
| **Website** | https://www.bing.com/images |
| **Website** | https://www.bing.com/news |
| **Website** | https://www.bing.com/videos |
| **Website** | https://www.bpb.de |
| **Website** | https://www.chefkoch.de |
| **Website** | https://www.chinaso.com/ |
| **Website** | https://www.crossref.org/ |
| **Website** | https://www.dailymotion.com |
| **Website** | https://www.destatis.de |
| **Website** | https://www.deviantart.com/ |
| **Website** | https://www.dokuwiki.org/ |
| **Website** | https://www.duden.de |
| **Website** | https://www.ebay.com |
| **Website** | https://www.ebi.ac.uk/pdbe |
| **Website** | https://www.elastic.co |
| **Website** | https://www.flickr.com |
| **Website** | https://www.goodreads.com |
| **Website** | https://www.google.com |
| **Website** | https://www.ilpost.it |
| **Website** | https://www.ina.fr/ |
| **Website** | https://www.ipernity.com |
| **Website** | https://www.iqiyi.com/ |
| **Website** | https://www.loc.gov/pictures/ |
| **Website** | https://www.mixcloud.com/ |
| **Website** | https://www.moviepilot.de |
| **Website** | https://www.ncbi.nlm.nih.gov/pubmed/ |
| **Website** | https://www.nicovideo.jp/ |
| **Website** | https://www.opensemanticsearch.org/ |
| **Website** | https://www.openstreetmap.org/ |
| **Website** | https://www.pexels.com |
| **Website** | https://www.pinterest.com/ |
| **Website** | https://www.pixiv.net/ |
| **Website** | https://www.portal.azure.com |
| **Website** | https://www.qwant.com/ |
| **Website** | https://www.radio-browser.info/ |
| **Website** | https://www.reddit.com/ |
| **Website** | https://www.reuters.com |
| **Website** | https://www.rottentomatoes.com/ |
| **Website** | https://www.semanticscholar.org/ |
| **Website** | https://www.senscritique.com/ |
| **Website** | https://www.seznam.cz/ |
| **Website** | https://www.so.com/ |
| **Website** | https://www.sogou.com/ |
| **Website** | https://www.solidtorrents.to/ |
| **Website** | https://www.spotify.com |
| **Website** | https://www.springernature.com/ |
| **Website** | https://www.svgrepo.com |
| **Website** | https://www.tokyotosho.info/ |
| **Website** | https://www.tootfinder.ch |
| **Website** | https://www.tubearchivist.com |
| **Website** | https://www.wikipedia.org/ |
| **Website** | https://www.wolframalpha.com |
| **Website** | https://www.wolframalpha.com/ |
| **Website** | https://www.wordnik.com |
| **Website** | https://www.youtube.com/ |
| **Website** | https://yacy.net/ |
| **Website** | https://yandex.com/ |
| **Website** | https://yep.com/ |
| Property | Value |
|----------|-------|
}
}"""
~~~~~~~~~~~~~~
~~~~~~~~~~~~~~~
~~~~~~~~~~~~~~~~~~
~~~~~~~~~~~~~~~~~~~~
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~~
