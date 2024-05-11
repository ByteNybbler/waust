use serde::*;

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct LevelTileLogic(i32);

impl LevelTileLogic {
    pub const FLOOR: Self = Self(0);
    pub const WALL: Self = Self(1);
    pub const WATER: Self = Self(2);
    pub const TELEPORTER: Self = Self(3);
    pub const BRIDGE: Self = Self(4);
    pub const LAVA: Self = Self(5);
    pub const SIX: Self = Self(6);
    pub const SEVEN: Self = Self(7);
    pub const CAGE: Self = Self(8);
    pub const BUTTON: Self = Self(9);
    pub const STINKER_EXIT: Self = Self(10);
    pub const ICE_STRAIGHT: Self = Self(11);
    pub const ICE_CURVED: Self = Self(12);
    pub const ICE_WALL: Self = Self(13);
    pub const ICE_FLOAT: Self = Self(14);
}

#[derive(Debug, Serialize, Deserialize, Clone, Copy)]
pub struct ObjectTileLogic(i32);

impl ObjectTileLogic {
    pub const PLAYER: Self = Self(1);
    pub const NPC_TALKABLE: Self = Self(2);
    pub const WEE_STINKER: Self = Self(3);
    pub const ITEM: Self = Self(4);
    pub const SCRITTER_TENTACLE: Self = Self(5);
    pub const BRIDGE: Self = Self(6);
    pub const NPC_UNTALKABLE: Self = Self(7);
    pub const ENEMY: Self = Self(8);
    pub const BARREL: Self = Self(9);
    pub const FROZEN: Self = Self(10);
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TileTypeCollision(i32);

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectTypeCollision(i32);

impl TileTypeCollision {
    pub const NONE: Self = Self(0);

    pub fn liquids() -> Self {
        Self::new([LevelTileLogic::WATER, LevelTileLogic::LAVA])
    }

    pub fn ice_floors() -> Self {
        Self::new([LevelTileLogic::ICE_STRAIGHT, LevelTileLogic::ICE_CURVED, LevelTileLogic::ICE_FLOAT])
    }

    pub fn all_ice() -> Self {
        let mut result = Self::ice_floors();
        result.add(LevelTileLogic::ICE_WALL);
        result
    }

    pub fn all_floors() -> Self {
        let mut result = Self::ice_floors();
        result.add(LevelTileLogic::FLOOR);
        result.add(LevelTileLogic::TELEPORTER);
        result.add(LevelTileLogic::BRIDGE);
        result.add(LevelTileLogic::BUTTON);
        result.add(LevelTileLogic::STINKER_EXIT);
        result
    }

    pub fn all_floors_and_06_07() -> Self {
        let mut result = Self::all_floors();
        result.add(LevelTileLogic::SIX);
        result.add(LevelTileLogic::SEVEN);
        result
    }

    pub fn all_floors_and_liquids() -> Self {
        let mut result = Self::all_floors();
        result.0 |= Self::liquids().0;
        result
    }

    pub fn spellball() -> Self {
        let mut result = Self::all_floors_and_liquids();
        result.add(LevelTileLogic::ICE_WALL);
        result
    }

    pub fn add(&mut self, level_tile_logic: LevelTileLogic) {
        self.0 |= 1 << level_tile_logic.0;
    }

    pub fn new<const N: usize>(level_tile_logics: [LevelTileLogic; N]) -> Self
    {
        let mut result = Self::NONE;
        for i in 0..N {
            result.add(level_tile_logics[i]);
        }
        result
    }
}

impl ObjectTypeCollision {
    pub const NONE: Self = Self(0);

    pub fn new<const N: usize>(object_tile_logics: [ObjectTileLogic; N]) -> TileTypeCollision
    {
        let mut result = 0;
        for i in 0..N {
            result |= 1 << object_tile_logics[i].0;
        }
        TileTypeCollision(result)
    }
}