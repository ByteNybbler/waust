pub mod movement_type;
pub use movement_type::*;
pub mod object_logic;
pub use object_logic::*;
pub mod text_data;
pub use text_data::*;

use crate::*;
use serde::{Serialize, Deserialize};
use std::fs::File;
use std::fmt;
use std::io::prelude::*;
use std::io::SeekFrom;
use std::path::Path;

#[derive(Serialize, Deserialize, Debug, Default)]
pub struct EulerAngles {
    pitch: f32,
    yaw: f32,
    roll: f32
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Object {
    model_name: String,
    texture_name: String,
    x_scale: f32,
    y_scale: f32,
    z_scale: f32,
    x_adjust: f32,
    y_adjust: f32,
    z_adjust: f32,
    rotation_adjust: EulerAngles,
    x: f32,
    y: f32,
    z: f32,
    old_x: f32,
    old_y: f32,
    old_z: f32,
    dx: f32,
    dy: f32,
    dz: f32,
    rotation: EulerAngles,
    rotation2: EulerAngles,
    x_goal: f32,
    y_goal: f32,
    z_goal: f32,
    movement_type: MovementType,
    movement_type_data: i32,
    speed: f32,
    radius: f32,
    radius_type: i32,
    data10: i32,
    push_dx: f32,
    push_dy: f32,
    attack_power: i32,
    defense_power: i32,
    destruction_type: i32,
    id: i32,
    logic: ObjectLogic,
    active: i32,
    last_active: i32,
    activation_type: i32,
    activation_speed: i32,
    status: i32,
    timer: i32,
    timer_max1: i32,
    timer_max2: i32,
    teleportable: i32,
    button_push: i32,
    water_react: i32,
    telekinesisable: i32,
    freezable: i32,
    reactive: i32,
    child: i32,
    parent: i32,
    data: [i32; 10],
    text_data: TextData,
    talkable: i32,
    current_anim: i32,
    standard_anim: i32,
    tile_x: i32,
    tile_y: i32,
    tile_x2: i32,
    tile_y2: i32,
    movement_timer: i32,
    movement_speed: i32,
    move_x_goal: i32,
    move_y_goal: i32,
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
    future_strings: [String; 2]
}

impl Object {
    pub fn new(model_name: String, texture_name: String, logic: ObjectLogic) -> Object {
        Self {
            model_name,
            texture_name,
            x_scale: 1.0,
            y_scale: 1.0,
            z_scale: 1.0,
            x_adjust: 0.0,
            y_adjust: 0.0,
            z_adjust: 0.0,
            rotation_adjust: EulerAngles::default(),
            x: 0.0,
            y: 0.0,
            z: 0.0,
            old_x: -999.0,
            old_y: -999.0,
            old_z: -999.0,
            dx: 0.0,
            dy: 0.0,
            dz: 0.0,
            rotation: EulerAngles::default(),
            rotation2: EulerAngles::default(),
            x_goal: 0.0,
            y_goal: 0.0,
            z_goal: 0.0,
            movement_type: MovementType::NONE,
            movement_type_data: 0,
            speed: 0.0,
            radius: 0.0,
            radius_type: 0,
            data10: 0,
            push_dx: 0.0,
            push_dy: 0.0,
            attack_power: 0,
            defense_power: 0,
            destruction_type: 0,
            id: -1,
            logic,
            active: 1001,
            last_active: 0,
            activation_type: 0,
            activation_speed: 0,
            status: 0,
            timer: 0,
            timer_max1: 0,
            timer_max2: 0,
            teleportable: 0,
            button_push: 0,
            water_react: 0,
            telekinesisable: 0,
            freezable: 0,
            reactive: 1,
            child: -1,
            parent: -1,
            data: [0; 10],
            text_data: TextData::new(),
            talkable: 0,
            current_anim: 0,
            standard_anim: 0,
            tile_x: 0,
            tile_y: 0,
            tile_x2: 0,
            tile_y2: 0,
            movement_timer: 0,
            movement_speed: 0,
            move_x_goal: 0,
            move_y_goal: 0,
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
            future_strings: Default::default()
        }
    }

    pub fn set_logic(&mut self, logic: ObjectLogic) {
        self.logic = logic;
    }

    pub fn set_model_name(&mut self, model_name: String) {
        self.model_name = model_name;
    }

    pub fn set_texture_name(&mut self, texture_name: String) {
        self.texture_name = texture_name;
    }

    pub fn set_tile_type_collision(&mut self, tile_type_collision: TileTypeCollision) {
        self.tile_type_collision = tile_type_collision;
    }

    pub fn set_object_type_collision(&mut self, object_type_collision: ObjectTypeCollision) {
        self.object_type_collision = object_type_collision;
    }

    pub fn set_last_active(&mut self, last_active: i32) {
        self.last_active = last_active;
    }

    pub fn set_scale_adjust(&mut self, scale_adjust: f32) {
        self.scale_adjust = scale_adjust;
    }

    pub fn set_tile_pos(&mut self, tile_x: i32, tile_y: i32) {
        self.tile_x = tile_x;
        self.tile_y = tile_y;
    }

    pub fn set_2d_pos(&mut self, x: f32, y: f32) {
        self.x = x;
        self.y = y;
    }

    pub fn set_2d_delta(&mut self, dx: f32, dy: f32) {
        self.dx = dx;
        self.dy = dy;
    }

    pub fn set_timer(&mut self, timer: i32) {
        self.timer = timer;
    }

    pub fn set_status(&mut self, status: i32) {
        self.status = status;
    }

    pub fn zero_data(&mut self) {
        for data in &mut self.data {
            *data = 0;
        }
    }

    pub fn set_data(&mut self, index: usize, value: i32) {
        self.data[index] = value;
    }

    pub fn set_movement_type(&mut self, value: MovementType) {
        self.movement_type = value;
    }

    pub fn set_movement_speed(&mut self, value: i32) {
        self.movement_speed = value;
    }

    pub fn set_button_push(&mut self, value: i32) {
        self.button_push = value;
    }

    pub fn to_wop(&self, filename: &str) -> Result<(), Error> {
        let file = File::create(format!("{}.wop", filename)).map_err(Error::InputOutput)?;
        serde_blitz3d::to_writer(file, self).map_err(Error::Serde)
    }

    pub fn from_wop<P: AsRef<Path>>(path: P) -> Result<Object, Error> {
        let file = File::open(path).map_err(Error::InputOutput)?;
        serde_blitz3d::from_reader(file).map_err(Error::Serde)
    }

    pub fn from_wlv<P: AsRef<Path>>(path: P, start: u64) -> Result<Object, Error> {
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