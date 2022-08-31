# Shark Collection
A Web application for **recording** and **sharing** e-Information (mostly article on website).
Use Rust/axum and PostgresQL.

## API
- `item`
  - `/api/item/save`: Save the item.
  - `/api/item/total`: Return item's total numbers.
  - `/api/item/get?page={page}`: Return item's infomation by pages.
- `tag`
  - `/api/tag/total`: Return tag's total numbers.
- `catalog`
  - `/api/catalog/total`: Return catalog's total numbers.
  - `/api/catalog/get`: Return catalog's information.


## 💡 Plan | Features
- RSS supports.
- Tags and Catalog support.
- Allowed to subscribe to some websites for receiving updating.
- Snapshot module for saving website's content.
- Url-shorten module as an extension.
- Light mode uses no full-text search egine, while Custom mode uses certain one.