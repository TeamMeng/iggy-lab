use anyhow::Result;
use iggy::prelude::*;

#[tokio::main]
async fn main() -> Result<()> {
    let client = IggyClient::from_connection_string("iggy://iggy:iggy@localhost:8090")?;
    client.connect().await?;

    client.create_stream("order-stream").await?;
    println!("order stream create successful");

    let stream_id = Identifier::from_str_value("order-stream")?;
    client
        .create_topic(
            &stream_id,
            "order-stream",
            &TopicCreateOptions {
                partitions_count: Some(4),
                compression_algorithm: Some(Default::default()),
                message_expiry: Some(IggyExpiry::NeverExpire),
                max_topic_size: Some(MaxTopicSize::ServerDefault),
                ..Default::default()
            },
        )
        .await?;
    println!("Topic create successful");

    let stats = client.get_stats().await?;
    println!("Server message has {}", stats.messages_count);

    Ok(())
}
