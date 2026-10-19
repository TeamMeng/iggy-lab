use iggy::{clients::client_builder::IggyClientBuilder, prelude::Client};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = IggyClientBuilder::new()
        .with_tcp()
        .with_server_address("127.0.0.1:8090".to_string())
        .build()?;

    client.connect().await?;
    println!("connect");

    Ok(())
}
