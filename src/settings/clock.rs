use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
#[serde(default)]
pub struct ClockSettings {
    pub format: String,
}
