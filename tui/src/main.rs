use std::{fmt::Display, str::FromStr};

use anyhow::Result;
use dialoguer::{theme::ColorfulTheme, BasicHistory, Completion, Input};
use node_network::system::System;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

enum InputCommand {
    Help,
    Quit,
    Null,
}
impl Display for InputCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputCommand::Help => write!(f, "/help"),
            InputCommand::Quit => write!(f, "/quit"),
            InputCommand::Null => write!(f, ""),
        }
    }
}
impl FromStr for InputCommand {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> std::result::Result<Self, Self::Err> {
        Ok(if s == Self::Help.to_string() {
            Self::Help
        } else if s == Self::Quit.to_string() {
            Self::Quit
        } else {
            Self::Null
        })
    }
}
struct InputCommandCompletion {
    options: Vec<String>,
}
impl InputCommandCompletion {
    fn new() -> Self {
        Self {
            options: vec![
                InputCommand::Help.to_string(),
                InputCommand::Quit.to_string(),
            ],
        }
    }
}
impl Completion for InputCommandCompletion {
    fn get(&self, input: &str) -> Option<String> {
        self.options
            .iter()
            .find(|option| option.starts_with(input))
            .cloned()
    }
}

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
    tracing::info!("初始化系统中，请稍候...");
    let system = System::new()?;
    system.connect_server().await?;
    let mut history = BasicHistory::new();
    println!("{}", include_str!("../../assets/tui/welcome.txt"));
    loop {
        let input_string = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("主菜单")
            .completion_with(&InputCommandCompletion::new())
            .history_with(&mut history)
            .interact_text()?;
        match input_string.parse::<InputCommand>()? {
            InputCommand::Help => {
                println!("{}", include_str!("../../assets/tui/main-menu-help.txt"))
            }
            InputCommand::Quit => break,
            InputCommand::Null => println!("{} 不是一个有效的输入", input_string),
        }
    }
    Ok(())
}
