use config::ConfigBuilder;
use futures::join;
use once_cell::sync::Lazy;
use std::net::SocketAddr;
use tracing::metadata::LevelFilter;
use tracing_appender::{non_blocking, rolling};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

mod common;
mod config;
mod error;
mod init;
mod model;
mod router;
mod types;

static CONFIG: Lazy<config::ConfigItems> = Lazy::new(|| {
    let builder = ConfigBuilder::default().add_env();
    builder.build()
});

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Logger subscribe
    // TODO: Make a LogWriter by self for more features such as filtering ansi
    // Generate none blocking logger in file
    let file_appender = rolling::daily("logs", CONFIG.log_file_name());
    let (none_blocking_file_appender, _guard) = non_blocking(file_appender);

    // Tracing subscriber
    tracing_subscriber::registry()
        // Set the Log level
        .with(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "TRACE".into()),
        ))
        // Set the file logger
        .with(
            tracing_subscriber::fmt::layer()
                .with_ansi(false)
                .with_writer(none_blocking_file_appender)
                .with_filter(LevelFilter::TRACE),
        )
        // Set the stdout/stderr logger
        .with(tracing_subscriber::fmt::layer())
        .init();

    // Make sure config loading is right
    tracing::debug!("\nShark Collection Config Info:{}", *CONFIG);

    let app = init::app().await?;

    let addr_port = CONFIG.app_port();
    let addr = SocketAddr::from(([127, 0, 0, 1], addr_port));
    tracing::info!("Listening on {}...", addr);

    // Server ready to start
    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .with_graceful_shutdown(shutdown_signel());

    // Start server
    let (res,) = join!(server);
    if res.is_err() {
        tracing::error!("Server Error: {}", res.err().unwrap());
    }
    Ok(())
}

/// Receive shutdown signel
async fn shutdown_signel() {
    let recv_ctrl_c = async {
        tokio::signal::ctrl_c()
            .await
            .expect("Failed to install Ctrl-C handler");
    };

    #[cfg(unix)]
    let recv_terminate = async {
        tokio::signal::unix::signal(tokio::signal::unix::SignalKind::terminate())
            .expect("Failed to install signal handler")
            .recv()
            .await;
    };

    #[cfg(not(unix))]
    let recv_terminate = std::future::pending::<()>();

    tokio::select! {
        _ = recv_ctrl_c => {work_before_shutdown()},
        _ = recv_terminate => {work_before_shutdown()},
    }
}

/// TODO: Can do something there before shutdown
fn work_before_shutdown() {
    tracing::info!("going to shutdown...");
}
