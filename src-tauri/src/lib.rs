use std::{env, fs, io, path::PathBuf};

pub mod command;
pub mod configs;
pub mod operations;
pub mod websocket;

// Function to get path to 7z.exe, creating it if necessary
pub fn get_7z_path() -> Result<PathBuf, io::Error> {
    let seven_zip_bytes = include_bytes!("../resources/7z.exe");
    let temp_dir = env::temp_dir().join("AppPorter");
    fs::create_dir_all(&temp_dir)?;
    let seven_zip_path = temp_dir.join("7z.exe");

    // Only write if file doesn't exist or different size
    if !seven_zip_path.exists()
        || fs::metadata(&seven_zip_path)?.len() != seven_zip_bytes.len() as u64
    {
        fs::write(&seven_zip_path, seven_zip_bytes)?;
    }

    Ok(seven_zip_path)
}
