#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use app_porter_lib::{
    command, get_7z_path,
    operations::{SubCommands, CHANNEL},
    websocket::start_websocket_server,
    SUPPORTED_EXTENSIONS,
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

async fn run() -> Result<(), Box<dyn Error + Send + Sync>> {
    if let Err(e) = get_7z_path() {
        eprintln!("Failed to extract 7z.exe: {}", e);
    }
    tokio::spawn(async {
        if let Err(e) = start_websocket_server().await {
            eprintln!("WebSocket server error: {}", e);
        }
    });

    tauri::Builder::default()
        .plugin(tauri_plugin_single_instance::init(
            move |app, args, _cwd| {
                match args[1].as_str() {
                    "install" => {
                        let value = args[2].clone();
                        if let Some(extension) = std::path::Path::new(&value)
                            .extension()
                            .and_then(|ext| ext.to_str())
                        {
                            if SUPPORTED_EXTENSIONS.contains(&extension.to_lowercase().as_str()) {
                                let value_clone = value.to_string();
                                let sender = CHANNEL.0.clone();
                                tokio::spawn(async move {
                                    sender.send(SubCommands::Install(value_clone)).unwrap();
                                });
                            }
                        }
                    }
                    "uninstall" => {
                        let value = args[2].clone();
                        if let Ok(timestamp) = value.parse::<i64>() {
                            let sender = CHANNEL.0.clone();
                            tokio::spawn(async move {
                                sender.send(SubCommands::Uninstall(timestamp)).unwrap();
                            });
                        }
                    }
                    _ => {}
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
