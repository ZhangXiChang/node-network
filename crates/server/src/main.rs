use anyhow::Result;
use quinn::{ConnectionError, Endpoint};
use utils::{ext::quinn::EndpointExtension, logger::Logger};

#[tokio::main]
async fn main() -> Result<()> {
    Logger::new().start()?;
    let endpoint = Endpoint::new_ext(
        "0.0.0.0:10270".parse()?,
        include_bytes!("../../../target/server.cer").to_vec(),
        include_bytes!("../../../target/server.key").to_vec(),
    )?;
    while let Some(incoming) = endpoint.accept().await {
        tokio::spawn(async move {
            async move {
                let connection = incoming.accept()?.await?;
                let (mut send, mut recv) = connection.accept_bi().await?;
                let login_name = String::from_utf8(recv.read_to_end(usize::MAX).await?)?;
                println!("{}", login_name);
                send.write_all("嫦娥迹象".as_bytes()).await?;
                send.finish()?;
                match connection.closed().await {
                    ConnectionError::ApplicationClosed(close_info) => {
                        println!("{}", String::from_utf8(close_info.reason.to_vec())?)
                    }
                    _ => println!("连接异常关闭"),
                }
                anyhow::Ok(())
            }
            .await
            .unwrap()
        });
    }
    Ok(())
}
