use std::{net::SocketAddr, sync::Arc};

use anyhow::{anyhow, Result};
use parking_lot::Mutex;
use protocol::{PeernodeAction, ServerAction};
use quinn::{Connection, Endpoint, VarInt};
use serde::Serialize;
use tauri::{AppHandle, Emitter, Manager, Window, WindowEvent};
use utils::ext::{logger_builder::LoggerBuilder, quinn::EndpointExtension, vecu8::borsh::Borsh};
use uuid::Uuid;

#[derive(Clone, Serialize)]
struct ChatMessage {
    node_name: String,
    value: String,
}

#[derive(Clone)]
struct Server {
    name: Arc<Mutex<Option<String>>>,
    connection: Connection,
}

struct App {
    endpoint: Endpoint,
    server: Arc<Mutex<Option<Server>>>,
    node_name: Arc<Mutex<Option<String>>>,
}
impl App {
    fn new() -> Result<Self> {
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
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
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
            .manage(App::new()?)
            .on_window_event(window_event)
            .invoke_handler(tauri::generate_handler![
                connect_server,
                login,
                get_node_name,
                send_message,
            ])
            .run(tauri::generate_context!())?;
        Ok(())
    })() {
        log::error!("{}", err);
    }
}

fn window_event(window: &Window, event: &WindowEvent) {
    match event {
        WindowEvent::CloseRequested { api: _, .. } => window
            .state::<App>()
            .endpoint
            .close(VarInt::from_u32(0), "程序正常关闭".as_bytes()),
        _ => (),
    }
}

#[tauri::command]
async fn connect_server(tauri_app: AppHandle, socketaddr: SocketAddr) -> Result<(), String> {
    async move {
        let app = tauri_app.state::<App>();
        let server = Server {
            name: Default::default(),
            connection: app
                .endpoint
                .connect_ext(
                    socketaddr,
                    include_bytes!("../../target/server.cer").to_vec(),
                )?
                .await?,
        };
        *app.server.lock() = Some(server.clone());
        tokio::spawn(async move {
            if let Err(result) = {
                let server = server.clone();
                async move {
                    loop {
                        match server
                            .connection
                            .accept_uni()
                            .await?
                            .read_to_end(usize::MAX)
                            .await?
                            .borsh_to::<PeernodeAction>()?
                        {
                            PeernodeAction::AcceptServerName { server_name } => {
                                log::info!("服务端名称:[{}]", server_name);
                                *server.name.lock() = Some(server_name)
                            }
                            PeernodeAction::AcceptMessage { message } => {
                                tauri_app.emit(
                                    "accept_message",
                                    ChatMessage {
                                        node_name: "12e12".to_string(),
                                        value: message,
                                    },
                                )?;
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
        });
        anyhow::Ok(())
    }
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
async fn login(tauri_app: AppHandle, login_name: String) -> Result<(), String> {
    async move {
        let app = tauri_app.state::<App>();
        *app.node_name.lock() = Some(login_name.clone());
        let server = app.get_server()?;
        let mut send = server.connection.open_uni().await?;
        send.write_all(&Vec::borsh_from(&ServerAction::PeernodeLogin {
            login_name,
        })?)
        .await?;
        send.finish()?;
        anyhow::Ok(())
    }
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
async fn get_node_name(tauri_app: AppHandle) -> Result<String, String> {
    async move {
        anyhow::Ok(
            tauri_app
                .state::<App>()
                .node_name
                .lock()
                .clone()
                .ok_or(anyhow!("未登录服务端"))?,
        )
    }
    .await
    .map_err(|err| err.to_string())
}

#[tauri::command]
async fn send_message(tauri_app: AppHandle, message: String) -> Result<(), String> {
    async move {
        let server = tauri_app.state::<App>().get_server()?;
        let mut send = server.connection.open_uni().await?;
        send.write_all(&Vec::borsh_from(&ServerAction::BroadcastMessage {
            message,
        })?)
        .await?;
        send.finish()?;
        anyhow::Ok(())
    }
    .await
    .map_err(|err| err.to_string())
}
