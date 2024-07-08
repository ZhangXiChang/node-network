use std::sync::Arc;

use eyre::Result;
use netprotocol::node::{Node, PeerNode};
use tool_code::lock::PointerPreNew;

const SERVER_CERT_DER: &[u8] = include_bytes!("../assets/server.cer");

pub struct App {
    node: Node,
    server: PointerPreNew<PeerNode>,
}
impl App {
    pub fn new() -> Result<Self> {
        Ok(Self {
            node: Node::new("[::]:0".parse()?, "节点", "节点描述", None)?,
            server: PointerPreNew::new(),
        })
    }
    pub async fn connect_server(&self) -> Result<()> {
        self.server.set(
            self.node
                .connect("[::1]:10270".parse()?, Arc::new(SERVER_CERT_DER.to_vec()))
                .await?,
        );
        Ok(())
    }
}
