use std::{
    collections::HashSet,
    fs::{self, File},
    io,
    path::{Component, Path, PathBuf},
    sync::{Arc, atomic::{AtomicUsize, Ordering}},
};

use futures_util::{stream::{self, StreamExt}, TryStreamExt};
use serde::Serialize;
use tauri::{AppHandle, Emitter};
use zip::ZipArchive;

use super::{
    download::download_file,
    version::{
        AssetIndex, AssetObject, Library, Rule, VersionMeta, apply_fabric_profile,
        fetch_version_meta, resolve_fabric_loader_version,
    },
};
use crate::storage::{instance::Instance, paths::AppPaths};

#[derive(Clone, Serialize)]
pub struct Progress {
    pub step: &'static str,
    pub message: String,
}

pub fn emit(app: &AppHandle, step: &'static str, message: impl Into<String>) {
    let _ = app.emit("launch-progress", Progress { step, message: message.into() });
}

pub struct Prepared {
    pub instance: Instance,
    pub meta: VersionMeta,
    pub client_jar: PathBuf,
    pub game_dir: PathBuf,
    pub libraries: Vec<PathBuf>,
    pub natives_dir: PathBuf,
    pub assets_dir: PathBuf,
}

fn safe_relative_path(path: &str) -> Result<PathBuf, String> {
    let path = Path::new(path);
    if path.components().any(|part| !matches!(part, Component::Normal(_))) {
        return Err(format!("Chemin de dépendance invalide : {path:?}"));
    }
    Ok(path.to_path_buf())
}

async fn verify_size(path: &Path, expected: u64, label: &str) -> Result<(), String> {
    let actual = tokio::fs::metadata(path).await.map_err(|e| e.to_string())?.len();
    if actual != expected {
        return Err(format!("Taille incorrecte pour {label} : attendu {expected} octets, reçu {actual}"));
    }
    Ok(())
}

fn rule_matches(rule: &Rule) -> bool {
    if let Some(os) = &rule.os {
        if let Some(name) = &os.name {
            if name != std::env::consts::OS {
                return false;
            }
        }
        if let Some(arch) = &os.arch {
            let is_64_bit = cfg!(target_pointer_width = "64");
            if arch.contains("64") != is_64_bit {
                return false;
            }
        }
    }

    // Les fonctionnalités conditionnelles du launcher officiel sont désactivées
    // pour ce profil local hors ligne (demo, résolution personnalisée, etc.).
    if let Some(features) = &rule.features {
        if features.values().any(|required| *required) {
            return false;
        }
    }
    true
}

pub fn rules_allow(rules: Option<&Vec<Rule>>) -> bool {
    let Some(rules) = rules else { return true };
    let mut allowed = false;
    for rule in rules {
        if rule_matches(rule) {
            allowed = rule.action == "allow";
        }
    }
    allowed
}

fn windows_native_classifier(library: &Library) -> Option<String> {
    let template = library.natives.as_ref()?.get("windows")?;
    let arch = if cfg!(target_pointer_width = "64") { "64" } else { "32" };
    Some(template.replace("${arch}", arch))
}

pub fn extract_archive(archive_path: &Path, destination: &Path, excludes: &[String]) -> Result<(), String> {
    let file = File::open(archive_path).map_err(|e| e.to_string())?;
    let mut archive = ZipArchive::new(file).map_err(|e| e.to_string())?;
    fs::create_dir_all(destination).map_err(|e| e.to_string())?;

    for index in 0..archive.len() {
        let mut entry = archive.by_index(index).map_err(|e| e.to_string())?;
        if entry.is_dir() {
            continue;
        }
        let Some(relative) = entry.enclosed_name() else { continue };
        if excludes.iter().any(|prefix| relative.starts_with(prefix)) {
            continue;
        }
        let output = destination.join(relative);
        if let Some(parent) = output.parent() {
            fs::create_dir_all(parent).map_err(|e| e.to_string())?;
        }
        let mut target = File::create(output).map_err(|e| e.to_string())?;
        io::copy(&mut entry, &mut target).map_err(|e| e.to_string())?;
    }
    Ok(())
}

