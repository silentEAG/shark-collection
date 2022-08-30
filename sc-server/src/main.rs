use config::ConfigBuilder;
use futures::join;
use once_cell::sync::Lazy;
use tracing_appender::{rolling, non_blocking};
use std::net::SocketAddr;
use tracing::metadata::LevelFilter;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt, Layer};

mod common;
mod config;
mod init;
mod model;
mod router;

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
        .with(
            tracing_subscriber::fmt::layer()
                .pretty()
        )
        .init();

    // Make sure config loading is right
    tracing::debug!("\nShark Collection Config Info:{}", *CONFIG);

    let app = init::app().await?;

    let addr_port = CONFIG.app_port();
    let addr = SocketAddr::from(([127, 0, 0, 1], addr_port));
    tracing::info!("Listening on {}...", addr);

    // Server ready to start
    let server = axum::Server::bind(&addr).serve(app.into_make_service());

    // Ctrl-C controller
    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        tracing::info!("going to shut down...");
        // TODO: Can do something there before shutdown
        std::process::exit(0);
    });

    // Start server
    let (res,) = join!(server);
    if res.is_err() {
        tracing::error!("Server Error: {}", res.err().unwrap());
    }
    Ok(())
}
