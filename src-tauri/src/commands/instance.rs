use std::sync::Mutex;
use tauri::State;
use crate::storage::{
    config::Config,
    instance::{slugify, Instance},
    paths::AppPaths,
};

#[tauri::command]
pub fn list_instances(paths: State<'_, AppPaths>) -> Result<Vec<Instance>, String> {
    Instance::list(&paths.instances)
}

#[tauri::command]
pub fn get_selected_instance(
    paths: State<'_, AppPaths>,
    config: State<'_, Mutex<Config>>,
) -> Result<Instance, String> {
    let id = config
        .lock()
        .unwrap()
        .selected_instance
        .clone()
        .ok_or("Aucune instance sélectionnée")?;
    Instance::load(&paths.instances, &id)
}

#[tauri::command]
pub fn create_instance(
    name: String,
    minecraft_version: String,
    loader: String,
    paths: State<'_, AppPaths>,
) -> Result<Instance, String> {
    let instance = Instance {
        id: slugify(&name),
        name,
        minecraft_version,
        loader,
        loader_version: None,
        server_address: None,
        installed: false,
    };
    Instance::create(&paths.instances, instance)
}

#[tauri::command]
pub fn select_instance(
    id: String,
    paths: State<'_, AppPaths>,
    config: State<'_, Mutex<Config>>,
) -> Result<(), String> {
    Instance::load(&paths.instances, &id)?; // vérifie qu'elle existe
    let mut cfg = config.lock().unwrap();
    cfg.selected_instance = Some(id);
    cfg.save(&paths.config)
}

#[tauri::command]
pub fn delete_instance(
    id: String,
    paths: State<'_, AppPaths>,
    config: State<'_, Mutex<Config>>,
) -> Result<(), String> {
    if config.lock().unwrap().selected_instance.as_deref() == Some(id.as_str()) {
        return Err("Impossible de supprimer l'instance sélectionnée".into());
    }
    Instance::delete(&paths.instances, &id)
}