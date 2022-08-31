use axum::{routing::get, Extension, Router};
use hyper::Method;
use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};
use std::{
    fs,
    path::{Path, PathBuf},
};
use tower::ServiceBuilder;
use tower_http::cors::{Any, CorsLayer};
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};

use crate::router::item;
use crate::router::pong::handler as pong;
use crate::CONFIG;

#[derive(Clone, Debug)]
pub struct LogWriter {
    path: PathBuf,
}

impl LogWriter {
    pub fn _spawn_logger<P: AsRef<Path>>(path: P) -> (NonBlocking, WorkerGuard) {
        let log = LogWriter {
            path: path.as_ref().to_path_buf(),
        };
        tracing_appender::non_blocking(log)
    }
}

impl std::io::Write for LogWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        let mut file = fs::OpenOptions::new()
            .append(true)
            .create(true)
            .open(&self.path)?;

        file.write_all(buf).map(|_| buf.len())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

pub async fn app() -> crate::types::Result<Router> {
    let pool = db().await?;
    Ok(Router::new()
        .route("/api", get(pong))
        .merge(item::router())
        .layer(
            ServiceBuilder::new()
                .layer(Extension(pool))
                .layer(
                    CorsLayer::new()
                        // Allow `GET` and `POST` when accessing the resource
                        .allow_methods([Method::GET, Method::POST])
                        // Allow requests from any origin
                        .allow_origin(Any),
                ),
        ))
}

pub async fn db() -> Result<Pool<Postgres>, Error> {
    PgPoolOptions::new()
        .max_connections(CONFIG.db_max_connections())
        .connect(&CONFIG.db_url())
        .await
}
