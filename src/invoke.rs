use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

pub fn close_window() {
    spawn_local(async {
        invoke("close_window", JsValue::null()).await;
    });
}

pub fn minimize_window() {
    spawn_local(async {
        invoke("minimize_window", JsValue::null()).await;
    });
}

pub fn maximize_window() {
    spawn_local(async {
        invoke("maximize_window", JsValue::null()).await;
    });
}
