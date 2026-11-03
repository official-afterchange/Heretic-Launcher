use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
    process::Stdio,
    sync::{Arc, Mutex, atomic::{AtomicBool, Ordering}},
};

use md5::{Digest, Md5};
use serde::Deserialize;
use sha2::Sha256;
use tauri::{AppHandle, Emitter, State};
use tokio::process::Command;

use crate::{
    launcher::{
        download::download_file,
        prepare::{Prepared, emit, extract_archive, prepare, rules_allow},
        version::{ArgumentValue, ConditionalArgument, VersionArgument},
    },
    storage::{config::Config, paths::AppPaths},
};

/// Empêche plusieurs préparations/lancements simultanés.
pub struct LaunchState(pub Arc<AtomicBool>);

fn offline_uuid(username: &str) -> String {
    let digest = Md5::digest(format!("OfflinePlayer:{username}").as_bytes());
    let mut bytes = [0_u8; 16];
    bytes.copy_from_slice(&digest);
    bytes[6] = (bytes[6] & 0x0f) | 0x30;
    bytes[8] = (bytes[8] & 0x3f) | 0x80;
    let raw = hex::encode(bytes);
    format!("{}-{}-{}-{}-{}", &raw[0..8], &raw[8..12], &raw[12..16], &raw[16..20], &raw[20..32])
}

fn validate_username(username: &str) -> Result<(), String> {
    if !(3..=16).contains(&username.len()) || !username.bytes().all(|c| c.is_ascii_alphanumeric() || c == b'_') {
        return Err("Le pseudo hors ligne doit contenir 3 à 16 lettres, chiffres ou caractères _".into());
    }
    Ok(())
}

fn arguments_from_value(value: &ArgumentValue) -> Vec<String> {
    match value {
        ArgumentValue::One(value) => vec![value.clone()],
        ArgumentValue::Many(values) => values.clone(),
    }
}

fn collect_arguments(arguments: &[VersionArgument]) -> Vec<String> {
    let mut result = Vec::new();
    for argument in arguments {
        match argument {
            VersionArgument::Plain(value) => result.push(value.clone()),
            VersionArgument::Conditional(ConditionalArgument { rules, value }) => {
                if rules.is_empty() || rules_allow(Some(rules)) {
                    result.extend(arguments_from_value(value));
                }
            }
        }
    }
    result
}

fn substitutions(prepared: &Prepared, username: &str, libraries_dir: &std::path::Path) -> HashMap<String, String> {
    let classpath_separator = if cfg!(windows) { ";" } else { ":" };
    let mut classpath = prepared.libraries.clone();
    classpath.push(prepared.client_jar.clone());
    let classpath = classpath.iter().map(|path| path.to_string_lossy()).collect::<Vec<_>>().join(classpath_separator);
    let mut values = HashMap::new();
    values.insert("${natives_directory}".into(), prepared.natives_dir.to_string_lossy().into_owned());
    values.insert("${launcher_name}".into(), "Heretic Launcher".into());
    values.insert("${launcher_version}".into(), env!("CARGO_PKG_VERSION").into());
    values.insert("${classpath}".into(), classpath);
    values.insert("${classpath_separator}".into(), classpath_separator.into());
    values.insert("${library_directory}".into(), libraries_dir.to_string_lossy().into_owned());
    values.insert("${version_name}".into(), prepared.meta.id.clone());
    values.insert("${version_type}".into(), "release".into());
    values.insert("${auth_player_name}".into(), username.into());
    values.insert("${auth_uuid}".into(), offline_uuid(username));
    values.insert("${auth_access_token}".into(), "0".into());
    values.insert("${user_type}".into(), "legacy".into());
    values.insert("${user_properties}".into(), "{}".into());
    values.insert("${auth_xuid}".into(), String::new());
    values.insert("${clientid}".into(), String::new());
    values.insert("${game_directory}".into(), prepared.game_dir.to_string_lossy().into_owned());
    values.insert("${assets_root}".into(), prepared.assets_dir.to_string_lossy().into_owned());
    values.insert("${assets_index_name}".into(), prepared.meta.asset_index.id.clone());
    values.insert("${resolution_width}".into(), "854".into());
    values.insert("${resolution_height}".into(), "480".into());
    values
}

fn expand(value: &str, values: &HashMap<String, String>) -> String {
    values.iter().fold(value.to_owned(), |expanded, (key, replacement)| {
        expanded.replace(key, replacement)
    })
}

