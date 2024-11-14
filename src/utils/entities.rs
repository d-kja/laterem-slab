use crossterm::style::Stylize;
use serde::{Deserialize, Serialize};
use std::{
    error::Error,
    fmt::Display,
    io::{Read, Write},
    process::{Command, Stdio},
};

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
        let mut git = Command::new("git")
            .args(["remote", "show", "origin"])
            .stdout(Stdio::piped())
            .spawn()
            .expect("Unable to spawn GIT instance");

        let mut sed = Command::new("sed")
            .args(["-n", "/HEAD branch/s/.*: //p"])
            .stdin(Stdio::piped())
            .stdout(Stdio::piped())
            .spawn()
            .expect("Unable to spawn SED instance");

        if let Some(ref mut git_stdout) = git.stdout {
            if let Some(ref mut stdin) = sed.stdin {
                let mut buffer: Vec<u8> = Vec::new();

                git_stdout
                    .read_to_end(&mut buffer)
                    .expect("Unable to read STDOUT from GIT instance");
                stdin
                    .write_all(&buffer)
                    .expect("Unable to write to STDIN using SED instance");
            }
        }

        let _ = git.wait().unwrap();
        let origin = sed
            .wait_with_output()
            .expect("Unable to retrieve STDOUT from SED instance")
            .stdout;
        let origin = String::from_utf8(origin)
            .expect("Unable to convert origin buffer into a utf8 string")
            .replace("\n", "");

        Self {
            branch: origin,
            stash_files: true,
            detach_container: true,
        }
    }
}

pub enum Target {
    Docker,
    Repository,
}

impl Display for Target {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Target::Docker => write!(f, "docker"),
            Target::Repository => write!(f, "repository"),
        }
    }
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
    /// git push origin $branch
    Push,
    /// git pull origin $branch
    Pull,
}

