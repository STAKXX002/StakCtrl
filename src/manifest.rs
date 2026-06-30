use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

pub const MANIFEST_FILE: &str = "manifest.toml";

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct Manifest {
    pub meta:     ManifestMeta,
    pub packages: PackageManifest,
    pub files:    HashMap<String, String>,
    pub cron:     CronManifest,
    pub systemd:  SystemdManifest,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct ManifestMeta {
    pub host:    String,
    pub snapped: String,
    pub snap_id: String,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct PackageManifest {
    pub explicit: Vec<String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct CronManifest {
    pub root:  Option<String>,
    pub users: HashMap<String, String>,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct SystemdManifest {
    pub enabled: Vec<String>,
}

pub fn load(host_dir: &Path) -> Result<Manifest> {
    let path    = host_dir.join(MANIFEST_FILE);
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("Cannot read manifest at {}", path.display()))?;
    toml::from_str(&content).context("Failed to parse manifest.toml")
}

pub fn save(host_dir: &Path, manifest: &Manifest) -> Result<()> {
    let path    = host_dir.join(MANIFEST_FILE);
    let content = toml::to_string_pretty(manifest)
        .context("Failed to serialize manifest")?;
    std::fs::write(&path, content)
        .with_context(|| format!("Cannot write manifest at {}", path.display()))
}
