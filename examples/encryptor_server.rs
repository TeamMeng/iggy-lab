use anyhow::Result;
use iggy::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> Result<()> {
    let encryption_key: Vec<u8> = vec![
        0x01, 0x02, 0x03, 0x04, 0x05, 0x06, 0x07, 0x08, 0x09, 0x0A, 0x0B, 0x0C, 0x0D, 0x0E, 0x0F,
        0x10, 0x11, 0x12, 0x13, 0x14, 0x15, 0x16, 0x17, 0x18, 0x19, 0x1A, 0x1B, 0x1C, 0x1D, 0x1E,
        0x1F, 0x20,
    ];

    let encryptor = Aes256GcmEncryptor::new(&encryption_key)?;

    let tcp_client = TcpClient::from_connection_string("iggy://iggy:iggy@localhost:8090")?;

    let client = IggyClient::create(
        ClientWrapper::Tcp(tcp_client),
        None,
        Some(Arc::new(EncryptorKind::Aes256Gcm(encryptor))),
    );

    client.connect().await?;

    let stream_name = "medical-stream";
    let topic_name = "patient-records";

    match client.create_stream(stream_name).await {
        Ok(_) => println!("Stream '{}' created", stream_name),
        Err(IggyError::StreamNameAlreadyExists(_)) => println!("Stream '{}' exists", stream_name),
        Err(e) => return Err(e.into()),
    }

    match client
        .create_topic(
            &Identifier::from_str_value(stream_name)?,
            topic_name,
            &TopicCreateOptions {
                partitions_count: Some(1),
                ..TopicCreateOptions::default()
            },
        )
        .await
    {
        Ok(_) => println!("Topic '{}' created", topic_name),
        Err(IggyError::TopicNameAlreadyExists(_, _)) => println!("Topic '{}' exists", topic_name),
        Err(e) => return Err(e.into()),
    }

    let msg = IggyMessage::builder()
        .payload("This is a message".into())
        .build()?;

    client
        .send_messages(
            &Identifier::from_str_value(stream_name)?,
            &Identifier::from_str_value(topic_name)?,
            &Partitioning::balanced(),
            &mut [msg],
        )
        .await?;

    println!("Send successful");

    Ok(())
}
