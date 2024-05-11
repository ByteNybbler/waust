use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct Weather(i32);

impl Weather {
    pub const NONE: Self = Self(0);
}