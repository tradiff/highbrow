use regex::RegexBuilder;
use serde::Deserialize;
use std::error::Error;
use std::fmt;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub browsers: Vec<BrowserConfig>,
    pub default_browser: Option<String>, // Label of the default browser
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
        default_browser: None,
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

    // Validate default browser if specified
    if let Some(ref default_label) = config.default_browser {
        let default_exists = config.browsers.iter().any(|b| b.label == *default_label);
        if !default_exists {
            return Err(ConfigError::ValidationError(format!(
                "Default browser '{}' not found in browsers list",
                default_label
            )));
        }
    }

    Ok(())
}

// Find the first browser that matches the given URL based on configured patterns
// If no pattern matches and a default browser is configured, return the default browser
pub fn find_browser_for_url(url: &str, config: &Config) -> Option<BrowserConfig> {
    // First, try to find a browser with matching patterns
    let pattern_match = config
        .browsers
        .iter()
        .find(|b| {
            b.patterns.as_ref().map_or(false, |pats| {
                pats.iter().any(|pat| {
                    RegexBuilder::new(pat)
                        .case_insensitive(true)
                        .build()
                        .map_or(false, |re| re.is_match(url))
                })
            })
        })
        .cloned();

    // If a pattern matched, return that browser
    if pattern_match.is_some() {
        return pattern_match;
    }

    // If no pattern matched, try to return the default browser
    if let Some(ref default_label) = config.default_browser {
        return config
            .browsers
            .iter()
            .find(|b| b.label == *default_label)
            .cloned();
    }

    // No pattern match and no default browser configured
    None
}
