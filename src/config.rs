use serde::{Deserialize, Serialize};
use std::{env, error::Error, fmt, fs, path::Path};
use toml;

#[derive(Debug)]
pub enum ConfigError {
    HomeNotSet,
    InvalidPath(String),
    InvalidUserIndex(usize),
    SerializationFailed(String),
    DeserializationFailed(String),
    DirectoryCreationFailed(String),
    WriteError(String),
    ReadError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ConfigError::HomeNotSet => write!(f, "HOME environment variable not set"),
            ConfigError::InvalidPath(path) => write!(f, "Invalid config path: {}", path),
            ConfigError::InvalidUserIndex(index) => {
                write!(f, "User not found at index: {}", index)
            }
            ConfigError::SerializationFailed(err) => {
                write!(f, "Failed to serialize config: {}", err)
            }
            ConfigError::DeserializationFailed(err) => {
                write!(f, "Failed to deserialize config: {}", err)
            }
            ConfigError::DirectoryCreationFailed(err) => {
                write!(f, "Failed to create config directory: {}", err)
            }
            ConfigError::WriteError(err) => write!(f, "Failed to write config file: {}", err),
            ConfigError::ReadError(err) => write!(f, "Failed to read config file: {}", err),
        }
    }
}

impl Error for ConfigError {}

#[derive(Serialize, Deserialize)]
pub struct Config {
    pub users: Vec<User>,
    pub previous_user: usize,
    pub current_user: usize,
}

#[derive(Serialize, Deserialize, Clone)]
pub struct User {
    pub name: String,
    pub email: String,
}

pub fn init(user: User) -> Result<Config, ConfigError> {
    let config_path = get_path()?;
    if Path::new(&config_path).exists() {
        return load_config();
    }

    let config = Config {
        current_user: 0,
        previous_user: 0,
        users: vec![user],
    };

    let toml_content =
        toml::to_string(&config).map_err(|e| ConfigError::SerializationFailed(e.to_string()))?;

    if let Some(parent) = Path::new(&config_path).parent() {
        fs::create_dir_all(parent)
            .map_err(|e| ConfigError::DirectoryCreationFailed(e.to_string()))?;
    } else {
        return Err(ConfigError::InvalidPath(config_path));
    }

    fs::write(&config_path, toml_content).map_err(|e| ConfigError::WriteError(e.to_string()))?;

    return load_config();
}

pub fn get_path() -> Result<String, ConfigError> {
    match env::var("HOME") {
        Ok(home) => Ok(format!("{}/.config/gus.toml", home)),
        Err(_) => Err(ConfigError::HomeNotSet),
    }
}

fn load_config() -> Result<Config, ConfigError> {
    let config_path = get_path()?;

    let contents =
        fs::read_to_string(config_path).map_err(|e| ConfigError::ReadError(e.to_string()))?;

    toml::from_str(&contents).map_err(|e| ConfigError::DeserializationFailed(e.to_string()))
}

pub fn update<F>(updater: F) -> Result<(), ConfigError>
where
    F: FnOnce(&mut Config),
{
    let mut config = load_config()?;
    updater(&mut config);

    let toml_content =
        toml::to_string(&config).map_err(|e| ConfigError::SerializationFailed(e.to_string()))?;

    let path = get_path()?;
    fs::write(path, toml_content).map_err(|e| ConfigError::WriteError(e.to_string()))?;

    Ok(())
}

pub fn get_user(index: usize, config: &Config) -> Result<User, ConfigError> {
    if let Some(user) = config.users.get(index) {
        Ok(user.clone())
    } else {
        Err(ConfigError::InvalidUserIndex(index))
    }
}
