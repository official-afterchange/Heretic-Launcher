mod commands;
mod storage;
mod launcher;

use std::sync::{Arc, Mutex, atomic::AtomicBool};
use storage::{config::Config, instance::Instance, paths::AppPaths};
use tauri::Manager;

use crate::commands::launch::LaunchState;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let paths = AppPaths::new(app.handle())?;
            let first_launch = paths.ensure()?;
            if first_launch {
                println!("Premier lancement : dossiers créés dans {:?}", paths.root);
            }

            let mut config = Config::load_or_create(&paths.config)?;

            let mut instances = Instance::list(&paths.instances)?;
            if instances.is_empty() {
                instances.push(Instance::create(&paths.instances, Instance::default())?);
            }

            let selected_ok = config
                .selected_instance
                .as_ref()
                .is_some_and(|id| instances.iter().any(|i| &i.id == id));
            if !selected_ok {
                config.selected_instance = Some(instances[0].id.clone());
                config.save(&paths.config)?;
            }

            app.manage(paths);
            app.manage(Mutex::new(config));
            app.manage(LaunchState(Arc::new(AtomicBool::new(false))));
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::minecraft::get_server_status,
            commands::config::get_config,
            commands::config::set_config,
            commands::instance::list_instances,
            commands::instance::get_selected_instance,
            commands::instance::create_instance,
            commands::instance::select_instance,
            commands::instance::delete_instance,
            commands::launch::launch_instance,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
