use crate::configs::library::*;
use crate::operations::install::flatten_nested_folders;
use crate::operations::{extract_archive_files, repair_app};
use anyhow::{Result, anyhow};
use fs_extra::dir::move_dir;

pub async fn reinstall_app(timestamp: i64, zip_path: &str) -> Result<()> {
    let library = Library::load().await?;
    let app_config = library
        .apps
        .iter()
        .find(|app| app.timestamp == timestamp)
        .ok_or(anyhow!("App not found in library"))?;

    let temp_dir = std::env::temp_dir()
        .join("AppPorter")
        .join("Reinstall")
        .join(format!(
            "{}-{}",
            app_config.details.info.name, app_config.timestamp
        ));

    tokio::fs::create_dir_all(&temp_dir).await?;
    extract_archive_files(
        zip_path,
        &temp_dir.to_string_lossy(),
        None,
        &app_config.archive_password,
        "",
    )
    .await?;

    flatten_nested_folders(&temp_dir.to_string_lossy(), None).await?;

    tokio::fs::create_dir_all(&app_config.details.install_path).await?;
    move_dir(
        &temp_dir,
        &app_config.details.install_path,
        &fs_extra::dir::CopyOptions::new()
            .overwrite(true)
            .content_only(true),
    )?;

    repair_app(timestamp).await?;

    Ok(())
}
