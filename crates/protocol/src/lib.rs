use borsh::{BorshDeserialize, BorshSerialize};

#[derive(BorshSerialize, BorshDeserialize)]
pub enum ServerAction {
    PeernodeLogin { login_name: String },
    BroadcastMessage { message: String },
}

#[derive(BorshSerialize, BorshDeserialize)]
pub enum PeernodeAction {
    AcceptServerName { server_name: String },
    AcceptMessage { message: String },
}
