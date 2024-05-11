use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Music(i32);

impl Music {
    pub const NONE: Self = Self(0);
}