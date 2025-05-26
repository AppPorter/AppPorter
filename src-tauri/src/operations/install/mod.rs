pub mod install_app;
pub mod install_lib;

use crate::core::download_file;
pub use install_app::*;
pub use install_lib::*;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tauri::{AppHandle, Emitter};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum InstallType {
    App,
    Lib,
    Pending,
}

pub async fn install_with_link(
    app: AppHandle,
    url: String,
    timestamp: i64,
    install_type: InstallType,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let downloaded = download_file(url).await.unwrap_or_default();

    match install_type {
        InstallType::App => app.emit("install_app", (downloaded, timestamp))?,
        InstallType::Lib => app.emit("install_lib", (downloaded, timestamp))?,
        InstallType::Pending => app.emit("install", (downloaded, timestamp))?,
    }

    Ok(())
}
