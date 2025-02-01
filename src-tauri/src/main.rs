#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app_porter_lib::command;
use std::error::Error;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        #[cfg(debug_assertions)]
        eprintln!("Application error: {}", e);
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    tauri::Builder::default()
        .plugin(tauri_plugin_clipboard_manager::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![command::execute_command])
        .run(tauri::generate_context!())?;
    Ok(())
}
