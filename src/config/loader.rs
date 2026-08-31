use crate::config::config::Config;
use anyhow::{Context, Result, bail};
use std::fs;
use std::path::Path;
pub const MAX_CONFIG_SIZE: u64 = 10 * 1024 * 1024; // 10 MB

pub fn load_config(path: &Path) -> Result<Config> {
    let metadata =
        fs::metadata(path).with_context(|| format!("failed to read meta data for {:?}", path))?;
    if metadata.len() > MAX_CONFIG_SIZE {
        bail!(
            "Config file exceeds the max config size of {} MB",
            MAX_CONFIG_SIZE / (1024 * 1024)
        )
    }
    let contents =
        fs::read_to_string(path).with_context(|| format!("failed to read file {:?}", path))?;
    let config: Config = toml::from_str(&contents)
        .with_context(|| format!("failed to parse TOML from {:?}", path))?;

    Ok(config)
}

pub fn print_comments_types(config: &Config) -> Result<()> {
    for (key, value) in &config.comment_types {
        println!(
            "Type : {}\t start: {} \t end: {} \t Type: {:?} \t",
            key, value.start, value.end, value.txt_type
        );
    }

    Ok(())
}
