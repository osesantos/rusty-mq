use std::env;

#[derive(Debug, Clone)]
pub struct Config {
    // RUSTYMQ_PORT is the port on which the RustyMQ server will listen for incoming connections.
    pub port: u16,

    // RUSTYMQ_BUFFER_SIZE is the size of the buffer used for reading messages.
    pub buffer_size: usize,
}

impl Config {
    pub fn from_env() -> Self {
        let _ = dotenvy::dotenv(); // Load environment variables from .env file if it exists

        let port = env::var("RUSTYMQ_PORT")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(8080); // Default to 8080 if not set or parse fails

        let buffer_size = env::var("RUSTYMQ_BUFFER_SIZE")
            .ok()
            .and_then(|v| v.parse().ok())
            .unwrap_or(100); // Default to 100 if not set or parse fails

        Self { port, buffer_size }
    }
}
