#![cfg_attr(
    not(debug_assertions),
    windows_subsystem = "windows",
    target_os = "windows"
)]

use app_porter_lib::{command, result_process, settings};
use settings::read_settings;
use std::{error::Error, result::Result};

fn main() {
    result_process(run());
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn run() -> Result<(), Box<dyn Error>> {
    println!(
        "{:#?}",
        app_porter_lib::installation::setup::preview_zip("D:/_u/do/UniExtractRC3.zip")?
    );
    read_settings()?.complete()?;
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![command::execute_command])
        .run(tauri::generate_context!())?;
    Ok(())
}
