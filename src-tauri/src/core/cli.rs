use crate::SUPPORTED_EXTENSIONS;
use lazy_static::lazy_static;
use std::error::Error;
use tauri::{AppHandle, Emitter};
use tokio::sync::broadcast;

lazy_static! {
    pub static ref CHANNEL: (
        broadcast::Sender<Vec<String>>,
        broadcast::Receiver<Vec<String>>
    ) = broadcast::channel(1);
}

pub async fn cli(app: AppHandle) -> Result<String, Box<dyn Error + Send + Sync>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 3 || args.len() == 4 {
        cases(app.clone(), args).await?;
    }

    let mut receiver = CHANNEL.1.resubscribe();
    loop {
        if let Ok(new_args) = receiver.recv().await {
            if new_args.len() == 3 || new_args.len() == 4 {
                cases(app.clone(), new_args).await?;
            }
        }
    }

    async fn cases(app: AppHandle, args: Vec<String>) -> Result<(), Box<dyn Error + Send + Sync>> {
        match args[1].as_str() {
            "install" => {
                let value = args[2].clone();
                if let Some(extension) = std::path::Path::new(&value)
                    .extension()
                    .and_then(|ext| ext.to_str())
                {
                    if SUPPORTED_EXTENSIONS.contains(&extension.to_lowercase().as_str()) {
                        if args.len() == 4 {
                            if let Ok(timestamp) = args[3].parse::<i64>() {
                                if timestamp > 0 {
                                    app.emit("install", (value, timestamp))?;
                                } else {
                                    app.emit("install", (value, chrono::Utc::now().timestamp()))?;
                                }
                            }
                        } else {
                            app.emit("install", (value, chrono::Utc::now().timestamp()))?;
                        }
                    }
                }
            }
            "install_app" => {
                let value = args[2].clone();
                if let Some(extension) = std::path::Path::new(&value)
                    .extension()
                    .and_then(|ext| ext.to_str())
                {
                    if SUPPORTED_EXTENSIONS.contains(&extension.to_lowercase().as_str()) {
                        if let Some(timestamp) = args[3].parse::<i64>().ok() {
                            if timestamp > 0 {
                                app.emit("install_app", (value, timestamp))?;
                            } else {
                                app.emit("install_app", (value, chrono::Utc::now().timestamp()))?;
                            }
                        }
                    }
                }
            }
            "install_lib" => {
                let value = args[2].clone();
                if let Some(extension) = std::path::Path::new(&value)
                    .extension()
                    .and_then(|ext| ext.to_str())
                {
                    if SUPPORTED_EXTENSIONS.contains(&extension.to_lowercase().as_str()) {
                        if let Some(timestamp) = args[3].parse::<i64>().ok() {
                            if timestamp > 0 {
                                app.emit("install_lib", (value, timestamp))?;
                            } else {
                                app.emit("install_lib", (value, chrono::Utc::now().timestamp()))?;
                            }
                        }
                    }
                }
            }
            "uninstall_app" => {
                let value = args[2].clone();
                if let Ok(timestamp) = value.parse::<i64>() {
                    app.emit("uninstall_app", timestamp)?;
                }
            }
            _ => {}
        }
        Ok(())
    }
}
