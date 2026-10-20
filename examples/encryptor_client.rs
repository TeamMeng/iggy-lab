use std::sync::Arc;

use anyhow::Result;
use iggy::prelude::*;

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

    let messages = client
        .poll_messages(
            &Identifier::from_str_value("medical-stream")?,
            &Identifier::from_str_value("patient-records")?,
            // 第 3 个参数是 partition_id，不是读取条数；分区从 0 开始。
            Some(0),
            &Consumer::default(),
            &PollingStrategy::next(),
            10,
            true,
        )
        .await?;

    for msg in messages.messages {
        let content = String::from_utf8_lossy(&msg.payload);
        println!("{}", content);
    }

    Ok(())
}
