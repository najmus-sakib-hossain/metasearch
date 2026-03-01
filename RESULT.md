Shohan@dx MINGW64 /f/metasearch (main)
$ cargo test -p metasearch-engine --test engine_probe -- --ignored --nocapture 2>&1
   Compiling windows-sys v0.61.2
   Compiling serde_core v1.0.228
   Compiling getrandom v0.2.17
   Compiling futures-task v0.3.32
   Compiling slab v0.4.12
   Compiling tracing-core v0.1.36
   Compiling log v0.4.29
   Compiling ring v0.17.14
   Compiling futures-channel v0.3.32
   Compiling aho-corasick v1.1.4
   Compiling selectors v0.26.0
   Compiling futures-util v0.3.32
   Compiling tracing v0.1.44                 
   Compiling rustls-webpki v0.103.9                 
   Compiling regex-automata v0.4.14
   Compiling rustls v0.23.37
   Compiling mio v1.1.1                 
   Compiling serde v1.0.228
   Compiling serde_json v1.0.149
   Compiling tokio v1.49.0                 
   Compiling string_cache v0.8.9
   Compiling serde_urlencoded v0.7.1
   Compiling chrono v0.4.44
   Compiling markup5ever v0.14.1                 
   Compiling metasearch-core v0.1.0 (F:\metasearch\crates\metasearch-core)                 
   Compiling html5ever v0.29.1
   Compiling regex v1.12.3
   Compiling scraper v0.22.0
   Compiling hyper v1.8.1
   Compiling async-compression v0.4.41
   Compiling tokio-rustls v0.26.4
   Compiling tower v0.5.3
   Compiling tokio-util v0.7.18
   Compiling tower-http v0.6.8
   Compiling hyper-util v0.1.20
   Compiling hyper-rustls v0.27.7
   Compiling reqwest v0.12.28 
   Compiling metasearch-engine v0.1.0 (F:\metasearch\crates\metasearch-engine)
warning: fields `port`, `username`, and `password` are never read
  --> crates\metasearch-engine\src\mariadb_engine.rs:40:5
   |
