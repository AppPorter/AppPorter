use crate::configs::library::Library;
use crate::utils::path::remove_from_path;
use anyhow::{Result, anyhow};
use std::path::Path;
use tokio::fs;

pub async fn uninstall_tool(id: &str) -> Result<()> {
    let mut library = Library::load().await?;
    let tool_config = library
        .get_tool(id)
        .await
        .ok_or(anyhow!("Tool with ID {} not found", id))?;

    let tool_path = &tool_config.details.install_path;
    if Path::new(tool_path).exists() {
        fs::remove_dir_all(tool_path).await?;
    }

    if tool_config.details.add_to_path.0 {
        remove_from_path(&tool_config.details.add_to_path.1, true)?;
    }

    library.uninstall_tool(id).await?;

    Ok(())
}
