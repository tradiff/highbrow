use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub browsers: Vec<BrowserConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BrowserConfig {
    pub label: String,
    pub command: String,
    pub icon_name: String,
    pub patterns: Option<Vec<String>>,
}

#[derive(Debug)]
pub enum ConfigError {
    FileNotFound(PathBuf),
    ParseError(String),
    ValidationError(String),
}

impl fmt::Display for ConfigError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            ConfigError::FileNotFound(path) => write!(f, "Config file not found: {:?}", path),
            ConfigError::ParseError(msg) => write!(f, "Failed to parse config: {}", msg),
            ConfigError::ValidationError(msg) => write!(f, "Config validation error: {}", msg),
        }
    }
}

impl Error for ConfigError {}

pub fn load_config() -> Result<Config, ConfigError> {
    let config_path = get_config_path();

    let config_str =
        fs::read_to_string(&config_path).map_err(|_| ConfigError::FileNotFound(config_path))?;

    let config: Config =
        toml::from_str(&config_str).map_err(|e| ConfigError::ParseError(e.to_string()))?;

    validate_config(&config)?;
    Ok(config)
}

pub fn load_config_or_default() -> Config {
    match load_config() {
        Ok(config) => config,
        Err(_) => create_default_config(),
    }
}

pub fn get_config_path() -> PathBuf {
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    [home_dir.as_str(), ".config", "highbrow.toml"]
        .iter()
        .collect()
}

fn create_default_config() -> Config {
    Config {
        browsers: vec![
            BrowserConfig {
                label: "_Firefox".to_string(),
                command: "firefox".to_string(),
                icon_name: "firefox".to_string(),
                patterns: None,
            },
            BrowserConfig {
                label: "_Chrome".to_string(),
                command: "google-chrome".to_string(),
                icon_name: "google-chrome".to_string(),
                patterns: None,
            },
        ],
    }
}

fn validate_config(config: &Config) -> Result<(), ConfigError> {
    if config.browsers.is_empty() {
        return Err(ConfigError::ValidationError(
            "No browsers configured".to_string(),
        ));
    }

    for browser in &config.browsers {
        if browser.label.trim().is_empty() {
            return Err(ConfigError::ValidationError(
                "Browser label cannot be empty".to_string(),
            ));
        }
        if browser.command.trim().is_empty() {
            return Err(ConfigError::ValidationError(
                "Browser command cannot be empty".to_string(),
            ));
        }
    }

    Ok(())
}
