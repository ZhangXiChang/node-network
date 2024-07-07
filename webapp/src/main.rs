#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use eyre::{eyre, Result};
//use node_network::app::App;
use tauri::Manager;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use window_shadows::set_shadow;

#[tokio::main]
async fn main() -> Result<()> {
    //初始化日志系统
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::filter::Targets::new()
                .with_targets(vec![("webapp", tracing::Level::INFO)]),
        )
        .init();
    tauri::Builder::default()
        .setup(|app| {
            if let Some(window) = app.get_window("main") {
                set_shadow(window, true).map_err(|e| eyre!("{}", e))?;
            }
            Ok(())
        })
        //.manage(App::new()?)
        .invoke_handler(tauri::generate_handler![
            open,
            //          connect_hubnode,
            //           get_user_star_hubnode_logo
        ])
        .run(tauri::generate_context!())?;
    Ok(())
}

#[tauri::command]
async fn open(path: String) -> Result<(), String> {
    async move {
        opener::open(path)?;
        eyre::Ok(())
    }
    .await
    .map_err(|e| e.to_string())
}

// #[tauri::command]
// async fn connect_hubnode<'a>(app: tauri::State<'a, App>) -> Result<(), String> {
//     async move {
//         app.connect_hubnode().await?;
//         eyre::Ok(())
//     }
//     .await
//     .map_err(|e| e.to_string())
// }

// #[tauri::command]
// async fn get_user_star_hubnode_logo<'a>(_app: tauri::State<'a, App>) -> Result<(), String> {
//     async move {
//         tracing::info!("获取用户收藏中枢节点图标");
//         eyre::Ok(())
//     }
//     .await
//     .map_err(|e| e.to_string())
// }
