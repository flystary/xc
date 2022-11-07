use anyhow::Result;
use std::path::{Path, PathBuf};

pub fn get_default_config(conf: &str) -> Result<PathBuf> {
    let paths = [
        format!("/etc/xc/{}", conf),
        format!("./{}", conf),
        format!("{}/xc/{}", std::env::var("HOME").unwrap(), conf),
    ];

    for path in paths.iter() {
        if Path::new(path).exists() {
            return Ok(Path::new(path).to_path_buf());
        }
    }
    Err(anyhow::anyhow!("Config file not found: {}", paths.join(", ")))
}
