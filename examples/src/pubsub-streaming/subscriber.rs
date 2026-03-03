use std::time::Duration;

use tokio_stream::StreamExt;

use dapr::client::subscribe_topic_events_response_alpha1::SubscribeTopicEventsResponseType;
use dapr::client::topic_event_response::TopicEventResponseStatus;
use dapr::client::SubscribeTopicEventsRequestProcessedAlpha1;
use dapr::client::TopicEventResponse;
use dapr::serde::{Deserialize, Serialize};
use dapr::serde_json;

#[derive(Serialize, Deserialize, Debug)]
struct Order {
    pub order_number: i32,
    pub order_details: String,
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // TODO: Handle this issue in the sdk
    // Introduce delay so that dapr grpc port is assigned before app tries to connect
    tokio::time::sleep(Duration::from_secs(2)).await;

    // Set address for Dapr connection
    let addr = "https://127.0.0.1".to_string();

    // Create the client
    let mut client = dapr::Client::<dapr::client::TonicClient>::connect(addr).await?;

    // Subscribe to topic "A" on the "pubsub" component via gRPC streaming
    let (ack_tx, mut stream) = client
        .subscribe_topic_events_alpha1("pubsub", "A", None, None::<&str>)
        .await?;

    println!("Streaming subscriber started, waiting for events...");

    while let Some(Ok(response)) = stream.next().await {
        match response.subscribe_topic_events_response_type {
            Some(SubscribeTopicEventsResponseType::InitialResponse(_)) => {
                println!("Subscription established");
            }
            Some(SubscribeTopicEventsResponseType::EventMessage(event)) => {
                let order: Order = serde_json::from_slice(&event.data)?;
                println!("Received event: {order:#?}");

                // Acknowledge the event
                ack_tx
                    .send(SubscribeTopicEventsRequestProcessedAlpha1 {
                        id: event.id,
                        status: Some(TopicEventResponse {
                            status: TopicEventResponseStatus::Success as i32,
                        }),
                    })
                    .await?;
            }
            None => break,
        }
    }

    Ok(())
}
