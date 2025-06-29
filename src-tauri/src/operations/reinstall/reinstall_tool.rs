use crate::{
    configs::library::*,
    operations::{extract_archive_files, flatten_nested_folders, repair_tool},
};
use anyhow::{Result, anyhow};
use fs_extra::dir::move_dir;

pub async fn reinstall_tool(timestamp: i64, zip_path: &str) -> Result<()> {
    let library = Library::load().await?;
    let tool_config = library
        .tools
        .iter()
        .find(|tool| tool.timestamp == timestamp)
        .ok_or(anyhow!("Tool not found in library"))?;

    let temp_dir = std::env::temp_dir()
        .join("AppPorter")
        .join("reinstall")
        .join(format!(
            "{}-{}",
            tool_config.details.name, tool_config.timestamp
        ));

    tokio::fs::create_dir_all(&temp_dir).await?;
    extract_archive_files(
        zip_path,
        &temp_dir.to_string_lossy(),
        None,
        &tool_config.archive_password,
        "",
    )
    .await?;

    flatten_nested_folders(&temp_dir.to_string_lossy(), None).await?;

    tokio::fs::create_dir_all(&tool_config.details.install_path).await?;
    move_dir(
        &temp_dir,
        &tool_config.details.install_path,
        &fs_extra::dir::CopyOptions::new()
            .overwrite(true)
            .content_only(true),
    )?;

    repair_tool(timestamp).await?;

    Ok(())
}
