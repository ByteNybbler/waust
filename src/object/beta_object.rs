use crate::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct BetaObject {
    ancient_object: AncientObject,
    future_int_1: i32,
    future_int_2: i32,
    future_int_3: i32,
    future_int_4: i32,
    future_int_5: i32,
    future_float_1: f32,
    future_float_2: f32,
    future_float_3: f32,
    future_float_4: f32,
    future_float_5: f32,
    future_string_1: String,
    future_string_2: String,
}

impl From<BetaObject> for GameObject {
    fn from(value: BetaObject) -> Self {
        Self::from_ancient_object(value.ancient_object)
    }
}