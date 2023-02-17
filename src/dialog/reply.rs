use crate::*;
use serde::*;

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Reply {
    text: String,
    fnc: ReplyFunction,
    cmd: Cmd
}

impl Reply {
    pub fn new(text: String, fnc: ReplyFunction, cmd: Cmd) -> Self {
        Self {
            text,
            fnc,
            cmd
        }
    }

    pub fn none(text: String) -> Self {
        Self::new(text, ReplyFunction::none(), Cmd::none())
    }

    pub fn end(destination_interchange: i32, text: String, cmd: Cmd) -> Self {
        Self::new(text, ReplyFunction::end(destination_interchange), cmd)
    }

    pub fn continue_to(destination_interchange: i32, text: String, cmd: Cmd) -> Self {
        Self::new(text, ReplyFunction::continue_to(destination_interchange), cmd)
    }

    pub fn start_askabout(text: String, cmd: Cmd) -> Self {
        Self::new(text, ReplyFunction::start_askabout(), cmd)
    }

    pub fn consume_coins(quantity: i32, text: String, cmd: Cmd) -> Self {
        Self::new(text, ReplyFunction::consume_coins(quantity), cmd)
    }

    pub fn consume_item(fnc_id: i32, text: String, cmd: Cmd) -> Self {
        Self::new(text, ReplyFunction::consume_item(fnc_id), cmd)
    }

    pub fn check_for_item(fnc_id: i32, text: String, cmd: Cmd) -> Self {
        Self::new(text, ReplyFunction::check_for_item(fnc_id), cmd)
    }

    pub fn find_and_replace(&mut self, target: &str, replacement: &str) {
        self.text = self.text.replace(target, replacement);
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn fnc(&self) -> ReplyFunction {
        self.fnc
    }

    pub fn cmd(&self) -> Cmd {
        self.cmd
    }
}