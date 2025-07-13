mod broker;
mod config;
mod error;
mod metrics;
mod service;
mod startup;
mod client;

use clap::Parser;
use tracing::info;
use tracing_subscriber;
use crate::startup::run;

#[derive(Parser, Debug)]
#[command(
    name = "RustyMQ",
    author = "osesantos",
    version = "0.1.0",
    about = "A lightweight message queue server written in Rust",
    long_about = None,
)]
struct Cli {
    #[arg(short, long, default_value = "50053")]
    port: u16,
    #[arg(short, long, default_value = "false")]
    client: bool,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    // Setup logging
    tracing_subscriber::fmt::init();

    // Check if the client flag is set
    if Cli::parse().client {
        info!("Running in client mode");
        client::run().await.unwrap();
        return Ok(());
    }

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

    let _ = run(&port.to_string()).await;

    // Wait for a shutdown signal
    tokio::signal::ctrl_c().await?;
    info!("Received shutdown signal, shutting down gracefully...");
    Ok(())
}
