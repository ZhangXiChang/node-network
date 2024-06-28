// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use window_shadows::set_shadow;

fn main() -> Result<(), tauri::Error> {
    tauri::Builder::default()
        .setup(|app| {
            set_shadow(app.get_window("main").unwrap(), true).unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![open])
        .run(tauri::generate_context!())
}

#[tauri::command]
fn open(path: String) {
    let _ = opener::open(path);
}
