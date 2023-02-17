use serde::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct Cmd {
    pub id: i32,
    pub data1: i32,
    pub data2: i32,
    pub data3: i32,
    pub data4: i32
}

impl Cmd {
    pub fn new(id: i32, data1: i32, data2: i32, data3: i32, data4: i32) -> Self {
        Cmd {
            id,
            data1,
            data2,
            data3,
            data4
        }
    }

    pub fn none() -> Self {
        Cmd::new(0, 0, 0, 0, 0)
    }

    pub fn activate(target_id: i32) -> Self {
        Cmd::new(1, target_id, 0, 0, 0)
    }

    pub fn deactivate(target_id: i32) -> Self {
        Cmd::new(2, target_id, 0, 0, 0)
    }

    pub fn toggle(target_id: i32) -> Self {
        Cmd::new(3, target_id, 0, 0, 0)
    }
}