37 | pub struct MariaDbEngine {
   |            ------------- fields in this struct
...
40 |     port: u16,
   |     ^^^^
41 |     database: String,
42 |     username: String,
   |     ^^^^^^^^
43 |     password: String,
   |     ^^^^^^^^
   |
   = note: `#[warn(dead_code)]` (part of `#[warn(unused)]`) on by default

warning: field `port` is never read
  --> crates\metasearch-engine\src\mongodb_engine.rs:36:5
   |
33 | pub struct MongoDbEngine {
   |            ------------- field in this struct
...
36 |     port: u16,
   |     ^^^^

warning: fields `port`, `username`, and `password` are never read
  --> crates\metasearch-engine\src\mysql_engine.rs:36:5
   |
33 | pub struct MysqlEngine {
   |            ----------- fields in this struct
...
36 |     port: u16,
   |     ^^^^
37 |     database: String,
38 |     username: String,
   |     ^^^^^^^^
39 |     password: String,
   |     ^^^^^^^^

warning: fields `port`, `username`, and `password` are never read
  --> crates\metasearch-engine\src\postgres_engine.rs:36:5
   |
33 | pub struct PostgresEngine {
   |            -------------- fields in this struct
...
36 |     port: u16,
   |     ^^^^
37 |     database: String,
38 |     username: String,
   |     ^^^^^^^^
39 |     password: String,
   |     ^^^^^^^^

warning: field `result_type` is never read
  --> crates\metasearch-engine\src\sqlite_engine.rs:47:5
   |
43 | pub struct SqliteEngine {
   |            ------------ field in this struct
...
47 |     result_type: SqliteResultType,
   |     ^^^^^^^^^^^

warning: fields `port` and `db` are never read
  --> crates\metasearch-engine\src\valkey_engine.rs:35:5
   |
32 | pub struct ValkeyEngine {
   |            ------------ fields in this struct
...
35 |     port: u16,
   |     ^^^^
36 |     password: Option<String>,
37 |     db: u8,
   |     ^^

warning: `metasearch-engine` (lib) generated 6 warnings
    Finished `test` profile [unoptimized + debuginfo] target(s) in 1m 09s
     Running tests\engine_probe.rs (target\debug\deps\engine_probe-f69d4be9c4720f9b.exe)

running 1 test

======================================================================
  ENGINE PROBE — testing 208 engines with query "rust programming"
======================================================================

  [  1/208] 1337x                          ⚠️   0 results (empty)
  [  2/208] 360 Search                     ❌  HTTP request failed: error following redirect for url (https
  [  3/208] 360 Search Videos              ❌  HTTP request failed: error sending request for url (https://
  [  4/208] 9GAG                           ✅  10 result(s)
  [  5/208] AppleMaps                      ✅  1 result(s)
  [  6/208] CurrencyConvert                ⚠️   0 results (empty)
  [  7/208] DictZone                       ❌  Engine error: DictZone request error: error sending request 
  [  8/208] DuckDuckGoWeather              ✅  6 result(s)
  [  9/208] Elasticsearch                  ⚠️   0 results (empty)
  [ 10/208] Google News                    ⚠️   0 results (empty)
  [ 11/208] Google Videos                  ✅  1 result(s)
  [ 12/208] Invidious                      ⚠️   0 results (empty)
  [ 13/208] MeiliSearch                    ⚠️   0 results (empty)
  [ 14/208] OpenAlex                       ✅  10 result(s)
  [ 15/208] OpenStreetMap                  ⚠️   0 results (empty)
  [ 16/208] Openverse                      ✅  20 result(s)
  [ 17/208] Searchcode                     ❌  Engine error: error decoding response body
  [ 18/208] The Pirate Bay                 ✅  38 result(s)
  [ 19/208] TinEye                         ⚠️   0 results (empty)
  [ 20/208] Tokyo Toshokan                 ✅  1 result(s)
  [ 21/208] Tootfinder                     ⚠️   0 results (empty)
  [ 22/208] Wordnik                        ⚠️   0 results (empty)
  [ 23/208] AcFun                          ⚠️   0 results (empty)
  [ 24/208] Adobe Stock                    ✅  10 result(s)
  [ 25/208] Ahmia                          ⚠️   0 results (empty)
  [ 26/208] Alpine Linux Packages          test probe_all_engines has been running for over 60 seconds
⚠️   0 results (empty)
  [ 27/208] Anna's Archive                 ❌  HTTP request failed: error sending request for url (https://
  [ 28/208] ANSA                           ⚠️   0 results (empty)
  [ 29/208] APKMirror                      ⚠️   0 results (empty)
  [ 30/208] Apple App Store                ✅  47 result(s)
  [ 31/208] Arch Linux Wiki                ⚠️   0 results (empty)
  [ 32/208] Art Institute of Chicago       ✅  20 result(s)
  [ 33/208] ArtStation                     ✅  9 result(s)
  [ 34/208] arXiv                          ✅  10 result(s)
  [ 35/208] Ask.com                        ⚠️   0 results (empty)
  [ 36/208] ADS                            ❌  Engine 'astrophysics_data_system' failed: No API key configu
  [ 37/208] Azure Search                   ⚠️   0 results (empty)
  [ 38/208] Baidu                          ❌  Parse error: JSON parse error: expected value at line 1 colu
  [ 39/208] Bandcamp                       ✅  18 result(s)
  [ 40/208] BASE                           ⚠️   0 results (empty)
  [ 41/208] Bilibili                       ✅  20 result(s)
  [ 42/208] Bing                           ✅  10 result(s)
  [ 43/208] Bing Images                    ✅  35 result(s)
  [ 44/208] Bing News                      ✅  10 result(s)
  [ 45/208] Bing Videos                    ⚠️   0 results (empty)
  [ 46/208] BitChute                       ✅  20 result(s)
  [ 47/208] BPB                            ✅  4 result(s)
  [ 48/208] Brave Search                   ✅  10 result(s)
  [ 49/208] Brave API                      ❌  Engine 'braveapi' failed: No API key configured. Get one at
  [ 50/208] BT4G                           ⚠️   0 results (empty)
  [ 51/208] BTDigg                         ⚠️   0 results (empty)
  [ 52/208] CachyOS                        ⚠️   0 results (empty)
  [ 53/208] CCC Media                      ✅  25 result(s)
  [ 54/208] Chefkoch                       ✅  14 result(s)
  [ 55/208] Chinaso                        ⚠️   0 results (empty)
  [ 56/208] Cloudflare AI                  ⚠️   0 results (empty)
  [ 57/208] CORE                           ❌  Engine 'core' failed: No API key configured. Register at htt
  [ 58/208] crates.io                      ✅  10 result(s)
  [ 59/208] Crossref                       ✅  20 result(s)
  [ 60/208] Dailymotion                    ✅  10 result(s)
  [ 61/208] DeepL                          ⚠️   0 results (empty)
  [ 62/208] Deezer                         ✅  3 result(s)
  [ 63/208] DeStatis                       ✅  1 result(s)
  [ 64/208] DeviantArt                     ⚠️   0 results (empty)
  [ 65/208] Devicons                       ✅  166 result(s)
  [ 66/208] DigBT                          ⚠️   0 results (empty)
  [ 67/208] Discourse                      ⚠️   0 results (empty)
  [ 68/208] Docker Hub                     ✅  10 result(s)
  [ 69/208] DokuWiki                       ⚠️   0 results (empty)
  [ 70/208] DuckDuckGo                     ⚠️   0 results (empty)
  [ 71/208] DuckDuckGo Definitions         ⚠️   0 results (empty)
  [ 72/208] DuckDuckGo Images              ❌  Parse error: error decoding response body
  [ 73/208] Duden                          ⚠️   0 results (empty)
  [ 74/208] eBay                           ⚠️   0 results (empty)
  [ 75/208] Emojipedia                     ✅  2 result(s)
  [ 76/208] F-Droid                        ⚠️   0 results (empty)
  [ 77/208] FindThatMeme                   ✅  50 result(s)
  [ 78/208] Flickr                         ✅  25 result(s)
  [ 79/208] Flickr (no API)                ⚠️   0 results (empty)
  [ 80/208] Freesound                      ❌  Configuration error: Freesound requires an API key. Get one 
  [ 81/208] Frinkiac                       ✅  36 result(s)
  [ 82/208] Fyyd                           ❌  Parse error: error decoding response body
  [ 83/208] Geizhals                       ⚠️   0 results (empty)
  [ 84/208] Genius                         ✅  13 result(s)
  [ 85/208] Gitea                          ⚠️   0 results (empty)
  [ 86/208] GitHub                         ✅  30 result(s)
  [ 87/208] GitHub Code                    ⚠️   0 results (empty)
  [ 88/208] GitLab                         ✅  20 result(s)
  [ 89/208] Goodreads                      ✅  10 result(s)
  [ 90/208] Google                         ⚠️   0 results (empty)
  [ 91/208] Google Images                  ✅  100 result(s)
  [ 92/208] Google Play                    ✅  30 result(s)
  [ 93/208] Google Scholar                 ✅  10 result(s)
  [ 94/208] Grokipedia                     ✅  10 result(s)
  [ 95/208] Hacker News                    ✅  30 result(s)
  [ 96/208] Hex.pm                         ✅  10 result(s)
  [ 97/208] Hugging Face                   ⚠️   0 results (empty)
  [ 98/208] Il Post                        ❌  Parse error: error decoding response body
  [ 99/208] IMDb                           ✅  8 result(s)
  [100/208] Imgur                          ⚠️   0 results (empty)
  [101/208] INA                            ⚠️   0 results (empty)
  [102/208] Ipernity                       ✅  3 result(s)
  [103/208] iQiyi                          ✅  25 result(s)
  [104/208] Jisho                          ⚠️   0 results (empty)
  [105/208] Kickass Torrents               ⚠️   0 results (empty)
  [106/208] Lemmy                          ✅  10 result(s)
  [107/208] lib.rs                         ✅  148 result(s)
  [108/208] LibreTranslate                 ⚠️   0 results (empty)
  [109/208] Lingva                         ⚠️   0 results (empty)
  [110/208] LiveSpace                      ❌  HTTP request failed: error sending request for url (https://
  [111/208] Library of Congress            ✅  8 result(s)
  [112/208] Lucide                         ✅  8 result(s)
  [113/208] Marginalia                     ❌  Engine 'marginalia' failed: No API key configured. Get one a
  [114/208] Mastodon                       ✅  40 result(s)
  [115/208] Material Icons                 ✅  20 result(s)
  [116/208] MediathekViewWeb               ⚠️   0 results (empty)
  [117/208] MediaWiki                      ⚠️   0 results (empty)
  [118/208] MetaCPAN                       ⚠️   0 results (empty)
  [119/208] Microsoft Learn                ✅  10 result(s)
  [120/208] Mixcloud                       ✅  4 result(s)
  [121/208] Mojeek                         ✅  10 result(s)
  [122/208] moviepilot                     ⚠️   0 results (empty)
  [123/208] Mozhi                          ⚠️   0 results (empty)
  [124/208] Matrix Rooms Search            ⚠️   0 results (empty)
  [125/208] Mwmbl                          ✅  53 result(s)
  [126/208] Naver                          ⚠️   0 results (empty)
  [127/208] Niconico                       ⚠️   0 results (empty)
  [128/208] npm                            ✅  12 result(s)
  [129/208] NVD                            ✅  10 result(s)
  [130/208] Nyaa                           ⚠️   0 results (empty)
  [131/208] Odysee                         ✅  20 result(s)
  [132/208] Ollama                         ⚠️   0 results (empty)
  [133/208] Open-Meteo                     ⚠️   0 results (empty)
  [134/208] OpenClipart                    ❌  HTTP request failed: error sending request for url (https://
  [135/208] openlibrary                    ✅  10 result(s)
  [136/208] Open Semantic                  ⚠️   0 results (empty)
  [137/208] PDBe                           ⚠️   0 results (empty)
  [138/208] PeerTube                       ✅  10 result(s)
  [139/208] Pexels                         ❌  Engine 'pexels' failed: No API key configured
  [140/208] Photon                         ⚠️   0 results (empty)
  [141/208] Pinterest                      ❌  Engine error: Pinterest parse failed: error decoding respons
  [142/208] Piped                          ⚠️   0 results (empty)
  [143/208] Pixabay                        ✅  100 result(s)
  [144/208] Pixiv                          ⚠️   0 results (empty)
  [145/208] pkg.go.dev                     ✅  22 result(s)
  [146/208] Podcast Index                  ❌  Engine error: PodcastIndex parse failed: error decoding resp
  [147/208] Presearch                      ⚠️   0 results (empty)
  [148/208] PDIA                           ⚠️   0 results (empty)
  [149/208] PubMed                         ✅  10 result(s)
  [150/208] PyPI                           ⚠️   0 results (empty)
  [151/208] Quark                          ✅  9 result(s)
  [152/208] Qwant                          ❌  Parse error: error decoding response body
  [153/208] Radio Browser                  ⚠️   0 results (empty)
  [154/208] Recoll                         ⚠️   0 results (empty)
  [155/208] Reddit                         ❌  Parse error: error decoding response body
  [156/208] Repology                       ⚠️   0 results (empty)
  [157/208] Reuters                        ⚠️   0 results (empty)
  [158/208] rottentomatoes                 ✅  20 result(s)
  [159/208] Rumble                         ❌  Engine error: Rumble request failed: error sending request f
  [160/208] ScanR Structures               ⚠️   0 results (empty)
  [161/208] Searchcode                     ❌  Parse error: error decoding response body
  [162/208] SearXNG                        ⚠️   0 results (empty)
  [163/208] Selfh.st Icons                 ⚠️   0 results (empty)
  [164/208] Semantic Scholar               ⚠️   0 results (empty)
  [165/208] SensCritique                   ⚠️   0 results (empty)
  [166/208] sepiasearch                    ✅  10 result(s)
  [167/208] Seznam                         ⚠️   0 results (empty)
  [168/208] Sogou                          ✅  9 result(s)
  [169/208] Sogou Images                   ✅  48 result(s)
  [170/208] Sogou Videos                   ✅  16 result(s)
  [171/208] Sogou WeChat                   ✅  10 result(s)
  [172/208] solidtorrents                  ❌  Engine error: SolidTorrents request failed: error sending re
  [173/208] Solr                           ⚠️   0 results (empty)
  [174/208] SoundCloud                     ✅  10 result(s)
  [175/208] SourceHut                      ⚠️   0 results (empty)
  [176/208] Spotify                        ❌  Configuration error: Spotify requires api_client_id and api_
  [177/208] Springer Nature                ❌  Engine 'springer' failed: No API key configured. Get one at
  [178/208] StackExchange                  ✅  10 result(s)
  [179/208] Startpage                      ✅  10 result(s)
  [180/208] Steam                          ⚠️   0 results (empty)
  [181/208] Stract                         ❌  Engine error: Stract parse failed: error decoding response b
  [182/208] SVG Repo                       ⚠️   0 results (empty)
  [183/208] Tagesschau                     ⚠️   0 results (empty)
  [184/208] Torznab                        ⚠️   0 results (empty)
  [185/208] MyMemory                       ✅  1 result(s)
  [186/208] TubeArchivist                  ⚠️   0 results (empty)
  [187/208] Unsplash                       ✅  20 result(s)
  [188/208] UXWing                         ⚠️   0 results (empty)
  [189/208] Vimeo                          ⚠️   0 results (empty)
  [190/208] Void Linux                     ✅  4 result(s)
  [191/208] Wallhaven                      ✅  9 result(s)
  [192/208] Wikimedia Commons              ✅  10 result(s)
  [193/208] Wikidata                       ✅  3 result(s)
  [194/208] Wikipedia                      ✅  10 result(s)
  [195/208] Wolfram|Alpha API              ❌  Engine 'wolframalpha_api' failed: No API key configured. Get
  [196/208] Wolfram|Alpha                  ✅  3 result(s)
  [197/208] wttr.in                        ❌  HTTP request failed: error sending request for url (https://
  [198/208] 1x                             ⚠️   0 results (empty)
  [199/208] YaCy                           ⚠️   0 results (empty)
  [200/208] Yahoo                          ✅  7 result(s)
  [201/208] Yahoo News                     ✅  20 result(s)
  [202/208] Yandex                         ⚠️   0 results (empty)
  [203/208] Yandex Music                   ✅  5 result(s)
  [204/208] Yep                            ✅  20 result(s)
  [205/208] YouTube                        ✅  16 result(s)
  [206/208] YouTube API                    ❌  Engine 'youtube_api' failed: No API key configured. Get one 
  [207/208] YouTube                        ✅  5 result(s)
  [208/208] Z-Library                      ❌  HTTP request failed: error sending request for url (https://

======================================================================
  RESULTS: 89 / 208 passed,  119 / 208 failed
======================================================================

  Failed engines:
    • 1337x: empty results
    • 360search: HTTP request failed: error following redirect for url (https
    • 360search_videos: HTTP request failed: error sending request for url (https://
    • CurrencyConvert: empty results
    • DictZone: Engine error: DictZone request error: error sending request
    • Elasticsearch: empty results
    • Google News: empty results
    • Invidious: empty results
    • MeiliSearch: empty results
    • OpenStreetMap: empty results
    • Searchcode: Engine error: error decoding response body
    • TinEye: empty results
    • Tootfinder: empty results
    • Wordnik: empty results
    • acfun: empty results
    • ahmia: empty results
    • alpinelinux: empty results
    • annas_archive: HTTP request failed: error sending request for url (https://
    • ansa: empty results
    • apkmirror: empty results
    • archlinux: empty results
    • ask: empty results
    • astrophysics_data_system: Engine 'astrophysics_data_system' failed: No API key configu
    • azure: empty results
    • baidu: Parse error: JSON parse error: expected value at line 1 colu
    • base_search: empty results
    • bing_videos: empty results
    • braveapi: Engine 'braveapi' failed: No API key configured. Get one at
    • bt4g: empty results
    • btdigg: empty results
    • cachy_os: empty results
    • chinaso: empty results
    • cloudflareai: empty results
    • core: Engine 'core' failed: No API key configured. Register at htt
    • deepl: empty results
    • deviantart: empty results
    • digbt: empty results
    • discourse: empty results
    • dokuwiki: empty results
    • duckduckgo: empty results
    • duckduckgo_definitions: empty results
    • duckduckgo_extra: Parse error: error decoding response body
    • duden: empty results
    • ebay: empty results
    • fdroid: empty results
    • flickr_noapi: empty results
    • freesound: Configuration error: Freesound requires an API key. Get one
    • fyyd: Parse error: error decoding response body
    • geizhals: empty results
    • gitea: empty results
    • github_code: empty results
    • google: empty results
    • huggingface: empty results
    • il_post: Parse error: error decoding response body
    • imgur: empty results
    • ina: empty results
    • jisho: empty results
    • kickass: empty results
    • libretranslate: empty results
    • lingva: empty results
    • livespace: HTTP request failed: error sending request for url (https://
    • marginalia: Engine 'marginalia' failed: No API key configured. Get one a
    • mediathekviewweb: empty results
    • mediawiki: empty results
    • metacpan: empty results
    • moviepilot: empty results
    • mozhi: empty results
    • mrs: empty results
    • naver: empty results
    • niconico: empty results
    • nyaa: empty results
    • ollama: empty results
    • open_meteo: empty results
    • openclipart: HTTP request failed: error sending request for url (https://
    • opensemantic: empty results
    • pdbe: empty results
    • pexels: Engine 'pexels' failed: No API key configured
    • photon: empty results
    • pinterest: Engine error: Pinterest parse failed: error decoding respons
    • piped: empty results
    • pixiv: empty results
    • podcastindex: Engine error: PodcastIndex parse failed: error decoding resp
    • presearch: empty results
    • public_domain_image_archive: empty results
    • pypi: empty results
    • qwant: Parse error: error decoding response body
    • radio_browser: empty results
    • recoll: empty results
    • reddit: Parse error: error decoding response body
    • repology: empty results
    • reuters: empty results
    • rumble: Engine error: Rumble request failed: error sending request f
    • scanr_structures: empty results
    • searchcode_code: Parse error: error decoding response body
    • searx_engine: empty results
    • selfhst: empty results
    • semantic_scholar: empty results
    • senscritique: empty results
    • seznam: empty results
    • solidtorrents: Engine error: SolidTorrents request failed: error sending re
    • solr: empty results
    • sourcehut: empty results
    • spotify: Configuration error: Spotify requires api_client_id and api_
    • springer: Engine 'springer' failed: No API key configured. Get one at
    • steam: empty results
    • stract: Engine error: Stract parse failed: error decoding response b
    • svgrepo: empty results
    • tagesschau: empty results
    • torznab: empty results
    • tubearchivist: empty results
    • uxwing: empty results
    • vimeo: empty results
    • wolframalpha_api: Engine 'wolframalpha_api' failed: No API key configured. Get
    • wttr: HTTP request failed: error sending request for url (https://
    • www1x: empty results
    • yacy: empty results
    • yandex: empty results
    • youtube_api: Engine 'youtube_api' failed: No API key configured. Get one
    • zlibrary: HTTP request failed: error sending request for url (https://

test probe_all_engines ... ok

test result: ok. 1 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 402.50s
