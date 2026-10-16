use iggy::prelude::*;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let http_client = HttpClient::new("http://127.0.0.1:3000")?;

    let client = IggyClient::create(ClientWrapper::Http(http_client), None, None);

    // HTTP 传输没有 AutoLogin 参数（TcpClient/QuicClient 的构造函数才有），
    // 连接字符串里的用户名密码也会被丢弃（HttpClientConfig.jwt 恒为 None），
    // 所以必须显式登录。否则任何非公开接口（如 GET /stats）都会在 SDK 的
    // fail_if_not_authenticated 处直接返回 IggyError::Unauthenticated。
    client.connect().await?;
    client.login_user("iggy", "iggy").await?;
    println!("Login successfully...");

    let stats = client.get_stats().await?;
    println!("Server version: {:?}", stats);

    Ok(())
}
