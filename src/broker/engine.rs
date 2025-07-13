use std::{
    collections::HashMap,
    sync::{Arc, RwLock},
    usize,
};

use tokio::sync::broadcast;

use crate::broker::{message::Message, topic::match_topic};

type Topic = String;

pub struct BrokerEngine {
    subscribers: Arc<RwLock<HashMap<Topic, broadcast::Sender<Message>>>>,
    buffer_size: usize,
}

impl BrokerEngine {
    pub fn new(buffer_size: usize) -> Self {
        Self {
            subscribers: Arc::new(RwLock::new(HashMap::new())),
            buffer_size: buffer_size.max(1), // Ensure buffer size is at least 1
        }
    }

    pub fn subscribe(&self, topic: &str) -> broadcast::Receiver<Message> {
        let mut subs = self.subscribers.write().unwrap();

        if let Some(sender) = subs.get(topic) {
            return sender.subscribe();
        }

        let (tx, _) = broadcast::channel(self.buffer_size);
        subs.insert(topic.to_string(), tx.clone());

        tx.subscribe()
    }

    pub fn publish(&self, msg: Message) {
        let subs = self.subscribers.read().unwrap();

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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::broker::message::Message;

    #[tokio::test]
    async fn test_publish_and_subscribe() {
        let broker = BrokerEngine::new(10);
        let mut subscriber = broker.subscribe("test.topic");

        let msg = Message {
            topic: "test.topic".to_string(),
            payload: serde_json::json!({"key": "value"}),
        };

        broker.publish(msg.clone());

        let received_msg = subscriber.recv().await.unwrap();
        assert_eq!(received_msg.topic, msg.topic);
        assert_eq!(received_msg.payload, msg.payload);
    }

    #[tokio::test]
    async fn test_wildcard_subscription() {
        let broker = BrokerEngine::new(10);
        let mut subscriber = broker.subscribe("test.*");

        let msg1 = Message {
            topic: "test.topic1".to_string(),
            payload: serde_json::json!({"key": "value1"}),
        };
        let msg2 = Message {
            topic: "test.topic2".to_string(),
            payload: serde_json::json!({"key": "value2"}),
        };

        broker.publish(msg1.clone());
        broker.publish(msg2.clone());

        let received_msg1 = subscriber.recv().await.unwrap();
        assert_eq!(received_msg1.topic, msg1.topic);
        assert_eq!(received_msg1.payload, msg1.payload);

        let received_msg2 = subscriber.recv().await.unwrap();
        assert_eq!(received_msg2.topic, msg2.topic);
        assert_eq!(received_msg2.payload, msg2.payload);
    }

    #[tokio::test]
    async fn test_wildcard_two_levels_subscription() {
        let broker = BrokerEngine::new(10);
        let mut subscriber = broker.subscribe("test.>");

        let msg1 = Message {
            topic: "test.topic1.key1".to_string(),
            payload: serde_json::json!({"key": "value1"}),
        };
        let msg2 = Message {
            topic: "test.topic2.key2".to_string(),
            payload: serde_json::json!({"key": "value2"}),
        };

        broker.publish(msg1.clone());
        broker.publish(msg2.clone());

        let received_msg1 = subscriber.recv().await.unwrap();
        assert_eq!(received_msg1.topic, msg1.topic);
        assert_eq!(received_msg1.payload, msg1.payload);

        let received_msg2 = subscriber.recv().await.unwrap();
        assert_eq!(received_msg2.topic, msg2.topic);
        assert_eq!(received_msg2.payload, msg2.payload);
    }
}
