pub mod node;
pub mod packet;
pub mod tls;

#[cfg(test)]
mod tests {
    use std::{net::SocketAddr, sync::Arc};

    use anyhow::{Context, Result};
    use quinn::{ConnectionError, VarInt};

    use crate::{
        node::{IPAddress, Incoming, Node, NodeNewInfo},
        tls::CertKey,
    };

    #[tokio::test]
    async fn server() -> Result<()> {
        let server_node = Node::new(NodeNewInfo {
            ip_address: IPAddress::new(10270),
            cert_key: Some(CertKey {
                cert_der: Arc::new(include_bytes!("../../../assets/server/server.cer").to_vec()),
                key_der: Arc::new(include_bytes!("../../../assets/server/server.key").to_vec()),
            }),
            user_name: "北方中枢",
            ..Default::default()
        })
        .context("创建服务器节点失败")?;
        let client_node = Node::new(NodeNewInfo {
            ip_address: IPAddress::new(0),
            user_name: "张喜昌",
            ..Default::default()
        })
        .context("创建客户端节点失败")?;
        tokio::spawn({
            let server_node = server_node.clone();
            let client_node = client_node.clone();
            async move {
                if let Err(err) = async move {
                    if let Some(incoming) = server_node.accept_ipv4().await {
                        accept_handling(incoming, client_node).await?;
                    }
                    anyhow::Ok(())
                }
                .await
                {
                    panic!("{:?}", err);
                }
            }
        });
        tokio::spawn({
            let server_node = server_node.clone();
            let client_node = client_node.clone();
            async move {
                if let Err(err) = async move {
                    if let Some(incoming) = server_node.accept_ipv6().await? {
                        accept_handling(incoming, client_node).await?;
                    }
                    anyhow::Ok(())
                }
                .await
                {
                    panic!("{:?}", err);
                }
            }
        });
        connect_handling("[::1]:10270".parse()?, client_node, server_node).await?;
        Ok(())
    }

    async fn connect_handling(
        ip_address: SocketAddr,
        client_node: Node,
        server_node: Node,
    ) -> Result<()> {
        let server_peer_node = client_node
            .connect_node(
                ip_address,
                Arc::new(include_bytes!("../../../assets/server/server.cer").to_vec()),
            )
            .await
            .context("连接服务端节点失败")?;
        assert_eq!(server_peer_node.info(), server_node.info());
        let mut send = server_peer_node
            .open_uni()
            .await
            .context("打开服务器节点流失败")?;
        send.write_all("关闭".as_bytes())
            .await
            .context("发送消息失败")?;
        send.finish().context("结束发送失败")?;
        assert!(match server_peer_node.closed().await {
            ConnectionError::ApplicationClosed(app_closed_info) =>
                app_closed_info.error_code == VarInt::from_u32(0),
            _ => false,
        });
        Ok(())
    }

    async fn accept_handling(incoming: Incoming, client_node: Node) -> Result<()> {
        let client_peer_node = incoming.accept().await.context("接受客户端节点连接失败")?;
        assert_eq!(client_peer_node.info(), client_node.info());
        let mut recv = client_peer_node
            .accept_uni()
            .await
            .context("接收客户端节点流失败")?;
        if String::from_utf8(
            recv.read_to_end(usize::MAX)
                .await
                .context("接收客户端节点数据")?,
        )
        .context("解析客户端节点数据失败")?
            == "关闭"
        {
            client_peer_node.close(0, "正常关闭".as_bytes());
        }
        Ok(())
    }
}
