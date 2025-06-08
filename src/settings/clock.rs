use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
#[serde(default)]
pub struct ClockSettings {
    #[serde(default = "default_format")]
    pub format: String,
}

fn default_format() -> String {
    String::from("%H:%M:%S")
}
