use std::fs;

use bytes::Bytes;
use sl_meta::json::version_manifest::VersionManifest;
use sl_utils::utils::{self, errors::BackendError};

use crate::MANIFEST_PATH;

pub async fn fetch_version_manifest() {
    let res = utils::download::get_as_bytes(
        "https://launchermeta.mojang.com/mc/game/version_manifest_v2.json",
    )
    .await;

    if let Ok(res) = res {
        fs::write(&MANIFEST_PATH.as_path(), res)
            .expect("Failed writing into the file: version_manifest.json");
    }
}

pub fn manifest_read() -> VersionManifest {
    let buffer = fs::read_to_string(MANIFEST_PATH.as_path())
        .expect("Failed reading the file: version_manifest.json");
    serde_json::from_str(buffer.as_str()).expect("Failed to parse file: version_manifest.json")
}

pub async fn download_version(
    manifest: &VersionManifest,
    version: &str,
) -> Result<Bytes, BackendError> {
    let Some(version) = manifest.versions().find(|x| x.id == version) else {
        return Err(BackendError::MinecraftVersionNotFound);
    };

    let res = utils::download::get_as_bytes(&version.url).await?;
    Ok(res)
}
