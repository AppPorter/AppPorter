#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app_porter_lib::{command, get_7z_path, websocket};
use std::error::Error;
use tauri::Manager;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

// Main application setup and initialization
async fn run() -> Result<(), Box<dyn Error>> {
    // Extract and setup 7z tools
    if let Err(e) = get_7z_path() {
        eprintln!("Failed to extract 7z.exe: {}", e);
    }

    // Start WebSocket server for browser extension communication
    tokio::spawn(async {
        if let Err(e) = websocket::start_websocket_server().await {
            eprintln!("WebSocket server error: {}", e);
        }
    });

    // Initialize Tauri with plugins and run the application
    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let window = app.get_webview_window("main").expect("no main window");
            window.show().expect("window failed to show");
            window.unminimize().expect("window failed to unminimize");
            window.set_focus().expect("window failed to focus");
        }))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![command::execute_command])
        .run(tauri::generate_context!())?;
    Ok(())
}
