use crate::{configs::Library, utils::*};
use anyhow::{Result, anyhow};

pub async fn repair_tool(id: &str) -> Result<()> {
    let library = Library::load().await?;
    let config = library
        .get_tool(id)
        .await
        .ok_or(anyhow!("Tool with ID {} not found", id))?;

    if config.details.add_to_path.0 {
        remove_from_path(&config.details.add_to_path.1, true)?;
    }

    if config.details.add_to_path.0 {
        add_to_path(&config.details.add_to_path.1, true)?;
    }

    Ok(())
}
