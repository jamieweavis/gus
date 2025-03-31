use std::{env, error::Error, fmt, process::Command};

use crate::config;
use crate::git;

#[derive(Debug)]
pub enum CliError {
    InvalidUsage(String),
    ParseError(String),
    GitError(String),
    ConfigError(String),
    CommandError(String),
}

impl fmt::Display for CliError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            CliError::InvalidUsage(msg) => write!(f, "Invalid usage: {}", msg),
            CliError::ParseError(msg) => write!(f, "Parse error: {}", msg),
            CliError::GitError(msg) => write!(f, "Git error: {}", msg),
            CliError::ConfigError(msg) => write!(f, "Config error: {}", msg),
            CliError::CommandError(msg) => write!(f, "Command error: {}", msg),
        }
    }
}

impl Error for CliError {}

impl From<git::GitError> for CliError {
    fn from(error: git::GitError) -> Self {
        CliError::GitError(error.to_string())
    }
}

impl From<config::ConfigError> for CliError {
    fn from(error: config::ConfigError) -> Self {
        CliError::ConfigError(error.to_string())
    }
}

pub fn parse(config: &config::Config) -> Result<(), CliError> {
    let args: Vec<String> = env::args().collect();

    if args.len() == 1 {
        print_help();
        return Ok(());
    }

    if args.len() > 2 {
        return Err(CliError::InvalidUsage(
            "Too many arguments provided".to_string(),
        ));
    }

    let command = &args[1];
    match command.as_str() {
        "config" => edit_config(),
        "ls" | "list" => list_users(config),
        "-" => Ok(switch_user(config.previous_user, config)?),
        _ => {
            if let Ok(index) = command.parse::<usize>() {
                Ok(switch_user(index, config)?)
            } else {
                Err(CliError::ParseError(format!(
                    "'{}' is not a valid user ID or command",
                    command
                )))
            }
        }
    }
}

pub fn switch_user(index: usize, config: &config::Config) -> Result<(), CliError> {
    let user = config::get_user(index, config)?;

    git::set_config("user.name", &user.name)?;
    git::set_config("user.email", &user.email)?;

    let previous_user = config.current_user;
    if previous_user != index {
        config::update(|config| {
            config.current_user = index;
            config.previous_user = previous_user;
        })?;
    }

    println!("Switched git user to: {} <{}>", user.name, user.email);
    Ok(())
}

pub fn list_users(config: &config::Config) -> Result<(), CliError> {
    for (i, user) in config.users.iter().enumerate() {
        let marker = if i == config.current_user { "*" } else { " " };
        println!("{} {}: {} <{}>", marker, i, user.name, user.email);
    }
    Ok(())
}

fn edit_config() -> Result<(), CliError> {
    let config_path = config::get_path()?;

    let editor = match env::var("EDITOR") {
        Ok(editor) => editor,
        Err(_) => "vi".to_string(),
    };

    Command::new(editor)
        .arg(&config_path)
        .status()
        .map_err(|e| CliError::CommandError(format!("Failed to open editor: {}", e)))?;

    Ok(())
}

pub fn print_help() {
    println!(
        "gus {} ({})",
        env!("CARGO_PKG_VERSION"),
        env!("CARGO_PKG_REPOSITORY")
    );
    println!("");
    println!(env!("CARGO_PKG_DESCRIPTION"));
    println!("");
    println!("Usage: gus <command>");
    println!("");
    println!("COMMANDS");
    println!("  <id>       Switch to user with the provided ID");
    println!("  -          Switch to the previous user");
    println!("  list, ls   List users and their IDs");
    println!("  config     Open `~/.config/gus.toml` in your $EDITOR");
}
