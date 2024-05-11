use crate::*;
use serde::{Serialize, Deserialize};

/// An object saved to a WLV.
#[derive(Serialize, Deserialize, Debug)]
pub struct WlvObject {
    pub game_object: GameObject,
    pub adjusters: [String; 30],
}

impl From<GameObject> for WlvObject {
    fn from(value: GameObject) -> Self {
        Self {
            game_object: value,
            adjusters: Default::default(),
        }
    }
}

impl From<BetaObject> for WlvObject {
    fn from(value: BetaObject) -> Self {
        let value: GameObject = value.into();
        value.into()
    }
}