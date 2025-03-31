use std::{error::Error, fmt, process::Command, str};

#[derive(Debug)]
pub enum GitError {
    NotInstalled,
    ConfigUpdateError(String),
    ConfigReadError(String),
    CommandFailed(String),
    OutputParseError(String),
}

impl fmt::Display for GitError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            GitError::NotInstalled => write!(
                f,
                "Git is not installed. Please install Git to use this tool."
            ),
            GitError::ConfigUpdateError(err) => write!(f, "Failed to update Git config: {}", err),
            GitError::ConfigReadError(err) => write!(f, "Failed to read Git config: {}", err),
            GitError::CommandFailed(err) => write!(f, "Git command failed: {}", err),
            GitError::OutputParseError(err) => write!(f, "Failed to parse Git output: {}", err),
        }
    }
}

impl Error for GitError {}

pub fn verify_installed() -> Result<(), GitError> {
    if Command::new("git").output().is_err() {
        return Err(GitError::NotInstalled);
    }
    Ok(())
}

pub fn get_config(field: &str) -> Result<String, GitError> {
    let output = Command::new("git")
        .arg("config")
        .arg(field)
        .output()
        .map_err(|e| GitError::CommandFailed(e.to_string()))?;

    if !output.status.success() {
        let error_message = str::from_utf8(&output.stderr)
            .unwrap_or("Unknown error")
            .trim();
        return Err(GitError::ConfigReadError(format!(
            "Git config command failed for field '{}': {}",
            field, error_message
        )));
    }

    let stdout =
        str::from_utf8(&output.stdout).map_err(|e| GitError::OutputParseError(e.to_string()))?;

    let value = stdout.trim().to_string();
    if value.is_empty() {
        return Err(GitError::ConfigReadError(format!(
            "Git config field '{}' is not set",
            field
        )));
    }

    Ok(value)
}

pub fn set_config(field: &str, value: &str) -> Result<(), GitError> {
    Command::new("git")
        .args(["config", "--global", field, value])
        .status()
        .map_err(|e| GitError::ConfigUpdateError(e.to_string()))?;
    Ok(())
}
