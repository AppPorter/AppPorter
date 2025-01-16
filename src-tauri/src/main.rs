#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app_porter_lib::command;
use std::{error::Error, result::Result};
use windows_elevate::elevate;

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
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![command::execute_command])
        .run(tauri::generate_context!())?;
    elevate()?;
    Ok(())
}
