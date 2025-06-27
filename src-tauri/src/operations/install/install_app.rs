use crate::configs::library::*;
use crate::operations::extract_archive_files;
use crate::operations::install::flatten_nested_folders;
use crate::utils::convert_base64_to_ico;
use crate::utils::path::add_to_path;
use crate::utils::registry::create_registry_entries;
use crate::utils::shortcuts::{create_desktop_shortcut, create_start_menu_shortcut};
use anyhow::Result;
use mslnk::ShellLink;
use std::path::Path;
use tauri::{AppHandle, Emitter};

pub async fn install_app(config: App, zip_path: &str, app: &AppHandle) -> Result<(String, String)> {
    let mut config = config.clone();

    config.timestamp = chrono::Utc::now().timestamp();

    app.emit("app_install_progress", 0)?;

    config.details.paths.install_path = format!(
        "{}\\{}",
        config.details.paths.parent_install_path, config.details.info.name
    );

    tokio::fs::create_dir_all(&config.details.paths.install_path).await?;
    extract_archive_files(
        zip_path,
        &config.details.paths.install_path,
        app,
        &config.details.config.archive_password,
        "app_install_progress",
    )
    .await?;

    config.details.paths.full_path = flatten_nested_folders(
        &config.details.paths.install_path,
        Some(&config.details.config.archive_exe_path),
    )
    .await?;

    let mut shell_link = ShellLink::new(&config.details.paths.full_path)?;
    if config.details.config.custom_icon {
        let icon_path = convert_base64_to_ico(
            &config.details.info.icon,
            &format!("{}-{}", config.details.info.name, config.timestamp),
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

    if config.details.config.create_registry_key {
        create_registry_entries(&config)?;
    }

    if config.details.config.add_to_path {
        config.details.config.full_path_directory =
            if config.details.config.archive_path_directory.is_empty() {
                Path::new(&config.details.paths.full_path)
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
                format!("{}\\{}", config.details.paths.install_path, normalized_path)
            };

        add_to_path(
            &config.details.config.full_path_directory,
            config.details.config.current_user_only,
        )?;
    }

    let mut app_list = Library::load().await?;
    app_list.update_app_list_from_config(config.clone()).await?;

    app.emit("app_install_progress", 101)?;

    Ok((
        config.details.paths.install_path,
        config.details.paths.full_path,
    ))
}
