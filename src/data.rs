use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Copy, Clone, Default)]
pub enum Position {
    #[default]
    Top,
    Bottom,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
#[serde(default)]
pub struct Panel {
    pub(crate) height: i32,
    pub(crate) position: Position,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
#[serde(default)]
pub struct Launcher {
    pub(crate) on_click: String,
    #[serde(default = "default_icon")]
    pub(crate) icon: String,
}

fn default_icon() -> String {
    "view-app-grid-symbolic".to_string()
}

#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
#[serde(default)]
pub struct AppList {
    pub(crate) show_text: bool,
    pub(crate) favorites: Vec<String>,
}

#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
#[serde(default)]
pub(crate) struct Settings {
    pub(crate) panel: Panel,
    pub(crate) launcher: Launcher,
    pub(crate) app_list: AppList,
}
