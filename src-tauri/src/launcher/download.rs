use sha1::{Digest, Sha1};
use std::path::Path;
use tokio::fs;

async fn sha1_of(path: &Path) -> Result<String, String> {
    let bytes = fs::read(path).await.map_err(|e| e.to_string())?;
    Ok(hex::encode(Sha1::digest(&bytes)))
}

pub async fn download_file(
    client: &reqwest::Client,
    url: &str,
    dest: &Path,
    sha1: Option<&str>,
) -> Result<(), String> {
    // Déjà présent et valide : rien à faire
    if dest.exists() {
        match sha1 {
            Some(expected) if sha1_of(dest).await?.eq_ignore_ascii_case(expected) => return Ok(()),
            Some(_) => {} // hash différent : on retélécharge
            None => return Ok(()),
        }
    }

    if let Some(parent) = dest.parent() {
        fs::create_dir_all(parent).await.map_err(|e| e.to_string())?;
    }

    let bytes = client
        .get(url)
        .send()
        .await
        .map_err(|e| e.to_string())?
        .error_for_status()
        .map_err(|e| e.to_string())?
        .bytes()
        .await
        .map_err(|e| e.to_string())?;

    if let Some(expected) = sha1 {
        let actual = hex::encode(Sha1::digest(&bytes));
        if !actual.eq_ignore_ascii_case(expected) {
            return Err(format!("Hash invalide pour {url}"));
        }
    }

    // On écrit dans un .part puis on renomme : pas de fichier à moitié téléchargé
    let tmp = dest.with_extension("part");
    fs::write(&tmp, &bytes).await.map_err(|e| e.to_string())?;
    fs::rename(&tmp, dest).await.map_err(|e| e.to_string())
}