use std::{net::SocketAddr, sync::Arc};

use anyhow::{anyhow, Result};
use parking_lot::Mutex;
use protocol::ServerCommand;
use quinn::{Connection, Endpoint};
use utils::ext::{quinn::EndpointExtension, vecu8::borsh::Borsh};
use uuid::Uuid;

#[derive(Clone)]
struct Server {
    name: Arc<Mutex<Option<String>>>,
    connection: Connection,
}

pub struct App {
    endpoint: Endpoint,
    server: Arc<Mutex<Option<Server>>>,
    node_name: Arc<Mutex<Option<String>>>,
}
impl App {
    pub fn new() -> Result<Self> {
        log::info!("开始运行");
        let cert_key = rcgen::generate_simple_self_signed(vec![Uuid::new_v4().to_string()])?;
        let endpoint = Endpoint::new_ext(
            "0.0.0.0:10271".parse()?,
            cert_key.cert.der().to_vec(),
            cert_key.key_pair.serialize_der(),
        )?;
        Ok(Self {
            endpoint,
            server: Default::default(),
            node_name: Default::default(),
        })
    }
    pub async fn connect_server(&self, socketaddr: SocketAddr) -> Result<()> {
        *self.server.lock() = Some(Server {
            name: Default::default(),
            connection: self
                .endpoint
                .connect_ext(
                    socketaddr,
                    include_bytes!("../../target/server.cer").to_vec(),
                )?
                .await?,
        });
        Ok(())
    }
    pub async fn login(&self, login_name: String) -> Result<()> {
        *self.node_name.lock() = Some(login_name.clone());
        let server = self.server.lock().clone().ok_or(anyhow!("未连接服务器"))?;
        let (mut send, mut recv) = server.connection.open_bi().await?;
        send.write_all(&Vec::borsh_from(&ServerCommand::Login { login_name })?)
            .await?;
        send.finish()?;
        *server.name.lock() = Some(String::from_utf8(recv.read_to_end(usize::MAX).await?)?);
        Ok(())
    }
    pub async fn get_node_name(&self) -> Result<String> {
        Ok(self
            .node_name
            .lock()
            .clone()
            .ok_or(anyhow!("未登录服务器"))?)
    }
}
