use crate::{
    configs::library::*,
    operations::{extract_archive_files, flatten_nested_folders},
    utils::path::add_to_path,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ToolInstallConfig<'a> {
    pub zip_path: &'a str,
    pub password: Option<&'a str>,
    pub timestamp: i64,
    pub url: Option<&'a str>,
    pub details: ToolDetails,
}

pub async fn install_tool<'a>(config: ToolInstallConfig<'a>, app: &AppHandle) -> Result<String> {
    let timestamp = chrono::Utc::now().timestamp();

    app.emit("tool_install_progress", 0)?;

    let install_path = format!(
        "{}\\{}",
        config.details.paths.parent_install_path, config.details.name
    );

    tokio::fs::create_dir_all(&install_path).await?;
    extract_archive_files(
        config.zip_path,
        &install_path,
        app,
        config.password,
        "tool_install_progress",
    )
    .await?;

    flatten_nested_folders(&install_path, None).await?;

    let mut full_path_directory = String::new();
    if config.details.config.add_to_path {
        full_path_directory = if config.details.config.archive_path_directory.is_empty() {
            install_path.clone()
        } else {
            let normalized_path = config
                .details
                .config
                .archive_path_directory
                .trim_start_matches(['/', '\\'])
                .replace("/", "\\");
            format!("{}\\{}", install_path, normalized_path)
        };

        add_to_path(&full_path_directory, true)?;
    }

    let mut app_list = Library::load().await?;
    app_list
        .update_tool_list_from_config(config, &install_path, &full_path_directory, timestamp)
        .await?;

    app.emit("tool_install_progress", 101)?;

    Ok(install_path)
}
