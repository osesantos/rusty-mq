use clap::{Parser, Subcommand};
use colored::Colorize;
use rustymq::broker_service_client::BrokerServiceClient;
use tokio_stream::StreamExt;

const URL: &str = "http://[::1]:50053";

pub mod rustymq {
    tonic::include_proto!("rustymq");
}

#[derive(Debug)]
enum Command {
    Publish {
        topic: String,
        message: String,
    },
    Subscribe {
        topic: String,
    },
}

pub async fn run() -> Result<(), Box<dyn std::error::Error>> {
    println!("###########################################");
    println!("#                                         #");
    println!("#    🔥🔥🔥 RustyMQ Client 🔥🔥🔥         #");
    println!("#                                         #");
    println!("###########################################");

    // Ask for the command (publish or subscribe)
    let mut input = String::new();
    println!("🎯 Enter command (publish/subscribe): ");
    std::io::stdin().read_line(&mut input)?;
    let command = match input.trim().to_lowercase().as_str() {
        "publish" => {
            // Get topic and message for publishing
            let mut topic = String::new();
            println!("{}", "- Enter topic: ".yellow());
            std::io::stdin().read_line(&mut topic)?;
            let topic = topic.trim().to_string();

            let mut message = String::new();
            println!("{}", "- Enter message: ".yellow());
            std::io::stdin().read_line(&mut message)?;
            let message = message.trim().to_string();

            Command::Publish { topic, message }
        }
        "subscribe" => {
            // Get topic for subscribing
            let mut topic = String::new();
            println!("{}", "- Enter topic to subscribe: ".yellow());
            std::io::stdin().read_line(&mut topic)?;
            let topic = topic.trim().to_string();

            Command::Subscribe { topic }
        }
        _ => {
            println!("❌ Invalid command. Please enter 'publish' or 'subscribe'.");
            return Ok(());
        }
    };

    let mut confirmation = String::new();
    println!("🚀 You entered: {}", format!("{:?}", command).cyan());
    println!("😎 Do you want to proceed? (yes/no): ");
    std::io::stdin().read_line(&mut confirmation)?;
    if confirmation.trim().to_lowercase() != "yes" {
        println!("❌ Operation cancelled.");
        return Ok(());
    }

    // Create gRPC client and connect to the broker
    let mut client = BrokerServiceClient::connect(URL).await.expect("❌ Failed to connect to RustyMQ broker");
    println!("🔗 Connected to RustyMQ broker at {}", URL);

    // Here you would normally call the broker engine to handle the command
    match command {
        Command::Publish { topic, message } => {
            println!("🔥 Publishing message '{}' to topic '{}'", message, topic);
            let request = tonic::Request::new(rustymq::PublishRequest {
                topic,
                message,
            });
            let response = client.publish(request).await?;
            if response.into_inner().success {
                println!("✅ Message published!");
            } else {
                println!("❌ Failed");
            }
        }
        Command::Subscribe { topic } => {
            println!("🔥 Subscribing to topic '{}'", topic);
            let request = tonic::Request::new(rustymq::SubscribeRequest { topic });
            let mut stream = client.subscribe(request).await?.into_inner();
            println!("🧲 Subscribed! Waiting for messages...\n");

            while let Some(Ok(msg)) = stream.next().await {
                println!("📡 Topic: {}\n{}\n", msg.topic, msg.message);
                println!("----------------------\n");
            }
        }
    }

    Ok(())
}