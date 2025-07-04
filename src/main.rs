mod broker;
mod config;
mod error;
mod metrics;

use clap::Parser;
use tracing::info;
use tracing_subscriber;

#[derive(Parser, Debug)]
#[command(
    name = "RustyMQ",
    author = "osesantos",
    version = "0.1.0",
    about = "A lightweight message queue server written in Rust",
    long_about = None,
)]
struct Cli {
    // Port to bind the server to
    #[arg(short, long, default_value = "8080")]
    port: u16,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup logging
    tracing_subscriber::fmt::init();

    let cli = Cli::parse();
    let config = config::Config::from_env();
    let use_cli_port = cli.port != config.port;
    if use_cli_port {
        info!(
            "Command line port {} does not match config port {}, using command line value.",
            cli.port, config.port
        );
    }

    let port = if use_cli_port { cli.port } else { config.port };

    info!("Starting RustyMQ on port {}", port);

    // TODO - Initialize the broker with the specified port

    tokio::signal::ctrl_c().await?;
    info!("Received shutdown signal, shutting down gracefully...");
    Ok(())
}
