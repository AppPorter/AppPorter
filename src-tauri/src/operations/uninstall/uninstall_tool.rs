use crate::configs::ConfigFile;
use crate::configs::library::Library;
use crate::operations::uninstall::{remove_from_path, update_tool_list_after_uninstall};
use anyhow::{Result, anyhow};
use std::path::Path;
use tauri::{AppHandle, Emitter};
use tokio::fs;

pub async fn uninstall_tool(timestamp: i64, app: AppHandle) -> Result<()> {
    app.emit("tool_uninstall_progress", 0)?;

    // Get tool configuration from library
    let library = Library::read().await?;
    let tool_config = library
        .tools
        .iter()
        .find(|tool| tool.timestamp == timestamp)
        .ok_or(anyhow!("Tool not found in library"))?;

    // Remove tool directory
    let tool_path = &tool_config.details.paths.install_path;
    if Path::new(tool_path).exists() {
        fs::remove_dir_all(tool_path).await?;
    }

    app.emit("tool_uninstall_progress", 50)?;

    // Remove from PATH if it was added
    if tool_config.details.config.add_to_path {
        // Use the pre-calculated full_path_directory
        let path_to_remove = &tool_config.details.config.full_path_directory;
        // Tools always use current user only for PATH
        remove_from_path(path_to_remove, true).await?;
    }

    app.emit("tool_uninstall_progress", 75)?;

    // Update app list
    update_tool_list_after_uninstall(timestamp).await?;

    app.emit("tool_uninstall_progress", 100)?;

    Ok(())
}
