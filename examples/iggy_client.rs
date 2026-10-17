use iggy::prelude::*;
use std::sync::Arc;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let config = Arc::new(ClientProviderConfig {
        transport: TransportProtocol::Tcp,
        tcp: Some(Arc::new(TcpClientConfig {
            server_address: "127.0.0.1:8090".to_string(),
            auto_login: AutoLogin::Enabled(Credentials::UsernamePassword(
                "iggy".to_string(),
                "iggy".into(),
            )),
            ..Default::default()
        })),
        ..Default::default()
    });

    let client = client_provider::get_client(config).await?;

    client.connect().await?;

    println!("Connect successfully...");

    Ok(())
}
