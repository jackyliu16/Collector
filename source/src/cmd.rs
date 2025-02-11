use std::{env, fmt::format, path::PathBuf, process};

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
            _ => self
                .history
                .last_mut()
                .unwrap()
                .push(String::from("Unrecognized Command")),
        }

        self.input.clear();
    }

    fn handle_cd(&mut self, parts: &[&str]) {
        if let Some(path) = parts.get(1) {
            if let Ok(new_dir) = env::set_current_dir(path) {
                self.current_dir = env::current_dir().unwrap();
                self.history
                    .last_mut()
                    .unwrap()
                    .push(format!("cd to: {}", self.current_dir.to_str().unwrap()));
            }
        }
    }

    fn handle_ls(&mut self, parts: &[&str]) {
        let output = if cfg!(target_os = "windwos") {
            std::process::Command::new("cmd")
                .args(&["/C", "dir"])
                .output()
                .expect("failed to execute ls")
        } else {
            std::process::Command::new("ls")
                .output()
                .expect("failed to execute ls")
        };

        let a = String::from_utf8(output.stdout).unwrap_or("Error".into());

        for item in a.split("\n") {
            self.history
                .last_mut()
                .unwrap()
                .push(format!("  {}", item.trim()));
        }
    }
}
