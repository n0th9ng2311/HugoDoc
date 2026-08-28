use serde::Deserialize;

#[derive(Deserialize, Debug, Default, Clone)]
pub struct HugoConfig{
    pub loc : String,
    pub owner: Option<String>,
    pub date: Option<String>,
    pub draft: String,
}
