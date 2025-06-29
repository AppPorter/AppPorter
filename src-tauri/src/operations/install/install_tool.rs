use crate::{
    configs::library::*,
    operations::{extract_archive_files, flatten_nested_folders},
    utils::path::add_to_path,
};
use anyhow::Result;
use tauri::{AppHandle, Emitter};

pub async fn install_tool(config: Tool, zip_path: &str, app: &AppHandle) -> Result<String> {
    let mut config = config.clone();

    config.timestamp = chrono::Utc::now().timestamp();

    app.emit("tool_install_progress", 0)?;

    config.details.paths.install_path = format!(
        "{}\\{}",
        config.details.paths.parent_install_path, config.details.name
    );

    tokio::fs::create_dir_all(&config.details.paths.install_path).await?;
    extract_archive_files(
        zip_path,
        &config.details.paths.install_path,
        Some(app),
        &config.details.config.archive_password,
        "tool_install_progress",
    )
    .await?;

    flatten_nested_folders(&config.details.paths.install_path, None).await?;

    if config.details.config.add_to_path {
        config.details.config.full_path_directory =
            if config.details.config.archive_path_directory.is_empty() {
                config.details.paths.install_path.clone()
            } else {
                let normalized_path = config
                    .details
                    .config
                    .archive_path_directory
                    .trim_start_matches(['/', '\\'])
                    .replace("/", "\\");
                format!("{}\\{}", config.details.paths.install_path, normalized_path)
            };

        add_to_path(&config.details.config.full_path_directory, true)?;
    }

    let mut app_list = Library::load().await?;
    app_list
        .update_tool_list_from_config(config.clone())
        .await?;

    app.emit("tool_install_progress", 101)?;

    Ok(config.details.paths.install_path)
}
