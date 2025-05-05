use std::fs;
use std::path::PathBuf;
use serde::Deserialize;

#[derive(Debug, Clone, Deserialize)]
pub struct BrowserConfig {
    pub label: String,
    pub command: String,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Config {
    pub browsers: Vec<BrowserConfig>,
}

pub fn load_config() -> Config {
    let home_dir = std::env::var("HOME").unwrap_or_else(|_| ".".into());
    let config_path: PathBuf = [home_dir.as_str(), ".config", "crossroads.toml"]
        .iter()
        .collect();
    let config_str = fs::read_to_string(&config_path)
        .expect(&format!("Failed to read config file: {:?}", config_path));
    toml::from_str(&config_str)
        .expect("Failed to parse config file")
}