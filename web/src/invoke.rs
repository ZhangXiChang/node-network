use serde::{Deserialize, Serialize};
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub fn close_window() {
    spawn_local(async {
        invoke("close_window", JsValue::NULL).await;
    });
}

pub fn minimize_window() {
    spawn_local(async {
        invoke("minimize_window", JsValue::NULL).await;
    });
}

pub fn maximize_or_unmaximize_window() {
    spawn_local(async {
        invoke("maximize_or_unmaximize_window", JsValue::NULL).await;
    });
}

pub async fn window_is_maximized() -> bool {
    invoke("window_is_maximized", JsValue::NULL)
        .await
        .as_bool()
        .unwrap()
}

#[derive(Serialize, Deserialize)]
struct OpenArg {
    path: String,
}
pub fn open(path: &str) {
    let path = path.to_string();
    spawn_local(async move {
        invoke("open", to_value(&OpenArg { path }).unwrap()).await;
    });
}
