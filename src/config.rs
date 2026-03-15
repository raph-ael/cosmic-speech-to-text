use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub api_key: String,
    pub hotkey: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            api_key: String::new(),
            hotkey: "Click panel icon".to_string(),
        }
    }
}

fn config_path() -> Option<PathBuf> {
    let dirs = ProjectDirs::from("", "", "cosmic-speech-to-text")?;
    Some(dirs.config_dir().join("config.json"))
}

pub fn load_config() -> Config {
    let Some(path) = config_path() else {
        return Config::default();
    };
    if !path.exists() {
        return Config::default();
    }
    let content = fs::read_to_string(&path).unwrap_or_default();
    serde_json::from_str(&content).unwrap_or_default()
}

pub fn save_config(config: &Config) {
    let Some(path) = config_path() else {
        eprintln!("Could not determine config path");
        return;
    };
    if let Some(parent) = path.parent() {
        let _ = fs::create_dir_all(parent);
    }
    match serde_json::to_string_pretty(config) {
        Ok(json) => {
            if let Err(e) = fs::write(&path, json) {
                eprintln!("Failed to save config: {e}");
            }
        }
        Err(e) => eprintln!("Failed to serialize config: {e}"),
    }
}
