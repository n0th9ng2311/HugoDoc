use crate::config::config::{TYPE, Text, TextStorage};
use std::fs::File;
use std::io;
use std::io::Write;
use std::path::Path;

//this function will simply create the file with a specific name (for now we input it from user but
//later it may be generated automatically based on where the docs/comments are written although this is
//for later)
//this function asks for name of the file from the user but another one will just extract it from the
//file we are currently processing
pub(crate) fn create_md() -> io::Result<File> {
    let mut input = String::new();
    io::stdin().read_line(&mut input)?;

    let file_name = input.trim();
    let file = File::create(format!("{}.md", file_name))?;

    Ok(file)
}

//so this function will be called inside a loop ofcourse when we do dir parsing for files nice
//also it will create a new dir if the file path does not exist
pub(crate) fn create_md_from_file(dir: &Path, stem: &str) -> io::Result<File> {
    let mut path = dir.to_path_buf();
    path.push(stem);
    path.set_extension("md");

    if let Some(parent) = path.parent() {
        std::fs::create_dir_all(parent)?;
    }

    File::create(path)
}
pub fn get_md_string(text: &Text) -> io::Result<String> {
    let mut string_to_write = String::new();

    let prefix = match text.comment_type.txt_type {
        TYPE::HEADING => "# ",
        TYPE::SUBHEADING => "## ",
        TYPE::NORMAL => "",
    };

    for line in text.comment.lines() {
        if line.trim().is_empty() {
            string_to_write.push('\n');
        } else {
            string_to_write.push_str(prefix);
            string_to_write.push_str(line);
            string_to_write.push('\n');
        }
    }

    if string_to_write.ends_with('\n') {
        string_to_write.pop();
    }

    Ok(string_to_write)
}

//this file will have functions that will convert the
pub fn write_to_md(file: &mut File, text_store: &TextStorage) -> io::Result<()> {
    //now this function will read the textstroage and then after reading each Text it will just write
    //it to the file, now we need to have some rules
    for text in text_store {
        let string_to_write = get_md_string(text)?; //so this function will obtain the string
        //in md format
        file.write_all(string_to_write.as_bytes())?;
        file.write_all(b"\n\n")?;
    }

    Ok(())
}
