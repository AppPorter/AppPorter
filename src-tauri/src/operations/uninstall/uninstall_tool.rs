use crate::configs::library::Library;
use crate::utils::path::remove_from_path;
use anyhow::{Result, anyhow};
use std::path::Path;
use tokio::fs;

pub async fn uninstall_tool(timestamp: i64) -> Result<()> {
    let mut library = Library::load().await?;
    let tool_config = library
        .tools
        .iter()
        .find(|tool| tool.timestamp == timestamp)
        .ok_or(anyhow!("Tool not found in library"))?;

    let tool_path = &tool_config.details.paths.install_path;
    if Path::new(tool_path).exists() {
        fs::remove_dir_all(tool_path).await?;
    }

    if tool_config.details.config.add_to_path {
        remove_from_path(&tool_config.details.config.full_path_directory, true)?;
    }

    library.update_tool_list_after_uninstall(timestamp).await?;

    Ok(())
}
