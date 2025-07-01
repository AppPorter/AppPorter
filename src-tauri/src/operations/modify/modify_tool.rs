use crate::configs::{ConfigFile, library::*};
use crate::utils::path::{add_to_path, remove_from_path};
use anyhow::{Result, anyhow};
use std::path::Path;
use tokio::fs as tokio_fs;

pub async fn modify_tool(new_tool: Tool, timestamp: i64) -> Result<()> {
    let mut library = Library::load().await?;
    let old_tool = library
        .tools
        .iter()
        .find(|t| t.timestamp == timestamp)
        .ok_or(anyhow!("Tool not found in library"))?
        .clone();

    if old_tool.details.install_path != new_tool.details.install_path
        && Path::new(&old_tool.details.install_path).exists()
    {
        tokio_fs::create_dir_all(&new_tool.details.install_path).await?;
        fs_extra::dir::move_dir(
            &old_tool.details.install_path,
            &new_tool.details.install_path,
            &fs_extra::dir::CopyOptions::new()
                .overwrite(true)
                .content_only(true),
        )?;
    }

    if old_tool.details.add_to_path != new_tool.details.add_to_path {
        if old_tool.details.add_to_path.0 {
            remove_from_path(&old_tool.details.add_to_path.1, true)?;
        }
        if new_tool.details.add_to_path.0 {
            add_to_path(&new_tool.details.add_to_path.1, true)?;
        }
    }

    if let Some(tool) = library.tools.iter_mut().find(|t| t.timestamp == timestamp) {
        *tool = new_tool;
    }
    library.save().await?;
    Ok(())
}
