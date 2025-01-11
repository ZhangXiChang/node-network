use std::{net::SocketAddr, sync::Arc, time::Duration};

use anyhow::Result;
use quinn::{
    rustls::{pki_types::PrivateKeyDer, RootCertStore},
    ClientConfig, Connecting, Endpoint, ServerConfig, TransportConfig,
};

use super::vecu8::certder::CertDer;

pub trait EndpointExtension
where
    Self: Sized,
{
    fn new_ext(socket_addr: SocketAddr, cert_der: Vec<u8>, key_der: Vec<u8>) -> Result<Self>;
    fn connect_ext(&self, socket_addr: SocketAddr, cert_der: Vec<u8>) -> Result<Connecting>;
}

impl EndpointExtension for Endpoint {
    fn new_ext(socket_addr: SocketAddr, cert_der: Vec<u8>, key_der: Vec<u8>) -> Result<Self> {
        let mut endpoint_config = ServerConfig::with_single_cert(
            vec![cert_der.into()],
            PrivateKeyDer::Pkcs8(key_der.into()),
        )?;
        endpoint_config.transport_config(Arc::new({
            let mut a = TransportConfig::default();
            a.keep_alive_interval(Some(Duration::from_secs(5)));
            a
        }));
        Ok(Self::server(endpoint_config, socket_addr)?)
    }
    fn connect_ext(&self, socket_addr: SocketAddr, cert_der: Vec<u8>) -> Result<Connecting> {
        let dns_name = cert_der.get_dns_name()?;
        Ok(self.connect_with(
            ClientConfig::with_root_certificates(Arc::new({
                let mut a = RootCertStore::empty();
                a.add(cert_der.into())?;
                a
            }))?,
            socket_addr,
            &dns_name,
        )?)
    }
}
