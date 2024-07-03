// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eyre::{eyre, Result};
use node_network::app::App;
use tauri::Manager;
use tracing_subscriber::fmt::SubscriberBuilder;
use window_shadows::set_shadow;

#[tokio::main]
async fn main() -> Result<()> {
    //初始化日志系统
    SubscriberBuilder::default()
        .with_max_level(tracing::Level::INFO)
        .init();
    tauri::Builder::default()
        .setup(|app| {
            if let Some(window) = app.get_window("main") {
                set_shadow(window, true).map_err(|e| eyre!("{}", e))?;
            }
            Ok(())
        })
        .manage(App::new()?)
        .invoke_handler(tauri::generate_handler![open, connect_hubnode])
        .run(tauri::generate_context!())?;
    Ok(())
}

#[tauri::command]
fn open(path: String) {
    let _ = opener::open(path);
}

#[tauri::command]
async fn connect_hubnode<'a>(app: tauri::State<'a, App>) -> Result<(), ()> {
    async {
        app.connect_hubnode().await?;
        eyre::Ok(())
    }
    .await
    .map_err(|_| ())?;
    Ok(())
}
