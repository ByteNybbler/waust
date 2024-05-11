use crate::ColorRgb;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LightGoal {
    pub goal: ColorRgb<i32>,
    pub change_speed: i32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct LightGoals {
    pub light: LightGoal,
    pub ambient: LightGoal,
}