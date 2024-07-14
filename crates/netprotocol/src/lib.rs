pub mod node;
pub mod packet;
pub mod tls;

#[cfg(test)]
mod tests {
    use std::sync::Arc;

    use anyhow::Result;
    use hickory_resolver::{
        config::{ResolverConfig, ResolverOpts},
        AsyncResolver,
    };
    use tool_code::lock::Pointer;

    use crate::{node::Node, tls::CertKey};

    #[tokio::test]
    async fn main() -> Result<()> {
        //服务端
        let node_s = Node::new(
            "[::]:10270".parse()?,
            "服务端",
            "服务端描述",
            Some(CertKey {
                cert_der: Arc::new(include_bytes!("../../../assets/server.cer").to_vec()),
                key_der: Arc::new(include_bytes!("../../../assets/server.key").to_vec()),
            }),
        )?;
        let peer_node_s = Pointer::new(None);
        let verify_msg = Arc::new("Test".as_bytes().to_vec());
        tokio::spawn({
            let node_s = node_s.clone();
            let peer_node_s = peer_node_s.clone();
            let verify_msg = verify_msg.clone();
            async move {
                let peer_node = node_s.accept().await.await??;
                peer_node_s.set(Some(peer_node.clone()));
                let mut send = peer_node.open_uni().await?;
                send.write_all(&verify_msg).await?;
                send.finish()?;
                anyhow::Ok(())
            }
        });
        //节点
        let node_c = Node::new("[::]:0".parse()?, "节点", "节点描述", None)?;
        let peer_node_c = node_c
            .connect("[::1]:10270".parse()?, node_s.info().cert_der)
            .await?;
        assert_eq!(peer_node_c.info(), node_s.info());
        assert_eq!(
            peer_node_c
                .accept_uni()
                .await?
                .read_to_end(usize::MAX)
                .await?,
            *verify_msg
        );
        //中枢节点
        let node_h = Node::new("[::]:10271".parse()?, "中枢节点", "中枢节点描述", None)?;
        let peer_node_h = Pointer::new(None);
        tokio::spawn({
            let node_h = node_h.clone();
            let peer_node_h = peer_node_h.clone();
            async move {
                let peer_node = node_h.accept().await.await??;
                peer_node_h.set(Some(peer_node.clone()));
                anyhow::Ok(())
            }
        });
        //接入中枢节点
        node_c
            .access_hubnode("[::1]:10271".parse()?, node_h.info().cert_der)
            .await?;
        Ok(())
    }

    #[tokio::test]
    async fn dns_resolver() -> Result<()> {
        println!(
            "IP address: {:?}",
            AsyncResolver::tokio(ResolverConfig::cloudflare(), ResolverOpts::default())
                .lookup_ip("www.bilibili.com")
                .await?
                .iter()
                .next()
        );
        Ok(())
    }
}
