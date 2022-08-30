use crate::types::Result;
use crate::{
    error::ServerError,
    model::{entity::ScItem, request::ItemJson},
};
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use sqlx::{Pool, Postgres};
use tracing::instrument;

#[instrument(skip(frm, pool))]
pub async fn handler(
    State(pool): State<Pool<Postgres>>,
    Json(frm): Json<ItemJson>,
) -> Result<Response> {
    let item = ScItem::from(frm);

    let res = sqlx::query!(
        "INSERT INTO sc_item (url, title, tags, catalog) VALUES ($1, $2, $3, $4)",
        item.url,
        item.title,
        item.tags,
        item.catalog
    )
    .execute(&pool)
    .await?;

    match res.rows_affected() {
        0 => Err(ServerError::OtherWithMessage("Insert Failed.".to_string())),
        _ => Ok(Json(item).into_response()),
    }
}
