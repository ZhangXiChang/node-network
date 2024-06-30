// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::sync::Mutex;

use node_network::Node;
use tauri::Manager;
use window_shadows::set_shadow;

#[derive(Default)]
struct State {
    node: Mutex<Option<Node>>,
}

#[tokio::main]
async fn main() {
    tauri::Builder::default()
        .setup(|app| {
            set_shadow(app.get_window("main").unwrap(), true).unwrap();
            Ok(())
        })
        .manage(State::default())
        .invoke_handler(tauri::generate_handler![open, create_node, connect_hubnode])
        .run(tauri::generate_context!())
        .unwrap();
}

#[tauri::command]
fn open(path: String) {
    let _ = opener::open(path);
}

#[tauri::command]
fn create_node(state: tauri::State<State>) {
    *state.node.lock().unwrap() = Some(Node::new("0.0.0.0:10270".parse().unwrap()).unwrap());
}

#[tauri::command]
fn connect_hubnode() {
    println!("连接中枢节点");
}
