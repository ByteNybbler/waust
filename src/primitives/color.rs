use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct ColorRgb<T> {
    pub red: T,
    pub green: T,
    pub blue: T,
}