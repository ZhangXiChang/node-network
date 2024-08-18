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
