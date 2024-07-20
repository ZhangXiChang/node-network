use std::sync::Arc;

use anyhow::{anyhow, Context, Result};
use db_types::HubNodeInfo;
use netprotocol::{
    node::{Node, PeerNode},
    packet::Packet,
};
use tool_code::lock::Pointer;

const SERVER_CERT_DER: &[u8] = include_bytes!("../assets/server/server.cer");

#[derive(Clone)]
pub struct System {
    node: Node,
    server: Pointer<Option<PeerNode>>,
}
impl System {
    pub fn new() -> Result<Self> {
        Ok(Self {
            node: Node::new(
                "[::]:0".parse().context("解析IP地址失败")?,
                "节点",
                "节点描述",
                None,
            )
            .context("创建节点失败")?,
            server: Pointer::new(None),
        })
    }
    pub async fn connect_server(&self) -> Result<()> {
        *self.server.lock() = Some(
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
    pub async fn get_hubnode_table(&self) -> Result<Vec<HubNodeInfo>> {
        let (mut send, mut recv) = {
            let a = self.server.lock().clone();
            a
        }
        .ok_or(anyhow!("找不到服务端句柄"))?
        .open_bi()
        .await
        .context("打开全双工通道失败")?;
        send.write_all(&rmp_serde::to_vec(&Packet::GetHubNodeTable).context("编码数据包失败")?)
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
