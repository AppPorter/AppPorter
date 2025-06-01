use crate::configs::app_list::AppList;
use crate::configs::ConfigFile;
use std::error::Error;
use std::path::Path;
use tauri::{AppHandle, Emitter};
use tokio::fs;
use windows_registry::CURRENT_USER;

pub async fn uninstall_lib(
    timestamp: i64,
    app: AppHandle,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    app.emit("uninstall_lib", 0)?;

    // Get lib configuration from app list
    let app_list = AppList::read().await?;
    let lib_config = app_list
        .libs
        .iter()
        .find(|lib| lib.timestamp == timestamp)
        .ok_or("Library not found in app list")?;

    // Remove library directory
    let lib_path = Path::new(&lib_config.details.paths.install_path);
    if lib_path.exists() {
        fs::remove_dir_all(&lib_path).await?;
    }

    app.emit("uninstall_lib", 50)?;

    // Remove from PATH if it was added
    if lib_config.details.config.add_to_path {
        // Use the pre-calculated full_path_directory
        let path_to_remove = &lib_config.details.config.full_path_directory;

        // Remove from CURRENT_USER environment
        let key = CURRENT_USER.create("Environment")?;
        if let Ok(current_path) = key.get_string("Path") {
            // Split the path by semicolons and filter out the path we want to remove
            let new_path: String = current_path
                .split(';')
                .filter(|p| p.trim() != path_to_remove.trim())
                .collect::<Vec<&str>>()
                .join(";");

            key.set_expand_string("Path", new_path)?;
        }
    }

    app.emit("uninstall_lib", 75)?;

    // Update app list
    let mut app_list = AppList::read().await?;

    // Find the lib to be uninstalled
    let lib_index = app_list
        .libs
        .iter()
        .position(|existing_lib| existing_lib.timestamp == timestamp);

    if let Some(index) = lib_index {
        // If the lib has a URL, just mark it as not installed
        // Otherwise, remove it completely from the list
        if !app_list.libs[index].url.is_empty() {
            app_list.libs[index].installed = false;
        } else {
            app_list.libs.remove(index);
        }
    }

    app_list.save().await?;

    app.emit("uninstall_lib", 100)?;

    Ok(())
}