fn build_game_arguments(prepared: &Prepared, username: &str, ram_mb: u32, libraries_dir: &std::path::Path) -> Vec<String> {
    let values = substitutions(prepared, username, libraries_dir);
    let mut jvm = collect_arguments(&prepared.meta.arguments.jvm)
        .iter()
        .map(|arg| expand(arg, &values))
        .collect::<Vec<_>>();
    let game_args = if let Some(legacy) = &prepared.meta.minecraft_arguments {
        legacy.split_whitespace().map(|arg| expand(arg, &values)).collect::<Vec<_>>()
    } else {
        collect_arguments(&prepared.meta.arguments.game)
            .iter()
            .map(|arg| expand(arg, &values))
            .collect::<Vec<_>>()
    };

    let classpath = values.get("${classpath}").cloned().unwrap_or_default();
    if !jvm.iter().any(|arg| arg == "-cp" || arg == "-classpath") {
        jvm.push("-cp".into());
        jvm.push(classpath);
    }
    if !jvm.iter().any(|arg| arg.starts_with("-Xmx")) {
        jvm.push(format!("-Xmx{}M", ram_mb.max(1024)));
    }
    if !jvm.iter().any(|arg| arg.starts_with("-Djava.library.path=")) {
        jvm.push(format!("-Djava.library.path={}", prepared.natives_dir.display()));
    }
    jvm.push(prepared.meta.main_class.clone());
    jvm.extend(game_args);
    jvm
}

fn parse_java_major(output: &[u8]) -> Option<u32> {
    let output = String::from_utf8_lossy(output);
    let quote = output.find('"')?;
    let version = &output[quote + 1..];
    let first = version.split(['.', '"', '-']).next()?.parse::<u32>().ok()?;
    if first == 1 {
        version.split('.').nth(1)?.parse().ok()
    } else {
        Some(first)
    }
}

#[derive(Deserialize)]
struct RuntimeAsset {
    binary: RuntimeBinary,
}

#[derive(Deserialize)]
struct RuntimeBinary {
    package: RuntimePackage,
}

#[derive(Deserialize)]
struct RuntimePackage {
    link: String,
    checksum: String,
}

fn find_java(root: &Path) -> Option<PathBuf> {
    let direct = root.join("bin").join("java.exe");
    if direct.is_file() {
        return Some(direct);
    }
    let entries = fs::read_dir(root).ok()?;
    for entry in entries.flatten() {
        if entry.file_type().ok()?.is_dir() {
            if let Some(java) = find_java(&entry.path()) {
                return Some(java);
            }
        }
    }
    None
}

async fn java_major(java_path: &str) -> Option<u32> {
    let output = Command::new(java_path).arg("-version").output().await.ok()?;
    if !output.status.success() {
        return None;
    }
    parse_java_major(&[output.stdout.as_slice(), output.stderr.as_slice()].concat())
}

fn usable_java_path(configured: &str) -> String {
    let path = Path::new(configured);
    if path.file_name().is_some_and(|name| name.to_string_lossy().eq_ignore_ascii_case("javaw.exe")) {
        if let Some(parent) = path.parent() {
            let console_java = parent.join("java.exe");
            if console_java.is_file() {
                return console_java.to_string_lossy().into_owned();
            }
        }
    }
    configured.to_owned()
}

async fn ensure_java(
    app: &AppHandle,
    paths: &AppPaths,
    configured_path: Option<&str>,
    required_major: u32,
) -> Result<String, String> {
    if let Some(configured) = configured_path.map(str::trim).filter(|path| !path.is_empty() && *path != "java") {
        let configured = usable_java_path(configured);
        let major = java_major(&configured).await.ok_or_else(|| format!("Java introuvable avec le chemin configuré « {configured} »."))?;
        if major < required_major {
            return Err(format!("Java {required_major} minimum est requis, mais le chemin configuré utilise Java {major}."));
        }
        return Ok(configured);
    }

    if java_major("java").await.is_some_and(|major| major >= required_major) {
        return Ok("java".into());
    }

    let install_dir = paths.java.join(format!("temurin-{required_major}"));
    if let Some(java) = find_java(&install_dir) {
        return Ok(java.to_string_lossy().into_owned());
    }

    if !cfg!(all(target_os = "windows", target_arch = "x86_64")) {
        return Err(format!("Aucun Java {required_major} compatible trouvé. Configure son chemin dans Paramètres."));
    }

    emit(app, "java", format!("Téléchargement de Java {required_major}…"));
    let client = reqwest::Client::builder()
        .user_agent("Heretic Launcher")
        .build()
        .map_err(|error| error.to_string())?;
    let api = format!("https://api.adoptium.net/v3/assets/latest/{required_major}/hotspot?architecture=x64&image_type=jre&os=windows&vendor=eclipse");
    let assets: Vec<RuntimeAsset> = client.get(api).send().await
        .map_err(|error| format!("Impossible de récupérer Java {required_major} : {error}"))?
        .error_for_status().map_err(|error| error.to_string())?
        .json().await.map_err(|error| format!("Réponse invalide du service Java : {error}"))?;
    let package = assets.into_iter().next()
        .ok_or_else(|| format!("Aucun runtime Java {required_major} disponible pour Windows x64."))?
        .binary.package;
    let archive_path = paths.java.join(format!("temurin-{required_major}.zip"));
    download_file(&client, &package.link, &archive_path, None).await?;
    let archive_bytes = tokio::fs::read(&archive_path).await.map_err(|error| error.to_string())?;
    let actual_checksum = hex::encode(Sha256::digest(&archive_bytes));
    if !actual_checksum.eq_ignore_ascii_case(&package.checksum) {
        let _ = tokio::fs::remove_file(&archive_path).await;
        return Err("La vérification SHA-256 de l’archive Java a échoué.".into());
    }

    let extraction_dir = install_dir.clone();
    let archive_to_extract = archive_path.clone();
    tokio::task::spawn_blocking(move || extract_archive(&archive_to_extract, &extraction_dir, &[]))
        .await.map_err(|error| format!("Erreur lors de l’installation de Java : {error}"))??;
    let java = find_java(&install_dir)
        .ok_or_else(|| format!("Java {required_major} a été téléchargé, mais java.exe est introuvable dans l’archive."))?;
    emit(app, "java", format!("Java {required_major} installé."));
    Ok(java.to_string_lossy().into_owned())
}

