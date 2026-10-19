use iggy::{
    client_wrappers::client_wrapper::ClientWrapper,
    clients::client::IggyClient,
    prelude::{AutoLogin, Client, Credentials},
    quic::quic_client::QuicClient,
};

#[tokio::main]
async fn main() -> anyhow::Result<()> {
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
    println!("connect");

    Ok(())
}
