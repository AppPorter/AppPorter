use crate::configs::app_list::AppList;
use crate::configs::ConfigFile;
use crate::operations::uninstall::{remove_from_path, update_lib_list_after_uninstall};
use std::error::Error;
use std::path::Path;
use tauri::{AppHandle, Emitter};
use tokio::fs;

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
        // Libraries always use current user only for PATH
        remove_from_path(path_to_remove, true).await?;
    }

    app.emit("uninstall_lib", 75)?;

    // Update app list
    update_lib_list_after_uninstall(timestamp).await?;

    app.emit("uninstall_lib", 100)?;

    Ok(())
}
