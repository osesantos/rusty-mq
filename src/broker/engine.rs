use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    usize,
};
use tokio::sync::broadcast;
use chrono::Utc;
use crate::broker::{topic::match_topic};

type Topic = String;

#[derive(Default, Clone)]
pub struct BrokerEngine {
    subscribers: Arc<RwLock<HashMap<Topic, broadcast::Sender<String>>>>,
    buffer_size: usize,
}

impl BrokerEngine {
    pub fn new(buffer_size: usize) -> Self {
        Self {
            subscribers: Arc::new(RwLock::new(HashMap::new())),
            buffer_size: buffer_size.max(1), // Ensure buffer size is at least 1
        }
    }

    pub fn subscribe(&self, topic: &str) -> broadcast::Receiver<String> {
        let mut subs = self.subscribers.write().unwrap();

        if let Some(sender) = subs.get(topic) {
            return sender.subscribe();
        }

        let (tx, _) = broadcast::channel(self.buffer_size);
        subs.insert(topic.to_string(), tx.clone());

        println!("{} | Client subscribed to topic '{}'", Utc::now(), topic);

        tx.subscribe()
    }

    pub fn publish(&self, topic: &str , msg: &str) {
        let subs = self.subscribers.read().unwrap();

        for (pattern, sender) in subs.iter() {
            if match_topic(pattern, topic) {
                let _ = sender.send(msg.to_string());

                println!("{} | Client published message '{}' to topic '{}'", Utc::now(), msg, pattern);
            } else {
                println!("{} | Client tried to publish message '{}' to topic '{}' but there is no subs", Utc::now(), msg, topic);
            }
        }

    }

    pub async fn run(&self) {
        // This method can be used to run the broker engine if needed.
        // Currently, it does nothing but can be extended for future use.
        print!("{} | BrokerEngine is running. Waiting for messages...\n", Utc::now());
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_publish_and_subscribe() {
        let broker = BrokerEngine::new(10);
        let mut subscriber = broker.subscribe("test.topic");

        let topic = "test.topic".to_string();
        let payload = serde_json::json!({"key": "value"});

        broker.publish(&topic, &payload.to_string());

        let received_msg = subscriber.recv().await.unwrap();
        assert_eq!(received_msg, payload);
    }

    #[tokio::test]
    async fn test_wildcard_subscription() {
        let broker = BrokerEngine::new(10);
        let mut subscriber = broker.subscribe("test.*");

        let topic1 = "test.topic1".to_string();
        let payload2 = serde_json::json!({"key": "value1"});
        let topic2 = "test.topic2".to_string();
        let payload2 = serde_json::json!({"key": "value2"});

        broker.publish(&topic1, &payload2.to_string());
        broker.publish(&topic2, &payload2.to_string());

        let received_msg1 = subscriber.recv().await.unwrap();
        assert_eq!(received_msg1, payload2);

        let received_msg2 = subscriber.recv().await.unwrap();
        assert_eq!(received_msg2, payload2);
    }

    #[tokio::test]
    async fn test_wildcard_two_levels_subscription() {
        let broker = BrokerEngine::new(10);
        let mut subscriber = broker.subscribe("test.>");

        let topic1 = "test.topic1.key1".to_string();
        let payload1 = serde_json::json!({"key": "value1"});
        let topic2 = "test.topic2.key2".to_string();
        let payload2 = serde_json::json!({"key": "value2"});

        broker.publish(&topic1, &payload1.to_string());
        broker.publish(&topic2, &payload2.to_string());

        let received_msg1 = subscriber.recv().await.unwrap();
        assert_eq!(received_msg1, payload1);

        let received_msg2 = subscriber.recv().await.unwrap();
        assert_eq!(received_msg2, payload2);
    }
}
