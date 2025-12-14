use iggy::clients::client::IggyClient;
use iggy::prelude::{
    Client, CompressionAlgorithm, IggyExpiry, IggyMessage, MaxTopicSize,
    MessageClient, Partitioning, StreamClient, TopicClient, UserClient, DEFAULT_ROOT_PASSWORD, DEFAULT_ROOT_USERNAME,
};
use std::error::Error;
use std::str::FromStr;
use std::time::Duration;
use tokio::time::sleep;
use tracing::{info, warn};

const STREAM_ID: &str = "sample-stream";
const TOPIC_ID: &str = "sample-topic";
const PARTITION_ID: u32 = 1;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let client = IggyClient::default();
    client.connect().await?;
    client
        .login_user(DEFAULT_ROOT_USERNAME, DEFAULT_ROOT_PASSWORD)
        .await?;
    init_system(&client).await;
    produce_messages(&client).await?;
    Ok(())
}

async fn init_system(client: &IggyClient) {
    match client.create_stream(STREAM_ID).await {
        Ok(_) => info!("Stream was created: {}", STREAM_ID),
        Err(_) => warn!(
            "Stream already exists and will not be created again: {}",
            STREAM_ID
        ),
    }

    match client
        .create_topic(
            &STREAM_ID.try_into().unwrap(),
            TOPIC_ID,
            1,
            CompressionAlgorithm::default(),
            None,
            IggyExpiry::NeverExpire,
            MaxTopicSize::ServerDefault,
        )
        .await
    {
        Ok(_) => info!("Topic was created: {}", TOPIC_ID),
        Err(_) => warn!(
            "Topic already exists and will not be created again: {}",
            TOPIC_ID
        ),
    }
}

async fn produce_messages(client: &IggyClient) -> Result<(), Box<dyn Error>> {
    let interval = Duration::from_millis(500);
    info!(
        "Messages will be sent to stream: {}, topic: {}, partition: {} with interval {} ms.",
        STREAM_ID,
        TOPIC_ID,
        PARTITION_ID,
        interval.as_millis()
    );

    let mut current_id = 0;
    let messages_per_batch = 10;
    let partitioning = Partitioning::partition_id(PARTITION_ID);
    loop {
        let mut messages = Vec::new();
        for _ in 0..messages_per_batch {
            current_id += 1;
            let payload = format!("message-{current_id}");
            let message = IggyMessage::from_str(&payload)?;
            messages.push(message);
        }
        client
            .send_messages(
                &STREAM_ID.try_into().unwrap(),
                &TOPIC_ID.try_into().unwrap(),
                &partitioning,
                &mut messages,
            )
            .await?;
        info!("Sent {messages_per_batch} message(s).");
        sleep(interval).await;
    }
}
