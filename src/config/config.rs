use crate::hugo::hugo_config::HugoConfig;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Debug, Clone, PartialEq)]
pub enum TYPE {
    HEADING,
    SUBHEADING,
    NORMAL,
}

//Comment type : this will tell us what type of comment we are working with its start and end as well
//the start and end will store the type of str marking the start and end
#[derive(Deserialize, Debug, Clone)]
pub struct CommentType {
    pub start: String,
    pub end: String,

    //defines the type of the comment(can be a heading or something idk yet)
    pub txt_type: TYPE,
    // Here will go other fields, maybe types and stuff idk yet
}

#[derive(Deserialize, Debug, Default, Clone)]
pub struct IgnoreConfig {
    pub prefixes: Vec<String>,
}

#[derive(Deserialize, Debug, Clone, Default)]
pub struct FileLoc {
    pub start: String,
    pub end: String,
}

#[derive(Deserialize, Debug, Clone)]
pub struct Config {
    pub comment_types: HashMap<String, CommentType>, // will work like "simple comment",
    // value = comment type
    #[serde(default)]
    pub ignore: IgnoreConfig,

    #[serde(default, rename = "hugo")]
    pub hugo_config: HugoConfig,

    #[serde(default)]
    pub file_loc: FileLoc,
}

impl Config {
    pub fn check_match(&self, line: &str) -> Option<(&String, &CommentType)> {
        for (name, ctype) in &self.comment_types {
            if line.contains(&ctype.start) {
                return Some((name, ctype));
            }
        }
        None
    }
}

//this struct will store the actual contents of the comments along with their info
#[derive(Debug)]
pub struct Text {
    pub(crate) comment_type: CommentType,
    pub(crate) comment: String,
}

impl Text {
    pub fn new(comment_type: CommentType, comment: String) -> Text {
        Text {
            comment_type,
            comment,
        }
    }
}

#[derive(Debug)]
pub struct TextStorage {
    pub(crate) storage: Vec<Text>,
}

impl<'a> IntoIterator for &'a TextStorage {
    type Item = &'a Text;
    type IntoIter = std::slice::Iter<'a, Text>;
    fn into_iter(self) -> Self::IntoIter {
        self.storage.iter()
    }
}

impl TextStorage {
    pub fn new() -> TextStorage {
        TextStorage {
            storage: Vec::new(),
        }
    }
    pub fn add(&mut self, text: Text) {
        self.storage.push(text);
    }
    pub fn print(&self) {
        println!("{:#?}", self);
    }
}
