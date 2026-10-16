use anyhow::Result;
use iggy::prelude::*;
use std::sync::Arc;
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let config = Arc::new(WebSocketClientConfig {
        server_address: "127.0.0.1:8092".to_string(),
        auto_login: AutoLogin::Enabled(Credentials::UsernamePassword(
            "iggy".to_string(),
            "iggy".into(),
        )),
        ..WebSocketClientConfig::default()
    });

    let websocket_client = WebSocketClient::create(config)?;

    let client = IggyClient::create(ClientWrapper::WebSocket(websocket_client), None, None);

    client.connect().await?;
    println!("Connect successfully...");

    Ok(())
}
