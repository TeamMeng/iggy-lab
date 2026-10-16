use iggy::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let client = IggyClient::from_connection_string("iggy://iggy:iggy@127.0.0.1:8090")?;

    client.connect().await?;

    println!("Connect successfully...");

    Ok(())
}
