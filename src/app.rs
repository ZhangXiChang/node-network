use std::{sync::Arc, time::Duration};

use eyre::Result;
use quinn::{ClientConfig, Connection, Endpoint, ServerConfig, TransportConfig};
use rustls::{pki_types::PrivateKeyDer, RootCertStore};
use tool_code::{lock::ArcMutex, x509::x509_dns_name_from_cert_der};
use uuid::Uuid;

const SERVER_CERT_DER: &[u8] = include_bytes!("../target/server.cer");

#[derive(Clone)]
pub struct App {
    endpoint: Endpoint,
    cert_der: Arc<Vec<u8>>,
    server_conn: ArcMutex<Option<Connection>>,
}
impl App {
    pub fn new() -> Result<Self> {
        let rcgen::CertifiedKey { cert, key_pair } =
            rcgen::generate_simple_self_signed(vec![Uuid::new_v4().to_string()])?;
        let cert_der = Arc::new(cert.der().to_vec());
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
                Endpoint::server(server_config, "0.0.0.0:0".parse()?)?
            },
            cert_der,
            server_conn: ArcMutex::new(None),
        })
    }
    pub async fn connect_hubnode(&self) -> Result<()> {
        *self.server_conn.lock() = Some({
            let server_name = x509_dns_name_from_cert_der(SERVER_CERT_DER)?;
            self.endpoint
                .connect_with(
                    ClientConfig::with_root_certificates(Arc::new({
                        let mut a = RootCertStore::empty();
                        a.add(SERVER_CERT_DER.to_vec().into())?;
                        a
                    }))?,
                    "127.0.0.1:10270".parse()?,
                    &server_name,
                )?
                .await?
        });
        Ok(())
    }
}
