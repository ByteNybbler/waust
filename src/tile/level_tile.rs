use crate::*;
use serde::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct LevelTileBeta {
    texture: i32,
    rotation: i32,
    side_texture: i32,
    side_rotation: i32,
    random: f32,
    height: f32,
    extrusion: f32,
    rounding: Rounding,
    edge_random: EdgeRandom,
}

impl Default for LevelTileBeta {
    fn default() -> Self {
        Self {
            texture: 0,
            rotation: 0,
            side_texture: 13,
            side_rotation: 0,
            random: 0.0,
            height: 0.0,
            extrusion: 0.0,
            rounding: Rounding::SQUARE,
            edge_random: EdgeRandom::SMOOTH,
        }
    }
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct LevelTileLatest {
    beta: LevelTileBeta,
    logic: LevelTileLogic,
}

impl Default for LevelTileLatest {
    fn default() -> Self {
        Self {
            beta: Default::default(),
            logic: LevelTileLogic::FLOOR,
        }
    }
}

impl From<LevelTileBeta> for LevelTileLatest {
    fn from(beta: LevelTileBeta) -> Self {
        let logic = if beta.extrusion == 0.0 {
            LevelTileLogic::FLOOR
        } else if beta.extrusion > 0.0 {
            LevelTileLogic::WALL
        } else {
            LevelTileLogic::WATER
        };
        Self {
            beta,
            logic,
        }
    }
}