use anyhow::Result;
use std::process::Command;

pub fn collect_user(user: &str) -> Result<Option<String>> {
    let out = Command::new("crontab")
        .args(["-l", "-u", user])
        .output()?;

    if out.status.success() {
        Ok(Some(String::from_utf8(out.stdout)?.trim().to_string()))
    } else {
        Ok(None)
    }
}

pub fn collect_cron_d() -> Result<Vec<(String, String)>> {
    let dir = std::path::Path::new("/etc/cron.d");
    if !dir.exists() {
        return Ok(vec![]);
    }
    let mut entries = vec![];
    for entry in std::fs::read_dir(dir)? {
        let entry   = entry?;
        let name    = entry.file_name().to_string_lossy().to_string();
        let content = std::fs::read_to_string(entry.path())?;
        entries.push((name, content));
    }
    entries.sort_by(|a, b| a.0.cmp(&b.0));
    Ok(entries)
}
