use std::{net::SocketAddr, sync::Arc, time::Duration};

use anyhow::{anyhow, Result};
use quinn::{ClientConfig, Connection, Endpoint, ServerConfig, TransportConfig, VarInt};
use rustls::{pki_types::PrivateKeyDer, RootCertStore};
use serde::{Deserialize, Serialize};
use tokio::task::JoinHandle;
use tool_code::{lock::Pointer, x509::x509_dns_name_from_cert_der};
use uuid::Uuid;

pub use quinn::{RecvStream, SendStream};

use crate::tls::CertKey;

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct NodeInfo {
    pub uuid: Arc<String>,
    pub user_name: Arc<String>,
    pub description: Arc<String>,
    pub cert_der: Arc<Vec<u8>>,
}

#[derive(Clone)]
pub struct PeerNode {
    connection: Connection,
    info: NodeInfo,
}
impl PeerNode {
    fn new(connection: Connection, info: NodeInfo) -> Self {
        Self { connection, info }
    }
    pub fn info(&self) -> NodeInfo {
        self.info.clone()
    }
    pub async fn accept_bi(&self) -> Result<(SendStream, RecvStream)> {
        Ok(self.connection.accept_bi().await?)
    }
    pub async fn open_bi(&self) -> Result<(SendStream, RecvStream)> {
        Ok(self.connection.open_bi().await?)
    }
    pub async fn accept_uni(&self) -> Result<RecvStream> {
        Ok(self.connection.accept_uni().await?)
    }
    pub async fn open_uni(&self) -> Result<SendStream> {
        Ok(self.connection.open_uni().await?)
    }
    pub fn remote_ip_address(&self) -> SocketAddr {
        self.connection.remote_address()
    }
    pub fn close(&self, code: u32, reason: Arc<Vec<u8>>) {
        self.connection.close(VarInt::from_u32(code), &*reason);
    }
}

#[derive(Clone)]
pub struct Node {
    endpoint: Endpoint,
    info: NodeInfo,
    peer_hubnode: Pointer<Option<PeerNode>>,
}
impl Node {
    pub fn new(
        ip_address: SocketAddr,
        user_name: &str,
        description: &str,
        cert_key: Option<CertKey>,
    ) -> Result<Self> {
        let (mut server_config, cert_der) = if let Some(cert_key) = cert_key {
            (
                ServerConfig::with_single_cert(
                    vec![(*cert_key.cert_der).clone().into()],
                    PrivateKeyDer::Pkcs8((*cert_key.key_der).clone().into()),
                )?,
                cert_key.cert_der,
            )
        } else {
            let rcgen::CertifiedKey { cert, key_pair } =
                rcgen::generate_simple_self_signed(vec![Uuid::new_v4().to_string()])?;
            (
                ServerConfig::with_single_cert(
                    vec![cert.der().to_vec().into()],
                    PrivateKeyDer::Pkcs8(key_pair.serialize_der().into()),
                )?,
                Arc::new(cert.der().to_vec()),
            )
        };
        server_config.transport_config(Arc::new({
            let mut a = TransportConfig::default();
            a.keep_alive_interval(Some(Duration::from_secs(5)));
            a
        }));
        Ok(Self {
            endpoint: Endpoint::server(server_config, ip_address)?,
            info: NodeInfo {
                uuid: Arc::new(Uuid::new_v4().to_string()),
                user_name: Arc::new(user_name.to_string()),
                description: Arc::new(description.to_string()),
                cert_der,
            },
            peer_hubnode: Pointer::new(None),
        })
    }
    pub fn info(&self) -> NodeInfo {
        self.info.clone()
    }
    pub async fn accept(&self) -> JoinHandle<Result<PeerNode>> {
        let incoming_opt = self.endpoint.accept().await;
        tokio::spawn({
            let node_info = self.info.clone();
            async move {
                let connection = incoming_opt
                    .ok_or(anyhow!("没有传入的连接"))?
                    .accept()?
                    .await?;
                //发送自身节点信息并接收对方节点信息
                let (mut send, mut recv) = connection.accept_bi().await?;
                send.write_all(&rmp_serde::to_vec(&node_info)?).await?;
                send.finish()?;
                Ok(PeerNode::new(
                    connection,
                    rmp_serde::from_slice(&recv.read_to_end(usize::MAX).await?)?,
                ))
            }
        })
    }
    pub async fn connect(
        &self,
        ip_address: SocketAddr,
        cert_der: Arc<Vec<u8>>,
    ) -> Result<PeerNode> {
        let connection = self
            .endpoint
            .connect_with(
                ClientConfig::with_root_certificates(Arc::new({
                    let mut a = RootCertStore::empty();
                    a.add((*cert_der).clone().into())?;
                    a
                }))?,
                ip_address,
                &x509_dns_name_from_cert_der(cert_der)?,
            )?
            .await?;
        //发送自身节点信息并接收对方节点信息
        let (mut send, mut recv) = connection.open_bi().await?;
        send.write_all(&rmp_serde::to_vec(&self.info)?).await?;
        send.finish()?;
        Ok(PeerNode::new(
            connection,
            rmp_serde::from_slice(&recv.read_to_end(usize::MAX).await?)?,
        ))
    }
    pub async fn access_hubnode(
        &self,
        ip_address: SocketAddr,
        cert_der: Arc<Vec<u8>>,
    ) -> Result<()> {
        let connection = self
            .endpoint
            .connect_with(
                ClientConfig::with_root_certificates(Arc::new({
                    let mut a = RootCertStore::empty();
                    a.add((*cert_der).clone().into())?;
                    a
                }))?,
                ip_address,
                &x509_dns_name_from_cert_der(cert_der)?,
            )?
            .await?;
        //发送自身节点信息并接收对方节点信息
        let (mut send, mut recv) = connection.open_bi().await?;
        send.write_all(&rmp_serde::to_vec(&self.info)?).await?;
        send.finish()?;
        *self.peer_hubnode.lock() = Some(PeerNode::new(
            connection,
            rmp_serde::from_slice(&recv.read_to_end(usize::MAX).await?)?,
        ));
        Ok(())
    }
}
