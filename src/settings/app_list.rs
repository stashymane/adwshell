use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
#[serde(default)]
pub struct AppList {
    pub show_text: bool,
    pub favorites: Vec<String>,
}
