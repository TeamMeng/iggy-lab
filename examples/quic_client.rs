use anyhow::Result;
use iggy::{prelude::*, quic::quic_client::QuicClient};
use tracing_subscriber::{EnvFilter, fmt, prelude::*};

#[tokio::main]
async fn main() -> Result<()> {
    // SDK 的重连/失败日志走 tracing；不初始化 subscriber 时连接失败是静默的，
    // 而默认 reconnection 会无限重试，表现为程序挂住且无任何输出。
    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::try_from_default_env().unwrap_or_else(|_| EnvFilter::new("info")))
        .init();

    let quic_client = QuicClient::new(
        "0.0.0.0:0",
        "127.0.0.1:8080",
        "localhost",
        false,
        AutoLogin::Enabled(Credentials::UsernamePassword(
            "iggy".to_string(),
            "iggy".into(),
        )),
    )?;

    let client = IggyClient::create(ClientWrapper::Quic(quic_client), None, None);

    client.connect().await?;
    println!("Connect successfully...");

    Ok(())
}
