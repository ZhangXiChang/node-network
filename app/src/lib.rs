use anyhow::Result;
use prost::Message;
use protocol::{
    data_packet::{
        data,
        packet_type::{self, server_command::EnumServerCommand},
    },
    DataPacket,
};
use quinn::{Endpoint, VarInt};
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
        .plugin(tauri_plugin_prevent_default::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_os::init())
        .manage(State::new()?)
        .invoke_handler(tauri::generate_handler![login])
        .run(tauri::generate_context!())?;
    Ok(())
}

#[tauri::command]
async fn login(app: AppHandle, login_name: String) -> Result<(), String> {
    async move {
        let connection = app
            .state::<State>()
            .endpoint
            .connect_ext(
                "127.0.0.1:10270".parse()?,
                include_bytes!("../../target/server.cer").to_vec(),
            )
            .await?
            .await?;
        let (mut send, mut recv) = connection.open_bi().await?;
        send.write_all(
            &DataPacket {
                packet_type: packet_type::ServerCommand {
                    enum_self: EnumServerCommand::Login.into(),
                }
                .encode_to_vec(),
                data: data::LoginInfo { login_name }.encode_to_vec(),
            }
            .encode_to_vec(),
        )
        .await?;
        send.finish()?;
        let server_name = String::from_utf8(recv.read_to_end(usize::MAX).await?)?;
        println!("[{}]连接", server_name);
        connection.close(VarInt::from_u32(0), "主动关闭连接".as_bytes());
        anyhow::Ok(())
    }
    .await
    .map_err(|err| err.to_string())
}
