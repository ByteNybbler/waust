use serde::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Rounding(i32);

impl Rounding {
    pub const SQUARE: Self = Self(0);
    pub const ROUND: Self = Self(1);
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct EdgeRandom(i32);

impl EdgeRandom {
    pub const SMOOTH: Self = Self(0);
    pub const JAGGED: Self = Self(1);
}