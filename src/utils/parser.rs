use clap::Parser;

use super::entities::{Action, Config, Target};

#[derive(Parser)]
#[command(version)]
pub struct Cli {
    /// TODO: Configure the behaviour of the cli
    #[arg(short, long)]
    config: Option<String>,

    /// The target of the action, e.g: `docker` or `repository`
    target: String,

    /// Select the target action, e.g: `commit`, `reset`, `down`, and `up`
    action: Option<String>,

    /// Arguments for the base cli command
    #[arg(short, long = "args")]
    arguments: Vec<String>,
}

pub fn parse() -> Config {
    let cli = Cli::parse();

    let cli_target = cli.target;
    let cli_action = cli.action.unwrap_or(String::from("reset"));
    let path = cli
        .config
        .unwrap_or(String::from("$HOME/.config/laterem/config.json"));

    let target = match cli_target.as_str() {
        "d" | "docker" => Target::Docker,
        "r" | "repository" => Target::Repository,
        _ => Target::Repository,
    };

    let action = match cli_action.as_str() {
        "c" | "commit" => Action::Commit,
        "r" | "reset" => Action::Reset,
        "d" | "down" => Action::Down,
        "u" | "up" => Action::Up,
        _ => Action::Reset,
    };

    Config {
        path,
        target,
        action,
        defaults: None,
        arguments: cli.arguments,
    }
}
