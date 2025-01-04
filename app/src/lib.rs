use anyhow::Result;
use quinn::Endpoint;
use tauri::{AppHandle, Manager};
use utils::ext::quinn::EndpointExtension;
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
pub async fn main() -> Result<()> {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_prevent_default::init())
        .manage(State::new()?)
        .invoke_handler(tauri::generate_handler![connect])
        .run(tauri::generate_context!())?;
    Ok(())
}

#[tauri::command]
async fn connect(app: AppHandle) -> Result<(), String> {
    async move {
        let connection = app
            .state::<State>()
            .endpoint
            .connect_ext(
                "127.0.0.1:10270".parse()?,
                include_bytes!("../../target/cert1.cer").to_vec(),
            )
            .await?
            .await?;
        println!("{}", connection.remote_address());
        anyhow::Ok(())
    }
    .await
    .map_err(|err| err.to_string())
}
