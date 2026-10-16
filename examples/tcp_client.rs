use anyhow::Result;
use iggy::prelude::*;
use std::str::FromStr;

#[tokio::main]
async fn main() -> Result<()> {
    let tcp_client = TcpClient::new(
        "127.0.0.1:8090",
        AutoLogin::Enabled(Credentials::UsernamePassword(
            "iggy".to_string(),
            "iggy".into(),
        )),
        NonZeroIggyDuration::from_str("5s")?,
    )?;

    let client = IggyClient::create(ClientWrapper::Tcp(tcp_client), None, None);

    client.connect().await?;
    println!("Connect successfully...");

    Ok(())
}
