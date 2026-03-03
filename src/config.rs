use serde::Deserialize;
use std::fs;

#[derive(Debug, Deserialize, Clone)]
pub struct Config {
    pub general: General,
    #[allow(dead_code)]
    pub lua: Lua,
    pub ui: Ui,
}

#[derive(Debug, Deserialize, Clone)]
pub struct General {
    pub refresh_rate: u64,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Lua {
    #[allow(dead_code)]
    pub rules_dir: String,
}

#[derive(Debug, Deserialize, Clone)]
pub struct Ui {
    #[allow(dead_code)]
    pub show_processes: bool,
    pub history_size: Option<usize>,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            general: General { refresh_rate: 800 },
            lua: Lua {
                rules_dir: "lua/rules".to_string(),
            },
            ui: Ui {
                show_processes: true,
                history_size: Some(60),
            },
        }
    }
}

pub fn load_config() -> Config {
    // Try to load from current directory first, then fall back to default
    if let Ok(content) = fs::read_to_string("config.toml") {
        toml::from_str(&content).unwrap_or_default()
    } else {
        Config::default()
    }
}
