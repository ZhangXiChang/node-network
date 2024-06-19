use wasm_bindgen::prelude::*;

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    fn invoke(cmd: &str) -> JsValue;
}

pub fn exit() {
    invoke("exit");
}
