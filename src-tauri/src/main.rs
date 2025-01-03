#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app_porter_lib::{command, settings};
use settings::load_settings;
use std::{error::Error, result::Result};

fn main() {
    match run() {
        Ok(_) => {}
        Err(e) => {
            #[cfg(debug_assertions)]
            eprintln!("{}", e.to_string());
        }
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn run() -> Result<(), Box<dyn Error>> {
    load_settings()?;
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![command::execute_command])
        .run(tauri::generate_context!())?;
    Ok(())
}
