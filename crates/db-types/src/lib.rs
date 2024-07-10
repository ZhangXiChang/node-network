use serde::{Deserialize, Serialize};
use sqlx::prelude::FromRow;

#[derive(FromRow, Serialize, Deserialize)]
pub struct HubNodeTable {
    pub logo: Vec<u8>,
}
