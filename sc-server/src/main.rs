use std::net::SocketAddr;
use config::ConfigBuilder;
use futures::join;
use once_cell::sync::Lazy;

mod common;
mod router;
mod config;
mod model;
mod init;

static CONFIG: Lazy<config::ConfigItems> = Lazy::new(||{
    let builder = ConfigBuilder::from_env().unwrap();
    builder.build()
});


#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Init work
    init::log();

    tracing::debug!("\nShark Collection Config Info:{}", *CONFIG);

    let app = init::app().await?;



    let addr_port = CONFIG.app_port();
    let addr = SocketAddr::from(([127, 0, 0, 1], addr_port));
    tracing::debug!("listening on {}", addr);

    // Server ready to start
    let server = axum::Server::bind(&addr)
        .serve(app.into_make_service());

    tokio::spawn(async move {
        tokio::signal::ctrl_c().await.unwrap();
        tracing::info!("Going to shut down...");
        // Can do something there before shutdown
        std::process::exit(0);
    });
    
    // Start server
    let (res, ) = join!(server);
    if res.is_err() {
        tracing::error!("Server Error: {}", res.err().unwrap());
    }
    Ok(())
}
