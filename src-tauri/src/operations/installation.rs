use super::sanitize_path;
use crate::configs::app_list::ValidationStatus;
use crate::configs::ConfigFile;
use crate::configs::{
    app_list::{App, AppList, InstalledApp},
    settings::Settings,
};
use crate::get_7z_path;
use mslnk::ShellLink;
use serde::Deserialize;
use std::io::Read;
use std::os::windows::process::CommandExt;
use std::process::Stdio;
use std::{error::Error, path::Path};
use tauri::{AppHandle, Emitter};
use windows_registry::{CURRENT_USER, LOCAL_MACHINE};

#[derive(Deserialize, Debug, Clone)]
pub struct InstallationConfig {
    zip_path: String,
    details: InstalledApp,
    timestamp: i64,
}

pub async fn installation(
    config: InstallationConfig,
    app: AppHandle,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let zip_path = config.zip_path.clone();
    let app_path = format!(
        "{}\\{}",
        config.details.install_path,
        config.details.name.replace(" ", "-")
    );

    app.emit("installation", 0)?;

    let target_path = Path::new(&app_path);
    if !target_path.exists() {
        tokio::fs::create_dir_all(target_path).await?;
    }
    extract_files(&zip_path, &app_path, app.clone()).await?;

    app.emit("installation", 101)?;

    let single_root = find_single_root_folder(&app_path).await?;

    let full_executable_path = get_full_executable_path(
        &app_path,
        &config.details.executable_path,
        single_root.as_deref(),
    );

    let settings = Settings::read().await?;

    // Create shortcuts
    if config.details.create_start_menu_shortcut {
        create_start_menu_shortcut(
            &settings.system_drive_letter,
            &settings.username,
            &config.details.name,
            &full_executable_path,
            config.details.current_user_only,
        )?;
    }

    if config.details.create_desktop_shortcut {
        create_desktop_shortcut(&full_executable_path, &config.details.name)?;
    }

    let timestamp = chrono::Utc::now().timestamp();

    if config.details.create_registry_key {
        create_registry_entries(&config, &full_executable_path, &app_path, timestamp)?;
    }

    if config.details.add_to_path {
        let exe_path = Path::new(&full_executable_path)
            .parent()
            .expect("Failed to get parent directory")
            .to_string_lossy();

        if config.details.current_user_only {
            let key = CURRENT_USER.create("Environment")?;
            let path = key.get_string("Path")?;

            if !path.split(';').any(|p| p.trim() == exe_path.trim()) {
                let new_path = format!("{};{}", path, exe_path);
                key.set_expand_string("Path", new_path)?;
            }
        } else {
            let key = LOCAL_MACHINE
                .create(r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment")?;
            let path = key.get_string("path")?;

            if !path.split(';').any(|p| p.trim() == exe_path.trim()) {
                let new_path = format!("{};{}", path, exe_path);
                key.set_expand_string("path", new_path)?;
            }
        }
    }

    // Add to app list
    let mut app_list = AppList::read().await?;

    // Create a mutable copy of the details
    let mut updated_details = config.details.clone();
    updated_details.full_path = full_executable_path.clone();
    updated_details.validation_status = ValidationStatus {
        file_exists: true,
        registry_valid: true,
    };

    let new_app = App {
        timestamp: if config.timestamp != 0 {
            config.timestamp
        } else {
            timestamp
        },
        installed: true,
        details: updated_details,
        url: "".to_owned(),
    };

    if config.timestamp != 0 {
        // Update existing app with matching timestamp
        if let Some(existing_app) = app_list
            .links
            .iter_mut()
            .find(|app| app.timestamp == config.timestamp)
        {
            existing_app.installed = true;
            existing_app.details = new_app.details;
        }
    } else {
        // Remove existing similar app and add new one
        app_list.links.retain(|existing_app| {
            let mut app1 = existing_app.clone();
            let mut app2 = new_app.clone();
            app1.timestamp = 0;
            app2.timestamp = 0;
            app1.details.version = String::new();
            app2.details.version = String::new();
            app1 != app2
        });
        app_list.links.push(new_app);
    }

    app_list.save().await?;

    Ok(full_executable_path)
}

async fn extract_files(
    zip_path: &str,
    app_path: &str,
    app: AppHandle,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let path_7z = get_7z_path()?;

    let output_list = tokio::process::Command::new(&path_7z)
        .args([
            "l", // List contents command
            zip_path, "-y", // Yes to all prompts
        ])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output()
        .await?;

    if !output_list.status.success() {
        return Err("Failed to list archive contents".into());
    }

    let output_str = String::from_utf8_lossy(&output_list.stdout);
    let paths = parse_7z_list_output(&output_str);

    // Validate all paths to make sure there's no path traversal attack
    let canonical_app_path = std::fs::canonicalize(app_path)?;
    for path in &paths {
        let target_path = Path::new(app_path).join(sanitize_path(path));

        // Try to get the canonical path (resolving any ../ etc)
        if let Ok(canonical_path) =
            std::fs::canonicalize(target_path.parent().unwrap_or(Path::new(app_path)))
        {
            // Check if this file would extract outside our target directory
            if !canonical_path.starts_with(&canonical_app_path) {
                return Err(format!(
                    "Security violation: Path traversal detected in archive: {}",
                    path
                )
                .into());
            }
        }
    }

    // If all paths are safe, proceed with extraction
    let mut child = std::process::Command::new(&path_7z)
        .args([
            "-bsp2", // set output stream
            "x",     // Extract files with full paths
            zip_path,
            &format!("-o{}", app_path),
            "-y",   // Yes to all prompts
            "-aoa", // Overwrite all existing files without prompt
            "-snl", // Disable symbolic links
        ])
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .stderr(Stdio::piped())
        .spawn()?;

    let mut stderr = child.stderr.take().unwrap();
    let mut buffer = [0; 1024];
    let app_clone = app.clone();

    let handle = std::thread::spawn(move || {
        while let Ok(n) = stderr.read(&mut buffer) {
            if n == 0 {
                break;
            }

            if let Ok(output) = String::from_utf8(buffer[..n].to_vec()) {
                if output.contains("%") {
                    let parts: Vec<&str> = output.split('%').collect();
                    if let Ok(percent) = parts[0].trim().parse::<u32>() {
                        let _ = app_clone.emit("installation_extract", percent);
                    }
                }
            }
        }
    });

    let status = child.wait()?;
    if !status.success() {
        return Err("7-Zip extraction failed".into());
    }
    handle.join().unwrap();

    app.emit("installation_extract", 100)?;

    Ok(())
}

fn parse_7z_list_output(output: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut is_output_section = false;

    for line in output.lines() {
        if line.contains("------------------------") {
            // Toggle output section when separator line is found
            is_output_section = !is_output_section;
            continue;
        }

        // Only process lines between separators
        if is_output_section {
            if let Some(last_field) = line.split_whitespace().last() {
                result.push(last_field.to_string());
            }
        }
    }

    result
}

// Detect if extraction created a single root folder
async fn find_single_root_folder(
    app_path: &str,
) -> Result<Option<String>, Box<dyn Error + Send + Sync>> {
    let mut entries = tokio::fs::read_dir(app_path).await?;
    let mut items = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        items.push(entry.file_name().to_string_lossy().to_string());
    }

    if items.len() == 1 {
        let single_item = &items[0];
        let full_path = Path::new(app_path).join(single_item);
        if tokio::fs::metadata(full_path).await?.is_dir() {
            return Ok(Some(single_item.clone()));
        }
    }

    Ok(None)
}

