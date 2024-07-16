use std::sync::Arc;

use anyhow::{Context, Result};
use db_types::HubNodeTable;
use netprotocol::{
    node::{Node, PeerNode, RecvStream, SendStream},
    packet::Packet,
    tls::CertKey,
};
use sqlx::SqlitePool;
use tokio::task::JoinHandle;
use tool_code::lock::Container;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const CERT_DER: &[u8] = include_bytes!("../../../assets/server/server.cer");
const KEY_DER: &[u8] = include_bytes!("../../../assets/server/server.key");

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
        "[::]:10270".parse().context("解析IP地址失败")?,
        "节点服务端",
        "用于提供公共节点服务",
        Some(CertKey {
            cert_der: Arc::new(CERT_DER.to_vec()),
            key_der: Arc::new(KEY_DER.to_vec()),
        }),
    )
    .context("创建节点服务端")?;
    tracing::info!("节点服务端初始化完成");
    //初始化数据库连接池
    let db_conn_pool = SqlitePool::connect("./assets/server/server.db")
        .await
        .context("创建数据库连接池失败")?;
    tracing::info!("数据库连接池初始化完成");
    //连接列表
    let conn_list = Container::new();
    //主循环
    loop {
        let peer_node_future = node.accept().await;
        tokio::spawn(incoming_handling(
            peer_node_future,
            conn_list.clone(),
            db_conn_pool.clone(),
        ));
    }
}

async fn incoming_handling(
    peer_node_future: JoinHandle<Result<PeerNode>>,
    conn_list: Container<PeerNode>,
    db_conn_pool: SqlitePool,
) {
    if let Err(err) = async move {
        let peer_node = peer_node_future
            .await
            .context("接收节点连接线程出错")?
            .context("接收节点连接失败")?;
        conn_list.add(peer_node.clone());
        tracing::info!("[{}]连接成功", peer_node.remote_ip_address());
        loop {
            match peer_node.accept_bi().await {
                Ok((send, recv)) => {
                    tokio::spawn(handling_packet(
                        send,
                        recv,
                        peer_node.clone(),
                        db_conn_pool.clone(),
                    ));
                }
                Err(_) => break,
            }
        }
        anyhow::Ok(())
    }
    .await
    {
        panic!("{:?}", err);
    }
}

async fn handling_packet(
    mut send: SendStream,
    mut recv: RecvStream,
    peer_node: PeerNode,
    db_conn_pool: SqlitePool,
) {
    if let Err(err) = async move {
        match rmp_serde::from_slice::<Packet>(
            &recv
                .read_to_end(usize::MAX)
                .await
                .context("读取数据包失败")?,
        )
        .context("解析数据包失败")?
        {
            Packet::GetHubNodeTable => {
                tracing::info!("[{}]获取中枢节点表", peer_node.remote_ip_address());
                let mut db_conn = db_conn_pool.acquire().await.context("获取数据库连接失败")?;
                let hubnode_table = sqlx::query_as::<_, HubNodeTable>("SELECT * FROM HubNode")
                    .fetch_all(&mut *db_conn)
                    .await
                    .context("从数据库查询所有中枢节点Logo失败")?;
                send.write_all(&rmp_serde::to_vec(&hubnode_table).context("编码数据包失败")?)
                    .await
                    .context("写入数据包失败")?;
                send.finish().context("发送数据包失败")?;
            }
        }
        anyhow::Ok(())
    }
    .await
    {
        panic!("{:?}", err);
    }
}
