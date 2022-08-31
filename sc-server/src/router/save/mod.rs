use crate::model::{entity::ScItem, request::ItemJson};
use crate::types::Result;
use axum::{
    extract::State,
    response::{IntoResponse, Response},
    Json,
};
use serde_json::json;
use sqlx::{Pool, Postgres};
use tracing::instrument;

#[instrument(skip(frm, pool))]
pub async fn handler(
    State(pool): State<Pool<Postgres>>,
    Json(frm): Json<ItemJson>,
) -> Result<Response> {
    let item = ScItem::from(frm);
    let mut save_transaction = pool.begin().await?;

    item.insert(&mut save_transaction).await?;

    save_transaction.commit().await?;

    Ok(Json(json!({"status": "ok"})).into_response())
}
