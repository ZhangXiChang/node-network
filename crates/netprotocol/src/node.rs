use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr},
    sync::Arc,
    time::Duration,
};

use anyhow::{anyhow, Context, Result};
use quinn::{
    ClientConfig, Connection, ConnectionError, Endpoint, ServerConfig, TransportConfig, VarInt,
};
use rustls::{pki_types::PrivateKeyDer, RootCertStore};
use serde::{Deserialize, Serialize};
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

pub struct IPAddress {
    pub ipv4_address: SocketAddr,
    pub ipv6_address: SocketAddr,
}
impl IPAddress {
    pub fn new(port: u16) -> Self {
        Self {
            ipv4_address: SocketAddr::new(IpAddr::V4(Ipv4Addr::UNSPECIFIED), port),
            ipv6_address: SocketAddr::new(IpAddr::V6(Ipv6Addr::UNSPECIFIED), port),
        }
    }
}
impl Default for IPAddress {
    fn default() -> Self {
        Self::new(0)
    }
}

#[derive(Clone)]
pub struct PeerNode {
    connection: Connection,
    info: NodeInfo,
}
impl PeerNode {
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
    pub fn close(&self, code: u32, reason: &[u8]) {
        self.connection.close(VarInt::from_u32(code), reason);
    }
    pub async fn closed(&self) -> ConnectionError {
        self.connection.closed().await
    }
}
impl Drop for PeerNode {
    fn drop(&mut self) {
        self.close(0, "对等体在远端被释放".as_bytes());
    }
}

pub struct Incoming {
    incoming: quinn::Incoming,
    info: NodeInfo,
}
impl Incoming {
    pub async fn accept(self) -> Result<PeerNode> {
        let connection = self.incoming.accept()?.await?;
        let (mut send, mut recv) = connection.accept_bi().await?;
        send.write_all(&rmp_serde::to_vec(&self.info)?).await?;
        send.finish()?;
        Ok(PeerNode {
            connection,
            info: rmp_serde::from_slice(&recv.read_to_end(usize::MAX).await?)?,
        })
    }
}

#[derive(Default)]
pub struct NodeNewInfo<'a> {
    pub ip_address: IPAddress,
    pub user_name: &'a str,
    pub description: &'a str,
    pub cert_key: Option<CertKey>,
}

