mod broker;
mod config;
mod error;
mod metrics;

use std::time::Duration;

use broker::engine::BrokerEngine;
use broker::message::Message;

use clap::Parser;
use serde_json::json;
use tokio::time::sleep;
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

    let broker = BrokerEngine::new(config.buffer_size);
    let mut subscriber = broker.subscribe("test.*");

    tokio::spawn(async move {
        while let Ok(msg) = subscriber.recv().await {
            info!("Received message: {:?}", msg);
        }
    });

    // Testing the publish method
    sleep(Duration::from_secs(1)).await; // Wait a bit before publishing

    broker.publish(Message {
        topic: "test.topic".to_string(),
        payload: json!({"response": "Hello, World!"}),
    });

    sleep(Duration::from_secs(1)).await; // Wait a bit before publishing

    // Wait for a shutdown signal
    tokio::signal::ctrl_c().await?;
    info!("Received shutdown signal, shutting down gracefully...");
    Ok(())
}
