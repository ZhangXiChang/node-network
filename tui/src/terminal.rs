use std::{
    io::{stdout, Stdout},
    time::Duration,
};

use anyhow::Result;
use ratatui::{
    backend::CrosstermBackend,
    crossterm::{
        event, execute,
        terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    },
};

use crate::app::App;

pub struct Terminal {
    terminal: ratatui::Terminal<CrosstermBackend<Stdout>>,
}
impl Terminal {
    pub fn new() -> Result<Self> {
        execute!(stdout(), EnterAlternateScreen)?;
        enable_raw_mode()?;
        Ok(Self {
            terminal: ratatui::Terminal::new(CrosstermBackend::new(stdout()))?,
        })
    }
    pub async fn run(mut self) -> Result<()> {
        let app = App::new().await?;
        while app.is_loop() {
            self.terminal.draw(|frame| app.draw(frame))?;
            if event::poll(Duration::from_millis(16))? {
                app.handle_event(event::read()?)?;
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
