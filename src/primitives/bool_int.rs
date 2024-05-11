use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BoolNonzero(i32);

impl BoolNonzero {
    pub const TRUE: Self = Self(0);
    pub const FALSE: Self = Self(1);

    pub fn is_true(&self) -> bool {
        self.0 != 0
    }
}