use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::path::Path;

pub const CONFIG_FILE: &str = "stakctrl.toml";

#[derive(Debug, Serialize, Deserialize)]
pub struct StakCtrlConfig {
    pub host:  HostConfig,
    pub track: TrackConfig,
}

#[derive(Debug, Serialize, Deserialize, Default)]
pub struct HostConfig {
    pub name:   String,
    pub remote: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TrackConfig {
    pub files:    Vec<String>,
    pub packages: bool,
    pub cron:     bool,
    pub systemd:  bool,
}

impl Default for TrackConfig {
    fn default() -> Self {
        Self {
            files: vec![
                "/boot/firmware/config.txt".to_string(),
                "/etc/rc.local".to_string(),
            ],
            packages: true,
            cron:     true,
            systemd:  true,
        }
    }
}

impl Default for StakCtrlConfig {
    fn default() -> Self {
        Self {
            host:  HostConfig::default(),
            track: TrackConfig::default(),
        }
    }
}

pub fn load(repo_root: &Path) -> Result<StakCtrlConfig> {
    let path    = repo_root.join(CONFIG_FILE);
    let content = std::fs::read_to_string(&path)
        .with_context(|| format!("Cannot read {CONFIG_FILE} — run `stakctrl init` first"))?;
    toml::from_str(&content).context("Failed to parse stakctrl.toml")
}

pub fn save(repo_root: &Path, cfg: &StakCtrlConfig) -> Result<()> {
    let path    = repo_root.join(CONFIG_FILE);
    let content = toml::to_string_pretty(cfg).context("Failed to serialize config")?;
    std::fs::write(&path, content)
        .with_context(|| format!("Cannot write {CONFIG_FILE}"))
}
