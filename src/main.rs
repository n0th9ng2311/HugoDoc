use crate::config::config::TextStorage;
use crate::config::loader::load_config;
use crate::markdown::conversions::{create_md_from_file, write_to_md};
use crate::scanner::checker::{get_file_write_loc, parse_file};
use anyhow::{Context, Result};
use std::env;
use std::path::Path;
use walkdir::WalkDir;
pub mod config;
pub mod hugo;
pub mod markdown;
pub mod scanner;

pub mod testing;

fn main() -> Result<()> {
    let args: Vec<String> = env::args().collect();
    let config_path = Path::new(&args[1]);
    let input_path = Path::new(&args[2]);

    let config = load_config(config_path).with_context(|| "failed to load config")?;

    for entry in WalkDir::new(input_path)
        .into_iter()
        .filter_map(|e| e.ok())
        .filter(|e| e.file_type().is_file())
    {
        let file_path = entry.path();
        let mut file_stor = TextStorage::new();
        parse_file(file_path, &config, &mut file_stor).with_context(|| "failed to parse")?;

        let loc_to_write = get_file_write_loc(&config, file_path)
            .with_context(|| "failed to get file write loc")?;

        let mut output_dir = Path::new(&config.hugo_config.loc).to_path_buf();
        if !loc_to_write.is_empty() {
            output_dir.push(&loc_to_write);
        }

        let file_stem = file_path
            .file_stem()
            .with_context(|| format!("Invalid file name: {:?}", file_path))?
            .to_str()
            .with_context(|| format!("Non-UTF8 filename: {:?}", file_path))?;

        let mut file = create_md_from_file(&output_dir, file_stem).with_context(|| {
            format!(
                "Unable to create file: {:?} at dir {:?}",
                file_stem, output_dir
            )
        })?;

        write_to_md(&mut file, &file_stor, &config).with_context(|| "failed to write to file")?;
    }

    Ok(())
}
