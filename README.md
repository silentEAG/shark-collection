# Shark Collection
**Under Coding**

A Web application for **recording** and **sharing** e-Information (mostly article on website).

Use Rust/axum and PostgresQL.

## API (Not Finish, Not stable)

Parameters are ignored.

- `common`
  - `/api`: Health checker.
- `item`
  - `/api/item/save`: Save the item. (Auth required)
  - `/api/item/total`: Return item's total numbers.
  - `/api/item/get`: Return item's infomation by pages.
- `tag`
  - `/api/tag/total`: Return tag's total numbers.
  - `/api/tag/get`: Return tag's all information.
- `catalog`
  - `/api/catalog/total`: Return catalog's total numbers.
  - `/api/catalog/get`: Return catalog's all information.
  - `/api/catalog/add`: Add new catalog. (Auth required)
- `search` (TODO)
  - `/api/search/light`: Search items by its name, tags and catalog.
  - `/api/search/engine`: Using full-text search engine, return item infomation related. (Should invert index in item's content first)
- `rss` (TODO)
  - `/api/rss/generate`: Generate rss by parameters given.
- `trace` (TODO)
- `shot` (TODO)
  - `/api/shot/save`: Save item's content.

## 💡 Plan | Features
- RSS supports.
- Tags and Catalog support.
- Allowed to subscribe to some websites for receiving updating.
- Snapshot module for saving website's content.
- Url-shorten module as an extension.
- Light mode uses no full-text search egine, while Custom mode uses certain one.
