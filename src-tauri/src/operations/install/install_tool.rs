use crate::{
    configs::{ConfigFile, library::*},
    operations::{extract_archive_files, flatten_nested_folders},
    utils::path::add_to_path,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tauri::{AppHandle, Emitter};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ToolInstallConfig {
    pub zip_path: String,
    pub password: Option<String>,
    pub timestamp: i64,
    pub details: ToolDetails,
    pub url: Option<String>,
}

pub async fn install_tool(config: ToolInstallConfig, app: &AppHandle) -> Result<String> {
    let timestamp = chrono::Utc::now().timestamp();

    app.emit("tool_install_progress", 0)?;

    let install_path = setup_installation_directory(&config).await?;
    extract_archive_files(
        &config.zip_path,
        &install_path,
        app,
        config.password.as_deref(),
        "tool_install_progress",
    )
    .await?;

    flatten_nested_folders(&install_path, None).await?;

    if config.details.config.add_to_path {
        add_to_path(&install_path, false)?;
    }

    let mut app_list = Library::read().await?;

    app_list
        .update_tool_list_from_config(config.clone(), install_path.clone(), timestamp)
        .await?;

    app.emit("tool_install_progress", 101)?;

    Ok(install_path)
}

async fn setup_installation_directory(config: &ToolInstallConfig) -> Result<String> {
    if !Path::new(&config.details.paths.parent_install_path).exists() {
        tokio::fs::create_dir_all(&config.details.paths.parent_install_path).await?;
    }

    let install_path = format!(
        "{}\\{}",
        config.details.paths.parent_install_path, config.details.name
    );

    let target_path = Path::new(&install_path);
    if !target_path.exists() {
        tokio::fs::create_dir_all(target_path).await?;
    }

    Ok(install_path)
}
