use serde::Deserialize;
#[derive(Deserialize, Debug, Default, Clone)]
pub struct HugoConfig {
    pub loc: String,
    pub title: String,
    pub date: String,
    pub author: Option<String>,
    pub tags: Option<Vec<String>>,
    pub draft: String,
}
