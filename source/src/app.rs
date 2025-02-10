use crate::cmd::Cmd;
use ratatui::{
    style::{palette::tailwind, Stylize},
    text::Line,
};
use std::error;
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

/// Application result type.
pub type AppResult<T> = std::result::Result<T, Box<dyn error::Error>>;

/// Application.
#[derive(Debug)]
pub struct App {
    /// Is the application running?
    pub mode: Mode,
    /// counter
    pub counter: u8,
    /// selected tabs
    pub selected_tab: SelectedTab,
    /// message in input box
    pub message: String,
    /// Cmd
    pub cmd: Cmd,

    pub help_message: String,
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Mode {
    #[default]
    Editing,
    Convert,
    Exiting,
}

#[derive(Default, Debug, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum SelectedTab {
    #[default]
    #[strum(to_string = "Add Item")]
    Tab1,
    #[strum(to_string = "Convert")]
    Tab2,
}

impl SelectedTab {
    pub fn title(self) -> Line<'static> {
        format!("  {self}  ")
            .fg(tailwind::SLATE.c200)
            .bg(tailwind::BLUE.c900)
            .into()
    }
}

impl Default for App {
    fn default() -> Self {
        Self {
            mode: Mode::Editing,
            counter: 0,
            selected_tab: SelectedTab::Tab1,
            message: String::new(),
            cmd: Cmd::new(),
            help_message: String::new(),
        }
    }
}

impl App {
    /// Constructs a new instance of [`App`].
    pub fn new() -> Self {
        Self::default()
    }

    /// Handles the tick event of the terminal.
    pub fn tick(&self) {}

    /// Set running to false to quit the application.
    pub fn quit(&mut self) {
        self.mode = Mode::Exiting;
    }

    pub fn switch_mode(&mut self, mode: Mode) {
        match (self.mode, mode) {
            (Mode::Editing, Mode::Convert) => self.selected_tab = SelectedTab::Tab2,
            (Mode::Convert, Mode::Editing) => self.selected_tab = SelectedTab::Tab1,
            (_, _) => {}
        }
        self.mode = mode
    }

    pub fn enter_char(&mut self, new_char: char) {
        match self.mode {
            Mode::Editing => self.message.push(new_char),
            Mode::Convert => self.cmd.input.push(new_char),
            _ => {}
        }
    }

    pub fn delete_char(&mut self) {
        let _ = self.message.pop();
    }

    pub fn handler_key_enter(&mut self) {
        match self.mode {
            Mode::Editing => self.submit_message(),
            Mode::Convert => self.submit_cmd(),
            _ => {}
        }
    }

    fn submit_message(&mut self) {
        let pwd = std::mem::take(&mut self.message).trim().to_string();
        self.help_message = pwd.clone();
        crate::fs::add_record(pwd).unwrap(); // WARN: throw AleadyExists

        assert!(self.message == "");
    }

    fn submit_cmd(&mut self) {
        let cmd = std::mem::take(&mut self.cmd.input).trim().to_string();
        self.help_message = cmd.clone();
        self.cmd.execute_command(cmd);
    }
}
