use serde::{de::DeserializeOwned, Serialize};
use std::{fs, path::Path};

pub fn load_or_create<T>(file: &Path) -> Result<T, String>
where
    T: Serialize + DeserializeOwned + Default,
{
    if file.exists() {
        let raw = fs::read_to_string(file).map_err(|e| e.to_string())?;
        match serde_json::from_str::<T>(&raw) {
            Ok(value) => return Ok(value),
            Err(e) => {
                // JSON cassé : on le garde en .bak au lieu de l'écraser
                eprintln!("{} invalide ({e}), réinitialisation", file.display());
                let _ = fs::rename(file, file.with_extension("json.bak"));
            }
        }
    }

    let value = T::default();
    save(file, &value)?;
    Ok(value)
}

pub fn save<T: Serialize>(file: &Path, value: &T) -> Result<(), String> {
    let json = serde_json::to_string_pretty(value).map_err(|e| e.to_string())?;
    fs::write(file, json).map_err(|e| e.to_string())
}