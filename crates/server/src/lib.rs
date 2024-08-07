use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum Packet {
    GetHubNodeInfoList,
}

#[derive(Serialize, Deserialize)]
pub struct HubNodeInfo {
    pub name: String,
    pub ipv4_address: String,
    pub ipv6_address: String,
    pub cert_der: Vec<u8>,
    pub description: String,
    pub logo: Vec<u8>,
}
