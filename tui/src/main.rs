use std::{
    io::{stdout, Stdout},
    sync::Arc,
    time::Duration,
};

use anyhow::{Context, Result};
use node_network::system::System;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{self, Event, KeyCode, KeyEvent, KeyEventKind},
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Constraint, Layout},
    style::Style,
    widgets::{Block, Borders},
    Frame,
};
use tool_code::lock::{Get, Pointer};
use tui_textarea::{CursorMove, TextArea};

#[derive(Clone)]
struct TerminalApp<'a> {
    is_loop: Pointer<bool>,
    system: System,
    root_layout: Arc<Layout>,
    textinput: Pointer<TextArea<'a>>,
}
impl<'a> TerminalApp<'a> {
    async fn new() -> Result<Self> {
        Ok(Self {
            is_loop: Pointer::new(true),
            system: {
                let a = System::new()?;
                a.connect_server().await.context("系统初始化失败")?;
                a
            },
            root_layout: Arc::new(Layout::vertical([
                Constraint::Length(3),
                Constraint::Min(0),
            ])),
            textinput: {
                let a = Pointer::new(TextArea::default());
                {
                    let mut a = a.lock();
                    a.set_cursor_line_style(Style::new());
                    a.set_block(Block::new().borders(Borders::ALL).title("输入"));
                }
                a
            },
        })
    }
    fn draw(&self, frame: &mut Frame) {
        let root_layout_area = self.root_layout.split(frame.size());
        frame.render_widget(self.textinput.get().widget(), root_layout_area[0]);
    }
    fn handle_event(&self, event: Event) -> Result<()> {
        if let Event::Key(key) = event {
            if key.kind == KeyEventKind::Press {
                if key.code == KeyCode::Esc {
                    self.is_loop.set(false);
                }
            }
        }
        {
            let mut textinput = self.textinput.lock();
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    ..
                }) => {
                    if !textinput.is_empty() {
                        textinput.move_cursor(CursorMove::Head);
                        textinput.delete_line_by_end();
                    }
                }
                _ => {
                    textinput.input(event);
                }
            }
        }
        Ok(())
    }
}

struct Terminal {
    terminal: Pointer<ratatui::Terminal<CrosstermBackend<Stdout>>>,
}
impl Terminal {
    fn new() -> Result<Self> {
        execute!(stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;
        Ok(Self {
            terminal: Pointer::new(ratatui::Terminal::new(CrosstermBackend::new(stdout()))?),
        })
    }
    async fn run(self) -> Result<()> {
        let tapp = TerminalApp::new().await?;
        while tapp.is_loop.get() {
            {
                let tapp = tapp.clone();
                self.terminal.lock().draw(move |frame| {
                    tapp.draw(frame);
                })?;
            }
            if event::poll(Duration::from_millis(16))? {
                tapp.handle_event(event::read()?)?;
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
    Terminal::new()?.run().await?;
    Ok(())
}
