use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct WaterTile {
    pub texture: i32,
    pub rotation: i32,
    pub height: f32,
    pub turbulence: f32,
}