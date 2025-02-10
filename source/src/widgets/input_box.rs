//! A simple input widget
//!
//! 1. Could Insert and Delete Char from App::message
//!
//! TODO: support cursor move

use ratatui::buffer::Buffer;
use ratatui::layout::Rect;
use ratatui::widgets::{Block, Paragraph, Widget};

use crate::app::{App, Mode};

impl App {
    pub fn enter_char(&mut self, new_char: char) {
        self.message.push(new_char)
    }

    pub fn delete_char(&mut self) {
        let _ = self.message.pop();
    }

    pub fn submit_message(&mut self) {
        let pwd = std::mem::take(&mut self.message).trim().to_string();
        self.help_message = pwd.clone();
        crate::fs::add_record(pwd).unwrap(); // WARN: throw AleadyExists

        assert!(self.message == "");
    }

    pub fn render_input_box(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new(self.message.as_str())
            .block(Block::bordered().title("Input"))
            .render(area, buf);
    }
}
