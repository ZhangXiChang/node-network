use anyhow::{anyhow, Result};
use quinn::{Connection, ConnectionError, Endpoint};
use server::{HubNodeInfo, Packet};
use sqlx::{Row, SqlitePool};
use tool_code::{lock::Pointer, quinn::Extension, rmp_serde::MessagePack};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const CERT_DER: &[u8] = include_bytes!("../../../assets/server.cer");
const KEY_DER: &[u8] = include_bytes!("../../../assets/server.key");

#[derive(Clone)]
struct App {
    is_loop: Pointer<bool>,
    endpoint: Endpoint,
    conn_list: Pointer<Vec<Connection>>,
    db_conn_pool: SqlitePool,
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
            conn_list: Pointer::new(Vec::new()),
            db_conn_pool: SqlitePool::connect("./assets/server.db").await?,
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
                Ok((mut send, mut recv)) => match recv.read_to_end(usize::MAX).await?.decode()? {
                    Packet::GetHubNodeInfoList => {
                        let mut db_conn = self.db_conn_pool.acquire().await?;
                        let mut hubnode_info_list = Vec::new();
                        for row in sqlx::query("SELECT * FROM HubNodeInfo")
                            .fetch_all(&mut *db_conn)
                            .await?
                        {
                            hubnode_info_list.push(HubNodeInfo {
                                name: row.get("name"),
                                description: row.get("description"),
                                ipv4_addr: row.get::<String, _>("ipv4_addr").parse()?,
                                cert_der: row.get("cert_der"),
                                logo: row.get("logo"),
                            });
                        }
                        send.write_all(&Vec::encode(&hubnode_info_list)?).await?;
                        send.finish()?;
                    }
                },
                Err(err) => match err {
                    ConnectionError::ApplicationClosed(closed_info) => {
                        tracing::info!(
                            "[{}]连接关闭: {}",
                            connection.remote_address(),
                            closed_info
                        );
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
                .with_targets(vec![("server", tracing::Level::INFO)]),
        )
        .init();
    tracing::info!("日志系统初始化完成");
    let app = App::new().await?;
    tracing::info!("应用程序初始化完毕");
    app.run().await;
    Ok(())
}
