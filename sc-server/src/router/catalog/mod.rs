use axum::{Extension, response::{Response, IntoResponse}, Router, routing::get, Json};
use serde_json::json;
use sqlx::{Postgres, Pool};
use tracing::instrument;

use crate::types::Result;

pub mod entity;

pub fn router() -> Router {
    Router::new()
        .route("/api/catalog/total", get(total))
}

#[instrument(skip(pool))]
pub async fn total(
    Extension(pool): Extension<Pool<Postgres>>,
) -> Result<Response> {
    let r = sqlx::query!(
        r#"
            SELECT count(*) FROM sc_catalog
        "#
    )
    .fetch_one(&pool)
    .await?;
    Ok(Json(json!({
        "status": "ok",
        "catalog_total": r.count
    })).into_response())
}