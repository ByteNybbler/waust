use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EulerAngles {
    pitch: f32,
    yaw: f32,
    roll: f32
}