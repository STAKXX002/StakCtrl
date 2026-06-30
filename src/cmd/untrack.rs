use crate::{config, util};
use anyhow::Result;
use colored::Colorize;

pub fn run(path: &str) -> Result<()> {
    let repo_root = util::find_repo_root()?;
    let mut cfg   = config::load(&repo_root)?;

    let before = cfg.track.files.len();
    cfg.track.files.retain(|f| f != path);

    if cfg.track.files.len() == before {
        println!("{} {} was not in the tracking list", "·".dimmed(), path);
        return Ok(());
    }

    config::save(&repo_root, &cfg)?;

    println!("{} stopped tracking {}", "✓".green(), path.bold());
    println!("  History is preserved in git. The files/ copy will be");
    println!("  removed on the next `stakctrl snap`.");

    Ok(())
}
