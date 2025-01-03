use anyhow::Result;
use quinn::Endpoint;
use utils::ext::quinn::QuinnExtension;
use uuid::Uuid;

struct State {
    endpoint: Endpoint,
}
impl State {
    fn new() -> Result<Self> {
        let cert_key = rcgen::generate_simple_self_signed(vec![Uuid::new_v4().to_string()])?;
        let endpoint = Endpoint::new_ext(
            "0.0.0.0:10271".parse()?,
            cert_key.cert.der().to_vec(),
            cert_key.key_pair.serialize_der(),
        )?;
        Ok(Self { endpoint })
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
#[tokio::main]
pub async fn run() -> Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_prevent_default::init())
        .manage(State::new()?)
        .run(tauri::generate_context!())?;
    Ok(())
}
