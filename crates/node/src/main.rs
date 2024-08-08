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
    style::{palette::tailwind, Style},
    widgets::{Cell, Row, Table, TableState},
    Frame,
};
use server::HubNodeInfo;
use tool_code::lock::Pointer;

const SERVER_CERT_DER: &[u8] = include_bytes!("../../../assets/server.cer");

#[derive(Clone)]
struct App {
    is_loop: Pointer<bool>,
    node: Node,
    root_layout: Arc<Layout>,
    hubnode_list: Pointer<Vec<HubNodeInfo>>,
    hubnode_table_state: Pointer<TableState>,
}
impl App {
    async fn new() -> Result<Self> {
        Ok(Self {
            is_loop: Pointer::new(true),
            node: Node::new("127.0.0.1:10270".parse()?, SERVER_CERT_DER.to_vec()).await?,
            root_layout: Arc::new(Layout::vertical([Constraint::Min(0)])),
            hubnode_list: Pointer::new(Vec::new()),
            hubnode_table_state: Pointer::new(TableState::new()),
        })
    }
    fn quit(&self) {
        *self.is_loop.lock() = false;
    }
    async fn start(&self) -> Result<()> {
        *self.hubnode_list.lock() = self.node.get_hubnode_info_list().await?;
        Ok(())
    }
    fn draw(&self, frame: &mut Frame) {
        let root_layout_area = self.root_layout.split(frame.area());
        frame.render_stateful_widget(
            Table::new(
                self.hubnode_list.lock().iter().map(|hubnode_info| {
                    Row::new([hubnode_info.name.clone(), hubnode_info.description.clone()])
                }),
                [Constraint::Min(0), Constraint::Min(0)],
            )
            .header(
                ["名称", "描述"]
                    .into_iter()
                    .map(Cell::from)
                    .collect::<Row>()
                    .style(Style::new().fg(tailwind::SLATE.c200))
                    .height(1),
            ),
            root_layout_area[0],
            &mut *self.hubnode_table_state.lock(),
        );
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
