use serde::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct ReplyFunction {
    id: i32,
    data: i32
}

impl ReplyFunction {
    pub fn new(id: i32, data: i32) -> Self {
        Self {
            id,
            data
        }
    }

    pub fn none() -> Self {
        Self::new(0, 0)
    }

    pub fn end(destination_interchange: i32) -> Self {
        Self::new(1, destination_interchange)
    }

    pub fn continue_to(destination_interchange: i32) -> Self {
        Self::new(2, destination_interchange)
    }

    pub fn start_askabout() -> Self {
        Self::new(3, 0)
    }

    pub fn consume_coins(quantity: i32) -> Self {
        Self::new(4, quantity)
    }

    pub fn consume_item(fnc_id: i32) -> Self {
        Self::new(5, fnc_id)
    }

    pub fn check_for_item(fnc_id: i32) -> Self {
        Self::new(6, fnc_id)
    }

    pub fn id(&self) -> i32 {
        self.id
    }

    pub fn data(&self) -> i32 {
        self.data
    }
}