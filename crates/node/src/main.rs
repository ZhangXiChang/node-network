use std::{
    io::{stdout, Stdout},
    sync::Arc,
    time::Duration,
};

use anyhow::Result;
use node::Node;
use ratatui::{
    crossterm::{
        event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Constraint, Layout},
    prelude::CrosstermBackend,
    style::{palette::tailwind, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame,
};
use tool_code::lock::Pointer;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tui_textarea::{CursorMove, TextArea};

#[derive(Clone)]
struct App<'a> {
    is_loop: Pointer<bool>,
    node: Node,
    root_layout: Arc<Layout>,
    hubnode_table: Pointer<Table<'a>>,
    hubnode_table_state: Pointer<TableState>,
    textinput: Pointer<TextArea<'a>>,
}
impl<'a> App<'a> {
    fn new() -> Result<Self> {
        Ok(Self {
            is_loop: Pointer::new(true),
            node: Node::new()?,
            root_layout: Arc::new(Layout::vertical([Constraint::Min(0)])),
            hubnode_table: Pointer::new(
                Table::new(
                    [Row::new(["1111", "2222", "3333"])],
                    [Constraint::Min(0), Constraint::Min(0), Constraint::Min(0)],
                )
                .header(
                    ["ID", "名称", "地址"]
                        .into_iter()
                        .map(Cell::from)
                        .collect::<Row>()
                        .style(Style::new().fg(tailwind::SLATE.c200))
                        .height(1),
                ),
            ),
            hubnode_table_state: Pointer::new(TableState::new()),
            textinput: Pointer::new({
                let mut a = TextArea::default();
                a.set_cursor_line_style(Style::new());
                a.set_block(Block::new().borders(Borders::ALL).title("输入"));
                a
            }),
        })
    }
    fn draw(&self, frame: &mut Frame) {
        let root_layout_area = self.root_layout.split(frame.size());
        //frame.render_widget(self.textinput.lock().widget(), root_layout_area[0]);
        frame.render_stateful_widget(
            self.hubnode_table.lock().clone(),
            root_layout_area[0],
            &mut *self.hubnode_table_state.lock(),
        );
    }
    fn event(&self, event: Event) -> Result<()> {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                if key.code == KeyCode::Esc {
                    *self.is_loop.lock() = false;
                }
            }
        }
        match event {
            Event::Key(KeyEvent {
                code: KeyCode::Enter,
                ..
            }) => {
                if !self.textinput.lock().is_empty() {
                    self.textinput.lock().move_cursor(CursorMove::Head);
                    self.textinput.lock().delete_line_by_end();
                }
            }
            _ => {
                self.textinput.lock().input(event);
            }
        }
        Ok(())
    }
}

struct Terminal<'a> {
    terminal: ratatui::Terminal<CrosstermBackend<Stdout>>,
    app: App<'a>,
}
impl<'a> Terminal<'a> {
    fn new() -> Result<Self> {
        execute!(stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;
        Ok(Self {
            terminal: ratatui::Terminal::new(CrosstermBackend::new(stdout()))?,
            app: App::new()?,
        })
    }
    fn run(&mut self) -> Result<()> {
        while *self.app.is_loop.lock() {
            self.terminal.draw(|frame| self.app.draw(frame))?;
            if event::poll(Duration::from_millis(16))? {
                self.app.event(event::read()?)?;
            }
        }
        Ok(())
    }
}
impl<'a> Drop for Terminal<'a> {
    fn drop(&mut self) {
        disable_raw_mode().unwrap();
        execute!(stdout(), LeaveAlternateScreen).unwrap();
    }
}

#[tokio::main]
async fn main() -> Result<()> {
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::filter::Targets::new()
                .with_targets(vec![("hubnode", tracing::Level::INFO)]),
        )
        .init();
    tracing::info!("日志系统初始化完成");
    Terminal::new()?.run()?;
    Ok(())
}
