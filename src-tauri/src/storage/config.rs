use serde::{Deserialize, Serialize};
use std::path::Path;
use super::json;

const FILE_NAME: &str = "config.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct Config {
    pub language: String,
    pub ram_mb: u32,
    pub java_path: Option<String>,
    pub close_on_launch: bool,
    pub selected_instance: Option<String>,
    pub player_username: String,
}

impl Default for Config {
    fn default() -> Self {
        Self {
            language: "fr".into(),
            ram_mb: 4096,
            java_path: None,
            close_on_launch: false,
            selected_instance: None,
            player_username: String::new(),
        }
    }
}

impl Config {
    pub fn load_or_create(dir: &Path) -> Result<Self, String> {
        json::load_or_create(&dir.join(FILE_NAME))
    }

    pub fn save(&self, dir: &Path) -> Result<(), String> {
        json::save(&dir.join(FILE_NAME), self)
    }
}