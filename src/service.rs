use std::pin::Pin;
use tonic::{Response, Status};
use rustymq::broker_service_server::{BrokerService};
use rustymq::{PublishRequest, PublishResponse, SubscribeRequest, Message};
use crate::broker::engine::BrokerEngine;
use async_stream::stream;

pub mod rustymq {
    tonic::include_proto!("rustymq");
}

#[derive(Default)]
pub struct Service{
    pub engine: BrokerEngine,
}

#[tonic::async_trait]
impl BrokerService for Service {
    async fn publish(
        &self,
        request: tonic::Request<PublishRequest>,
    ) -> Result<tonic::Response<PublishResponse>, tonic::Status> {
        let msg = request.into_inner();
        self.engine.publish(&msg.topic, &msg.message);
        Ok(tonic::Response::new(
            PublishResponse {
                success: true,
            },
        ))
    }

    type SubscribeStream = Pin<Box<dyn tokio_stream::Stream<Item = Result<Message, Status>> + Send + 'static>>;

    async fn subscribe(
        &self,
        request: tonic::Request<SubscribeRequest>,
    ) -> Result<tonic::Response<Self::SubscribeStream>, tonic::Status> {
        let topic = request.into_inner().topic;
        let mut receiver = self.engine.subscribe(&topic);

        let stream = stream! {
            while let Ok(msg) = receiver.recv().await {
                yield Ok(Message {
                    topic: topic.clone(),
                    message: msg,
                });
            }
        };

        Ok(Response::new(Box::pin(stream) as Self::SubscribeStream))
    }
}
