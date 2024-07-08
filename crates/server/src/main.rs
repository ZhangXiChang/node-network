use std::sync::Arc;

use eyre::Result;
use netprotocol::{node::Node, tls::CertKey};
use sqlx::{prelude::FromRow, SqlitePool};
use tool_code::lock::Container;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const CERT_DER: &[u8] = include_bytes!("../../../assets/server.cer");
const KEY_DER: &[u8] = include_bytes!("../../../assets/server.key");

#[derive(FromRow)]
struct HubNodeListRow {
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
    tracing::info!("日志系统初始化完成");
    //初始化服务端
    let node = Node::new(
        "[::]:10270".parse()?,
        "节点服务端",
        "用于提供公共节点服务",
        Some(CertKey {
            cert_der: Arc::new(CERT_DER.to_vec()),
            key_der: Arc::new(KEY_DER.to_vec()),
        }),
    )?;
    tracing::info!("节点服务端初始化完成");
    //初始化数据库连接池
    let db_conn_pool = SqlitePool::connect("./assets/server.db").await?;
    tracing::info!("数据库连接池初始化完成");
    //连接列表
    let conn_list = Container::new();
    //主循环
    loop {
        let peer_node_future = node.accept().await;
        tokio::spawn({
            let conn_list = conn_list.clone();
            let db_conn_pool = db_conn_pool.clone();
            async move {
                let peer_node = peer_node_future.await??;
                conn_list.add(peer_node.clone());
                tracing::info!("[{}]连接成功", peer_node.remote_ip_address());
                let mut db_conn = db_conn_pool.acquire().await?;
                let hubnode_list_rows =
                    sqlx::query_as::<_, HubNodeListRow>("SELECT * FROM HubNodeList")
                        .fetch_all(&mut *db_conn)
                        .await?;
                for hubnode_list_row in hubnode_list_rows {
                    tracing::info!("{}", hubnode_list_row.id);
                    tracing::info!("{}", hubnode_list_row.name);
                    tracing::info!("{}", hubnode_list_row.ipv4_address);
                    tracing::info!("{}", hubnode_list_row.ipv6_address);
                    let _ = hubnode_list_row.cert_der;
                    tracing::info!("{}", hubnode_list_row.description);
                }
                eyre::Ok(())
            }
        });
    }
}
