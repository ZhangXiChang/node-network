use std::net::SocketAddr;

use anyhow::Result;
use quinn::{Connection, Endpoint};
use server::{HubNodeInfo, Packet};
use tool_code::{lock::Pointer, quinn::Extension, rmp_serde::MessagePack};
use uuid::Uuid;

#[derive(Clone)]
pub struct Node {
    endpoint: Endpoint,
    server_conn: Connection,
    hubnode_conn: Pointer<Option<Connection>>,
}
impl Node {
    pub async fn new(server_ip_addr: SocketAddr, server_cert_der: Vec<u8>) -> Result<Self> {
        let rcgen::CertifiedKey { cert, key_pair } =
            rcgen::generate_simple_self_signed(vec![Uuid::new_v4().to_string()])?;
        let endpoint = Endpoint::new_ext(
            "0.0.0.0:0".parse()?,
            cert.der().to_vec(),
            key_pair.serialize_der(),
        )?;
        Ok(Self {
            endpoint: endpoint.clone(),
            server_conn: endpoint
                .connect_ext(server_ip_addr, server_cert_der)
                .await?
                .await?,
            hubnode_conn: Pointer::new(None),
        })
    }
    pub async fn get_hubnode_info_list(&self) -> Result<Vec<HubNodeInfo>> {
        let (mut send, mut recv) = self.server_conn.open_bi().await?;
        send.write_all(&Vec::encode(&Packet::GetHubNodeInfoList)?)
            .await?;
        send.finish()?;
        Ok(recv.read_to_end(usize::MAX).await?.decode()?)
    }
}
