use std::{error::Error, fmt::Display, process::Command};

use serde::{Deserialize, Serialize};

#[derive(Debug)]
pub enum LateremError {
    InvalidArgument,
}

impl Error for LateremError {}

impl Display for LateremError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LateremError::InvalidArgument => write!(f, "invalid argument"),
        }
    }
}

#[derive(Serialize, Deserialize)]
pub struct DefaultConfig {
    branch: String,
    stash_files: bool,
    detach_container: bool,
}

impl Default for DefaultConfig {
    fn default() -> Self {
        Self {
            branch: "main".to_string(),
            stash_files: true,
            detach_container: true,
        }
    }
}

pub enum Target {
    Docker,
    Repository,
}

pub enum Action {
    /// restart docker or reset repository
    Reset,

    /// docker compose down
    Down,

    /// docker compose up -d
    Up,

    /// git commit -m "$1" && git push origin $branch
    Commit,

    /// git push oringin $branch
    Push,
}

pub struct Config {
    pub path: String,
    pub target: Target,
    pub action: Action,
    pub defaults: Option<Box<DefaultConfig>>,
    pub arguments: Vec<String>,
}

impl Action {
    pub fn run(config: &Config) -> Result<(), LateremError> {
        match config.target {
            Target::Docker => match &config.action {
                Action::Reset => {
                    Command::new("docker")
                        .args(["compose", "down"])
                        .status()
                        .expect("Didn't manage to take the instance down");

                    Command::new("docker")
                        .args(["compose", "up", "-d"])
                        .status()
                        .expect("Didn't manage to create a new instance");

                    Ok(())
                }
                Action::Down => {
                    Command::new("docker")
                        .args(["compose", "down"])
                        .status()
                        .expect("Didn't manage to take the instance down");

                    Ok(())
                }
                Action::Up => {
                    Command::new("docker")
                        .args(["compose", "up", "-d"])
                        .status()
                        .expect("Didn't manage to create a new instance");

                    Ok(())
                }
                _ => Err(LateremError::InvalidArgument),
            },
            Target::Repository => {
                let defaults = config
                    .defaults
                    .as_ref()
                    .expect("Unable to retrieve the default configuration");
                let args: Vec<&str> = config.arguments.iter().map(|item| item.as_str()).collect();

                let branch = Command::new("git")
                    .args(["branch", "--show-current"])
                    .output()
                    .expect("Didn't manage to retrieve the active branch");
                let branch = String::from_utf8(branch.stdout)
                    .expect("Unable to convert the stdout response");
                let branch = branch.replace("\n", "");

                match &config.action {
                    Action::Reset => {
                        Command::new("git")
                            .args(["add", "."])
                            .status()
                            .expect("Couldn't stage the changed files");

                        Command::new("git")
                            .args(["stash"])
                            .status()
                            .expect("Couldn't stash the changes");

                        Command::new("git")
                            .args(["checkout", &defaults.branch])
                            .status()
                            .expect("Unable to go back to the main branch");

                        Command::new("git")
                            .args(["checkout", &branch])
                            .status()
                            .expect("Unable to checkout to the old branch");

                        Ok(())
                    }
                    Action::Commit => {
                        Command::new("git")
                            .args([["commit", "-m"].to_vec(), args].concat())
                            .status()
                            .expect("Unable to commit files");

                        Ok(())
                    }
                    Action::Push => {
                        Command::new("git")
                            .args(["push", "origin", branch.as_str()])
                            .status()
                            .expect("An error occurred when committing files");

                        Ok(())
                    }
                    _ => Err(LateremError::InvalidArgument),
                }
            }
        }
    }
}

impl Config {
    pub fn setup(&mut self) {
        // TODO: prompt user to create first config file
        // let config = std::fs::read(&self.path);

        let defaults = DefaultConfig::default();
        self.defaults = Some(Box::new(defaults));

        let response = Action::run(self);

        match response {
            Err(message) => println!("An error ocurred: {}", message),
            _ => {}
        }
    }
}
