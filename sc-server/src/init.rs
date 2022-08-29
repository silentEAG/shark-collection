use axum::{Router, routing::{get, post}, Extension};
use sqlx::{Pool, Postgres, postgres::PgPoolOptions, Error};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

use crate::{router::pong::handler as pong, CONFIG};
use crate::router::save::handler as save;

#[inline]
pub fn log() {
    tracing_subscriber::registry()
    .with(tracing_subscriber::EnvFilter::new(
        std::env::var("RUST_LOG").unwrap_or_else(|_| "sc_server=TRACE".into()),
    ))
    .with(tracing_subscriber::fmt::layer())
    .init();
}

#[inline]
pub async fn app() -> anyhow::Result<Router> {
    let pool = db().await?;
    Ok(Router::new()
        .route("/", get(pong))
        .route("/save", post(save))
        .layer(Extension(pool)))
}

pub async fn db() -> Result<Pool<Postgres>, Error> {
    PgPoolOptions::new()
        .max_connections(CONFIG.db_max_connections())
        .connect(&CONFIG.db_url()).await
}