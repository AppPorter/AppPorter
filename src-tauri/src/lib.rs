#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

use mslnk::ShellLink; //https://crates.io/crates/mslnk
use serde::{Deserialize, Serialize};
use std::{path::Path, result::Result};
use windows_registry::*; //https://crates.io/crates/windows-registry
use zip::read::ZipArchive; //https://crates.io/crates/zip

#[derive(Debug, Serialize, Deserialize)]
struct ZipEntry {
    name: String,
    size: u64,
}

fn preview_zip(zip_path: &Path) -> Result<Vec<ZipEntry>, Box<dyn std::error::Error>> {
    let file = std::fs::File::open(zip_path)?;
    let mut archive = ZipArchive::new(file)?;
    let mut file_list = Vec::new();
    for i in 0..archive.len() {
        let file = archive.by_index(i)?;
        let file_entry = ZipEntry {
            name: file.name().to_owned(),
            size: file.size(),
        };
        file_list.push(file_entry)
    }
    Ok(file_list)
}

fn create_shortcut(
    target_path: &Path,
    shortcut_path: &Path,
) -> Result<(), Box<dyn std::error::Error>> {
    let link = ShellLink::new(target_path)?;
    link.create_lnk(shortcut_path)?;
    Ok(())
}
