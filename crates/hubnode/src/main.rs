use anyhow::{anyhow, Result};
use hubnode::Packet;
use quinn::{Connection, ConnectionError, Endpoint};
use tool_code::{lock::Pointer, quinn::Extension, rmp_serde::MessagePack};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const CERT_DER: &[u8] = include_bytes!("../../../assets/hubnode.cer");
const KEY_DER: &[u8] = include_bytes!("../../../assets/hubnode.key");

#[derive(Clone)]
struct App {
    is_loop: Pointer<bool>,
    endpoint: Endpoint,
    conn_list: Pointer<Vec<Connection>>,
}
impl App {
    fn new() -> Result<Self> {
        Ok(Self {
            is_loop: Pointer::new(true),
            endpoint: Endpoint::new_ext(
                "0.0.0.0:10271".parse()?,
                CERT_DER.to_vec(),
                KEY_DER.to_vec(),
            )?,
            conn_list: Pointer::new(Vec::new()),
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
                            self_handle.conn_list.lock().push(connection.clone());
                            let result = self_handle.connection_handling(connection.clone()).await;
                            self_handle
                                .conn_list
                                .lock()
                                .retain(|conn| conn.stable_id() != connection.stable_id());
                            if let Err(err) = result {
                                panic!("{:?}", err);
                            }
                            anyhow::Ok(())
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
                Ok((_, mut recv)) => match Vec::decode(&recv.read_to_end(usize::MAX).await?)? {
                    Packet::Test => tracing::info!("测试"),
                },
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
    let app = App::new()?;
    tracing::info!("应用程序初始化完毕");
    app.run().await;
    Ok(())
}
