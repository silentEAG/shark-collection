use axum::{
    response::{IntoResponse, Response},
    routing::{get, post},
    Extension, Json, Router,
};
use serde_json::json;
use sqlx::{Pool, Postgres};
use tracing::instrument;

use crate::types::Result;

use self::entity::{Catalog, CatalogBody, NewICatalog};

pub mod entity;

pub fn router() -> Router {
    Router::new()
        .route("/api/catalog/total", get(total))
        .route("/api/catalog/get", get(get_catalog))
        .route("/api/catalog/add", post(add))
}

#[instrument(skip(pool))]
pub async fn total(Extension(pool): Extension<Pool<Postgres>>) -> Result<Response> {
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
    }))
    .into_response())
}

#[instrument(skip(pool))]
pub async fn get_catalog(Extension(pool): Extension<Pool<Postgres>>) -> Result<Response> {
    let catalogs = Catalog::get_all(&pool).await?;
    Ok(Json(json!({
        "status": "ok",
        "catalogs": catalogs
    }))
    .into_response())
}

#[instrument(skip(pool))]
pub async fn add(
    Extension(pool): Extension<Pool<Postgres>>,
    Json(frm): Json<CatalogBody<NewICatalog>>,
) -> Result<Response> {
    let catalog = Catalog::new(frm.catalog.name);
    catalog.add(&pool).await?;
    Ok(Json(json!({
        "status": "ok"
    }))
    .into_response())
}
