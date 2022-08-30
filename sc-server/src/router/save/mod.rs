use crate::model::request::UrlJson;
use axum::{Extension, Json};
use sqlx::{Pool, Postgres};
use tracing::instrument;

#[instrument(skip(frm, pool))]
pub async fn handler(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(frm): Json<UrlJson>
) {
    let title = frm.title;
    let catalog = frm.catalog;
    let tags = frm.tags;
}
