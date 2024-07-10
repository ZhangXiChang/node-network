pub mod app;

#[cfg(test)]
mod tests {
    use anyhow::{Context, Result};

    use crate::app::App;

    #[tokio::test]
    async fn main() -> Result<()> {
        let app = App::new().context("创建App失败")?;
        app.connect_server().await.context("连接服务器失败")?;
        app.get_user_star_hubnode_logo()
            .await
            .context("获取用户收藏中枢节点Logo失败")?;
        Ok(())
    }
}
