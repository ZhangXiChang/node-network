use std::{sync::Arc, time::Duration};

use eyre::Result;
use quinn::{Endpoint, ServerConfig, TransportConfig};
use rustls::pki_types::PrivateKeyDer;

const CERT_DER: &[u8] = include_bytes!("../../../target/server.cer");
const KEY_DER: &[u8] = include_bytes!("../../../target/server.key");

#[tokio::main]
async fn main() -> Result<()> {
    //初始化日志系统
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();
    //初始化服务端
    let mut server_config = ServerConfig::with_single_cert(
        vec![CERT_DER.to_vec().into()],
        PrivateKeyDer::Pkcs8(KEY_DER.to_vec().into()),
    )?;
    server_config.transport_config(Arc::new({
        let mut a = TransportConfig::default();
        a.keep_alive_interval(Some(Duration::from_secs(5)));
        a
    }));
    let endpoint = Endpoint::server(server_config, "0.0.0.0:10270".parse()?)?;
    tracing::info!("服务端初始化完成");
    //主循环
    loop {
        if let Some(incoming) = endpoint.accept().await {
            if let Ok(connection) = incoming.await {
                tokio::spawn({
                    let connection = connection.clone();
                    async move {
                        tracing::info!("{}", connection.remote_address());
                    }
                });
            }
        }
    }
}
