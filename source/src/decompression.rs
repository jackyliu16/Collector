//! THEORETICAL FOUNDATION
//!
//! The head of the file(Internal metadata) contains servious magic number which could be use to identify the format of file.

// MAGIC NUMBER OF 0xSUPPORTED PARSE DECOMPRESSION FILE FORMATS
//
// source: https://devtool.tech/filetype
// const MAGIC_RAR: &[u8] = &[0x52, 0x61, 0x72, 0x21];
// const MAGIC_7Z: &[u8] = &[0x37, 0x7A, 0xBC, 0xAF, 0x27, 0x1C];

use tokio::fs::File;

use crate::app::App;
use std::path::Path;

use zip::ZipArchive;

pub fn auto_decompress_file(app: &mut App, file_path: &Path, password: Option<&str>) -> bool {
    if let Ok(Some(type_name)) = infer::get_from_path(&file_path) {
        // https://crates.io/crates/infer
        match type_name.mime_type() {
            "application/zip" => return decompress_zip(app, file_path, password),
            _ => app.cmd.display_in_last_history(format!(
                "[Error] Currently format {} haven't been support yet",
                type_name.mime_type()
            )),
        }
    }
    false
}

fn decompress_zip(app: &mut App, file_path: &Path, password: Option<&str>) -> bool {
    app.cmd
        .display_in_last_history(format!("Identify as zip Archieve"));
    todo!();
}
