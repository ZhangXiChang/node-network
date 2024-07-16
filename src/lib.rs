pub mod system;

#[cfg(test)]
mod tests {
    use anyhow::{Context, Result};

    use crate::system::System;

    #[tokio::test]
    async fn main() -> Result<()> {
        let system = System::new().context("创建App失败")?;
        system.connect_server().await.context("连接服务器失败")?;
        system
            .get_user_star_hubnode_logo()
            .await
            .context("获取用户收藏中枢节点Logo失败")?;
        Ok(())
    }
}
