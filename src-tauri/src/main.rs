#![cfg_attr(
    not(debug_assertions),
    windows_subsystem = "windows",
    target_os = "windows"
)]

mod settings;
use settings::Settings;
use std::{error::Error, result::Result};

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            let e = e.to_string();
            #[cfg(debug_assertions)]
            eprintln!("{}", e);
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn run() -> Result<(), Box<dyn Error>> {
    Settings::read()?.complete()?;
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![])
        .run(tauri::generate_context!())?;
    Ok(())
}
