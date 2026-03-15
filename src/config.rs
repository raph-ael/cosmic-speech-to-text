use directories::ProjectDirs;
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TranscribeMode {
    Mistral,
    OpenAI,
    LocalWhisper,
}

impl Default for TranscribeMode {
    fn default() -> Self {
        Self::Mistral
    }
}

impl TranscribeMode {
    pub fn index(&self) -> usize {
        match self {
            Self::Mistral => 0,
            Self::OpenAI => 1,
            Self::LocalWhisper => 2,
        }
    }

    pub fn from_index(i: usize) -> Self {
        match i {
            1 => Self::OpenAI,
            2 => Self::LocalWhisper,
            _ => Self::Mistral,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Config {
    pub mode: TranscribeMode,
    pub mistral_api_key: String,
    pub openai_api_key: String,
    pub whisper_cpp_path: String,
    pub whisper_model_path: String,
    pub hotkey: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            mode: TranscribeMode::Mistral,
            mistral_api_key: String::new(),
            openai_api_key: String::new(),
            whisper_cpp_path: String::new(),
            whisper_model_path: String::new(),
            hotkey: "Ctrl+Y".to_string(),
        }
    }
}

impl Config {
    /// Returns the active API key for the current mode, or empty string.
    pub fn active_api_key(&self) -> &str {
        match self.mode {
            TranscribeMode::Mistral => &self.mistral_api_key,
            TranscribeMode::OpenAI => &self.openai_api_key,
            TranscribeMode::LocalWhisper => "",
        }
    }

    /// Check if the current mode is properly configured.
    pub fn is_configured(&self) -> bool {
        match self.mode {
            TranscribeMode::Mistral => !self.mistral_api_key.is_empty(),
            TranscribeMode::OpenAI => !self.openai_api_key.is_empty(),
            TranscribeMode::LocalWhisper => {
                !self.whisper_cpp_path.is_empty() && !self.whisper_model_path.is_empty()
            }
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
    let mut config: Config = serde_json::from_str(&content).unwrap_or_default();

    // Migrate old "api_key" field to "mistral_api_key"
    if config.mistral_api_key.is_empty() {
        if let Ok(old) = serde_json::from_str::<serde_json::Value>(&content) {
            if let Some(key) = old.get("api_key").and_then(|v| v.as_str()) {
                if !key.is_empty() {
                    config.mistral_api_key = key.to_string();
                    save_config(&config);
                }
            }
        }
    }

    config
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
