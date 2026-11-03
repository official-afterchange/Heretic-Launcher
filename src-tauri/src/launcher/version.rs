use serde::Deserialize;
use std::collections::HashMap;
use tokio::fs;
use super::download::download_file;
use crate::storage::paths::AppPaths;

const MANIFEST_URL: &str = "https://piston-meta.mojang.com/mc/game/version_manifest_v2.json";

#[derive(Deserialize)]
struct Manifest {
    versions: Vec<ManifestEntry>,
}

#[derive(Deserialize)]
struct ManifestEntry {
    id: String,
    url: String,
    sha1: String,
}

// On ne déclare que ce dont on a besoin pour l'instant, serde ignore le reste
#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VersionMeta {
    pub id: String,
    pub main_class: String,
    pub asset_index: AssetIndexRef,
    pub downloads: Downloads,
    pub java_version: Option<JavaVersion>,
    #[serde(default)]
    pub libraries: Vec<Library>,
    #[serde(default)]
    pub arguments: VersionArguments,
    #[serde(default)]
    pub minecraft_arguments: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Downloads {
    pub client: Artifact,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Artifact {
    pub url: String,
    pub sha1: String,
    pub size: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct AssetIndexRef {
    pub id: String,
    pub url: String,
    pub sha1: String,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(rename_all = "camelCase")]
pub struct JavaVersion {
    pub component: String,
    pub major_version: u32,
}

#[derive(Deserialize, Clone, Debug, Default)]
pub struct VersionArguments {
    #[serde(default)]
    pub game: Vec<VersionArgument>,
    #[serde(default)]
    pub jvm: Vec<VersionArgument>,
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum VersionArgument {
    Plain(String),
    Conditional(ConditionalArgument),
}

#[derive(Deserialize, Clone, Debug)]
#[serde(untagged)]
pub enum ArgumentValue {
    One(String),
    Many(Vec<String>),
}

#[derive(Deserialize, Clone, Debug)]
pub struct ConditionalArgument {
    #[serde(default)]
    pub rules: Vec<Rule>,
    pub value: ArgumentValue,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Rule {
    pub action: String,
    pub os: Option<RuleOs>,
    pub features: Option<HashMap<String, bool>>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct RuleOs {
    pub name: Option<String>,
    pub arch: Option<String>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct Library {
    pub name: String,
    #[serde(default)]
    pub url: Option<String>,
    #[serde(default)]
    pub sha1: Option<String>,
    #[serde(default)]
    pub size: Option<u64>,
    pub downloads: Option<LibraryDownloads>,
    pub rules: Option<Vec<Rule>>,
    pub natives: Option<HashMap<String, String>>,
    pub extract: Option<ExtractRules>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LibraryDownloads {
    pub artifact: Option<LibraryArtifact>,
    #[serde(default)]
    pub classifiers: HashMap<String, LibraryArtifact>,
}

#[derive(Deserialize, Clone, Debug)]
pub struct LibraryArtifact {
    pub path: String,
    pub url: String,
    pub sha1: String,
    pub size: u64,
}

#[derive(Deserialize, Clone, Debug)]
pub struct ExtractRules {
    #[serde(default)]
    pub exclude: Vec<String>,
}

#[derive(Deserialize)]
pub struct AssetIndex {
    pub objects: HashMap<String, AssetObject>,
}

#[derive(Deserialize, Clone)]
pub struct AssetObject {
    pub hash: String,
    pub size: u64,
}

#[derive(Deserialize)]
struct FabricLoaderVersion {
    loader: FabricLoaderInfo,
}

#[derive(Deserialize)]
struct FabricLoaderInfo {
    version: String,
    #[serde(default)]
    stable: bool,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct FabricProfile {
    main_class: String,
    #[serde(default)]
    arguments: VersionArguments,
    #[serde(default)]
    libraries: Vec<Library>,
}

pub async fn resolve_fabric_loader_version(
    client: &reqwest::Client,
    minecraft_version: &str,
    requested_version: Option<&str>,
) -> Result<String, String> {
    if let Some(version) = requested_version.map(str::trim).filter(|version| !version.is_empty()) {
        return Ok(version.to_owned());
    }

    let url = format!("https://meta.fabricmc.net/v2/versions/loader/{minecraft_version}");
    let versions: Vec<FabricLoaderVersion> = client
        .get(url)
        .send()
        .await
        .map_err(|error| format!("Impossible de récupérer les versions Fabric : {error}"))?
        .error_for_status()
        .map_err(|error| format!("Réponse invalide du service Fabric : {error}"))?
        .json()
        .await
        .map_err(|error| format!("Liste des versions Fabric invalide : {error}"))?;

    versions
        .iter()
        .find(|entry| entry.loader.stable)
        .or_else(|| versions.first())
        .map(|entry| entry.loader.version.clone())
        .ok_or_else(|| format!("Aucune version Fabric disponible pour Minecraft {minecraft_version}."))
}

pub async fn apply_fabric_profile(
    client: &reqwest::Client,
    meta: &mut VersionMeta,
    minecraft_version: &str,
    loader_version: &str,
) -> Result<(), String> {
    let url = format!(
        "https://meta.fabricmc.net/v2/versions/loader/{minecraft_version}/{loader_version}/profile/json"
    );
    let profile: FabricProfile = client
        .get(url)
        .send()
        .await
        .map_err(|error| format!("Impossible de récupérer le profil Fabric {loader_version} : {error}"))?
        .error_for_status()
        .map_err(|error| format!("Profil Fabric {loader_version} indisponible : {error}"))?
        .json()
        .await
        .map_err(|error| format!("Profil Fabric invalide : {error}"))?;

    meta.main_class = profile.main_class;
    meta.arguments.game.extend(profile.arguments.game);
    meta.arguments.jvm.extend(profile.arguments.jvm);
    meta.libraries.extend(profile.libraries);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::{FabricProfile, FabricLoaderVersion};

    #[test]
    fn parses_fabric_profile_libraries_and_arguments() {
        let profile: FabricProfile = serde_json::from_str(
            r#"{
                "mainClass":"net.fabricmc.loader.impl.launch.knot.KnotClient",
                "arguments":{"game":[],"jvm":["-DFabricMcEmu= net.minecraft.client.main.Main "]},
                "libraries":[
                    {"name":"org.ow2.asm:asm:9.7.1","url":"https://maven.fabricmc.net/","sha1":"abc123","size":126093},
                    {"name":"net.fabricmc:intermediary:1.20.1","url":"https://maven.fabricmc.net/"}
                ]
            }"#,
        ).unwrap();

        assert_eq!(profile.main_class, "net.fabricmc.loader.impl.launch.knot.KnotClient");
        assert_eq!(profile.arguments.jvm.len(), 1);
        assert_eq!(profile.libraries.len(), 2);
        assert_eq!(profile.libraries[0].sha1.as_deref(), Some("abc123"));
        assert_eq!(profile.libraries[1].size, None);
    }

    #[test]
    fn parses_fabric_loader_version_response() {
        let versions: Vec<FabricLoaderVersion> = serde_json::from_str(
            r#"[{"loader":{"version":"0.16.10","stable":true}}]"#,
        ).unwrap();
        assert!(versions[0].loader.stable);
        assert_eq!(versions[0].loader.version, "0.16.10");
    }
}

pub async fn fetch_version_meta(
    client: &reqwest::Client,
    paths: &AppPaths,
    mc_version: &str,
) -> Result<VersionMeta, String> {
    let local = paths.versions.join(mc_version).join(format!("{mc_version}.json"));

    // Déjà en cache et lisible : pas besoin du réseau
    if let Ok(raw) = fs::read_to_string(&local).await {
        if let Ok(meta) = serde_json::from_str::<VersionMeta>(&raw) {
            return Ok(meta);
        }
    }

    let manifest: Manifest = client
        .get(MANIFEST_URL)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?
        .json()
        .await
        .map_err(|e| e.to_string())?;

    let entry = manifest
        .versions
        .iter()
        .find(|v| v.id == mc_version)
        .ok_or_else(|| format!("Version Minecraft inconnue : {mc_version}"))?;

    download_file(client, &entry.url, &local, Some(&entry.sha1)).await?;

    let raw = fs::read_to_string(&local).await.map_err(|e| e.to_string())?;
    serde_json::from_str(&raw).map_err(|e| e.to_string())
}