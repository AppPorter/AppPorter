#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app_porter_lib::{command, configs::*, core::*, operations::*};
use std::error::Error;
use tauri::Manager;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    Env::read().await?;
    Settings::initialize().await?;

    if let Err(e) = get_7z_path() {
        eprintln!("Failed to extract 7z.exe: {}", e);
    }

    tauri::Builder::default()
        .setup(|app| {
            let window = app.get_webview_window("main").expect("no main window");

            let args: Vec<String> = std::env::args().collect();
            if !args.contains(&"--silent".to_owned()) {
                window.show()?;
                window.unminimize()?;
                window.set_focus()?;
            }

            let handle = app.handle().clone();
            tokio::spawn(async move {
                if let Err(e) = start_websocket_server(handle).await {
                    eprintln!("WebSocket server error: {}", e);
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_single_instance::init(move |app, args, _| {
            tokio::spawn(async move {
                CHANNEL.0.send(args).expect("Failed to send args");
            });

            let window = app.get_webview_window("main").expect("no main window");
            window.show().expect("window failed to show");
            window.unminimize().expect("window failed to unminimize");
            window.set_focus().expect("window failed to focus");
        }))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .plugin(tauri_plugin_clipboard_manager::init())
        .invoke_handler(tauri::generate_handler![command::execute_command])
        .run(tauri::generate_context!())?;
    Ok(())
}
