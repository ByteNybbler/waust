use crate::*;
use serde::*;

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct Interchange {
    body: Body,
    replies: Vec<Reply>
}

impl Interchange {
    pub fn new(body: Body, replies: Vec<Reply>) -> Self {
        Self {
            body,
            replies
        }
    }

    pub fn placeholder(text_line: String) -> Self {
        Self::new(Body::one_liner(text_line), vec![])
    }

    pub fn plain_text(body: Vec<String>) -> Self {
        Self::new(Body::plain_text(body), vec![])
    }

    pub fn plain_text_with_reply(body: Vec<String>, reply: Reply) -> Self {
        Self::new(Body::plain_text(body), vec![reply])
    }

    pub fn plain_text_with_replies(body: Vec<String>, replies: Vec<Reply>) -> Self {
        Self::new(Body::plain_text(body), replies)
    }

    pub fn body_text_lines(&self) -> &[String] {
        self.body.text_lines()
    }

    pub fn body_text_lines_mut(&mut self) -> &mut [String] {
        self.body.text_lines_mut()
    }

    pub fn body_text_commands(&self) -> &[TextCommand] {
        self.body.text_commands()
    }

    pub fn replies(&self) -> &[Reply] {
        &self.replies
    }

    pub fn add_reply(&mut self, reply: Reply) {
        self.replies.push(reply);
    }

    pub fn find_and_replace(&mut self, target: &str, replacement: &str) {
        self.body.find_and_replace(target, replacement);
        for reply in &mut self.replies {
            reply.find_and_replace(target, replacement);
        }
    }
}