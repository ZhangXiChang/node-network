mod app;

use std::net::SocketAddr;

use anyhow::Result;
use app::App;
use tauri::{AppHandle, Manager};
use utils::ext::logger_builder::LoggerBuilder;

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
            .invoke_handler(tauri::generate_handler![
                connect_server,
                login,
                get_node_name
            ])
            .run(tauri::generate_context!())?;
        Ok(())
    })() {
        log::error!("{}", err);
    }
}

#[tauri::command]
async fn connect_server(tauri_app: AppHandle, socketaddr: SocketAddr) -> Result<(), String> {
    tauri_app
        .state::<App>()
        .connect_server(socketaddr)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
async fn login(tauri_app: AppHandle, login_name: String) -> Result<(), String> {
    tauri_app
        .state::<App>()
        .login(login_name)
        .await
        .map_err(|err| err.to_string())
}

#[tauri::command]
async fn get_node_name(tauri_app: AppHandle) -> Result<String, String> {
    tauri_app
        .state::<App>()
        .get_node_name()
        .await
        .map_err(|err| err.to_string())
}
