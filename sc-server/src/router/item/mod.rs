use self::entity::Item;
use crate::common::err_message;
use crate::model::request::{ItemBody, NewItem};
use crate::types::Result;
use axum::routing::post;
use axum::{
    response::{IntoResponse, Response},
    Json,
};
use axum::{Extension, Router};
use serde_json::json;
use sqlx::{Pool, Postgres};
use tracing::instrument;

use super::catalog::entity::Catalog;
use super::tag::entity::Tag;

pub mod entity;

pub fn router() -> Router {
    Router::new().route("/api/item/save", post(save))
}

#[instrument(skip(frm, pool))]
pub async fn save(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(frm): Json<ItemBody<NewItem>>,
) -> Result<Response> {
    let mut item = Item::from(frm.item);

    // Judge if item exists
    if item.find_id_by_url(&pool).await != 0 {
        return Err(err_message("Item already exists."));
    }

    // Update catalog
    let mut catalog = Catalog::new(item.catalog.clone());

    // Check catalog if exists
    let catalog_id = catalog.is_existd(&pool).await?;
    catalog.id = Some(catalog_id);

    // Enable transaction
    let mut save_transaction = pool.begin().await?;

    // Update catalog
    catalog.update(&mut save_transaction).await?;

    // Insert to item table, `tags_id` is not update yet
    let id = item.insert(&mut save_transaction).await?;
    item.id = Some(id);

    // 1. Upsert tag in table
    // 2. Collect tag's id for updating item table in `tags_id`
    // 3. Record mapping between tag and item
    let mut tag_id_vec = Vec::with_capacity(item.tags.len());
    for tag in &item.tags {
        let tag_id = Tag::new(tag.clone()).upsert(&mut save_transaction).await?;
        tag_id_vec.push(tag_id as i32);
        sqlx::query!(
            r#"
                INSERT INTO sc_tag_map
                (tag_id, item_id)
                VALUES ($1, $2)
            "#,
            tag_id as i32,
            item.id.unwrap() as i32
        )
        .execute(&mut *save_transaction)
        .await?;
    }

    // Update `tags_id` in item table
    item.update_tag_id_list(&tag_id_vec, &mut save_transaction)
        .await?;

    // Commit transaction
    save_transaction.commit().await?;

    Ok(Json(json!({"status": "ok"})).into_response())
}