impl Display for Action {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Action::Reset => write!(f, "reset"),
            Action::Down => write!(f, "down"),
            Action::Up => write!(f, "up"),
            Action::Commit => write!(f, "commit"),
            Action::Push => write!(f, "push"),
            Action::Pull => write!(f, "pull"),
        }
    }
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
        println!(
            "{}\t\n",
            " RUNNING ACTIONS ".on_dark_magenta().white().bold()
        );

        match config.target {
            Target::Docker => match &config.action {
                Action::Reset => {
                    println!(
                        "{}{}{}{}",
                        "Taking instance down".dim(),
                        ".".rapid_blink(),
                        ".".rapid_blink(),
                        ".".dim(),
                    );
                    Command::new("docker")
                        .args(["compose", "down"])
                        .status()
                        .expect("Didn't manage to take the instance down");

                    println!(
                        "\t\n{}{}{}{}",
                        "Launching a new instance".dim(),
                        ".".rapid_blink(),
                        ".".rapid_blink(),
                        ".".dim(),
                    );
                    Command::new("docker")
                        .args(["compose", "up", "-d"])
                        .status()
                        .expect("Didn't manage to create a new instance");

                    Ok(())
                }
                Action::Down => {
                    println!(
                        "{}{}{}{}",
                        "Taking instance down".dim(),
                        ".".rapid_blink(),
                        ".".rapid_blink(),
                        ".".dim(),
                    );
                    Command::new("docker")
                        .args(["compose", "down"])
                        .status()
                        .expect("Didn't manage to take the instance down");

                    Ok(())
                }
                Action::Up => {
                    println!(
                        "{}{}{}{}",
                        "Launching a new instance".dim(),
                        ".".rapid_blink(),
                        ".".rapid_blink(),
                        ".".dim(),
                    );
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
                        println!(
                            "{}{}{}{}",
                            "Staging files".dim(),
                            ".".rapid_blink(),
                            ".".rapid_blink(),
                            ".".dim(),
                        );
                        Command::new("git")
                            .args(["add", "."])
                            .status()
                            .expect("Couldn't stage the changed files");

                        println!(
                            "\t\n{}{}{}{}",
                            "Stashing staged files".dim(),
                            ".".rapid_blink(),
                            ".".rapid_blink(),
                            ".".dim(),
                        );
                        Command::new("git")
                            .args(["stash"])
                            .status()
                            .expect("Couldn't stash the changes");

                        println!(
                            "\t\n{} {}{}{}{}",
                            "Checking out to".dim(),
                            defaults.branch.clone().magenta(),
                            ".".rapid_blink(),
                            ".".rapid_blink(),
                            ".".dim(),
                        );
                        Command::new("git")
                            .args(["checkout", &defaults.branch])
                            .status()
                            .expect("Unable to go back to the main branch");

                        println!(
                            "\t\n{} {} {}{}{}",
                            "Pulling changes from".dim(),
                            defaults.branch.clone().magenta(),
                            ".".rapid_blink(),
                            ".".rapid_blink(),
                            ".".dim(),
                        );
                        Command::new("git")
                            .args(["pull", "origin", &defaults.branch])
                            .status()
                            .expect("Unable to pull the updates");

                        println!(
                            "\t\n{} {}{}{}{}",
                            "Going back to original branch".dim(),
                            branch.clone().magenta(),
                            ".".rapid_blink(),
                            ".".rapid_blink(),
                            ".".dim(),
                        );
                        Command::new("git")
                            .args(["checkout", &branch])
                            .status()
                            .expect("Unable to checkout to the old branch");

                        println!(
                            "\t\n{}{}{}{}",
                            "Popping stash".dim(),
                            ".".rapid_blink(),
                            ".".rapid_blink(),
                            ".".dim(),
                        );
                        Command::new("git")
                            .args(["stash", "pop"])
                            .status()
                            .expect("Couldn't pop the stash");

                        Ok(())
                    }
                    Action::Commit => {
                        if args.is_empty() {
                            return Err(LateremError::InvalidArgument);
                        }

                        println!(
                            "{} {}{}{}{}",
                            "Committing staged changes to".dim(),
                            branch.clone().magenta(),
                            ".".rapid_blink(),
                            ".".rapid_blink(),
                            ".".dim(),
                        );
                        Command::new("git")
                            .args([["commit", "-m"].to_vec(), args].concat())
                            .status()
                            .expect("Unable to commit files");

                        Ok(())
                    }
                    Action::Push => {
                        println!(
                            "{} {}{}{}{}",
                            "Pushing committed changes to".dim(),
                            branch.clone().magenta(),
                            ".".rapid_blink(),
                            ".".rapid_blink(),
                            ".".dim(),
                        );
                        Command::new("git")
                            .args(["push", "origin", branch.as_str()])
                            .status()
                            .expect("Unable to commit the files");

                        Ok(())
                    }
                    Action::Pull => {
                        println!(
                            "{}{}{}{}",
                            "Staging changes".dim(),
                            ".".rapid_blink(),
                            ".".rapid_blink(),
                            ".".dim(),
                        );
                        Command::new("git")
                            .args(["add", "*"])
                            .status()
                            .expect("Unable to stage the files");

                        println!(
                            "\t\n{}{}{}{}",
                            "Stashing staged changes".dim(),
                            ".".rapid_blink(),
                            ".".rapid_blink(),
                            ".".dim(),
                        );
                        Command::new("git")
                            .args(["stash"])
                            .status()
                            .expect("Unable to stash the staged files");

                        println!(
                            "\t\n{} {}{}{}{}",
                            "Pulling changes from".dim(),
                            branch.clone().magenta(),
                            ".".rapid_blink(),
                            ".".rapid_blink(),
                            ".".dim(),
                        );
                        Command::new("git")
                            .args(["pull", "origin", &branch])
                            .status()
                            .expect("Unable to pull the updates");

                        println!(
                            "\t\n{}{}{}{}",
                            "Popping stash".dim(),
                            ".".rapid_blink(),
                            ".".rapid_blink(),
                            ".".dim(),
                        );
                        Command::new("git")
                            .args(["stash", "pop"])
                            .status()
                            .expect("Unable to pop the stash");

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
            Ok(()) => {
                println!(
                    "\t\n{}\t\n",
                    " THE ACTION RAN SUCCESSFULLY "
                        .on_dark_magenta()
                        .white()
                        .bold(),
                );
            }
            Err(message) => {
                println!("\t\n{}\t\n", " ERROR OUTPUT ".on_dark_red().white().bold(),);

                println!(
                    " {} {}\t\n",
                    "An error ocurred:".slow_blink().bold(),
                    message.to_string().slow_blink().underlined(),
                );
            }
        }
    }
}
