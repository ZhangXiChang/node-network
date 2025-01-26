use std::sync::Arc;

use anyhow::Result;
use parking_lot::Mutex;
use protocol::ServerCommand;
use quinn::{Connection, Endpoint};
use utils::ext::{logger_builder::LoggerBuilder, quinn::EndpointExtension, vecu8::borsh::Borsh};

struct Peernode {
    name: Option<String>,
    connection: Connection,
}

#[tokio::main]
async fn main() -> Result<()> {
    flexi_logger::Logger::builder(log::LevelFilter::Info)
        .log_file_dir("./log/")
        .start()?;
    let endpoint = Endpoint::new_ext(
        "0.0.0.0:10270".parse()?,
        include_bytes!("../../../target/server.cer").to_vec(),
        include_bytes!("../../../target/server.key").to_vec(),
    )?;
    let peernode_list = Arc::new(Mutex::new(Vec::new()));
    let server_name = Arc::new("嫦娥迹象".to_string());
    log::info!("开始运行...");
    while let Some(incoming) = endpoint.accept().await {
        tokio::spawn({
            let peernode_list = peernode_list.clone();
            let server_name = server_name.clone();
            async move {
                let incoming_socketaddr = incoming.remote_address();
                match async move { anyhow::Ok(incoming.accept()?.await?) }.await {
                    Ok(connection) => {
                        let peernode = Arc::new(Mutex::new(Peernode {
                            name: None,
                            connection: connection.clone(),
                        }));
                        peernode_list.lock().push(peernode.clone());
                        log::info!("[{}]连接", connection.remote_address());
                        log::info!("当前负载数[{}]", peernode_list.lock().len());
                        if let Err(result) = {
                            let connection = connection.clone();
                            let peernode = peernode.clone();
                            async move {
                                loop {
                                    let (mut send, mut recv) = connection.accept_bi().await?;
                                    match recv
                                        .read_to_end(usize::MAX)
                                        .await?
                                        .borsh_to::<ServerCommand>()?
                                    {
                                        ServerCommand::Login { login_name } => {
                                            log::info!(
                                                "[{}]登录，名称[{}]",
                                                connection.remote_address(),
                                                login_name
                                            );
                                            peernode.lock().name = Some(login_name);
                                            send.write_all(server_name.as_bytes()).await?;
                                            send.finish()?;
                                        }
                                    }
                                }
                                #[allow(unreachable_code)]
                                anyhow::Ok(())
                            }
                        }
                        .await
                        {
                            log::info!(
                                "[{}]断开连接:{}",
                                match &peernode.lock().name {
                                    Some(name) => name.clone(),
                                    None => connection.remote_address().to_string(),
                                },
                                result
                            );
                            peernode_list.lock().retain(|peernode| {
                                peernode.lock().connection.stable_id() != connection.stable_id()
                            });
                            log::info!("当前负载数[{}]", peernode_list.lock().len());
                        }
                    }
                    Err(err) => log::info!("[{}]连接失败:{}", incoming_socketaddr, err),
                }
            }
        });
    }
    Ok(())
}
