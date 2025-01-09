use anyhow::{anyhow, Result};
use protocol::{
    data_packet::{
        data,
        packet_type::{self, server_command::EnumServerCommand},
    },
    DataPacket,
};
use quinn::{ConnectionError, Endpoint};
use utils::{
    ext::{quinn::EndpointExtension, vecu8::protobuf::Protobuf},
    logger::Logger,
};

#[tokio::main]
async fn main() -> Result<()> {
    Logger::new().start()?;
    let endpoint = Endpoint::new_ext(
        "0.0.0.0:10270".parse()?,
        include_bytes!("../../../target/server.cer").to_vec(),
        include_bytes!("../../../target/server.key").to_vec(),
    )?;
    while let Some(incoming) = endpoint.accept().await {
        tokio::spawn(async move {
            async move {
                let connection = incoming.accept()?.await?;
                loop {
                    match connection.accept_bi().await {
                        Ok((mut send, mut recv)) => {
                            let data_packet =
                                recv.read_to_end(usize::MAX).await?.decode::<DataPacket>()?;
                            match data_packet
                                .packet_type
                                .decode::<packet_type::ServerCommand>()?
                                .enum_self()
                            {
                                EnumServerCommand::Login => {
                                    log::info!(
                                        "[{}]登录",
                                        data_packet.data.decode::<data::LoginInfo>()?.login_name
                                    );
                                    send.write_all("嫦娥迹象".as_bytes()).await?;
                                    send.finish()?;
                                }
                            }
                        }
                        Err(err) => match err {
                            ConnectionError::ApplicationClosed(close_info) => {
                                log::info!(
                                    "连接正常关闭[{}]",
                                    String::from_utf8(close_info.reason.to_vec())?
                                );
                                break;
                            }
                            _ => return Err(anyhow!("{}", err)),
                        },
                    }
                }
                anyhow::Ok(())
            }
            .await
            .unwrap()
        });
    }
    Ok(())
}
