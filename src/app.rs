use std::sync::Arc;

use anyhow::{Context, Result};
use db_types::HubNodeTable;
use netprotocol::{
    node::{Node, PeerNode},
    packet::Packet,
};
use tool_code::lock::PointerPreNew;

const SERVER_CERT_DER: &[u8] = include_bytes!("../assets/server.cer");

pub struct App {
    node: Node,
    server: PointerPreNew<PeerNode>,
}
impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            node: Node::new(
                "[::]:0".parse().context("解析IP地址失败")?,
                "节点",
                "节点描述",
                None,
            )
            .context("创建节点失败")?,
            server: PointerPreNew::new(),
        })
    }
    pub async fn connect_server(&self) -> Result<()> {
        self.server.set(
            self.node
                .connect(
                    "[::1]:10270".parse().context("解析IP地址失败")?,
                    Arc::new(SERVER_CERT_DER.to_vec()),
                )
                .await
                .context("连接节点服务端失败")?,
        );
        Ok(())
    }
    pub async fn get_user_star_hubnode_logo(&self) -> Result<Vec<HubNodeTable>> {
        let (mut send, mut recv) = self
            .server
            .get()
            .context("获取节点服务端句柄失败")?
            .open_bi()
            .await
            .context("打开全双工通道失败")?;
        send.write_all(
            &rmp_serde::to_vec(&Packet::GetUserStarHubnodeLogo).context("编码数据包失败")?,
        )
        .await
        .context("写入数据包失败")?;
        send.finish().context("发送数据包失败")?;
        Ok(rmp_serde::from_slice(
            &recv
                .read_to_end(usize::MAX)
                .await
                .context("读取数据包失败")?,
        )
        .context("解码数据包失败")?)
    }
}
