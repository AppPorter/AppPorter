use crate::{
    configs::library::*,
    operations::{extract_archive_files, flatten_nested_folders},
    utils::path::add_to_path,
};
use anyhow::Result;
use serde::{Deserialize, Serialize};
use tauri::{AppHandle, Emitter};
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, TS)]
#[ts(export)]
pub struct ToolInstallConfig {
    pub tool: Tool,
    pub archive_path_dir: String,
    pub zip_path: String,
}

pub async fn install_tool(config: ToolInstallConfig, app: &AppHandle) -> Result<String> {
    let mut config = config;
    Library::init_tool(&mut config.tool).await?;

    app.emit("tool_install_progress", 0)?;

    tokio::fs::create_dir_all(&config.tool.details.install_path).await?;
    extract_archive_files(
        &config.zip_path,
        &config.tool.details.install_path,
        Some(app),
        &config.tool.archive_password,
        "tool_install_progress",
    )
    .await?;

    flatten_nested_folders(&config.tool.details.install_path, None).await?;

    if config.tool.details.add_to_path.0 {
        config.tool.details.add_to_path.1 = if config.tool.details.add_to_path.1.is_empty() {
            config.tool.details.install_path.clone()
        } else {
            let normalized_path = config
                .archive_path_dir
                .trim_start_matches(['/', '\\'])
                .replace("/", "\\");
            format!("{}\\{}", config.tool.details.install_path, normalized_path)
        };

        add_to_path(&config.tool.details.add_to_path.1, true)?;
    }

    let mut app_list = Library::load().await?;
    app_list.add_tool(config.tool.clone()).await?;

    app.emit("tool_install_progress", 101)?;

    Ok(config.tool.details.install_path)
}
