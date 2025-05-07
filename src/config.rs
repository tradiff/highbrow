use serde::Deserialize;
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub browsers: Vec<BrowserConfig>,
    pub rules: Vec<RuleConfig>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct BrowserConfig {
    pub id: String,
    pub label: String,
    pub command: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct RuleConfig {
    pub regex: String,
    pub browser_id: String,
}

pub fn load_config() -> Config {
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    let config_path: PathBuf = [home_dir.as_str(), ".config", "browser-fork.toml"]
        .iter()
        .collect();

    let config_str = fs::read_to_string(&config_path)
        .unwrap_or_else(|_| panic!("Failed to read config file: {:?}", config_path));
    toml::from_str(&config_str).expect("Failed to parse config file")
}
