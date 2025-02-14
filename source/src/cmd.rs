use std::{env, path::PathBuf, vec};

use crossterm::style::Stylize;
use ratatui::widgets::ListState;

#[derive(Debug)]
pub struct Cmd {
    pub input: String,
    pub history: Vec<Vec<String>>,
    pub current_dir: PathBuf,
    pub state: ListState,
    /// option list of tab
    optional_list: Option<vec::IntoIter<PathBuf>>,
}

impl Cmd {
    pub fn new() -> Self {
        Self {
            input: String::new(),
            history: vec![vec![String::new()]],
            current_dir: std::env::current_dir().unwrap(),
            state: ListState::default(),
            optional_list: None,
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
        self.state.select_last();
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

    pub fn handle_tab(&mut self) {
        let mut parts: Vec<&str> = self.input.trim().split_whitespace().collect();

        if parts.len() <= 1 {
            return;
        }

        // base on the last word to prefix match
        if let Some(iter) = self.optional_list.as_mut() {
            if let Some(candidate) = iter.next() {
                if let Some(file_name_os) = candidate.file_name() {
                    if let Some(file_name) = file_name_os.to_str() {
                        *parts.last_mut().unwrap() = file_name;
                        self.input = parts.join(" ");
                        return;
                    }
                }
            }
        } else {
            self.optional_list =
                Some(prefix_match_files_in_curr_dir(parts[parts.len() - 1]).into_iter());
        }
    }

    pub fn display_in_last_history(&mut self, str: String) {
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

fn prefix_match_files_in_curr_dir(prefix: &str) -> Vec<PathBuf> {
    let mut match_list = Vec::new();

    for entry in get_files_in_curr_dir() {
        if entry
            .file_name()
            .unwrap()
            .to_string_lossy()
            .to_lowercase()
            .starts_with(&prefix.to_lowercase())
        {
            match_list.push(entry);
        }
    }

    match_list
}
