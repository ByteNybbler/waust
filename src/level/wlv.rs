use std::{fs::File, path::Path};

use crate::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub(crate) struct WlvLatest {
    terrain: Terrain<LevelTileLatest>,
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
    terrain: Terrain<LevelTileBeta>,
    water_flow: i32,
    water_transparent: i32,
    water_glow: i32,
    // Number of objects is written here.
    objects: Vec<BetaObject>,
    level_tile_logics: Vec<LevelTileLogic>,
    light_goals: LightGoals,
}

impl WlvLatest {
    pub fn to_file(&self, filename: &str) -> Result<(), Error> {
        let file = File::create(format!("{}.wlv", filename)).map_err(Error::InputOutput)?;
        serde_blitz3d::to_writer(file, self).map_err(Error::Serde)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = File::open(path).map_err(Error::InputOutput)?;
        serde_blitz3d::from_reader(file).map_err(Error::Serde)
    }
}

impl WlvBeta {
    pub fn to_file(&self, filename: &str) -> Result<(), Error> {
        let file = File::create(format!("{}.wlv", filename)).map_err(Error::InputOutput)?;
        serde_blitz3d::to_writer(file, self).map_err(Error::Serde)
    }

    pub fn from_file<P: AsRef<Path>>(path: P) -> Result<Self, Error> {
        let file = File::open(path).map_err(Error::InputOutput)?;
        serde_blitz3d::from_reader(file).map_err(Error::Serde)
    }

    pub fn modernize_to_file(self, filename: &str) -> Result<(), Error> {
        let modern: WlvLatest = self.into();
        modern.to_file(filename)
    }
}

impl From<WlvBeta> for WlvLatest {
    fn from(other: WlvBeta) -> Self {
        Self {
            terrain: other.terrain.into(),
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

pub fn convert_beta_to_wa3() -> Result<(), Error> {
    let output_folder = "WA-BETA-MODERNIZED";
    std::fs::create_dir_all(output_folder).map_err(Error::InputOutput)?;
    for entry in std::fs::read_dir("WA-BETA").map_err(Error::InputOutput)? {
        let entry = entry.map_err(Error::InputOutput)?;
        let filename = entry.file_name();
        println!("Converting {:?}", entry.path());
        let wlv = WlvBeta::from_file(entry.path())?;
        //println!("Conversion OK.");
        wlv.modernize_to_file(&format!("{output_folder}/{filename:?}"))?;
        //println!("To file OK.");
    }
    Ok(())
}