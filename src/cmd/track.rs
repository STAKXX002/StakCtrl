use crate::{config, util};
use anyhow::{bail, Result};
use colored::Colorize;
use std::path::Path;

pub fn run(path: &str) -> Result<()> {
    if !Path::new(path).is_absolute() {
        bail!("Path must be absolute (e.g. /boot/firmware/config.txt), got: {path}");
    }

    let repo_root = util::find_repo_root()?;
    let mut cfg   = config::load(&repo_root)?;

    if cfg.track.files.contains(&path.to_string()) {
        println!("{} {} is already tracked", "·".dimmed(), path);
        return Ok(());
    }

    cfg.track.files.push(path.to_string());
    cfg.track.files.sort();
    config::save(&repo_root, &cfg)?;

    println!("{} now tracking {}", "✓".green(), path.bold());
    println!("  Run `stakctrl snap` to capture its current state.");

    Ok(())
}
