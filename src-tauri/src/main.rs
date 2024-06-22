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
        .invoke_handler(tauri::generate_handler![
            close_window,
            minimize_window,
            maximize_window,
            window_is_maximized
        ])
        .run(tauri::generate_context!())
}

#[tauri::command]
fn close_window(window: tauri::Window) {
    window.close().unwrap()
}
#[tauri::command]
fn minimize_window(window: tauri::Window) {
    window.minimize().unwrap()
}
#[tauri::command]
fn maximize_window(window: tauri::Window) {
    if window.is_maximized().unwrap() {
        window.unmaximize().unwrap()
    } else {
        window.maximize().unwrap()
    }
}
#[tauri::command]
fn window_is_maximized(window: tauri::Window) -> bool {
    window.is_maximized().unwrap()
}
