use serde::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AskAbout {
    pub text: String,
    pub active: i32,
    pub interchange: i32,
    pub repeat: i32
}

impl AskAbout {
    pub fn new(text: String, active: i32, interchange: i32, repeat: i32) -> Self {
        Self {
            text,
            active,
            interchange,
            repeat
        }
    }

    pub fn find_and_replace(&mut self, target: &str, replacement: &str) {
        self.text = self.text.replace(target, replacement);
    }
}