use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub enum ServerDataPacket {
    IdentityAuthentication { login_name: String },
}

#[derive(Serialize, Deserialize)]
pub enum NodeDataPacket {}
