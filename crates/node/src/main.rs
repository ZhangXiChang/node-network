use std::{
    io::{stdout, Stdout},
    sync::Arc,
    time::Duration,
};

use anyhow::Result;
use node::Node;
use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Constraint, Layout},
    prelude::CrosstermBackend,
    Frame,
};
use tool_code::lock::Pointer;

const SERVER_CERT_DER: &[u8] = include_bytes!("../../../assets/server.cer");

#[derive(Clone)]
struct App {
    is_loop: Pointer<bool>,
    node: Node,
    root_layout: Arc<Layout>,
}
impl App {
    async fn new() -> Result<Self> {
        Ok(Self {
            is_loop: Pointer::new(true),
            node: Node::new("127.0.0.1:10270".parse()?, SERVER_CERT_DER.to_vec()).await?,
            root_layout: Arc::new(Layout::vertical([Constraint::Min(0)])),
        })
    }
    fn quit(&self) {
        *self.is_loop.lock() = false;
    }
    async fn start(&self) -> Result<()> {
        let hubnode_info_list = self.node.get_hubnode_info_list().await?;
        self.node
            .connect_hubnode(
                hubnode_info_list[0].ipv4_addr,
                hubnode_info_list[0].cert_der.clone(),
            )
            .await?;
        self.node.test_hubnode().await?;
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {
        let _root_layout_area = self.root_layout.split(frame.area());
    }
    fn event(&self, event: Event) -> Result<()> {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                if key.code == KeyCode::Esc {
                    self.quit();
                }
            }
        }
        Ok(())
    }
}

struct Terminal {
    terminal: ratatui::Terminal<CrosstermBackend<Stdout>>,
    app: App,
}
impl Terminal {
    async fn new() -> Result<Self> {
        execute!(stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;
        Ok(Self {
            terminal: ratatui::Terminal::new(CrosstermBackend::new(stdout()))?,
            app: App::new().await?,
        })
    }
    async fn run(&mut self) -> Result<()> {
        self.app.start().await?;
        while *self.app.is_loop.lock() {
            self.terminal.draw(|frame| self.app.draw(frame))?;
            if event::poll(Duration::from_millis(16))? {
                self.app.event(event::read()?)?;
            }
        }
        Ok(())
    }
}
impl Drop for Terminal {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(stdout(), LeaveAlternateScreen).unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    Terminal::new().await?.run().await?;
    Ok(())
}
