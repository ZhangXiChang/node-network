pub mod node;
pub mod packet;
pub mod tls;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use eyre::Result;
    use tool_code::lock::Pointer;

    use crate::{node::Node, tls::CertKey};

    #[tokio::test]
    async fn main() -> Result<()> {
        //服务端
        let node_s = Node::new(
            "0.0.0.0:10270".parse()?,
            "服务端",
            "服务端描述",
            Some(CertKey {
                cert_der: Arc::new(include_bytes!("../../../assets/server.cer").to_vec()),
                key_der: include_bytes!("../../../assets/server.key").to_vec(),
            }),
        )?;
        let peer_node_s = Pointer::new(None);
        let verify_msg = Arc::new("Test".as_bytes().to_vec());
        tokio::spawn({
            let node_s = node_s.clone();
            let peer_node_s = peer_node_s.clone();
            let verify_msg = verify_msg.clone();
            async move {
                let peer_node = node_s.accept().await?;
                peer_node_s.set(Some(peer_node.clone()));
                peer_node.send(verify_msg).await?;
                eyre::Ok(())
            }
        });
        //节点
        let node_c = Node::new("0.0.0.0:0".parse()?, "节点", "节点描述", None)?;
        let peer_node_c = node_c
            .connect("127.0.0.1:10270".parse()?, node_s.clone().info.cert_der)
            .await?;
        assert_eq!(peer_node_c.info, node_s.info);
        assert_eq!(peer_node_c.recv().await?, verify_msg);
        //中枢节点
        let node_h = Node::new("0.0.0.0:10271".parse()?, "中枢节点", "中枢节点描述", None)?;
        let peer_node_h = Pointer::new(None);
        tokio::spawn({
            let node_h = node_h.clone();
            let peer_node_h = peer_node_h.clone();
            async move {
                let peer_node = node_h.accept().await?;
                peer_node_h.set(Some(peer_node.clone()));
                eyre::Ok(())
            }
        });
        //接入中枢节点
        node_c
            .access_hubnode("127.0.0.1:10271".parse()?, node_h.clone().info.cert_der)
            .await?;
        Ok(())
    }
}
