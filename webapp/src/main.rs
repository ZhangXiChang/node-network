#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::{anyhow, Result};
use base64::{engine::general_purpose, Engine};
use db_types::HubNodeInfo;
use node_network::system::System;
use serde::Serialize;
use tauri::Manager;
use window_shadows::set_shadow;

trait ToBase64 {
    type Target;

    fn to_base64(&self) -> Self::Target;
}

impl ToBase64 for Vec<u8> {
    type Target = String;

    fn to_base64(&self) -> Self::Target {
        general_purpose::STANDARD.encode(self)
    }
}

#[derive(Serialize)]
struct HubNodeInfoBase64 {
    base: HubNodeInfo,
    logo: String,
    cert_der: String,
}
impl ToBase64 for HubNodeInfo {
    type Target = HubNodeInfoBase64;

    fn to_base64(&self) -> Self::Target {
        Self::Target {
            base: HubNodeInfo {
                name: self.name.clone(),
                ipv4_address: self.ipv4_address.clone(),
                ipv6_address: self.ipv6_address.clone(),
                description: self.description.clone(),
                ..Default::default()
            },
            logo: self.logo.to_base64(),
            cert_der: self.cert_der.to_base64(),
        }
    }
}

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
            get_hubnode_table
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
async fn get_hubnode_table<'a>(
    system: tauri::State<'a, System>,
) -> Result<Vec<HubNodeInfoBase64>, String> {
    async move {
        anyhow::Ok(
            system
                .get_hubnode_table()
                .await?
                .iter()
                .map(|hubnodeinfo| hubnodeinfo.to_base64())
                .collect(),
        )
    }
    .await
    .map_err(|e| e.to_string())
}
