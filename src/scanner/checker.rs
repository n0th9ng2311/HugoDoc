use crate::config::config::{CommentType, Config, Text, TextStorage};
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

fn strip_prefixes(line: &str, config: &Config) -> String {
    let trimmed = line.trim_start();
    for prefix in &config.ignore.prefixes {
        if trimmed.starts_with(prefix) {
            return trimmed[prefix.len()..].to_string();
        }
    }
    line.to_string() // no prefix found
}

//so this function extracts the location at which we want the file to be at
pub fn get_file_write_loc(config: &Config, file_path: &Path) -> Option<String> {
    let file = File::open(file_path).ok()?;
    let mut reader = BufReader::new(file);
    let mut lines = Vec::new();
    let limit = 10;

    for _ in 0..limit {
        let mut line = String::new();
        if reader.read_line(&mut line).ok()? == 0 {
            break;
        }
        let stripped = strip_prefixes(&line, config).trim().to_string();
        lines.push(stripped.trim().to_string());
    }

    for i in 0..lines.len().saturating_sub(2) {
        if lines[i] == config.file_loc.start && lines[i + 2] == config.file_loc.end {
            return Some(lines[i + 1].to_string());
        }
    }
    None
}

//this function should just parse the file and not return the textstorage
pub fn parse_file(
    file_path: &Path,
    config: &Config,
    storage: &mut TextStorage,
) -> Result<(), Box<dyn Error>> {
    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut curr_type: Option<&CommentType> = None;
    let mut buf: Vec<String> = Vec::new();

    let mut inside_code_block = false;
    let mut line_num = 0;

    for line in reader.lines() {
        let raw = line?;
        line_num += 1;

        let stripped = strip_prefixes(&raw, config);

        let trimmed_for_check = stripped.trim_start();

        if trimmed_for_check.starts_with("```") {
            inside_code_block = !inside_code_block;
        }

        let line_to_store = if inside_code_block {
            stripped.to_string() //preserving all the white spaces inside a code block
        } else {
            stripped.trim_start().to_string() // trim outside
        };

        if let Some(ctype) = curr_type {
            if trimmed_for_check.contains(&ctype.end) {
                let comment_txt = buf.join("\n");
                let text = Text::new(ctype.clone(), comment_txt);
                storage.add(text);
                curr_type = None;
                buf.clear();
            } else {
                //inside
                buf.push(line_to_store);
            }
        } else {
            //not inside
            if let Some((_name, ctype)) = config.check_match(trimmed_for_check) {
                curr_type = Some(ctype);
                buf.clear();
            }
        }
    }

    if let Some(ctype) = curr_type {
        eprintln!(
            "Warning: unclosed comment (type '{:?}') in {}",
            ctype,
            file_path.display()
        );
    }

    Ok(())
}
