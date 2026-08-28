use std::error::Error;
use std::fs;
use std::path::Path;
use crate::config::config::Config;

pub const MAX_CONFIG_SIZE: u64 = 10 * 1024 * 1024; // 10 MB

pub fn load_config(path: &Path) -> Result<Config, Box<dyn Error>> {
    let metadata = fs::metadata(path)?;
    if metadata.len() > MAX_CONFIG_SIZE {
        return Err(format!("Config file exceeds maximum size of {} MB",
                           MAX_CONFIG_SIZE / (1024 * 1024)).into());
    }
    let contents = std::fs::read_to_string(path)?;
    let config: Config = toml::from_str(&contents)?;
    Ok(config)
}

pub fn print_comments_types(config: &Config) -> Result<(), Box<dyn Error>> {
    for(key, value) in &config.comment_types{
        println!("Type : {}\t start: {} \t end: {} \t Type: {:?} \t",
                 key, value.start, value.end, value.txt_type);
    }

    Ok(())
}
