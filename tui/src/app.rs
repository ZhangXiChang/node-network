use std::sync::Arc;

use anyhow::{Context, Result};
use node_network::system::System;
use ratatui::{
    crossterm::event::{Event, KeyCode, KeyEvent, KeyEventKind},
    layout::{Constraint, Layout},
    style::{palette::tailwind, Style},
    widgets::{Block, Borders, Cell, Row, Table, TableState},
    Frame,
};
use tool_code::lock::Pointer;
use tui_textarea::{CursorMove, TextArea};

#[derive(Clone)]
pub struct App<'a> {
    is_loop: Pointer<bool>,
    system: System,
    root_layout: Arc<Layout>,
    hubnode_table: Pointer<Table<'a>>,
    hubnode_table_state: Pointer<TableState>,
    textinput: Pointer<TextArea<'a>>,
}
impl<'a> App<'a> {
    pub async fn new() -> Result<Self> {
        let system = System::new()?;
        system.connect_server().await.context("系统初始化失败")?;
        Ok(Self {
            is_loop: Pointer::new(true),
            system,
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
    pub fn is_loop(&self) -> bool {
        self.is_loop.lock().clone()
    }
    pub fn draw(&self, frame: &mut Frame) {
        //TODO 消除System警告
        let _ = self.system;
        let root_layout_area = self.root_layout.split(frame.size());
        //frame.render_widget(self.textinput.lock().widget(), root_layout_area[0]);
        frame.render_stateful_widget(
            self.hubnode_table.lock().clone(),
            root_layout_area[0],
            &mut *self.hubnode_table_state.lock(),
        );
    }
    pub fn handle_event(&self, event: Event) -> Result<()> {
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
