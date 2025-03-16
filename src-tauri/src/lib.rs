use std::{env, fs, io, path::PathBuf};

pub mod command;
pub mod configs;
pub mod operations;
pub mod websocket;

// Function to get path to 7z.exe, creating both exe and dll if necessary
pub fn get_7z_path() -> Result<PathBuf, io::Error> {
    let temp_dir = env::temp_dir().join("AppPorter");
    fs::create_dir_all(&temp_dir)?;

    // Extract 7z.dll
    let seven_zip_dll_bytes = include_bytes!("../resources/7z.dll");
    let seven_zip_dll_path = temp_dir.join("7z.dll");
    if !seven_zip_dll_path.exists()
        || fs::metadata(&seven_zip_dll_path)?.len() != seven_zip_dll_bytes.len() as u64
    {
        fs::write(&seven_zip_dll_path, seven_zip_dll_bytes)?;
    }

    // Extract 7z.exe
    let seven_zip_bytes = include_bytes!("../resources/7z.exe");
    let seven_zip_path = temp_dir.join("7z.exe");
    if !seven_zip_path.exists()
        || fs::metadata(&seven_zip_path)?.len() != seven_zip_bytes.len() as u64
    {
        fs::write(&seven_zip_path, seven_zip_bytes)?;
    }

    Ok(seven_zip_path)
}
