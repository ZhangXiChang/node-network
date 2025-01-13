use anyhow::Result;
use parking_lot::Mutex;
use protocol::ServerCommand;
use quinn::{Connection, Endpoint};
use tauri::{AppHandle, Manager};
use utils::ext::{logger_builder::LoggerBuilder, quinn::EndpointExtension, vecu8::borsh::Borsh};
use uuid::Uuid;

struct Server {
    name: String,
    connection: Connection,
}

struct State {
    endpoint: Endpoint,
    server: Mutex<Option<Server>>,
}
impl State {
    fn new() -> Result<Self> {
        let cert_key = rcgen::generate_simple_self_signed(vec![Uuid::new_v4().to_string()])?;
        let endpoint = Endpoint::new_ext(
            "0.0.0.0:10271".parse()?,
            cert_key.cert.der().to_vec(),
            cert_key.key_pair.serialize_der(),
        )?;
        Ok(Self {
            endpoint,
            server: Default::default(),
        })
    }
}

#[tokio::main]
#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub async fn main() {
    if let Err(err) = (|| -> Result<()> {
        tauri::Builder::default()
            .plugin(tauri_plugin_prevent_default::init())
            .plugin(
                tauri_plugin_log::Builder::builder(log::LevelFilter::Info)
                    .log_file_dir("./log/")
                    .build(),
            )
            .plugin(tauri_plugin_opener::init())
            .plugin(tauri_plugin_os::init())
            .manage(State::new()?)
            .invoke_handler(tauri::generate_handler![login])
            .run(tauri::generate_context!())?;
        Ok(())
    })() {
        log::error!("{}", err);
    }
}

#[tauri::command]
async fn login(app: AppHandle, login_name: String) -> Result<(), String> {
    async move {
        let server_connection = app
            .state::<State>()
            .endpoint
            .connect_ext(
                "127.0.0.1:10270".parse()?,
                include_bytes!("../../target/server.cer").to_vec(),
            )?
            .await?;
        let (mut send, mut recv) = server_connection.open_bi().await?;
        send.write_all(&Vec::borsh_from(&ServerCommand::Login { login_name })?)
            .await?;
        send.finish()?;
        *app.state::<State>().server.lock() = Some(Server {
            name: String::from_utf8(recv.read_to_end(usize::MAX).await?)?,
            connection: server_connection.clone(),
        });
        anyhow::Ok(())
    }
    .await
    .map_err(|err| err.to_string())
}
