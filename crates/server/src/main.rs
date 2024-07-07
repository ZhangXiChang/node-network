use std::{sync::Arc, time::Duration};

use eyre::Result;
use netprotocol::{Packet, Verify};
use quinn::{Endpoint, ServerConfig, TransportConfig};
use rustls::pki_types::PrivateKeyDer;
use sqlx::{prelude::FromRow, SqlitePool};
use tool_code::lock::ArcMutexVec;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const CERT_DER: &[u8] = include_bytes!("../../../assets/server.cer");
const KEY_DER: &[u8] = include_bytes!("../../../assets/server.key");

#[derive(FromRow)]
struct HubNodeInfo {
    id: u32,
    name: String,
    ipv4_address: String,
    ipv6_address: String,
    cert_der: Vec<u8>,
    description: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    //初始化日志系统
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::filter::Targets::new()
                .with_targets(vec![("server", tracing::Level::INFO)]),
        )
        .init();
    //初始化服务端
    let mut server_config = ServerConfig::with_single_cert(
        vec![CERT_DER.to_vec().into()],
        PrivateKeyDer::Pkcs8(KEY_DER.to_vec().into()),
    )?;
    server_config.transport_config(Arc::new({
        let mut a = TransportConfig::default();
        a.keep_alive_interval(Some(Duration::from_secs(5)));
        a
    }));
    let endpoint = Endpoint::server(server_config, "0.0.0.0:10270".parse()?)?;
    tracing::info!("服务端初始化完成");
    //初始化数据库连接池
    let db_conn_pool = SqlitePool::connect("./assets/server.db").await?;
    tracing::info!("数据库连接池初始化完成");
    //主循环
    let connection_list = ArcMutexVec::new();
    loop {
        if let Some(incoming) = endpoint.accept().await {
            tokio::spawn({
                let connection_list = connection_list.clone();
                let _db_conn_pool = db_conn_pool.clone();
                async move {
                    tracing::info!("[{}]接入连接", incoming.remote_address());
                    if let Ok(connection) = incoming.await {
                        tracing::info!("[{}]连接成功", connection.remote_address());
                        //验证
                        let (mut send, mut recv) = connection.open_bi().await?;
                        send.write_all(&rmp_serde::to_vec(&Packet::Verify(Verify::default()))?)
                            .await?;
                        send.finish()?;
                        if let Packet::Verify(verify) =
                            rmp_serde::from_slice::<Packet>(&recv.read_to_end(usize::MAX).await?)?
                        {
                            if verify.version_sequence >= 1 {
                                tracing::info!("[{}]验证成功", connection.remote_address());
                                connection_list.push(connection.clone());
                                loop {}
                                // let mut db_conn = db_conn_pool.acquire().await?;
                                // let hubnode_info_list =
                                //     sqlx::query_as::<_, HubNodeInfo>("SELECT * FROM HubNodeList")
                                //         .fetch_all(&mut *db_conn)
                                //         .await?;
                                // for hubnode_info in hubnode_info_list {
                                //     tracing::info!("{}", hubnode_info.name);
                                // }
                            }
                        }
                    }
                    tracing::info!("1111111111111111111111111111");
                    eyre::Ok(())
                }
            });
        }
    }
}