fn get_full_executable_path(
    app_path: &str,
    executable_path: &str,
    single_root: Option<&str>,
) -> String {
    if let Some(root) = single_root {
        let exe_path = if executable_path.starts_with(&format!("{}/", root)) {
            executable_path
                .strip_prefix(&format!("{}/", root))
                .unwrap_or(executable_path)
                .to_string()
        } else {
            executable_path.to_string()
        };
        format!(r"{}\{}", app_path, exe_path.replace("/", r"\"))
    } else {
        format!(r"{}\{}", app_path, executable_path.replace("/", r"\"))
    }
}

fn create_start_menu_shortcut(
    system_drive: &str,
    username: &str,
    app_name: &str,
    executable_path: &str,
    current_user_only: bool,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let lnk_path = if current_user_only {
        format!(
            r"{}:\Users\{}\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\{}.lnk",
            system_drive, username, app_name
        )
    } else {
        format!(
            r"{}:\ProgramData\Microsoft\Windows\Start Menu\Programs\{}.lnk",
            system_drive, app_name
        )
    };
    ShellLink::new(executable_path)?.create_lnk(lnk_path)?;
    Ok(())
}

fn create_desktop_shortcut(
    executable_path: &str,
    app_name: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    ShellLink::new(executable_path)?.create_lnk(format!(
        r"{}\{}.lnk",
        dirs::desktop_dir()
            .ok_or("Failed to get desktop directory")?
            .to_string_lossy(),
        app_name
    ))?;
    Ok(())
}

fn create_registry_entries(
    config: &InstallationConfig,
    executable_path: &str,
    app_path: &str,
    timestamp: i64,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let key = if config.details.current_user_only {
        CURRENT_USER.create(format!(
            r"Software\Microsoft\Windows\CurrentVersion\Uninstall\{}",
            config.details.name
        ))?
    } else {
        LOCAL_MACHINE.create(format!(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\{}",
            config.details.name
        ))?
    };

    key.set_string("Comments", "Installed with AppPorter")?;
    key.set_string("DisplayIcon", executable_path)?;
    key.set_string("DisplayName", &config.details.name)?;
    key.set_string("DisplayVersion", &config.details.version)?;
    key.set_string("InstallLocation", app_path)?;
    key.set_u32("NoModify", 1)?;
    key.set_u32("NoRemove", 0)?;
    key.set_u32("NoRepair", 1)?;
    key.set_string("Publisher", &config.details.publisher)?;
    key.set_string(
        "UninstallString",
        format!(
            "\"{}\" uninstall {}",
            std::env::current_exe()?.to_string_lossy(),
            timestamp
        ),
    )?;
    Ok(())
}
