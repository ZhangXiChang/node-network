use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Packet {
    GetHubNodeInfoList,
}

#[derive(Serialize, Deserialize)]
pub struct HubNodeInfo {
    pub name: String,
    pub description: String,
    pub ipv4_addr: String,
    pub cert_der: Vec<u8>,
    pub logo: Vec<u8>,
}
