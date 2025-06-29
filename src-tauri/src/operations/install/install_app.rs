use crate::configs::library::*;
use crate::operations::extract_archive_files;
use crate::operations::install::flatten_nested_folders;
use crate::utils::convert_base64_to_ico;
use crate::utils::path::add_to_path;
use crate::utils::registry::create_registry_entries;
use crate::utils::shortcuts::{create_desktop_shortcut, create_start_menu_shortcut};
use anyhow::Result;
use mslnk::ShellLink;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tauri::{AppHandle, Emitter};

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct AppInstallConfig {
    pub app: App,
    pub archive_exe_path: String,
    pub archive_path_dir: String,
    pub zip_path: String,
}

pub async fn install_app(config: AppInstallConfig, app: &AppHandle) -> Result<(String, String)> {
    let mut config = config.clone();

    if config.app.timestamp == 0 {
        config.app.timestamp = chrono::Utc::now().timestamp();
    }

    app.emit("app_install_progress", 0)?;

    tokio::fs::create_dir_all(&config.app.details.install_path).await?;
    extract_archive_files(
        &config.zip_path,
        &config.app.details.install_path,
        Some(app),
        &config.app.archive_password,
        "app_install_progress",
    )
    .await?;

    config.app.details.full_path = flatten_nested_folders(
        &config.app.details.install_path,
        Some(&config.archive_exe_path),
    )
    .await?;

    let mut shell_link = ShellLink::new(&config.app.details.full_path)?;
    if config.app.details.config.custom_icon {
        let icon_path = convert_base64_to_ico(
            &config.app.details.info.icon,
            &format!("{}-{}", config.app.details.info.name, config.app.timestamp),
        )
        .await?;
        shell_link.set_icon_location(Some(icon_path));
    }

    if config.app.details.config.create_desktop_shortcut {
        create_desktop_shortcut(&shell_link, &config.app.details.info.name)?;
    }

    if config.app.details.config.create_start_menu_shortcut {
        create_start_menu_shortcut(
            &shell_link,
            config.app.details.current_user_only,
            &config.app.details.info.name,
        )
        .await?;
    }

    if config.app.details.config.create_registry_key {
        create_registry_entries(&config.app)?;
    }

    if config.app.details.config.add_to_path.0 {
        config.app.details.config.add_to_path.1 = if config.archive_path_dir.is_empty() {
            if config.app.details.config.add_to_path.1.is_empty() {
                Path::new(&config.app.details.full_path)
                    .parent()
                    .expect("Failed to get parent directory")
                    .to_string_lossy()
                    .to_string()
            } else {
                config.app.details.config.add_to_path.1
            }
        } else {
            let normalized_path = config
                .archive_path_dir
                .trim_start_matches(['/', '\\'])
                .replace("/", "\\");
            format!("{}\\{}", config.app.details.install_path, normalized_path)
        };

        add_to_path(
            &config.app.details.config.add_to_path.1,
            config.app.details.current_user_only,
        )?;
    }

    let mut app_list = Library::load().await?;
    app_list
        .update_app_list_from_config(config.app.clone())
        .await?;

    app.emit("app_install_progress", 101)?;

    Ok((
        config.app.details.install_path,
        config.app.details.full_path,
    ))
}
