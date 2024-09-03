use std::{
    io::{stdout, Stdout},
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
    style::{Color, Style},
    widgets::{Row, Table, TableState},
    Frame,
};
use tool_code::packet::NodeInfo;

const HUBNODE_CERT_DER: &[u8] = include_bytes!("../../../assets/hubnode.cer");

struct App {
    is_loop: bool,
    node: Node,
    node_info_list: Vec<NodeInfo>,
    node_info_table_state: TableState,
}
impl App {
    async fn new() -> Result<Self> {
        Ok(Self {
            is_loop: true,
            node: Node::new("127.0.0.1:10270".parse()?, HUBNODE_CERT_DER.to_vec()).await?,
            node_info_list: Vec::new(),
            node_info_table_state: TableState::new(),
        })
    }
    fn quit(&mut self) {
        self.is_loop = false;
    }
    async fn start(&mut self) -> Result<()> {
        self.node_info_list = self.node.get_node_info_list().await?;
        Ok(())
    }
    fn draw(&mut self, frame: &mut Frame) {
        let [node_info_table_area] = Layout::vertical([Constraint::Min(0)]).areas(frame.area());
        frame.render_stateful_widget(
            Table::new(
                self.node_info_list
                    .iter()
                    .map(|node_info| {
                        Row::new([node_info.name.clone(), node_info.description.clone()])
                    })
                    .collect::<Vec<_>>(),
                [Constraint::Min(0), Constraint::Min(0)],
            )
            .header(Row::new(["名称", "描述"]).style(Style::new().fg(Color::Gray))),
            node_info_table_area,
            &mut self.node_info_table_state,
        );
    }
    fn event(&mut self, event: Event) -> Result<()> {
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
        while self.app.is_loop {
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
