use anyhow::Result;
use quinn::Endpoint;
use utils::{ext::quinn::EndpointExtension, logger::Logger};

#[tokio::main]
async fn main() -> Result<()> {
    Logger::new().start()?;
    let endpoint = Endpoint::new_ext("0.0.0.0:10270".parse()?, todo!(), todo!())?;
    while let Some(incoming) = endpoint.accept().await {
        tokio::spawn(async move {
            async move {
                let connection = incoming.accept()?.await?;
                println!("{}", connection.remote_address());
                anyhow::Ok(())
            }
            .await
            .unwrap()
        });
    }
    Ok(())
}
