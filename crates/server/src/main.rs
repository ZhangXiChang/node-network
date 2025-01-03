use anyhow::Result;
use quinn::Endpoint;
use utils::{ext::quinn::QuinnExtension, logger::Logger};

#[tokio::main]
async fn main() -> Result<()> {
    Logger::new().start()?;
    let endpoint = Endpoint::new_ext(
        "0.0.0.0:10270".parse()?,
        include_bytes!("../../../target/cert1.cer").to_vec(),
        include_bytes!("../../../target/cert1.key").to_vec(),
    )?;
    while let Some(incoming) = endpoint.accept().await {
        tokio::spawn(async move {
            async move {
                let connection = incoming.accept()?.await?;
                anyhow::Ok(())
            }
            .await
            .unwrap()
        });
    }
    Ok(())
}
