use std::sync::Mutex;
use tauri::State;
use crate::storage::{config::Config, paths::AppPaths};

#[tauri::command]
pub fn get_config(config: State<'_, Mutex<Config>>) -> Config {
    config.lock().unwrap().clone()
}

#[tauri::command]
pub fn set_config(
    new_config: Config,
    config: State<'_, Mutex<Config>>,
    paths: State<'_, AppPaths>,
) -> Result<(), String> {
    new_config.save(&paths.config)?;
    *config.lock().unwrap() = new_config;
    Ok(())
}