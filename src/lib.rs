use std::{net::SocketAddr, sync::Arc, time::Duration};

use eyre::Result;
use quinn::{ServerConfig, TransportConfig};
use rustls::pki_types::PrivateKeyDer;
use uuid::Uuid;

pub use quinn::Endpoint;

pub struct Node {
    endpoint: Endpoint,
    cert_der: Vec<u8>,
}
impl Node {
    pub fn new(socket_addr: SocketAddr) -> Result<Self> {
        let rcgen::CertifiedKey { cert, key_pair } =
            rcgen::generate_simple_self_signed(vec![Uuid::new_v4().to_string()])?;
        let cert_der = cert.der().to_vec();
        Ok(Self {
            endpoint: {
                let mut server_config = ServerConfig::with_single_cert(
                    vec![cert.into()],
                    PrivateKeyDer::Pkcs8(key_pair.serialize_der().into()),
                )?;
                server_config.transport_config(Arc::new({
                    let mut a = TransportConfig::default();
                    a.keep_alive_interval(Some(Duration::from_secs(5)));
                    a
                }));
                Endpoint::server(server_config, socket_addr)?
            },
            cert_der,
        })
    }
}
