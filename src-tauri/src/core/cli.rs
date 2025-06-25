use crate::SUPPORTED_EXTENSIONS;
use anyhow::Result;
use lazy_static::lazy_static;
use tauri::{AppHandle, Emitter};
use tokio::sync::broadcast;

lazy_static! {
    pub static ref CHANNEL: (
        broadcast::Sender<Vec<String>>,
        broadcast::Receiver<Vec<String>>
    ) = broadcast::channel(1);
}

pub async fn cli(app: &AppHandle) -> Result<String> {
    let init_args: Vec<String> = std::env::args().collect();
    let args: Vec<String> = init_args
        .into_iter()
        .filter(|arg| arg != "--silent")
        .collect();
    if args.len() == 3 || args.len() == 4 {
        cases(app, args).await?;
    }

    let mut receiver = CHANNEL.1.resubscribe();
    loop {
        if let Ok(new_args) = receiver.recv().await {
            if new_args.len() == 3 || new_args.len() == 4 {
                cases(app, new_args).await?;
            }
        }
    }

    async fn cases(app: &AppHandle, args: Vec<String>) -> Result<()> {
        match args[1].as_str() {
            "preview" => {
                let value = args[2].clone();
                if let Some(extension) = std::path::Path::new(&value)
                    .extension()
                    .and_then(|ext| ext.to_str())
                {
                    if SUPPORTED_EXTENSIONS.contains(&extension.to_lowercase().as_str()) {
                        app.emit("preview", (value, chrono::Utc::now().timestamp()))?;
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
