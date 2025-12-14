use iggy::clients::client::IggyClient;
use iggy::prelude::{
    Client, Consumer, IggyMessage, MessageClient, PollingStrategy, UserClient,
    DEFAULT_ROOT_PASSWORD, DEFAULT_ROOT_USERNAME,
};
use std::error::Error;
use std::time::Duration;
use tokio::time::sleep;
use tracing::info;

const STREAM_ID: &str = "sample-stream";
const TOPIC_ID: &str = "sample-topic";
const PARTITION_ID: u32 = 1;
const STOP_AFTER_X_EMPTY_BATCHES: u32 = 10;

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    tracing_subscriber::fmt::init();
    let client = IggyClient::default();
    client.connect().await?;
    client
        .login_user(DEFAULT_ROOT_USERNAME, DEFAULT_ROOT_PASSWORD)
        .await?;
    consume_messages(&client).await
}

async fn consume_messages(client: &IggyClient) -> Result<(), Box<dyn Error>> {
    let interval = Duration::from_millis(500);
    info!(
        "Messages will be consumed from stream: {}, topic: {}, partition: {} with interval {} ms.",
        STREAM_ID,
        TOPIC_ID,
        PARTITION_ID,
        interval.as_millis()
    );

    let mut offset = 0;
    let messages_per_batch = 10;
    let consumer = Consumer::default();
    let mut strategy = PollingStrategy::offset(offset);
    let mut empty_counter = 0;
    loop {
        let polled_messages = client
            .poll_messages(
                &STREAM_ID.try_into()?,
                &TOPIC_ID.try_into()?,
                Some(PARTITION_ID),
                &consumer,
                &PollingStrategy::offset(offset),
                messages_per_batch,
                false,
            )
            .await?;

        if polled_messages.messages.is_empty() {
            empty_counter += 1;
            info!("No messages found #{}", empty_counter);
            sleep(interval).await;
            if empty_counter >= STOP_AFTER_X_EMPTY_BATCHES {
                break;
            }
            continue;
        }

        empty_counter = 0;
        offset += polled_messages.messages.len() as u64;
        strategy.set_value(offset);
        for message in polled_messages.messages {
            handle_message(&message)?;
        }
        sleep(interval).await;
    }
    Ok(())
}

fn handle_message(message: &IggyMessage) -> Result<(), Box<dyn Error>> {
    // The payload can be of any type as it is a raw byte array. In this case it's a simple string.
    let payload = std::str::from_utf8(&message.payload)?;
    info!(
        "Handling message at offset: {}, payload: {}...",
        message.header.offset, payload
    );
    Ok(())
}
