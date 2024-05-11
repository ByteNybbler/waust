use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LevelEdgeStyle(i32);

impl LevelEdgeStyle {
    pub const DEFAULT: Self = Self(1);
    pub const BORDER: Self = Self(2);
    pub const BORDER_X: Self = Self(3);
    pub const NONE: Self = Self(4);
}

impl Default for LevelEdgeStyle {
    fn default() -> Self {
        Self::DEFAULT
    }
}