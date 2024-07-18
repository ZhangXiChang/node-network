use std::{io::stdout, time::Duration};

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
    Terminal,
};
use tool_code::lock::{Get, Pointer};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tui_textarea::{CursorMove, TextArea};

#[tokio::main]
async fn main() -> Result<()> {
    //初始化日志系统
    tracing_subscriber::registry()
        .with(tracing_subscriber::fmt::layer())
        .with(
            tracing_subscriber::filter::Targets::new()
                .with_targets(vec![("tui", tracing::Level::INFO)]),
        )
        .init();
    //初始化系统
    let system = System::new()?;
    system.connect_server().await.context("系统初始化失败")?;
    //初始化终端屏幕
    execute!(stdout(), EnterAlternateScreen)?;
    enable_raw_mode()?;
    let terminal_result = async {
        //初始化终端
        let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
        //设定控件布局
        let root_layout = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]);
        //初始化文本输入控件
        let textinput = Pointer::new(TextArea::default());
        {
            let mut textinput = textinput.lock();
            textinput.set_cursor_line_style(Style::new());
            textinput.set_block(Block::new().borders(Borders::ALL).title("输入"));
        }
        let is_loop = Pointer::new(true);
        while is_loop.get() {
            terminal.draw({
                let root_layout = root_layout.clone();
                let textinput = textinput.clone();
                move |frame| {
                    let root_layout_area = root_layout.split(frame.size());
                    frame.render_widget(textinput.get().widget(), root_layout_area[0]);
                }
            })?;
            if event::poll(Duration::from_millis(16))? {
                let event = event::read()?;
                tokio::spawn({
                    let is_loop = is_loop.clone();
                    let textinput = textinput.clone();
                    async move {
                        if let Event::Key(key) = event {
                            if key.kind == KeyEventKind::Press {
                                if key.code == KeyCode::Esc {
                                    is_loop.set(false);
                                }
                            }
                        }
                        {
                            let mut textinput = textinput.lock();
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
                    }
                });
            }
        }
        anyhow::Ok(())
    }
    .await;
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen)?;
    if let Err(err) = terminal_result {
        eprintln!("{:?}", err);
    }
    Ok(())
}
