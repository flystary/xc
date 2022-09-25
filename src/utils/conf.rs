use std::path::{PathBuf, Path};
use anyhow::Result;

pub fn get_default_config(conf: &str) -> Result<PathBuf> {
    let paths = [
        format!("C:/xc/{}", conf),
        format!("{}/.xc/{}", std::env::var("HOME").unwrap(), conf),
        format!("./{}", conf),
    ];

    for path in paths.iter() {
        if Path::new(path).exists() {
            return Ok(Path::new(path).to_path_buf());
        }
    }
    Err(anyhow::anyhow!("Config file not found. You can either specify it with the --config option or put it in one of the following locations: {}", paths.join(", ")))
}
