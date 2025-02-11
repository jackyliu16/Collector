use std::{env, fmt::format, path::PathBuf, process, thread::panicking};

use crossterm::style::Stylize;
use ratatui::{
    style::Style,
    widgets::{Block, List, ListState, Paragraph, Widget},
};

#[derive(Debug)]
pub struct Cmd {
    pub input: String,
    pub history: Vec<Vec<String>>,
    pub current_dir: PathBuf,
    pub state: ListState,
}

impl Cmd {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            history: vec![vec![String::new()]],
            current_dir: std::env::current_dir().unwrap(),
            state: ListState::default(),
        }
    }

    pub fn execute_command(&mut self, cmd: String) {
        let parts: Vec<&str> = cmd.split_whitespace().collect();
        self.history.push(vec![cmd.clone()]);

        if parts.is_empty() {
            return;
        }

        match parts[0] {
            "ls" => self.handle_ls(&parts),
            "cd" => self.handle_cd(&parts),
            _ => self.display_in_last_history("Unrecognized Command".to_owned()),
        }

        self.input.clear();
    }

    fn handle_cd(&mut self, parts: &[&str]) {
        if let Some(path) = parts.get(1) {
            if let Ok(()) = env::set_current_dir(path) {
                self.current_dir = env::current_dir().unwrap();
                self.display_in_last_history(format!(
                    "cd to: {}",
                    self.current_dir.to_str().unwrap()
                ));
            }
        }
    }

    fn handle_ls(&mut self, parts: &[&str]) {
        if parts.len() >= 2 {
            self.display_in_last_history("currently not support extra param".yellow().to_string());
        }
        let file_names = get_files_in_curr_dir();

        for file in &file_names {
            self.display_in_last_history(format!(
                "  {}",
                file.file_name().unwrap().to_str().unwrap()
            ));
        }
    }

    fn display_in_last_history(&mut self, str: String) {
        self.history.last_mut().unwrap().push(str);
    }
}

fn get_files_in_curr_dir() -> Vec<PathBuf> {
    let entries = std::fs::read_dir(".").expect("Failure to read current directory");
    let mut file_names: Vec<PathBuf> = Vec::new();
    for entry in entries {
        let entry = entry.expect("Failure to read item");
        let path = entry.path();
        file_names.push(path);
    }
    file_names
}
