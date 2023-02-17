use serde::*;

#[derive(Serialize, Deserialize, Debug)]
pub struct TextData {
    text_data: [String; 4]
}

impl TextData {
    pub fn new() -> TextData {
        Self {
            text_data: Default::default()
        }
    }
}