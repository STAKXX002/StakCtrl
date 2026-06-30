use crate::{collectors, config, git, manifest, util};
use anyhow::{Context, Result};
use colored::Colorize;
use std::collections::HashMap;

pub fn run(message: &str) -> Result<()> {
    let repo_root  = util::find_repo_root()?;
    let cfg        = config::load(&repo_root)?;
    let host_dir   = repo_root.join("hosts").join(&cfg.host.name);
    let files_dir  = host_dir.join("files");
    std::fs::create_dir_all(&files_dir)?;

    let mut man = manifest::load(&host_dir).unwrap_or_else(|_| manifest::Manifest {
        meta: manifest::ManifestMeta {
            host: cfg.host.name.clone(),
            ..Default::default()
        },
        ..Default::default()
    });

    let mut new_files: HashMap<String, String> = HashMap::new();
    let (mut added, mut updated) = (0, 0);

    for path in &cfg.track.files {
        let content = match collectors::files::read(path) {
            Ok(c) => c,
            Err(e) => {
                println!("{} skipping {} ({})", "!".yellow(), path, e);
                continue;
            }
        };

        let rel  = format!("files/{}", collectors::files::slugify(path));
        let dest = host_dir.join(&rel);

        let is_new  = !man.files.contains_key(path.as_str());
        let changed = std::fs::read_to_string(&dest)
            .map(|old| old != content)
            .unwrap_or(true);

        std::fs::write(&dest, &content)
            .with_context(|| format!("Failed to write {}", dest.display()))?;

        new_files.insert(path.clone(), rel);

        if is_new {
            added += 1;
            println!("{} {}", "+".green(), path);
        } else if changed {
            updated += 1;
            println!("{} {}", "~".yellow(), path);
        }
    }

    let mut removed = 0;
    for (old_path, old_rel) in man.files.iter() {
        if !new_files.contains_key(old_path) {
            let stale = host_dir.join(old_rel);
            if stale.exists() {
                std::fs::remove_file(&stale)?;
            }
            println!("{} {}", "-".red(), old_path);
            removed += 1;
        }
    }

    man.files        = new_files;
    man.meta.host    = cfg.host.name.clone();
    man.meta.snapped = chrono::Utc::now().to_rfc3339();

    manifest::save(&host_dir, &man)?;

    git::add_all(&repo_root)?;
    git::commit(&repo_root, message)?;
    let sha = git::short_sha(&repo_root)?;

    if added == 0 && updated == 0 && removed == 0 {
        println!("\n{} no file changes  —  snapshot {}", "·".dimmed(), sha);
    } else {
        println!(
            "\n{} snapshot {}  ({} added, {} updated, {} removed)",
            "★".yellow().bold(),
            sha.bold(),
            added,
            updated,
            removed
        );
    }

    Ok(())
}
