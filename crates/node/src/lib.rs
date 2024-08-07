use std::{sync::Arc, time::Duration};

use anyhow::Result;
use quinn::{Connection, Endpoint, ServerConfig, TransportConfig};
use rustls::pki_types::PrivateKeyDer;
use tool_code::lock::Pointer;
use uuid::Uuid;

#[derive(Clone)]
pub struct Node {
    endpoint: Endpoint,
    server_conn: Pointer<Option<Connection>>,
    hubnode_conn: Pointer<Option<Connection>>,
}
impl Node {
    pub fn new() -> Result<Self> {
        let rcgen::CertifiedKey { cert, key_pair } =
            rcgen::generate_simple_self_signed(vec![Uuid::new_v4().to_string()])?;
        let mut endpoint_config = ServerConfig::with_single_cert(
            vec![cert.der().to_vec().into()],
            PrivateKeyDer::Pkcs8(key_pair.serialize_der().into()),
        )?;
        endpoint_config.transport_config(Arc::new({
            let mut a = TransportConfig::default();
            a.keep_alive_interval(Some(Duration::from_secs(5)));
            a
        }));
        Ok(Self {
            endpoint: Endpoint::server(endpoint_config, "0.0.0.0:0".parse()?)?,
            server_conn: Pointer::new(None),
            hubnode_conn: Pointer::new(None),
        })
    }
}