async fn prepare_libraries(
    app: &AppHandle,
    client: &reqwest::Client,
    paths: &AppPaths,
    meta: &VersionMeta,
) -> Result<(Vec<PathBuf>, PathBuf), String> {
    let mut classpath = Vec::new();
    let natives_dir = paths.versions.join(&meta.id).join(format!("{}-natives", meta.id));
    tokio::fs::create_dir_all(&natives_dir).await.map_err(|e| e.to_string())?;
    let mut extracted = HashSet::new();

    for library in &meta.libraries {
        if !rules_allow(library.rules.as_ref()) {
            continue;
        }
        let Some(downloads) = &library.downloads else {
            if library.url.is_some() {
                classpath.push(prepare_maven_library(client, paths, library).await?);
            }
            continue;
        };

        let mut has_artifact = false;
        if let Some(artifact) = &downloads.artifact {
            let relative = safe_relative_path(&artifact.path)?;
            let destination = paths.libraries.join(relative);
            download_file(client, &artifact.url, &destination, Some(&artifact.sha1)).await
                .map_err(|error| format!("Bibliothèque {} : {error}", library.name))?;
            verify_size(&destination, artifact.size, &library.name).await?;
            classpath.push(destination);
            has_artifact = true;
        }

        if !has_artifact && library.url.is_some() {
            classpath.push(prepare_maven_library(client, paths, library).await?);
        }

        if let Some(classifier) = windows_native_classifier(library) {
            if let Some(artifact) = downloads.classifiers.get(&classifier) {
                let relative = safe_relative_path(&artifact.path)?;
                let archive = paths.libraries.join(relative);
                download_file(client, &artifact.url, &archive, Some(&artifact.sha1)).await
                    .map_err(|error| format!("Natives de {} : {error}", library.name))?;
                verify_size(&archive, artifact.size, &library.name).await?;
                if extracted.insert(archive.clone()) {
                    let excludes = library.extract.as_ref().map(|rules| rules.exclude.as_slice()).unwrap_or(&[]);
                    extract_archive(&archive, &natives_dir, excludes)?;
                }
            }
        }
    }

    emit(app, "libraries", format!("{} bibliothèques Minecraft prêtes.", classpath.len()));
    Ok((classpath, natives_dir))
}

fn maven_artifact_path(coordinate: &str) -> Result<PathBuf, String> {
    let (coordinate, extension) = coordinate
        .split_once('@')
        .map_or((coordinate, "jar"), |(coordinate, extension)| (coordinate, extension));
    let parts: Vec<_> = coordinate.split(':').collect();
    if !(3..=4).contains(&parts.len()) || parts.iter().any(|part| part.is_empty()) || extension.is_empty() {
        return Err(format!("Coordonnées Maven invalides : {coordinate}"));
    }

    let group = parts[0].replace('.', "/");
    let artifact = parts[1];
    let version = parts[2];
    let classifier = parts.get(3).map(|value| format!("-{value}")).unwrap_or_default();
    let filename = format!("{artifact}-{version}{classifier}.{extension}");
    let path = PathBuf::from(group).join(artifact).join(version).join(filename);
    safe_relative_path(&path.to_string_lossy())
}

async fn prepare_maven_library(
    client: &reqwest::Client,
    paths: &AppPaths,
    library: &Library,
) -> Result<PathBuf, String> {
    let base_url = library.url.as_deref().unwrap_or_default();
    let relative = maven_artifact_path(&library.name)?;
    let destination = paths.libraries.join(&relative);
    let url_path = relative.to_string_lossy().replace('\\', "/");
    let url = format!("{}/{}", base_url.trim_end_matches('/'), url_path);
    download_file(client, &url, &destination, library.sha1.as_deref()).await
        .map_err(|error| format!("Bibliothèque {} : {error}", library.name))?;
    if let Some(size) = library.size {
        verify_size(&destination, size, &library.name).await?;
    }
    Ok(destination)
}

