use crate::configs::library::Library;
use crate::utils::path::remove_from_path;
use crate::utils::registry::remove_registry_entries;
use crate::utils::shortcuts::{
    remove_custom_icon, remove_desktop_shortcut, remove_start_menu_shortcut,
};
use anyhow::{Result, anyhow};
use std::path::Path;
use tokio::fs;

pub async fn uninstall_app(timestamp: i64) -> Result<()> {
    let mut library = Library::load().await?;
    let app_config = library
        .apps
        .iter()
        .find(|app| app.timestamp == timestamp)
        .ok_or(anyhow!("App not found in library"))?;

    let app_path = &app_config.details.paths.install_path;
    if Path::new(app_path).exists() {
        fs::remove_dir_all(app_path).await?;
    }

    if app_config.details.config.create_desktop_shortcut {
        remove_desktop_shortcut(&app_config.details.info.name).await?;
    }

    if app_config.details.config.create_start_menu_shortcut {
        remove_start_menu_shortcut(
            app_config.details.config.current_user_only,
            &app_config.details.info.name,
        )
        .await?;
    }

    if app_config.details.config.custom_icon {
        remove_custom_icon(&app_config.details.info.name, timestamp).await?;
    }

    if app_config.details.config.create_registry_key {
        remove_registry_entries(
            &app_config.details.info.name,
            app_config.details.config.current_user_only,
        )?;
    }

    if app_config.details.config.add_to_path {
        remove_from_path(
            &app_config.details.config.full_path_directory,
            app_config.details.config.current_user_only,
        )?;
    }

    library.update_app_list_after_uninstall(timestamp).await?;

    Ok(())
}
