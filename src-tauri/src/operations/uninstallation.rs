use crate::configs::app_list::AppList;
use crate::configs::settings::Settings;
use crate::configs::ConfigFile;
use std::path::Path;
use std::{error::Error, os::windows::process::CommandExt};
use tauri::{AppHandle, Emitter};
use tokio::fs;
use windows_registry::{CURRENT_USER, LOCAL_MACHINE};

pub async fn uninstallation(timestamp: i64, app: AppHandle) -> Result<String, Box<dyn Error>> {
    app.emit("uninstallation", 0)?;

    // Get app configuration from app list
    let app_list = AppList::read().await?;
    let app_config = app_list
        .links
        .iter()
        .find(|app| app.timestamp == timestamp)
        .ok_or("App not found in app list")?;

    // Remove application directory
    let app_path = Path::new(&app_config.details.install_path)
        .join(&app_config.details.name.replace(" ", "-"));
    if app_path.exists() {
        fs::remove_dir_all(&app_path).await?;
    }

    app.emit("uninstallation", 25)?;

    let settings = Settings::read().await?;

    // Remove shortcuts
    if app_config.details.create_start_menu_shortcut {
        let start_menu_path = if app_config.details.current_user_only {
            format!(
                r"{}:\Users\{}\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\{}.lnk",
                settings.system_drive_letter, settings.username, app_config.details.name
            )
        } else {
            format!(
                r"{}:\ProgramData\Microsoft\Windows\Start Menu\Programs\{}.lnk",
                settings.system_drive_letter, app_config.details.name
            )
        };
        let start_menu_shortcut = Path::new(&start_menu_path);
        if start_menu_shortcut.exists() {
            fs::remove_file(start_menu_shortcut).await?;
        }
    }

    app.emit("uninstallation", 50)?;

    if app_config.details.create_desktop_shortcut {
        if let Some(desktop_dir) = dirs::desktop_dir() {
            let desktop_shortcut = desktop_dir.join(format!("{}.lnk", app_config.details.name));
            if desktop_shortcut.exists() {
                fs::remove_file(desktop_shortcut).await?;
            }
        }
    }

    app.emit("uninstallation", 75)?;

    // Remove from PATH if it was added
    if app_config.details.add_to_path {
        let exe_path = Path::new(&app_config.details.full_path)
            .parent()
            .expect("Failed to get parent directory")
            .to_string_lossy();

        if app_config.details.current_user_only {
            std::process::Command::new("powershell")
                .args([
                    "-Command",
                    &format!(
                        "[Environment]::SetEnvironmentVariable('Path', [string]::join(';', ([Environment]::GetEnvironmentVariable('Path', 'User').Split(';') | Where-Object {{ $_ -ne '{}' }})), 'User')",
                        exe_path
                    ),
                ])
                .creation_flags(0x08000000) // CREATE_NO_WINDOW
                .output()?;
        } else {
            std::process::Command::new("powershell")
                .args([
                    "-Command",
                    &format!(
                        "[Environment]::SetEnvironmentVariable('Path', [string]::join(';', ([Environment]::GetEnvironmentVariable('Path', 'Machine').Split(';') | Where-Object {{ $_ -ne '{}' }})), 'Machine')",
                        exe_path
                    ),
                ])
                .creation_flags(0x08000000) // CREATE_NO_WINDOW
                .output()?;
        }
    }

    // Remove registry entries
    if app_config.details.create_registry_key {
        let key: String;

        if app_config.details.current_user_only {
            key = format!(
                r"Software\Microsoft\Windows\CurrentVersion\Uninstall\{}",
                app_config.details.name
            );
            if let Ok(_) = CURRENT_USER.open(&key) {
                let _ = CURRENT_USER.remove_tree(&key);
            }
        } else {
            key = format!(
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\{}",
                app_config.details.name
            );
            if let Ok(_) = LOCAL_MACHINE.open(&key) {
                let _ = LOCAL_MACHINE.remove_tree(&key);
            }
        }
    }

    // Update app list
    let mut app_list = AppList::read().await?;
    app_list
        .links
        .retain(|existing_app| existing_app.timestamp != timestamp);
    app_list.save().await?;

    app.emit("uninstallation", 100)?;

    Ok("".to_owned())
}