async fn run(
    app: &AppHandle,
    paths: &AppPaths,
    config: &Mutex<Config>,
    launch_lock: Arc<AtomicBool>,
) -> Result<(), String> {
    let (id, java_path, ram_mb, username) = {
        let config = config.lock().map_err(|e| e.to_string())?;
        (
            config.selected_instance.clone().ok_or("Aucune instance sélectionnée")?,
            config.java_path.clone(),
            config.ram_mb,
            config.player_username.trim().to_owned(),
        )
    };
    validate_username(&username)?;

    let instance = crate::storage::instance::Instance::load(&paths.instances, &id)?;
    let prepared = prepare(app, paths, instance).await?;
    let required_component = prepared.meta.java_version.as_ref().map(|version| version.component.as_str()).unwrap_or("Java système");
    let required_java = prepared.meta.java_version.as_ref().map(|version| version.major_version).unwrap_or(8);

    let java_path = ensure_java(app, paths, java_path.as_deref(), required_java).await
        .map_err(|error| format!("{error} Minecraft demande {required_component} (Java {required_java})."))?;

    let args = build_game_arguments(&prepared, &username, ram_mb, &paths.libraries);
    let mut child = Command::new(&java_path)
        .args(args)
        .current_dir(&prepared.game_dir)
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .map_err(|error| format!("Impossible de démarrer Minecraft : {error}"))?;

    let stdout = child.stdout.take();
    let stderr = child.stderr.take();
    let stdout_path = paths.logs.join(format!("{}-stdout.log", prepared.instance.id));
    let stderr_path = paths.logs.join(format!("{}-stderr.log", prepared.instance.id));
    let monitor_app = app.clone();
    tauri::async_runtime::spawn(async move {
        let stdout_task = tokio::spawn(async move {
            if let Some(mut output) = stdout {
                if let Ok(mut log) = tokio::fs::File::create(stdout_path).await {
                    let _ = tokio::io::copy(&mut output, &mut log).await;
                }
            }
        });
        let stderr_task = tokio::spawn(async move {
            if let Some(mut output) = stderr {
                if let Ok(mut log) = tokio::fs::File::create(stderr_path).await {
                    let _ = tokio::io::copy(&mut output, &mut log).await;
                }
            }
        });

        let result = child.wait().await;
        let _ = stdout_task.await;
        let _ = stderr_task.await;
        launch_lock.store(false, Ordering::SeqCst);
        match result {
            Ok(status) => {
                let message = format!("Minecraft s’est fermé (code {}).", status.code().map_or_else(|| "inconnu".into(), |code| code.to_string()));
                let _ = monitor_app.emit("launch-progress", Progress { step: "stopped", message });
            }
            Err(error) => {
                let _ = monitor_app.emit("launch-progress", Progress { step: "error", message: format!("Erreur du processus Minecraft : {error}") });
            }
        }
    });

    let _ = app.emit("launch-progress", Progress { step: "started", message: "Minecraft a démarré.".into() });
    println!("Minecraft démarré : {}, version {}, logs dans {:?}", prepared.instance.name, prepared.meta.id, paths.logs);
    Ok(())
}

#[derive(Clone, serde::Serialize)]
struct Progress {
    step: &'static str,
    message: String,
}

#[tauri::command]
pub async fn launch_instance(
    app: AppHandle,
    paths: State<'_, AppPaths>,
    config: State<'_, Mutex<Config>>,
    launching: State<'_, LaunchState>,
) -> Result<(), String> {
    if launching.0.swap(true, Ordering::SeqCst) {
        return Err("Un lancement est déjà en cours".into());
    }
    let lock = launching.0.clone();
    let result = run(&app, &paths, &config, lock.clone()).await;
    if result.is_err() {
        lock.store(false, Ordering::SeqCst);
    }
    result
}
