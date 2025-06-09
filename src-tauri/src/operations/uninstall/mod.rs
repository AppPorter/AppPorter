pub mod uninstall_app;
pub mod uninstall_tool;

pub use uninstall_app::*;
pub use uninstall_tool::*;

use crate::configs::library::Library;
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
    let mut app_list = Library::read().await?;

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

/// Update tool list by either marking as uninstalled or removing completely
pub async fn update_tool_list_after_uninstall(
    timestamp: i64,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut app_list = Library::read().await?;

    // Find the tool to be uninstalled
    let tool_index = app_list
        .tools
        .iter()
        .position(|existing_tool| existing_tool.timestamp == timestamp);

    if let Some(index) = tool_index {
        // If the tool has a URL, just mark it as not installed
        // Otherwise, remove it completely from the list
        if !app_list.tools[index].url.is_empty() {
            app_list.tools[index].installed = false;
        } else {
            app_list.tools.remove(index);
        }
    }

    app_list.save().await?;
    Ok(())
}