#[derive(Clone)]
pub struct Node {
    ipv4_endpoint: Endpoint,
    ipv6_endpoint: Arc<Option<Endpoint>>,
    info: NodeInfo,
    peer_hubnode: Pointer<Option<PeerNode>>,
}
impl Node {
    pub fn new(node_new_info: NodeNewInfo) -> Result<Self> {
        let (mut server_config, cert_der) = if let Some(cert_key) = node_new_info.cert_key {
            (
                ServerConfig::with_single_cert(
                    vec![(*cert_key.cert_der).clone().into()],
                    PrivateKeyDer::Pkcs8((*cert_key.key_der).clone().into()),
                )
                .context("使用外部证书配置服务端失败")?,
                cert_key.cert_der,
            )
        } else {
            let rcgen::CertifiedKey { cert, key_pair } =
                rcgen::generate_simple_self_signed(vec![Uuid::new_v4().to_string()])?;
            (
                ServerConfig::with_single_cert(
                    vec![cert.der().to_vec().into()],
                    PrivateKeyDer::Pkcs8(key_pair.serialize_der().into()),
                )
                .context("使用自动生成证书配置服务端失败")?,
                Arc::new(cert.der().to_vec()),
            )
        };
        server_config.transport_config(Arc::new({
            let mut a = TransportConfig::default();
            a.keep_alive_interval(Some(Duration::from_secs(5)));
            a
        }));
        Ok(Self {
            ipv4_endpoint: Endpoint::server(
                server_config.clone(),
                node_new_info.ip_address.ipv4_address,
            )
            .context("创建IPv4服务端失败")?,
            ipv6_endpoint: Arc::new(match node_new_info.ip_address.ipv6_address {
                Some(ipv6_address) => Some(
                    Endpoint::server(server_config, ipv6_address).context("创建IPv6服务端失败")?,
                ),
                None => None,
            }),
            info: NodeInfo {
                uuid: Arc::new(Uuid::new_v4().to_string()),
                user_name: Arc::new(node_new_info.user_name.to_string()),
                description: Arc::new(node_new_info.description.to_string()),
                cert_der,
            },
            peer_hubnode: Pointer::new(None),
        })
    }
    pub fn info(&self) -> NodeInfo {
        self.info.clone()
    }
    pub fn close(&self) {
        self.ipv4_endpoint
            .close(VarInt::from_u32(0), "对方节点关闭".as_bytes());
        if let Some(ipv6_endpoint) = &*self.ipv6_endpoint {
            ipv6_endpoint.close(VarInt::from_u32(0), "对方节点关闭".as_bytes());
        }
    }
    pub async fn accept_ipv4(&self) -> Option<Incoming> {
        if let Some(incoming) = self.ipv4_endpoint.accept().await {
            return Some(Incoming {
                incoming,
                info: self.info.clone(),
            });
        }
        None
    }
    pub async fn accept_ipv6(&self) -> Result<Option<Incoming>> {
        if let Some(endpoint) = &*self.ipv6_endpoint {
            if let Some(incoming) = endpoint.accept().await {
                return Ok(Some(Incoming {
                    incoming,
                    info: self.info.clone(),
                }));
            }
        }
        Ok(None)
    }
    async fn endpoint_by_ip_address(&self, ip_address: SocketAddr) -> Result<Endpoint> {
        Ok(match ip_address {
            SocketAddr::V4(_) => self.ipv4_endpoint.clone(),
            SocketAddr::V6(_) => (*self.ipv6_endpoint)
                .clone()
                .ok_or(anyhow!("没有配置IPv6地址"))?,
        })
    }
    async fn connect(&self, ip_address: SocketAddr, cert_der: Arc<Vec<u8>>) -> Result<Connection> {
        Ok(self
            .endpoint_by_ip_address(ip_address)
            .await
            .context(format!("根据IP地址[{}]类型获取服务端失败", ip_address))?
            .connect_with(
                ClientConfig::with_root_certificates(Arc::new({
                    let mut a = RootCertStore::empty();
                    a.add((*cert_der).clone().into())
                        .context(format!("添加[{}]的证书失败", ip_address))?;
                    a
                }))?,
                ip_address,
                &x509_dns_name_from_cert_der(cert_der)?,
            )
            .context(format!("建立与[{}]的连接失败", ip_address))?
            .await
            .context(format!("连接[{}]失败", ip_address))?)
    }
    pub async fn connect_node(
        &self,
        ip_address: SocketAddr,
        cert_der: Arc<Vec<u8>>,
    ) -> Result<PeerNode> {
        let connection = self.connect(ip_address, cert_der).await?;
        let (mut send, mut recv) = connection.open_bi().await.context("打开流失败")?;
        send.write_all(&rmp_serde::to_vec(&self.info).context("编码自身信息")?)
            .await
            .context("发送自身信息失败")?;
        send.finish().context("结束自身信息发送失败")?;
        Ok(PeerNode {
            connection,
            info: rmp_serde::from_slice(
                &recv
                    .read_to_end(usize::MAX)
                    .await
                    .context("接收对方信息失败")?,
            )
            .context("解码对方信息失败")?,
        })
    }
    pub async fn access_hubnode(
        &self,
        ip_address: SocketAddr,
        cert_der: Arc<Vec<u8>>,
    ) -> Result<()> {
        *self.peer_hubnode.lock() = Some(
            self.connect_node(ip_address, cert_der)
                .await
                .context("连接中心节点失败")?,
        );
        Ok(())
    }
}
