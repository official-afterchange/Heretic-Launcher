use serde::{Deserialize, Serialize};
use std::{fs, path::{Path, PathBuf}};
use super::json;

const FILE_NAME: &str = "instance.json";

#[derive(Serialize, Deserialize, Clone, Debug)]
#[serde(default)]
pub struct Instance {
    pub id: String,
    pub name: String,
    pub minecraft_version: String,
    pub loader: String,
    pub loader_version: Option<String>,
    pub server_address: Option<String>,
    pub installed: bool,
}

impl Default for Instance {
    fn default() -> Self {
        Self {
            id: "heretic".into(),
            name: "Heretic".into(),
            minecraft_version: "1.20.1".into(), // à adapter
            loader: "vanilla".into(),           // à adapter
            loader_version: None,
            server_address: Some("185.146.193.136:38742".into()),
            installed: false,
        }
    }
}

fn is_valid_id(id: &str) -> bool {
    !id.is_empty()
        && id.len() <= 64
        && id.chars().all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_')
}

pub fn slugify(name: &str) -> String {
    let slug: String = name
        .to_lowercase()
        .chars()
        .map(|c| if c.is_ascii_alphanumeric() { c } else { '-' })
        .collect();
    slug.trim_matches('-').to_string()
}

impl Instance {
    pub fn dir(instances: &Path, id: &str) -> PathBuf {
        instances.join(id)
    }

    pub fn game_dir(&self, instances: &Path) -> PathBuf {
        Self::dir(instances, &self.id).join("minecraft")
    }

    pub fn create(instances: &Path, instance: Instance) -> Result<Instance, String> {
        if !is_valid_id(&instance.id) {
            return Err("Identifiant d'instance invalide".into());
        }
        let dir = Self::dir(instances, &instance.id);
        if dir.exists() {
            return Err("Une instance avec cet identifiant existe déjà".into());
        }
        fs::create_dir_all(dir.join("minecraft")).map_err(|e| e.to_string())?;
        json::save(&dir.join(FILE_NAME), &instance)?;
        Ok(instance)
    }

    pub fn load(instances: &Path, id: &str) -> Result<Instance, String> {
        if !is_valid_id(id) {
            return Err("Identifiant d'instance invalide".into());
        }
        let raw = fs::read_to_string(Self::dir(instances, id).join(FILE_NAME))
            .map_err(|e| e.to_string())?;
        serde_json::from_str(&raw).map_err(|e| e.to_string())
    }

    pub fn save(&self, instances: &Path) -> Result<(), String> {
        json::save(&Self::dir(instances, &self.id).join(FILE_NAME), self)
    }

    pub fn list(instances: &Path) -> Result<Vec<Instance>, String> {
        let mut out = Vec::new();
        for entry in fs::read_dir(instances).map_err(|e| e.to_string())? {
            let entry = entry.map_err(|e| e.to_string())?;
            if !entry.path().is_dir() {
                continue;
            }
            let id = entry.file_name().to_string_lossy().to_string();
            match Self::load(instances, &id) {
                Ok(inst) => out.push(inst),
                Err(e) => eprintln!("Instance « {id} » ignorée : {e}"),
            }
        }
        out.sort_by(|a, b| a.name.to_lowercase().cmp(&b.name.to_lowercase()));
        Ok(out)
    }

    pub fn delete(instances: &Path, id: &str) -> Result<(), String> {
        if !is_valid_id(id) {
            return Err("Identifiant d'instance invalide".into());
        }
        fs::remove_dir_all(Self::dir(instances, id)).map_err(|e| e.to_string())
    }
}