async fn prepare_assets(
    app: &AppHandle,
    client: &reqwest::Client,
    paths: &AppPaths,
    meta: &VersionMeta,
) -> Result<PathBuf, String> {
    let index_path = paths.assets.join("indexes").join(format!("{}.json", meta.asset_index.id));
    download_file(client, &meta.asset_index.url, &index_path, Some(&meta.asset_index.sha1)).await?;
    let index_bytes = tokio::fs::read(&index_path).await.map_err(|e| e.to_string())?;
    let index: AssetIndex = serde_json::from_slice(&index_bytes).map_err(|e| e.to_string())?;
    let objects: Vec<(String, AssetObject)> = index.objects.into_iter().collect();
    let total = objects.len();
    let completed = Arc::new(AtomicUsize::new(0));
    let assets_root = paths.assets.clone();
    let progress_app = app.clone();

    emit(app, "assets", format!("Téléchargement des ressources Minecraft (0/{total})…"));
    stream::iter(objects.into_iter().map(|(_, object)| {
        let client = client.clone();
        let assets_root = assets_root.clone();
        let completed = completed.clone();
        let app = progress_app.clone();
        async move {
            let prefix = object.hash.get(..2).ok_or_else(|| "Hash d’asset invalide".to_string())?;
            let destination = assets_root.join("objects").join(prefix).join(&object.hash);
            let url = format!("https://resources.download.minecraft.net/{}/{}", prefix, object.hash);
            download_file(&client, &url, &destination, Some(&object.hash)).await?;
            let actual_size = tokio::fs::metadata(&destination).await.map_err(|e| e.to_string())?.len();
            if actual_size != object.size {
                return Err(format!("Taille incorrecte pour l’asset {}", object.hash));
            }
            let count = completed.fetch_add(1, Ordering::Relaxed) + 1;
            if count == total || count % 25 == 0 {
                emit(&app, "assets", format!("Téléchargement des ressources Minecraft ({count}/{total})…"));
            }
            Ok::<(), String>(())
        }
    }))
    .buffer_unordered(8)
    .try_collect::<Vec<_>>()
    .await?;

    Ok(paths.assets.clone())
}

pub async fn prepare(
    app: &AppHandle,
    paths: &AppPaths,
    mut instance: Instance,
) -> Result<Prepared, String> {
    let loader = instance.loader.trim().to_ascii_lowercase();
    if loader != "vanilla" && loader != "fabric" {
        return Err(format!("Loader « {} » pas encore supporté", instance.loader));
    }

    let client = reqwest::Client::builder()
        .user_agent("heretic-launcher")
        .build()
        .map_err(|e| e.to_string())?;

    emit(app, "metadata", "Récupération des informations de la version…");
    let mut meta = fetch_version_meta(&client, paths, &instance.minecraft_version).await?;
    if loader == "fabric" {
        let loader_version = resolve_fabric_loader_version(
            &client,
            &instance.minecraft_version,
            instance.loader_version.as_deref(),
        ).await?;
        emit(app, "metadata", format!("Préparation de Fabric {loader_version}…"));
        apply_fabric_profile(&client, &mut meta, &instance.minecraft_version, &loader_version).await?;
        instance.loader_version = Some(loader_version);
    }

    emit(app, "client", "Téléchargement du client Minecraft…");
    let client_jar = paths.versions.join(&meta.id).join(format!("{}.jar", meta.id));
    download_file(&client, &meta.downloads.client.url, &client_jar, Some(&meta.downloads.client.sha1)).await?;
    verify_size(&client_jar, meta.downloads.client.size, "client Minecraft").await?;

    let (libraries, natives_dir) = prepare_libraries(app, &client, paths, &meta).await?;
    let assets_dir = prepare_assets(app, &client, paths, &meta).await?;
    let game_dir = instance.game_dir(&paths.instances);
    tokio::fs::create_dir_all(&game_dir).await.map_err(|e| e.to_string())?;

    instance.installed = true;
    instance.save(&paths.instances)?;
    emit(app, "ready", "Minecraft est prêt à démarrer.");
    Ok(Prepared { instance, meta, client_jar, game_dir, libraries, natives_dir, assets_dir })
}

#[cfg(test)]
mod tests {
    use super::maven_artifact_path;
    use std::path::Path;

    #[test]
    fn resolves_maven_coordinates_to_repository_paths() {
        assert_eq!(
            maven_artifact_path("net.fabricmc:fabric-loader:0.16.10").unwrap(),
            Path::new("net/fabricmc/fabric-loader/0.16.10/fabric-loader-0.16.10.jar"),
        );
        assert_eq!(
            maven_artifact_path("org.example:library:1.2.3:natives-windows@zip").unwrap(),
            Path::new("org/example/library/1.2.3/library-1.2.3-natives-windows.zip"),
        );
    }

    #[test]
    fn rejects_invalid_maven_coordinates() {
        assert!(maven_artifact_path("../escape:library:1.0").is_err());
        assert!(maven_artifact_path("library:1.0").is_err());
    }
}
