use tonic::transport::Server;
use crate::broker::engine::BrokerEngine;
use crate::service::rustymq::broker_service_server::BrokerServiceServer;
use crate::service::Service;

pub async fn run(port: &String) -> Result<(), Box<dyn std::error::Error>> {
    let addr = format!("[::1]:{}", port).parse()?;
    let broker = Service {
        engine: BrokerEngine::new(100), // Default buffer size
    };

    println!("🚀 RustyMQ gRPC listening on {}", addr);
    broker.engine.run().await;

    Server::builder()
        .add_service(BrokerServiceServer::new(broker))
        .serve(addr)
        .await?;

    Ok(())
}
