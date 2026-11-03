use std::{fs, io, path::PathBuf};
use tauri::{AppHandle, Manager};

#[derive(Clone)]
pub struct AppPaths {
    pub root: PathBuf,
    pub instances: PathBuf,
    pub versions: PathBuf,
    pub libraries: PathBuf,
    pub assets: PathBuf,
    pub java: PathBuf,
    pub logs: PathBuf,
    pub config: PathBuf,
}

impl AppPaths {
    pub fn new(app: &AppHandle) -> Result<Self, String> {
        let identifier = &app.config().identifier;
        let app_name = identifier.rsplit('.').next().unwrap_or(identifier);

        let root = app
            .path()
            .data_dir()
            .map_err(|e| e.to_string())?
            .join(app_name);

        Ok(Self {
            instances: root.join("instances"),
            versions: root.join("versions"),
            libraries: root.join("libraries"),
            assets: root.join("assets"),
            java: root.join("java"),
            logs: root.join("logs"),
            config: root.join("config"),
            root,
        })
    }

    pub fn ensure(&self) -> io::Result<bool> {
        let first_launch = !self.root.exists();
        for dir in [
            &self.root, &self.instances, &self.versions, &self.libraries,
            &self.assets, &self.java, &self.logs, &self.config,
        ] {
            fs::create_dir_all(dir)?;
        }
        Ok(first_launch)
    }
}