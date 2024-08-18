use std::net::SocketAddr;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Packet {
    GetNodeInfoList,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct NodeInfo {
    pub name: String,
    pub description: String,
    pub cert_der: Vec<u8>,
}

#[derive(Serialize, Deserialize)]
pub struct NodeConnectInfo {
    pub info: NodeInfo,
    pub ip_addr: SocketAddr,
}
