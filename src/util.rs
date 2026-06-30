use anyhow::{bail, Result};
use std::path::PathBuf;

/// Walk up from CWD until we find a directory containing stakctrl.toml.
pub fn find_repo_root() -> Result<PathBuf> {
    let mut dir = std::env::current_dir()?;
    loop {
        if dir.join(crate::config::CONFIG_FILE).exists() {
            return Ok(dir);
        }
        if !dir.pop() {
            bail!(
                "No stakctrl.toml found in this directory or any parent.\n\
                 Run `stakctrl init` to initialize."
            );
        }
    }
}
