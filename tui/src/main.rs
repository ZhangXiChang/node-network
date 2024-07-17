use std::{io::stdout, time::Duration};

use anyhow::{Context, Result};
use node_network::system::System;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event::{
            self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyEvent, KeyEventKind,
        },
        execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
    layout::{Constraint, Layout},
    style::Style,
    widgets::{Block, Borders},
    Terminal,
};
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};
use tui_textarea::TextArea;

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
    //初始化终端
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;
    enable_raw_mode()?;
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    //设定控件布局
    let root_layout = Layout::vertical([Constraint::Length(3), Constraint::Min(0)]);
    //初始化文本输入控件
    let mut textinput = TextArea::default();
    textinput.set_cursor_line_style(Style::new());
    textinput.set_block(Block::new().borders(Borders::ALL).title("输入"));
    loop {
        terminal.draw(|frame| {
            let root_layout_area = root_layout.split(frame.size());
            frame.render_widget(textinput.widget(), root_layout_area[0]);
        })?;
        if event::poll(Duration::from_millis(16))? {
            let event = event::read()?;
            if let Event::Key(key) = event {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Esc {
                        break;
                    }
                }
            }
            match event {
                Event::Key(KeyEvent {
                    code: KeyCode::Enter,
                    ..
                }) => (),
                _ => {
                    textinput.input(event);
                }
            }
        }
    }
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    Ok(())
}
