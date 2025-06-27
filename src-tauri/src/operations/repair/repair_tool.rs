use crate::{configs::Library, utils::*};
use anyhow::{Result, anyhow};

pub async fn repair_tool(timestamp: i64) -> Result<()> {
    let library = Library::load().await?;
    let config = library
        .tools
        .iter()
        .find(|tool| tool.timestamp == timestamp)
        .ok_or(anyhow!("Tool not found in library"))?;

    if config.details.config.add_to_path {
        remove_from_path(&config.details.config.full_path_directory, true)?;
    }

    if config.details.config.add_to_path {
        add_to_path(&config.details.config.full_path_directory, true)?;
    }

    Ok(())
}
