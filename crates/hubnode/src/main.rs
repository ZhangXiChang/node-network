use anyhow::{anyhow, Result};
use quinn::{Connection, ConnectionError, Endpoint};
use tool_code::{
    lock::Pointer,
    packet::{NodeConnectInfo, NodeInfo, Packet},
    quinn::Extension,
    rmp_serde::MessagePack,
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
                            let node_info = connection
                                .accept_uni()
                                .await?
                                .read_to_end(usize::MAX)
                                .await?
                                .decode::<NodeInfo>()?;
                            self_handle.node_list.lock().push(PeerNode {
                                connection: connection.clone(),
                                info: node_info,
                            });
                            let a = self_handle.connection_handling(connection.clone()).await;
                            self_handle.node_list.lock().retain(|node| {
                                node.connection.stable_id() != connection.stable_id()
                            });
                            a
                        }
                        .await
                        {
                            panic!("{:?}", err);
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
                    match Vec::decode(&recv.read_to_end(usize::MAX).await?)? {
                        Packet::GetNodeInfoList => {
                            let node_info_list = self
                                .node_list
                                .lock()
                                .iter()
                                .map(|node| NodeConnectInfo {
                                    info: node.info.clone(),
                                    ip_addr: node.connection.remote_address(),
                                })
                                .collect::<Vec<_>>();
                            send.write_all(&Vec::encode(&node_info_list)?).await?;
                            send.finish()?;
                        }
                    }
                }
                Err(err) => match err {
                    ConnectionError::ApplicationClosed(closed_info) => {
                        tracing::info!("对方连接关闭，信息: {}", closed_info);
                        break;
                    }
                    _ => return Err(anyhow!("{}", err)),
                },
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
