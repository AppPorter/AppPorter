use crate::SUPPORTED_EXTENSIONS;
use lazy_static::lazy_static;
use std::error::Error;
use tauri::{AppHandle, Emitter};
use tokio::sync::broadcast;

#[derive(Debug, Clone)]
pub enum SubCommands {
    Install(String),
    InstallWithTimestamp(String, i64),
    Uninstall(i64),
}

lazy_static! {
    pub static ref CHANNEL: (
        broadcast::Sender<SubCommands>,
        broadcast::Receiver<SubCommands>
    ) = broadcast::channel(1);
}

pub async fn cli(app: AppHandle) -> Result<String, Box<dyn Error + Send + Sync>> {
    let args: Vec<String> = std::env::args().collect();
    if args.len() == 3 {
        match args[1].as_str() {
            "install" => {
                let value = args[2].clone();
                if let Some(extension) = std::path::Path::new(&value)
                    .extension()
                    .and_then(|ext| ext.to_str())
                {
                    if SUPPORTED_EXTENSIONS.contains(&extension.to_lowercase().as_str()) {
                        app.emit("install", value)?;
                    }
                }
            }
            "uninstall" => {
                let value = args[2].clone();
                if let Ok(timestamp) = value.parse::<i64>() {
                    app.emit("uninstall", timestamp)?;
                }
            }
            _ => {}
        }
    }

    let mut receiver = CHANNEL.1.resubscribe();
    loop {
        if let Ok(msg) = receiver.recv().await {
            match msg {
                SubCommands::Install(zip_path) => {
                    app.emit("install", zip_path)?;
                }
                SubCommands::Uninstall(timestamp) => {
                    app.emit("uninstall", timestamp)?;
                }
                SubCommands::InstallWithTimestamp(zip_path, timestamp) => {
                    app.emit("installWithTimestamp", (zip_path, timestamp))?;
                }
            }
        }
    }
}
