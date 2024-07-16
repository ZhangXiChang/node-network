use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct HubNodeTable {
    pub id: i32,
    pub name: String,
    pub ipv4_address: String,
    pub ipv6_address: String,
    pub cert_der: Vec<u8>,
    pub description: String,
    pub logo: Vec<u8>,
}
