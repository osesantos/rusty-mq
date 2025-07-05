use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    usize,
};

use tokio::sync::broadcast;

use crate::broker::{message::Message, topic::match_topic};

type Topic = String;

pub struct BrokerEngine {
    subcribers: Arc<RwLock<HashMap<Topic, broadcast::Sender<Message>>>>,
    buffer_size: usize,
}

impl BrokerEngine {
    pub fn new(buffer_size: usize) -> Self {
        Self {
            subcribers: Arc::new(RwLock::new(HashMap::new())),
            buffer_size: buffer_size.max(1), // Ensure buffer size is at least 1
        }
    }

    pub fn subscribe(&self, topic: &str) -> broadcast::Receiver<Message> {
        let mut subs = self.subcribers.write().unwrap();

        if let Some(sender) = subs.get(topic) {
            return sender.subscribe();
        }

        let (tx, _) = broadcast::channel(self.buffer_size);
        subs.insert(topic.to_string(), tx.clone());

        tx.subscribe()
    }

    pub fn publish(&self, msg: Message) {
        let subs = self.subcribers.read().unwrap();

        for (pattern, sender) in subs.iter() {
            if match_topic(pattern, &msg.topic) {
                let _ = sender.send(msg.clone());
            }
        }
    }

    pub async fn run(&self) {
        // This method can be used to run the broker engine if needed.
        // Currently, it does nothing but can be extended for future use.
        print!("BrokerEngine is running. Waiting for messages...\n");
    }
}
