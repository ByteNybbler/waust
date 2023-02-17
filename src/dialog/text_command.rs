use serde::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct TextCommand {
    name: String,
    position: i32
}

impl TextCommand {
    pub fn new(name: String, position: i32) -> Self {
        Self {
            name,
            position
        }
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn position(&self) -> i32 {
        self.position
    }
}