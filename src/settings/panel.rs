use serde::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone, Default)]
#[serde(default)]
pub struct Panel {
    pub height: i32,
    pub position: Position,
}

#[derive(Debug, Deserialize, PartialEq, Copy, Clone, Default)]
pub enum Position {
    #[default]
    Top,
    Bottom,
}
