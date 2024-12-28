mod process;
pub mod settings;

//use mslnk::ShellLink;
use serde::{Deserialize, Serialize};
use std::{error::Error, path::Path, result::Result};
//use windows_registry::*;
use zip::read::ZipArchive;
//use zip_extensions::read::ZipArchiveExtensions;

#[derive(Debug, Serialize, Deserialize)]
struct ZipEntry {
    name: String,
    size: u64,
}

fn preview_zip(zip_path: &Path) -> Result<Vec<ZipEntry>, Box<dyn Error>> {
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
