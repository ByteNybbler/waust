use std::marker::PhantomData;

use crate::*;
use serde::{Serialize, Deserialize, Deserializer, de::{self, Visitor, SeqAccess}};

/// The type parameter T determines the data format of the level tiles,
/// which varies between game versions.
#[derive(Serialize, Debug)]

pub struct Terrain<T> {
    width: i32,
    height: i32,
    level_tiles: Vec<T>,
    water_tiles: Vec<WaterTile>,
}

impl From<Terrain<LevelTileBeta>> for Terrain<LevelTileLatest> {
    fn from(value: Terrain<LevelTileBeta>) -> Self {
        Self {
            width: value.width,
            height: value.height,
            level_tiles: value.level_tiles.into_iter().map(|tile| tile.into()).collect(),
            water_tiles: value.water_tiles,
        }
    }
}

impl<'de, T> Deserialize<'de> for Terrain<T>
where
    T: Deserialize<'de>
{
    fn deserialize<D>(deserializer: D) -> Result<Terrain<T>, D::Error>
    where
        D: Deserializer<'de>
    {
        deserializer.deserialize_struct("LevelContent", &[
            "width",
            "height",
            "level_tiles",
            "water_tiles",
        ], TerrainVisitor::<T>::new())
    }
}

pub struct TerrainVisitor<T> {
    phantom: PhantomData<T>
}

impl<T> TerrainVisitor<T> {
    pub fn new() -> Self {
        Self {
            phantom: PhantomData
        }
    }
}

impl<'de, T> Visitor<'de> for TerrainVisitor<T>
where
    T: Deserialize<'de>
{
    type Value = Terrain<T>;

    fn expecting(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "struct Terrain")
    }

    fn visit_seq<V>(self, mut seq: V) -> Result<Terrain<T>, V::Error>
    where
        V: SeqAccess<'de>
    {
        let width: i32 = seq.next_element()?.ok_or_else(|| de::Error::custom("unexpected end of sequence"))?;
        let height: i32 = seq.next_element()?.ok_or_else(|| de::Error::custom("unexpected end of sequence"))?;
        let total = width as usize * height as usize;
        let mut level_tiles = Vec::with_capacity(total);
        for _ in 0..total {
            let tile = seq.next_element()?.ok_or_else(|| de::Error::custom("unexpected end of sequence"))?;
            level_tiles.push(tile);
        }
        let mut water_tiles = Vec::with_capacity(total);
        for _ in 0..total {
            let tile = seq.next_element()?.ok_or_else(|| de::Error::custom("unexpected end of sequence"))?;
            water_tiles.push(tile);
        }
        Ok(Terrain {
            width,
            height,
            level_tiles,
            water_tiles,
        })
    }
}