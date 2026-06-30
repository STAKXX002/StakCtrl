use anyhow::{Context, Result};
use std::process::Command;

pub fn collect() -> Result<Vec<String>> {
    let out = Command::new("apt-mark")
        .arg("showmanual")
        .output()
        .context("Failed to run apt-mark showmanual (Debian-based host required)")?;

    let mut pkgs: Vec<String> = String::from_utf8(out.stdout)?
        .lines()
        .map(|l| l.trim().to_string())
        .filter(|l| !l.is_empty())
        .collect();

    pkgs.sort();
    Ok(pkgs)
}
