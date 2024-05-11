use crate::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct WlvLatest {
    width: i32,
    height: i32,
    level_tiles: Vec<LevelTileLatest>,
    water_tiles: Vec<WaterTile>,
    water_flow: i32,
    water_transparent: i32,
    water_glow: i32,
    level_texture_name: LevelTexture,
    water_texture_name: WaterTexture,
    // Number of objects is written here.
    objects: Vec<WlvObject>,
    edge_style: LevelEdgeStyle,
    light: ColorRgb<i32>,
    ambient: ColorRgb<i32>,
    music: Music,
    weather: Weather,
    adventure_title: String,
    // OpenWA writes -2 here.
    // OpenWA writes WidescreenRangeLevel here.
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WlvBeta {
    width: i32,
    height: i32,
    level_tiles: Vec<LevelTileBeta>,
    water_tiles: Vec<WaterTile>,
    water_flow: i32,
    water_transparent: i32,
    water_glow: i32,
    // Number of objects is written here.
    objects: Vec<BetaObject>,
    level_tile_logics: Vec<LevelTileLogic>,
    light_goals: LightGoals,
}

impl From<WlvBeta> for WlvLatest {
    fn from(other: WlvBeta) -> Self {
        Self {
            width: other.width,
            height: other.height,
            level_tiles: other.level_tiles.into_iter().map(|tile| tile.into()).collect(),
            water_tiles: other.water_tiles,
            water_flow: other.water_flow,
            water_transparent: other.water_transparent,
            water_glow: other.water_glow,
            objects: other.objects.into_iter().map(|object| object.into()).collect(),
            level_texture_name: LevelTexture::custom("wa_beta".to_owned()),
            water_texture_name: WaterTexture::default(),
            edge_style: Default::default(),
            light: other.light_goals.light.goal,
            ambient: other.light_goals.ambient.goal,
            music: Music::NONE,
            weather: Weather::NONE,
            adventure_title: String::new(),
        }
    }
}