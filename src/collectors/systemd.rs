use anyhow::{Context, Result};
use std::process::Command;

pub fn collect_enabled() -> Result<Vec<String>> {
    let out = Command::new("systemctl")
        .args(["list-unit-files", "--state=enabled", "--no-legend", "--no-pager"])
        .output()
        .context("Failed to run systemctl")?;

    let mut units: Vec<String> = String::from_utf8(out.stdout)?
        .lines()
        .filter_map(|l| l.split_whitespace().next())
        .map(|s| s.to_string())
        .collect();

    units.sort();
    Ok(units)
}
