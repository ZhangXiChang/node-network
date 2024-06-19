// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

fn main() -> Result<(), tauri::Error> {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![exit])
        .run(tauri::generate_context!())
}

#[tauri::command]
fn exit(app: tauri::AppHandle) {
    app.exit(0)
}
