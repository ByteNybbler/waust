use crate::*;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::fmt;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;

/// An object that exists during gameplay.
#[derive(Serialize, Deserialize, Debug)]
pub struct GameObject {
    ancient_object: AncientObject,
    talkable: i32,
    current_anim: i32,
    standard_anim: i32,
    tile_pos: Vector2<i32>,
    tile_pos2: Vector2<i32>,
    movement_timer: i32,
    movement_speed: i32,
    move_goal: Vector2<i32>,
    tile_type_collision: TileTypeCollision,
    object_type_collision: ObjectTypeCollision,
    caged: i32,
    dead: i32,
    dead_timer: i32,
    exclamation: i32,
    shadow: i32,
    linked: i32,
    link_back: i32,
    flying: i32,
    frozen: i32,
    indigo: i32,
    future_int24: i32,
    future_int25: i32,
    scale_adjust: f32,
    scale_x_adjust: f32,
    scale_y_adjust: f32,
    scale_z_adjust: f32,
    future_float5: f32,
    future_float6: f32,
    future_float7: f32,
    future_float8: f32,
    future_float9: f32,
    future_float10: f32,
    future_string_1: String,
    future_string_2: String,
}

impl GameObject {
    pub fn new(model_name: String, texture_name: String, logic: ObjectLogic) -> Self {
        Self::from_ancient_object(AncientObject::new(model_name, texture_name, logic))
    }

    pub fn from_ancient_object(ancient_object: AncientObject) -> Self {
        Self {
            ancient_object,
            talkable: 0,
            current_anim: 0,
            standard_anim: 0,
            tile_pos: Vector2::splat(0),
            tile_pos2: Vector2::splat(0),
            movement_timer: 0,
            movement_speed: 0,
            move_goal: Vector2::splat(0),
            tile_type_collision: TileTypeCollision::NONE,
            object_type_collision: ObjectTypeCollision::NONE,
            caged: 0,
            dead: 0,
            dead_timer: 0,
            exclamation: 0,
            shadow: -1,
            linked: -1,
            link_back: -1,
            flying: 0,
            frozen: 0,
            indigo: 0,
            future_int24: 0,
            future_int25: 0,
            scale_adjust: 1.0,
            scale_x_adjust: 1.0,
            scale_y_adjust: 1.0,
            scale_z_adjust: 1.0,
            future_float5: 0.0,
            future_float6: 0.0,
            future_float7: 0.0,
            future_float8: 0.0,
            future_float9: 0.0,
            future_float10: 0.0,
            future_string_1: Default::default(),
            future_string_2: Default::default(),
        }
    }

    pub fn set_logic(&mut self, logic: ObjectLogic) {
        self.ancient_object.set_logic(logic);
    }

    pub fn set_model_name(&mut self, model_name: String) {
        self.ancient_object.set_model_name(model_name);
    }

    pub fn set_texture_name(&mut self, texture_name: String) {
        self.ancient_object.set_texture_name(texture_name);
    }

    pub fn set_tile_type_collision(&mut self, tile_type_collision: TileTypeCollision) {
        self.tile_type_collision = tile_type_collision;
    }

    pub fn set_object_type_collision(&mut self, object_type_collision: ObjectTypeCollision) {
        self.object_type_collision = object_type_collision;
    }

    pub fn set_last_active(&mut self, last_active: i32) {
        self.ancient_object.set_last_active(last_active);
    }

    pub fn set_scale_adjust(&mut self, scale_adjust: f32) {
        self.scale_adjust = scale_adjust;
    }

    pub fn set_tile_pos(&mut self, pos: Vector2<i32>) {
        self.tile_pos = pos;
    }

    pub fn set_world_pos(&mut self, pos: Vector3<f32>) {
        self.ancient_object.set_world_pos(pos);
    }

    pub fn set_delta(&mut self, delta: Vector3<f32>) {
        self.ancient_object.set_delta(delta);
    }

    pub fn set_timer(&mut self, timer: i32) {
        self.ancient_object.set_timer(timer);
    }

    pub fn set_status(&mut self, status: i32) {
        self.ancient_object.set_status(status);
    }

    pub fn zero_data(&mut self) {
        self.ancient_object.zero_data();
    }

    pub fn set_data(&mut self, index: usize, value: i32) {
        self.ancient_object.set_data(index, value);
    }

    pub fn set_movement_type(&mut self, value: MovementType) {
        self.ancient_object.set_movement_type(value);
    }

    pub fn set_movement_speed(&mut self, value: i32) {
        self.movement_speed = value;
    }

    pub fn set_button_push(&mut self, value: i32) {
        self.ancient_object.set_button_push(value);
    }

    pub fn to_wop(&self, filename: &str) -> Result<(), Error> {
        let file = File::create(format!("{}.wop", filename)).map_err(Error::InputOutput)?;
        serde_blitz3d::to_writer(file, self).map_err(Error::Serde)
    }

    pub fn from_wop<P: AsRef<Path>>(path: P) -> Result<GameObject, Error> {
        let file = File::open(path).map_err(Error::InputOutput)?;
        serde_blitz3d::from_reader(file).map_err(Error::Serde)
    }

    pub fn from_wlv<P: AsRef<Path>>(path: P, start: u64) -> Result<GameObject, Error> {
        let mut file = File::open(path).map_err(Error::InputOutput)?;
        file.seek(SeekFrom::Start(start)).unwrap();
        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer).unwrap();
        serde_blitz3d::from_reader(&buffer[..]).map_err(Error::Serde)
    }
}

#[derive(Debug)]
pub enum Error {
    InputOutput(std::io::Error),
    Serde(serde_blitz3d::Error)
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Error::InputOutput(error) => write!(f, "{}", error),
            Error::Serde(error) => write!(f, "{}", error)
        }
    }
}

impl std::error::Error for Error { }