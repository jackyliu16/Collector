//! THEORETICAL FOUNDATION
//!
//! The head of the file(Internal metadata) contains servious magic number which could be use to identify the format of file.

// MAGIC NUMBER OF 0xSUPPORTED PARSE DECOMPRESSION FILE FORMATS
//
// source: https://devtool.tech/filetype
// const MAGIC_RAR: &[u8] = &[0x52, 0x61, 0x72, 0x21];
// const MAGIC_7Z: &[u8] = &[0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C];

use std::process::Command;
use tokio::fs::File;

use crate::app::App;
use std::path::Path;

pub fn auto_decompress_file(app: &mut App, file_path: &Path, password: Option<&str>) -> bool {
    if let Ok(Some(type_name)) = infer::get_from_path(&file_path) {
        // https://crates.io/crates/infer
        match type_name.mime_type() {
            // NOTE: maybe some format we should manually implement the operation
            _ => decompress_via_bandizip(app, file_path, password),
        }
    } else {
        false
    }
}

fn decompress_via_bandizip(app: &mut App, file_path: &Path, password: Option<&str>) -> bool {
    let out = if let Some(pwd) = password {
        Command::new("Bandizip.exe")
            .arg("-x")
            .arg(file_path)
            .arg("-p:")
            .arg(pwd)
            .output()
    } else {
        Command::new("Bandizip.exe")
            .arg("-x")
            .arg(file_path)
            .output()
    };

    if !out.is_ok() {
        return false;
    }

    match out.unwrap().status.code() {
        Some(0) => {
            app.cmd
                .display_in_last_history(format!("bandizip decompress sucess"));
            true
        }
        _ => false,
    }
}
