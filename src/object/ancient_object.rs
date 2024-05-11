use crate::*;
use serde::{Serialize, Deserialize};

/// A set of common object adjusters that have been present since the beta.
#[derive(Serialize, Deserialize, Debug)]
pub struct AncientObject {
    model_name: String,
    texture_name: String,
    xyz_scale: Vector3::<f32>,
    xyz_adjust: Vector3::<f32>,
    rotation_adjust: EulerAngles,
    world_pos: Vector3::<f32>,
    old_world_pos: Vector3::<f32>,
    delta: Vector3::<f32>,
    rotation: EulerAngles,
    rotation2: EulerAngles,
    goal: Vector3::<f32>,
    movement_type: MovementType,
    movement_type_data: i32,
    speed: f32,
    radius: f32,
    radius_type: i32,
    data10: i32, // Called "CollisionPower" in the beta.
    push_delta: Vector2::<f32>,
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
}

impl AncientObject {
    pub fn new(model_name: String, texture_name: String, logic: ObjectLogic) -> Self {
        Self {
            model_name,
            texture_name,
            xyz_scale: Vector3::splat(1.0),
            xyz_adjust: Vector3::splat(0.0),
            rotation_adjust: EulerAngles::default(),
            world_pos: Vector3::splat(0.0),
            old_world_pos: Vector3::splat(-999.0),
            delta: Vector3::splat(0.0),
            rotation: EulerAngles::default(),
            rotation2: EulerAngles::default(),
            goal: Vector3::splat(0.0),
            movement_type: MovementType::NONE,
            movement_type_data: 0,
            speed: 0.0,
            radius: 0.0,
            radius_type: 0,
            data10: 0,
            push_delta: Vector2::splat(0.0),
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

    pub fn set_last_active(&mut self, last_active: i32) {
        self.last_active = last_active;
    }

    pub fn set_world_pos(&mut self, pos: Vector3<f32>) {
        self.world_pos = pos;
    }

    pub fn set_delta(&mut self, delta: Vector3<f32>) {
        self.delta = delta;
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

    pub fn set_button_push(&mut self, value: i32) {
        self.button_push = value;
    }
}