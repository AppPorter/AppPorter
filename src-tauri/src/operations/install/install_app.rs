use crate::configs::ConfigFile;
use crate::configs::library::*;
use crate::operations::convert_base64_to_ico;
use crate::operations::extract_archive_files;
use crate::operations::install::flatten_nested_folders;
use crate::utils::path::add_to_path;
use crate::utils::registry::create_registry_entries;
use crate::utils::shortcuts::{create_desktop_shortcut, create_start_menu_shortcut};
use anyhow::Result;
use mslnk::ShellLink;
use serde::{Deserialize, Serialize};
use std::path::Path;
use tauri::{AppHandle, Emitter};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AppInstallConfig {
    pub zip_path: String,
    pub password: Option<String>,
    pub timestamp: i64,
    pub details: AppDetails,
    pub url: Option<String>,
}

pub async fn install_app(config: AppInstallConfig, app: &AppHandle) -> Result<(String, String)> {
    let timestamp = chrono::Utc::now().timestamp();

    // Send initial install progress event
    app.emit("app_install_progress", 0)?;

    let install_path = format!(
        "{}\\{}",
        config.details.paths.parent_install_path, config.details.info.name
    );

    // Setup installation directory and extract files
    tokio::fs::create_dir_all(&install_path).await?;
    extract_archive_files(
        &config.zip_path,
        &install_path,
        app,
        config.password.as_deref(),
        "app_install_progress",
    )
    .await?;

    // Flatten nested single folders to avoid deep nesting and get actual executable path
    let full_path =
        flatten_nested_folders(&install_path, Some(&config.details.config.archive_exe_path))
            .await?;

    // Create shortcuts
    let mut shell_link = ShellLink::new(&full_path)?;
    if config.details.config.custom_icon {
        let icon_path = convert_base64_to_ico(
            config.details.info.icon.clone(),
            format!("{}-{}", config.details.info.name.clone(), timestamp),
        )
        .await?;
        shell_link.set_icon_location(Some(icon_path));
    }

    if config.details.config.create_desktop_shortcut {
        create_desktop_shortcut(&shell_link, &config.details.info.name)?;
    }

    if config.details.config.create_start_menu_shortcut {
        create_start_menu_shortcut(
            &shell_link,
            config.details.config.current_user_only,
            &config.details.info.name,
        )
        .await?;
    }

    // Create registry entries if requested
    if config.details.config.create_registry_key {
        create_registry_entries(&config, &full_path, &install_path, timestamp)?;
    }

    let mut full_path_directory = String::new();
    // Add to PATH if requested
    if config.details.config.add_to_path {
        full_path_directory = if config.details.config.archive_path_directory.is_empty() {
            Path::new(&full_path)
                .parent()
                .expect("Failed to get parent directory")
                .to_string_lossy()
                .to_string()
        } else {
            let normalized_path = config
                .details
                .config
                .archive_path_directory
                .trim_start_matches(['/', '\\'])
                .replace("/", "\\");
            format!("{}\\{}", install_path, normalized_path)
        };

        add_to_path(
            &full_path_directory,
            config.details.config.current_user_only,
        )?;
    }

    // Update app list
    let mut app_list = Library::read().await?;
    app_list
        .update_app_list_from_config(config, full_path.clone(), full_path_directory, timestamp)
        .await?;

    app.emit("app_install_progress", 101)?;

    Ok((install_path, full_path))
}
