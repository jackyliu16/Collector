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

    pub fn increment_counter(&mut self) {
        if let Some(res) = self.counter.checked_add(1) {
            self.counter = res;
        }
    }

    pub fn decrement_counter(&mut self) {
        if let Some(res) = self.counter.checked_sub(1) {
            self.counter = res;
        }
    }

    pub fn switch_mode(&mut self, mode: Mode) {
        match (self.mode, mode) {
            (_, _) => self.mode = mode,
        }
    }
}
