use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize, Deserialize, Default)]
pub struct HubNodeInfo {
    pub name: String,
    pub ipv4_address: String,
    pub ipv6_address: String,
    pub cert_der: Vec<u8>,
    pub description: String,
    pub logo: Vec<u8>,
}
