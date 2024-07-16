#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine};
use node_network::system::System;
use tauri::Manager;
use window_shadows::set_shadow;

#[tokio::main]
async fn main() -> Result<()> {
    tauri::Builder::default()
        .setup(|app| {
            if let Some(window) = app.get_window("main") {
                set_shadow(window, true).map_err(|e| anyhow!("{}", e))?;
            }
            Ok(())
        })
        .manage(System::new()?)
        .invoke_handler(tauri::generate_handler![
            open,
            connect_server,
            get_user_star_hubnode_logo
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}

#[tauri::command]
async fn open(path: String) -> Result<(), String> {
    async move {
        opener::open(path)?;
        anyhow::Ok(())
    }
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn connect_server<'a>(system: tauri::State<'a, System>) -> Result<(), String> {
    async move {
        system.connect_server().await?;
        anyhow::Ok(())
    }
    .await
    .map_err(|e| e.to_string())
}

#[tauri::command]
async fn get_user_star_hubnode_logo<'a>(
    system: tauri::State<'a, System>,
) -> Result<Vec<String>, String> {
    async move {
        anyhow::Ok(
            system
                .get_user_star_hubnode_logo()
                .await?
                .iter()
                .map(|hubnode_table| general_purpose::STANDARD.encode(hubnode_table.logo.clone()))
                .collect(),
        )
    }
    .await
    .map_err(|e| e.to_string())
}
