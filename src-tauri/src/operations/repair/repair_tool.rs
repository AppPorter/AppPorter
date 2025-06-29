use crate::{configs::Library, utils::*};
use anyhow::{Result, anyhow};

pub async fn repair_tool(timestamp: i64) -> Result<()> {
    let library = Library::load().await?;
    let config = library
        .tools
        .iter()
        .find(|tool| tool.timestamp == timestamp)
        .ok_or(anyhow!("Tool not found in library"))?;

    if config.details.add_to_path.0 {
        remove_from_path(&config.details.add_to_path.1, true)?;
    }

    if config.details.add_to_path.0 {
        add_to_path(&config.details.add_to_path.1, true)?;
    }

    Ok(())
}
