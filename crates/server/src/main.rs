use std::{sync::Arc, time::Duration};

use eyre::Result;
use quinn::{Endpoint, ServerConfig, TransportConfig};
use rustls::pki_types::PrivateKeyDer;
use tool_code::lock::ArcMutex;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

const CERT_DER: &[u8] = include_bytes!("../../../target/server.cer");
const KEY_DER: &[u8] = include_bytes!("../../../target/server.key");

#[tokio::main]
async fn main() -> Result<()> {
    //初始化日志系统
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::filter::Targets::new()
                .with_targets(vec![("server", tracing::Level::INFO)]),
        )
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
    let connection_list = ArcMutex::new(Vec::new());
    loop {
        if let Some(incoming) = endpoint.accept().await {
            tokio::spawn({
                let connection_list = connection_list.clone();
                async move {
                    let remote_address = incoming.remote_address();
                    tracing::info!("[{}]接入连接", remote_address);
                    match incoming.await {
                        Ok(connection) => {
                            connection_list.lock().push(connection.clone());
                            tracing::info!("[{}]连接成功", connection.remote_address());
                        }
                        Err(err) => tracing::info!("[{}]{}", remote_address, err),
                    }
                }
            });
        }
    }
}
