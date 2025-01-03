use serde::{Deserialize, Serialize};
use std::{error::Error, path::Path, result::Result};
use zip::read::ZipArchive;

#[derive(Debug, Serialize, Deserialize)]
pub struct ZipEntry {
    name: String,
    size: u64,
}

pub fn preview_zip(zip_path: &str) -> Result<String, Box<dyn Error>> {
    let file = std::fs::File::open(Path::new(zip_path))?;
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
    Ok(serde_json::to_string(&file_list)?)
}
