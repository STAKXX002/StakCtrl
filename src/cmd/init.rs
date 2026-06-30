use crate::{config, git};
use anyhow::{bail, Result};
use colored::Colorize;

pub fn run() -> Result<()> {
    let repo_root = std::env::current_dir()?;

    if repo_root.join(config::CONFIG_FILE).exists() {
        bail!(
            "stakctrl.toml already exists here.\n\
             Did you mean `stakctrl snap`?"
        );
    }

    let hostname = detect_hostname()?;

    let mut cfg = config::StakCtrlConfig::default();
    cfg.host.name = hostname.clone();
    config::save(&repo_root, &cfg)?;
    println!("{} wrote stakctrl.toml  (host = {})", "✓".green(), hostname.bold());

    let host_dir  = repo_root.join("hosts").join(&hostname);
    let files_dir = host_dir.join("files");
    std::fs::create_dir_all(&files_dir)?;
    std::fs::write(files_dir.join(".gitkeep"), b"")?;
    println!("{} created  hosts/{}/files/", "✓".green(), hostname);

    let generic_keep = repo_root.join("hosts").join(".gitkeep");
    if generic_keep.exists() {
        std::fs::remove_file(&generic_keep)?;
    }

    if !repo_root.join(".git").exists() {
        git::init(&repo_root)?;
        println!("{} git init", "✓".green());
    } else {
        println!("{} .git already present, skipping git init", "·".dimmed());
    }

    git::add_all(&repo_root)?;
    git::commit(&repo_root, &format!("stakctrl: init {hostname}"))?;
    println!("{} initial commit", "✓".green());

    println!("\n{} StakCtrl initialized for '{}'\n", "★".yellow().bold(), hostname.bold());
    println!("  Add files :  stakctrl track /boot/firmware/config.txt");
    println!("  Snapshot  :  stakctrl snap --message \"initial state\"\n");

    Ok(())
}

fn detect_hostname() -> Result<String> {
    if let Ok(raw) = std::fs::read_to_string("/etc/hostname") {
        let h = raw.trim().to_string();
        if !h.is_empty() {
            return Ok(h);
        }
    }
    let out = std::process::Command::new("hostname").output()?;
    let h   = String::from_utf8(out.stdout)?.trim().to_string();
    if h.is_empty() {
        bail!(
            "Could not detect hostname.\n\
             Set [host] name = \"<name>\" in stakctrl.toml manually."
        );
    }
    Ok(h)
}
