pub mod uninstall_app;
pub mod uninstall_lib;

pub use uninstall_app::*;
pub use uninstall_lib::*;

use crate::configs::app_list::AppList;
use crate::configs::ConfigFile;
use std::error::Error;
use windows_registry::{CURRENT_USER, LOCAL_MACHINE};

/// Remove a path from the system PATH environment variable
pub async fn remove_from_path(
    path_to_remove: &str,
    current_user_only: bool,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    if current_user_only {
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
    } else {
        let key = LOCAL_MACHINE
            .create(r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment")?;
        if let Ok(current_path) = key.get_string("path") {
            // Split the path by semicolons and filter out the path we want to remove
            let new_path: String = current_path
                .split(';')
                .filter(|p| p.trim() != path_to_remove.trim())
                .collect::<Vec<&str>>()
                .join(";");

            key.set_expand_string("path", new_path)?;
        }
    }
    Ok(())
}

/// Update app list by either marking as uninstalled or removing completely
pub async fn update_app_list_after_uninstall(
    timestamp: i64,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut app_list = AppList::read().await?;

    // Find the app to be uninstalled
    let app_index = app_list
        .apps
        .iter()
        .position(|existing_app| existing_app.timestamp == timestamp);

    if let Some(index) = app_index {
        // If the app has a URL, just mark it as not installed
        // Otherwise, remove it completely from the list
        if !app_list.apps[index].url.is_empty() {
            app_list.apps[index].installed = false;
        } else {
            app_list.apps.remove(index);
        }
    }

    app_list.save().await?;
    Ok(())
}

/// Update lib list by either marking as uninstalled or removing completely
pub async fn update_lib_list_after_uninstall(
    timestamp: i64,
) -> Result<(), Box<dyn Error + Send + Sync>> {
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
    Ok(())
}
