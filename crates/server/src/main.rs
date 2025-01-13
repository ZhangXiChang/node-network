use std::sync::Arc;

use anyhow::Result;
use parking_lot::Mutex;
use protocol::ServerCommand;
use quinn::{Connection, Endpoint};
use utils::ext::{logger_builder::LoggerBuilder, quinn::EndpointExtension, vecu8::borsh::Borsh};

struct Peernode {
    login_name: String,
    connection: Connection,
}

#[tokio::main]
async fn main() -> Result<()> {
    flexi_logger::Logger::builder(log::LevelFilter::Info).start()?;
    let server_name = Arc::new("嫦娥迹象".to_string());
    let endpoint = Endpoint::new_ext(
        "0.0.0.0:10270".parse()?,
        include_bytes!("../../../target/server.cer").to_vec(),
        include_bytes!("../../../target/server.key").to_vec(),
    )?;
    let onlinelist = Arc::new(Mutex::new(Vec::new() as Vec<Peernode>));
    while let Some(incoming) = endpoint.accept().await {
        tokio::spawn({
            let onlinelist = onlinelist.clone();
            let server_name = server_name.clone();
            async move {
                if let Err(err) = async move {
                    let connection = incoming.accept()?.await?;
                    loop {
                        let (mut send, mut recv) = connection.accept_bi().await?;
                        match recv
                            .read_to_end(usize::MAX)
                            .await?
                            .borsh_to::<ServerCommand>()?
                        {
                            ServerCommand::Login { login_name } => {
                                onlinelist.lock().push(Peernode {
                                    login_name,
                                    connection: connection.clone(),
                                });
                                send.write_all(server_name.as_bytes()).await?;
                                send.finish()?;
                            }
                        }
                    }
                    #[allow(unreachable_code)]
                    anyhow::Ok(())
                }
                .await
                {
                    log::info!("{}", err);
                }
            }
        });
    }
    Ok(())
}
