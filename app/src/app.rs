use std::{net::SocketAddr, sync::Arc};

use anyhow::{anyhow, Result};
use parking_lot::Mutex;
use protocol::{PeernodeAction, ServerAction};
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
        log::info!("开始运行...");
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
    fn get_server(&self) -> Result<Server> {
        self.server.lock().clone().ok_or(anyhow!("未连接服务端"))
    }
    pub async fn connect_server(&self, socketaddr: SocketAddr) -> Result<()> {
        let server = Server {
            name: Default::default(),
            connection: self
                .endpoint
                .connect_ext(
                    socketaddr,
                    include_bytes!("../../target/server.cer").to_vec(),
                )?
                .await?,
        };
        *self.server.lock() = Some(server.clone());
        tokio::spawn({
            let server = server.clone();
            async move {
                if let Err(result) = {
                    let server = server.clone();
                    async move {
                        loop {
                            let (_send, mut recv) = server.connection.accept_bi().await?;
                            match recv
                                .read_to_end(usize::MAX)
                                .await?
                                .borsh_to::<PeernodeAction>()?
                            {
                                PeernodeAction::AcceptServerName { server_name } => {
                                    log::info!("服务端名称:[{}]", server_name);
                                    *server.name.lock() = Some(server_name)
                                }
                                PeernodeAction::AcceptMessage { message: _ } => {
                                    //TODO 接受消息
                                }
                            }
                        }
                        #[allow(unreachable_code)]
                        anyhow::Ok(())
                    }
                }
                .await
                {
                    log::info!(
                        "[{}]断开连接:{}",
                        match &*server.name.lock() {
                            Some(name) => name.clone(),
                            None => server.connection.remote_address().to_string(),
                        },
                        result
                    );
                }
            }
        });
        Ok(())
    }
    pub async fn login(&self, login_name: String) -> Result<()> {
        *self.node_name.lock() = Some(login_name.clone());
        let server = self.get_server()?;
        let (mut send, _recv) = server.connection.open_bi().await?;
        send.write_all(&Vec::borsh_from(&ServerAction::PeernodeLogin {
            login_name,
        })?)
        .await?;
        send.finish()?;
        Ok(())
    }
    pub async fn get_node_name(&self) -> Result<String> {
        Ok(self
            .node_name
            .lock()
            .clone()
            .ok_or(anyhow!("未登录服务端"))?)
    }
    pub async fn send_message(&self, message: String) -> Result<()> {
        let server = self.get_server()?;
        let (mut send, _recv) = server.connection.open_bi().await?;
        send.write_all(&Vec::borsh_from(&ServerAction::BroadcastMessage {
            message,
        })?)
        .await?;
        send.finish()?;
        Ok(())
    }
}
