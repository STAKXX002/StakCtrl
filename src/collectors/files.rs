use anyhow::{Context, Result};

pub fn read(path: &str) -> Result<String> {
    std::fs::read_to_string(path)
        .with_context(|| format!("Failed to read tracked file: {path}"))
}

pub fn slugify(path: &str) -> String {
    path.trim_start_matches('/').replace('/', "_")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slugify_boot_config() {
        assert_eq!(slugify("/boot/firmware/config.txt"), "boot_firmware_config.txt");
    }

    #[test]
    fn slugify_etc() {
        assert_eq!(slugify("/etc/rc.local"), "etc_rc.local");
    }
}
