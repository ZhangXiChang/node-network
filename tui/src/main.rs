use std::{
    fmt::{Debug, Display},
    str::FromStr,
};

use anyhow::{Context, Result};
use dialoguer::{theme::ColorfulTheme, BasicHistory, Completion, Input};
use node_network::system::System;
use tracing_subscriber::{layer::SubscriberExt, util::SubscriberInitExt};

enum InputCommand {
    Help,
    Quit,
    HNTable,
    Null,
}
impl Display for InputCommand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            InputCommand::Help => write!(f, "/help"),
            InputCommand::Quit => write!(f, "/quit"),
            InputCommand::HNTable => write!(f, "/hntable"),
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
        } else if s == Self::HNTable.to_string() {
            Self::HNTable
        } else {
            Self::Null
        })
    }
}

struct InputCommandCompletion {
    options: Vec<(String, String)>,
}
impl InputCommandCompletion {
    fn new() -> Self {
        Self {
            options: vec![
                (InputCommand::Help.to_string(), "显示帮助信息".to_string()),
                (InputCommand::Quit.to_string(), "退出程序".to_string()),
                (
                    InputCommand::HNTable.to_string(),
                    "显示节点列表".to_string(),
                ),
            ],
        }
    }
}
impl Completion for InputCommandCompletion {
    fn get(&self, input: &str) -> Option<String> {
        self.options
            .iter()
            .find_map(|(cmd_str, _)| cmd_str.starts_with(input).then_some(cmd_str))
            .cloned()
    }
}
impl Debug for InputCommandCompletion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_map()
            .entries(self.options.iter().map(|(cmd_str, desc)| (cmd_str, desc)))
            .finish()
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
    system
        .connect_server()
        .await
        .context("连接节点服务端失败")?;
    let mut history = BasicHistory::new();
    let input_command_completion = InputCommandCompletion::new();
    println!("{}", include_str!("../../assets/tui/welcome.txt"));
    loop {
        let input_string = Input::<String>::with_theme(&ColorfulTheme::default())
            .with_prompt("主菜单")
            .completion_with(&input_command_completion)
            .history_with(&mut history)
            .interact_text()?;
        match input_string.parse::<InputCommand>()? {
            InputCommand::Help => {
                println!(
                    "\n主菜单是什么你懂的！\n\n命令: {:#?}\n",
                    input_command_completion
                )
            }
            InputCommand::Quit => break,
            InputCommand::HNTable => system
                .get_hubnode_table()
                .await?
                .iter()
                .for_each(|hubnode_table| println!("{}", hubnode_table.name)),
            InputCommand::Null => println!("{} 不是一个有效的输入", input_string),
        }
    }
    Ok(())
}
