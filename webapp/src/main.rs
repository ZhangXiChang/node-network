// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use node_network::app::App;
use tauri::Manager;
use window_shadows::set_shadow;

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            set_shadow(app.get_window("main").unwrap(), true).unwrap();
            Ok(())
        })
        .manage(App::new().await.unwrap())
        .invoke_handler(tauri::generate_handler![open])
        .run(tauri::generate_context!())
        .unwrap();
}

#[tauri::command]
fn open(path: String) {
    let _ = opener::open(path);
}
