use anyhow::Result;
use terminal::Terminal;

mod app;
mod terminal;

#[tokio::main]
async fn main() -> Result<()> {
    Terminal::new()?.run().await?;
    Ok(())
}
