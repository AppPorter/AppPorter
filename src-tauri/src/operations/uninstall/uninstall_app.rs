use crate::configs::ConfigFile;
use crate::configs::{app_list::AppList, env::Env};
use std::error::Error;
use std::path::Path;
use tauri::{AppHandle, Emitter};
use tokio::fs;
use windows_registry::{CURRENT_USER, LOCAL_MACHINE};

pub async fn uninstall_app(
    timestamp: i64,
    app: AppHandle,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    app.emit("uninstall", 0)?;

    // Get app configuration from app list
    let app_list = AppList::read().await?;
    let app_config = app_list
        .apps
        .iter()
        .find(|app| app.timestamp == timestamp)
        .ok_or("App not found in app list")?;

    // Remove application directory
    let app_path = format!(
        "{}\\{}",
        app_config.details.paths.install_path,
        app_config.details.info.name.replace(" ", "-")
    );
    if Path::new(&app_path).exists() {
        fs::remove_dir_all(&app_path).await?;
    }

    app.emit("uninstall", 25)?;

    let env = Env::read().await?;

    // Remove shortcuts
    if app_config.details.config.create_start_menu_shortcut {
        let start_menu_path = if app_config.details.config.current_user_only {
            format!(
                r"{}:\Users\{}\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\{}.lnk",
                env.system_drive_letter, env.username, app_config.details.info.name
            )
        } else {
            format!(
                r"{}:\ProgramData\Microsoft\Windows\Start Menu\Programs\{}.lnk",
                env.system_drive_letter, app_config.details.info.name
            )
        };
        let start_menu_shortcut = Path::new(&start_menu_path);
        if start_menu_shortcut.exists() {
            fs::remove_file(start_menu_shortcut).await?;
        }
    }

    app.emit("uninstall", 50)?;

    if app_config.details.config.create_desktop_shortcut {
        if let Some(desktop_dir) = dirs::desktop_dir() {
            let desktop_shortcut =
                desktop_dir.join(format!("{}.lnk", app_config.details.info.name));
            if desktop_shortcut.exists() {
                fs::remove_file(desktop_shortcut).await?;
            }
        }
    }

    app.emit("uninstall", 75)?;

    // Remove from PATH if it was added
    if app_config.details.config.add_to_path {
        // Use the pre-calculated full_path_directory
        let path_to_remove = &app_config.details.config.full_path_directory;

        if app_config.details.config.current_user_only {
            let key = CURRENT_USER.create("Environment")?;
            let current_path = key.get_string("Path")?;

            // Split the path by semicolons and filter out the path we want to remove
            let new_path: String = current_path
                .split(';')
                .filter(|p| p.trim() != path_to_remove.trim())
                .collect::<Vec<&str>>()
                .join(";");

            key.set_expand_string("Path", new_path)?;
        } else {
            let key = LOCAL_MACHINE
                .create(r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment")?;
            let current_path = key.get_string("path")?;

            // Split the path by semicolons and filter out the path we want to remove
            let new_path: String = current_path
                .split(';')
                .filter(|p| p.trim() != path_to_remove.trim())
                .collect::<Vec<&str>>()
                .join(";");

            key.set_expand_string("path", new_path)?;
        }
    }

    // Remove registry entries
    if app_config.details.config.create_registry_key {
        let key: String;

        if app_config.details.config.current_user_only {
            key = format!(
                r"Software\Microsoft\Windows\CurrentVersion\Uninstall\{}",
                app_config.details.info.name
            );
            if CURRENT_USER.open(&key).is_ok() {
                let _ = CURRENT_USER.remove_tree(&key);
            }
        } else {
            key = format!(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\{}",
                app_config.details.info.name
            );
            if LOCAL_MACHINE.open(&key).is_ok() {
                let _ = LOCAL_MACHINE.remove_tree(&key);
            }
        }
    }

    // Update app list
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

    app.emit("uninstall", 100)?;

    Ok(())
}
