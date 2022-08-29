use axum::{Extension, Json};
use sqlx::{Pool, Postgres};
use crate::model::request::UrlJson;


pub async fn handler(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(frm): Json<UrlJson>
) {
    let title = frm.title;
    let catalog = frm.catalog;
    let tags = frm.tags;
    
}