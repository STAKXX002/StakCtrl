use anyhow::{bail, Context, Result};
use std::path::Path;
use std::process::Command;

fn run(args: &[&str], cwd: &Path) -> Result<()> {
    let status = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .status()
        .with_context(|| format!("failed to spawn: git {}", args.join(" ")))?;
    if !status.success() {
        bail!("git {} exited with {}", args.join(" "), status);
    }
    Ok(())
}

fn capture(args: &[&str], cwd: &Path) -> Result<String> {
    let out = Command::new("git")
        .args(args)
        .current_dir(cwd)
        .output()
        .with_context(|| format!("failed to spawn: git {}", args.join(" ")))?;
    Ok(String::from_utf8(out.stdout)?.trim().to_string())
}

pub fn init(repo_root: &Path) -> Result<()> {
    run(&["init"], repo_root)
}

pub fn add_all(repo_root: &Path) -> Result<()> {
    run(&["add", "-A"], repo_root)
}

pub fn commit(repo_root: &Path, message: &str) -> Result<()> {
    run(&["commit", "--allow-empty", "-m", message], repo_root)
}

pub fn push(repo_root: &Path, remote: &str, branch: &str) -> Result<()> {
    run(&["push", remote, branch], repo_root)
}

pub fn short_sha(repo_root: &Path) -> Result<String> {
    capture(&["rev-parse", "--short", "HEAD"], repo_root)
}

pub fn diff_staged(repo_root: &Path) -> Result<String> {
    run(&["add", "-N", "."], repo_root)?;
    capture(&["diff", "HEAD"], repo_root)
}

pub fn log_path(repo_root: &Path, rel_path: &Path) -> Result<()> {
    run(
        &["log", "--oneline", "--", rel_path.to_str().unwrap_or(".")],
        repo_root,
    )
}
