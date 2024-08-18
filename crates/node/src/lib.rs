use std::net::SocketAddr;

use anyhow::Result;
use quinn::{Connection, Endpoint};
use tool_code::{
    packet::{NodeConnectInfo, NodeInfo, Packet},
    quinn::Extension,
    rmp_serde::MessagePack,
};
use uuid::Uuid;

#[derive(Clone)]
pub struct Node {
    endpoint: Endpoint,
    hubnode_conn: Connection,
}
impl Node {
    pub async fn new(hubnode_ip_addr: SocketAddr, hubnode_cert_der: Vec<u8>) -> Result<Self> {
        let cert_key = rcgen::generate_simple_self_signed(vec![Uuid::new_v4().to_string()])?;
        let cert_der = cert_key.key_pair.serialize_der();
        let endpoint = Endpoint::new_ext(
            "0.0.0.0:0".parse()?,
            cert_key.cert.der().to_vec(),
            cert_der.clone(),
        )?;
        let hubnode_conn = endpoint
            .connect_ext(hubnode_ip_addr, hubnode_cert_der)
            .await?
            .await?;
        let mut send = hubnode_conn.open_uni().await?;
        send.write_all(&Vec::encode(&NodeInfo {
            name: "北方酱".to_string(),
            description: "测试节点描述".to_string(),
            cert_der,
        })?)
        .await?;
        send.finish()?;
        Ok(Self {
            endpoint,
            hubnode_conn,
        })
    }
    pub async fn get_node_info_list(&self) -> Result<Vec<NodeConnectInfo>> {
        let (mut send, mut recv) = self.hubnode_conn.open_bi().await?;
        send.write_all(&Vec::encode(&Packet::GetNodeInfoList)?)
            .await?;
        send.finish()?;
        Ok(recv.read_to_end(usize::MAX).await?.decode()?)
    }
}
