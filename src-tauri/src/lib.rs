pub mod command;
pub mod configs;
pub mod operations;
pub mod websocket;

use std::{env, fs, io, path::PathBuf};

pub const SUPPORTED_EXTENSIONS: [&str; 8] = ["zip", "7z", "rar", "tar", "gz", "bz2", "xz", "cab"];

// Returns path to 7z.exe, extracts both 7z.exe and 7z.dll from resources if needed
// The files will be stored in the temp directory: %TEMP%\AppPorter\
pub fn get_7z_path() -> Result<PathBuf, io::Error> {
    let temp_dir = env::temp_dir().join("AppPorter");
    fs::create_dir_all(&temp_dir)?;

    // Extract and verify 7z.dll
    let seven_zip_dll_bytes = include_bytes!("../resources/7z.dll");
    let seven_zip_dll_path = temp_dir.join("7z.dll");
    if !seven_zip_dll_path.exists()
        || fs::metadata(&seven_zip_dll_path)?.len() != seven_zip_dll_bytes.len() as u64
    {
        fs::write(&seven_zip_dll_path, seven_zip_dll_bytes)?;
    }

    // Extract and verify 7z.exe
    let seven_zip_bytes = include_bytes!("../resources/7z.exe");
    let seven_zip_path = temp_dir.join("7z.exe");
    if !seven_zip_path.exists()
        || fs::metadata(&seven_zip_path)?.len() != seven_zip_bytes.len() as u64
    {
        fs::write(&seven_zip_path, seven_zip_bytes)?;
    }

    Ok(seven_zip_path)
}
