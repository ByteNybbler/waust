use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct ObjectLogic {
    object_type: i32,
    object_subtype: i32
}

impl ObjectLogic {
    pub const NONE: Self = Self::new(0, 0);

    pub const fn new(object_type: i32, object_subtype: i32) -> ObjectLogic {
        Self {
            object_type,
            object_subtype
        }
    }

    pub fn spellball(color: i32) -> ObjectLogic {
        Self::new(50, color)
    }
}