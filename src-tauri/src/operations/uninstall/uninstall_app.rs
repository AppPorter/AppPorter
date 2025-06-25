use crate::configs::ConfigFile;
use crate::configs::{env::Env, library::Library};
use crate::operations::uninstall::{remove_from_path, update_app_list_after_uninstall};
use anyhow::{Result, anyhow};
use std::path::Path;
use tauri::{AppHandle, Emitter};
use tokio::fs;
use windows_registry::{CURRENT_USER, LOCAL_MACHINE};

pub async fn uninstall_app(timestamp: i64, app: &AppHandle) -> Result<()> {
    app.emit("app_uninstall_progress", 0)?;

    // Get app configuration from library
    let library = Library::read().await?;
    let app_config = library
        .apps
        .iter()
        .find(|app| app.timestamp == timestamp)
        .ok_or(anyhow!("App not found in library"))?;

    // Remove application directory - use the install_path directly
    let app_path = &app_config.details.paths.install_path;
    if Path::new(app_path).exists() {
        fs::remove_dir_all(app_path).await?;
    }

    app.emit("app_uninstall_progress", 25)?;

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

    app.emit("app_uninstall_progress", 50)?;

    if app_config.details.config.create_desktop_shortcut {
        if let Some(desktop_dir) = dirs::desktop_dir() {
            let desktop_shortcut =
                desktop_dir.join(format!("{}.lnk", app_config.details.info.name));
            if desktop_shortcut.exists() {
                fs::remove_file(desktop_shortcut).await?;
            }
        }
    }

    app.emit("app_uninstall_progress", 75)?;

    // Remove custom icon file if it was used
    if app_config.details.config.custom_icon {
        let icons_dir = dirs::config_local_dir()
            .ok_or(anyhow!("Failed to get local config directory"))?
            .join("AppPorter")
            .join("icons");

        let icon_filename = format!("{}-{}.ico", app_config.details.info.name, timestamp);
        let icon_path = icons_dir.join(&icon_filename);

        if icon_path.exists() {
            fs::remove_file(icon_path).await?;
        }
    }

    // Remove from PATH if it was added
    if app_config.details.config.add_to_path {
        // Use the pre-calculated full_path_directory
        let path_to_remove = &app_config.details.config.full_path_directory;
        remove_from_path(path_to_remove, app_config.details.config.current_user_only).await?;
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
    update_app_list_after_uninstall(timestamp).await?;

    app.emit("app_uninstall_progress", 100)?;

    Ok(())
}
