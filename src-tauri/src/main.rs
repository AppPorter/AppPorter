#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app_porter_lib::{
    command, get_7z_path, menu, websocket::start_websocket_server, CHANNEL, SUPPORTED_EXTENSIONS,
};
use std::error::Error;
use tauri::Manager;

#[tokio::main]
async fn main() {
    if let Err(e) = run().await {
        eprintln!("Application error: {}", e);
        std::process::exit(1);
    }
}

async fn run() -> Result<(), Box<dyn Error>> {
    if let Err(e) = get_7z_path() {
        eprintln!("Failed to extract 7z.exe: {}", e);
    }
    if let Err(e) = menu::register_context_menu() {
        eprintln!("Failed to register context menu: {}", e);
    }
    tokio::spawn(async {
        if let Err(e) = start_websocket_server().await {
            eprintln!("WebSocket server error: {}", e);
        }
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_cli::init())
        .plugin(tauri_plugin_single_instance::init(
            move |app, args, _cwd| {
                let value = args[1].clone();
                if let Some(extension) = std::path::Path::new(&value)
                    .extension()
                    .and_then(|ext| ext.to_str())
                {
                    if SUPPORTED_EXTENSIONS.contains(&extension.to_lowercase().as_str()) {
                        let value_clone = value.to_string();
                        let sender = CHANNEL.0.clone();
                        tokio::spawn(async move {
                            sender.send(value_clone).unwrap();
                        });
                    }
                }

                let window = app.get_webview_window("main").expect("no main window");
                window.show().expect("window failed to show");
                window.unminimize().expect("window failed to unminimize");
                window.set_focus().expect("window failed to focus");
            },
        ))
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![command::execute_command])
        .run(tauri::generate_context!())?;
    Ok(())
}
