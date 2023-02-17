use crate::*;
use serde::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Body {
    text_lines: Vec<String>,
    text_commands: Vec<TextCommand>
}

impl Body {
    pub fn new(text_lines: Vec<String>, text_commands: Vec<TextCommand>) -> Self {
        Self {
            text_lines,
            text_commands
        }
    }

    pub fn plain_text(text_lines: Vec<String>) -> Self {
        Self::new(text_lines, vec![])
    }

    pub fn one_liner(text_line: String) -> Self {
        Self::plain_text(vec![text_line])
    }

    pub fn text_lines(&self) -> &[String] {
        &self.text_lines
    }

    pub fn text_lines_mut(&mut self) -> &mut [String] {
        &mut self.text_lines
    }

    pub fn text_commands(&self) -> &[TextCommand] {
        &self.text_commands
    }

    pub fn find_and_replace(&mut self, target: &str, replacement: &str) {
        for text in &mut self.text_lines {
            *text = text.replace(target, replacement);
        }
    }
}