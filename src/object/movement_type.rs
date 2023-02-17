use serde::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct MovementType(i32);

impl MovementType {
    pub const NONE: Self = Self(0);
    pub const NORTH_LEFT: Self = Self(41);
    pub const NORTH_RIGHT: Self = Self(42);
    pub const EAST_LEFT: Self = Self(43);
    pub const EAST_RIGHT: Self = Self(44);
    pub const SOUTH_LEFT: Self = Self(45);
    pub const SOUTH_RIGHT: Self = Self(46);
    pub const WEST_LEFT: Self = Self(47);
    pub const WEST_RIGHT: Self = Self(48);
}