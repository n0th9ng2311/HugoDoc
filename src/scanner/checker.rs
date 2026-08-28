use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use crate::config::config::{CommentType, Config, Text, TextStorage};


fn strip_prefixes(line: &str, config: &Config) -> String {
    let trimmed = line.trim_start();
    for prefix in &config.ignore.prefixes {
        if trimmed.starts_with(prefix) {
            return trimmed[prefix.len()..].trim_start().to_string();
        }
    }
    trimmed.to_string() // no prefix found
}

//so this function extracts the location at which we want the file to be at
pub fn get_file_write_loc(config: &Config, file_path: &Path) -> Option<String> {
    let file = File::open(file_path).ok()?;
    let mut reader = BufReader::new(file);
    let mut lines = Vec::new();
    let limit = 10; 

    for _ in 0..limit {
        let mut line = String::new();
        if reader.read_line(&mut line).ok()? == 0 { break; }
        let stripped = strip_prefixes(&line, &config).trim().to_string();
        lines.push(stripped.trim().to_string());
    }

    for i in 0..lines.len().saturating_sub(2) {
        if lines[i] == config.file_loc.start
            && lines[i+2] == config.file_loc.end {
            return Some(lines[i+1].to_string());
        }
    }
    None
}



//this function should just parse the file and not return the textstorage
pub fn parse_file(file_path: &Path, config: &Config, storage: &mut TextStorage) -> Result<(), Box<dyn Error>> {

    let file = File::open(file_path)?;
    let reader = BufReader::new(file);

    let mut curr_type : Option<&CommentType> = None;
    let mut buf: Vec<String> = Vec::new();


    for line in reader.lines() {
        let raw = line?;
        let stripped = strip_prefixes(&raw, config);

        if let Some(ctype) = curr_type{
            if stripped.contains(&ctype.end){ //found the end
                let comment_txt = buf.join("\n");
                let text = Text::new(ctype.clone(), comment_txt);
                storage.add(text);

                curr_type = None;
                buf.clear();
            }else{ //inside the commenet
                buf.push(stripped);
            }
        }else{//not inside a comment checking for a start
            if let Some((_name, ctype)) = config.check_match(&stripped){
                curr_type = Some(ctype);
                buf.clear()
            }
        }
    }

    if let Some(ctype) = curr_type{
        eprintln!(
            "Warning: unclosed comment (type '{:?} in {})", ctype, file_path.display()
        );
    }

    Ok(())


}









