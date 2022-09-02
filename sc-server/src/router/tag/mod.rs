use axum::{Extension, response::{Response, IntoResponse}, Router, routing::get, Json};
use serde_json::json;
use sqlx::{Postgres, Pool};
use tracing::instrument;

use crate::types::Result;

use self::entity::Tag;

pub mod entity;

pub fn router() -> Router {
    Router::new()
        .route("/api/tag/total", get(total))
        .route("/api/tag/get", get(get_tag))
}

#[instrument(skip(pool))]
pub async fn total(
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<Response> {
    let r = sqlx::query!(
        r#"
            SELECT count(*) FROM sc_tag
        "#
    )
    .fetch_one(&pool)
    .await?;
    Ok(Json(json!({
        "status": "ok",
        "tag_total": r.count
    })).into_response())
}

#[instrument(skip(pool))]
pub async fn get_tag(
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<Response> {
    let records = sqlx::query!(
        r#"
            SELECT * FROM sc_tag ORDER BY sc_tag.num DESC, sc_tag.name ASC
        "#
    )
    .fetch_all(&pool)
    .await?;
    let res: Vec<Tag> = records.into_iter().map(|r| {
        Tag::new_with_num(r.name, r.num as isize)
    }).collect();
    
    Ok(Json(json!({
        "status": "ok",
        "tags": res
    })).into_response())
}