use axum::{
    response::{IntoResponse, Response},
    routing::get,
    Extension, Json, Router,
};
use serde_json::json;
use sqlx::{Pool, Postgres};
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
pub async fn total(Extension(pool): Extension<Pool<Postgres>>) -> Result<Response> {
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
    }))
    .into_response())
}

#[instrument(skip(pool))]
pub async fn get_tag(Extension(pool): Extension<Pool<Postgres>>) -> Result<Response> {
    let tags = Tag::get_all(&pool).await?;
    Ok(Json(json!({
        "status": "ok",
        "tags": tags
    }))
    .into_response())
}
