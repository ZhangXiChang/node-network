use anyhow::Result;
use quinn::{Connection, ConnectionError, Endpoint};
use tool_code::{
    ext::{quinn::Extension, rmp_serde::MessagePack},
    lock::Pointer,
    packet::{NodeInfo, Packet},
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const CERT_DER: &[u8] = include_bytes!("../../../assets/hubnode.cer");
const KEY_DER: &[u8] = include_bytes!("../../../assets/hubnode.key");

struct PeerNode {
    connection: Connection,
    info: NodeInfo,
}

#[derive(Clone)]
struct App {
    is_loop: Pointer<bool>,
    endpoint: Endpoint,
    node_list: Pointer<Vec<PeerNode>>,
}
impl App {
    async fn new() -> Result<Self> {
        Ok(Self {
            is_loop: Pointer::new(true),
            endpoint: Endpoint::new_ext(
                "0.0.0.0:10270".parse()?,
                CERT_DER.to_vec(),
                KEY_DER.to_vec(),
            )?,
            node_list: Pointer::new(Vec::new()),
        })
    }
    async fn run(&self) {
        while *self.is_loop.lock() {
            if let Some(incoming) = self.endpoint.accept().await {
                tokio::spawn({
                    let self_handle = self.clone();
                    async move {
                        if let Err(err) = async move {
                            let connection = incoming.accept()?.await?;
                            if let Err(err) = {
                                let connection = connection.clone();
                                async move {
                                    let node_info = connection
                                        .accept_uni()
                                        .await?
                                        .read_to_end(usize::MAX)
                                        .await?
                                        .message_pack_to::<NodeInfo>()?;
                                    self_handle.node_list.lock().push(PeerNode {
                                        connection: connection.clone(),
                                        info: node_info,
                                    });
                                    self_handle.connection_handling(connection.clone()).await?;
                                    self_handle.node_list.lock().retain(|node| {
                                        node.connection.stable_id() != connection.stable_id()
                                    });
                                    anyhow::Ok(())
                                }
                            }
                            .await
                            {
                                tracing::error!(
                                    "[{}]连接处理错误，原因：{:?}",
                                    connection.remote_address(),
                                    err
                                );
                            }
                            anyhow::Ok(())
                        }
                        .await
                        {
                            tracing::error!("连接传入失败，原因：{:?}", err);
                        }
                    }
                });
            }
        }
    }
    async fn connection_handling(&self, connection: Connection) -> Result<()> {
        loop {
            match connection.accept_bi().await {
                Ok((mut send, mut recv)) => {
                    match recv
                        .read_to_end(usize::MAX)
                        .await?
                        .message_pack_to::<Packet>()?
                    {
                        Packet::GetNodeInfoList => {
                            let node_info_list = self
                                .node_list
                                .lock()
                                .iter()
                                .map(|node| node.info.clone())
                                .collect::<Vec<_>>();
                            send.write_all(&Vec::message_pack_from(&node_info_list)?)
                                .await?;
                            send.finish()?;
                        }
                    }
                }
                Err(err) => {
                    match err {
                        ConnectionError::ApplicationClosed(close_info) => tracing::info!(
                            "[{}]连接关闭，原因：{:?}",
                            connection.remote_address(),
                            close_info
                        ),
                        _ => tracing::warn!(
                            "[{}]连接意外断开，原因：{:?}",
                            connection.remote_address(),
                            err
                        ),
                    }
                    break;
                }
            }
        }
        Ok(())
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::filter::Targets::new()
                .with_targets(vec![("hubnode", tracing::Level::INFO)]),
        )
        .init();
    tracing::info!("日志系统初始化完成");
    let app = App::new().await?;
    tracing::info!("应用程序初始化完毕");
    app.run().await;
    Ok(())
}
