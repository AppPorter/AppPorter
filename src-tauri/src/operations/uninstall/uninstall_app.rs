use crate::configs::library::Library;
use crate::utils::path::remove_from_path;
use crate::utils::registry::remove_registry_entries;
use crate::utils::shortcuts::{
    remove_custom_icon, remove_desktop_shortcut, remove_start_menu_shortcut,
};
use anyhow::{Result, anyhow};
use std::path::Path;
use tokio::fs;

pub async fn uninstall_app(id: &str) -> Result<()> {
    let mut library = Library::load().await?;
    let app_config = library
        .get_app(id)
        .await
        .ok_or(anyhow!("App with ID {} not found", id))?;

    let app_path = &app_config.details.install_path;
    if Path::new(app_path).exists() {
        fs::remove_dir_all(app_path).await?;
    }

    if app_config.details.config.create_desktop_shortcut {
        remove_desktop_shortcut(&app_config.details.info.name).await?;
    }

    if app_config.details.config.create_start_menu_shortcut {
        remove_start_menu_shortcut(
            app_config.details.current_user_only,
            &app_config.details.info.name,
        )
        .await?;
    }

    if app_config.details.config.custom_icon {
        remove_custom_icon(&app_config.details.info.name, id).await?;
    }

    if app_config.details.config.create_registry_key {
        remove_registry_entries(
            &app_config.details.info.name,
            app_config.details.current_user_only,
        )?;
    }

    if app_config.details.config.add_to_path.0 {
        remove_from_path(
            &app_config.details.config.add_to_path.1,
            app_config.details.current_user_only,
        )?;
    }

    library.uninstall_app(id).await?;

    Ok(())
}
