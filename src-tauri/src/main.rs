#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app_porter_lib::{command, handle_error, settings};
use settings::load_settings;
use std::{error::Error, result::Result};

fn main() {
    handle_error(run());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn run() -> Result<(), Box<dyn Error>> {
    println!(
        "{:#?}",
        app_porter_lib::installation::setup::preview_zip("D:/_u/do/UniExtractRC3.zip")?
    );
    load_settings()?.initialization()?;
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![command::execute_command])
        .run(tauri::generate_context!())?;
    Ok(())
}
