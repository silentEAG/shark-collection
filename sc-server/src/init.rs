use std::{
    fs,
    path::{Path, PathBuf},
};

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::{postgres::PgPoolOptions, Error, Pool, Postgres};
use tracing_appender::non_blocking::{NonBlocking, WorkerGuard};

use crate::router::pong::handler as pong;
use crate::router::save::handler as save;
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

pub async fn app() -> crate::types::Result<Router<Pool<Postgres>>> {
    let pool = db().await?;
    Ok(Router::with_state(pool)
        .route("/", get(pong))
        .route("/save", post(save)))
}

pub async fn db() -> Result<Pool<Postgres>, Error> {
    PgPoolOptions::new()
        .max_connections(CONFIG.db_max_connections())
        .connect(&CONFIG.db_url())
        .await
}
