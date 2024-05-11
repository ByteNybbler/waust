use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LevelTexture(String);

impl LevelTexture {
    pub fn standard_abstract() -> Self { Self::standard("abstract") }
    pub fn standard_hills() -> Self { Self::standard("hills") }

    fn standard(name: &str) -> Self {
        Self(format!("leveltex {name}.bmp"))
    }

    pub fn custom(name: String) -> Self {
        Self(name)
    }
}

impl Default for LevelTexture {
    fn default() -> Self {
        Self::standard_abstract()
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct WaterTexture(String);

impl WaterTexture {
    pub fn standard_1() -> Self { Self::standard("1") }
    pub fn standard_2() -> Self { Self::standard("2") }
    pub fn standard_3() -> Self { Self::standard("3") }
    pub fn standard_4() -> Self { Self::standard("4") }

    fn standard(name: &str) -> Self {
        Self(format!("watertex {name}.jpg"))
    }

    pub fn custom(name: String) -> Self {
        Self(name)
    }
}

impl Default for WaterTexture {
    fn default() -> Self {
        Self::standard_1()
    }
}