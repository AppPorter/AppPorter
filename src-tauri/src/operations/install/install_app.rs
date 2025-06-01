use crate::configs::app_list::AppValidationStatus;
use crate::configs::app_list::{App, AppDetails, AppList};
use crate::configs::env::Env;
use crate::configs::ConfigFile;
use crate::operations::{get_7z_path, sanitize_path};
use mslnk::ShellLink;
use serde::{Deserialize, Serialize};
use std::io::Read;
use std::os::windows::process::CommandExt;
use std::process::Stdio;
use std::{error::Error, path::Path};
use tauri::{AppHandle, Emitter};
use windows_registry::{CURRENT_USER, LOCAL_MACHINE};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AppInstallConfig {
    zip_path: String,
    password: Option<String>,
    details: AppDetails,
    timestamp: i64,
}

pub async fn install_app(
    config: AppInstallConfig,
    app: AppHandle,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let timestamp = chrono::Utc::now().timestamp();

    let install_path = format!(
        "{}\\{}",
        config.details.paths.install_path,
        config.details.info.name.replace(" ", "-")
    );

    // Setup installation directory and extract files
    tokio::fs::create_dir_all(&install_path).await?;
    extract_files(
        &config.zip_path,
        &install_path,
        app.clone(),
        config.password.as_deref(),
    )
    .await?;

    // Determine executable paths
    let single_root = find_single_root_folder(&install_path).await?;
    let full_path = get_full_executable_path(
        &install_path,
        &config.details.config.archive_exe_path,
        single_root.as_deref(),
    );

    let full_path_directory = if config.details.config.archive_path_directory.is_empty() {
        Path::new(&full_path)
            .parent()
            .expect("Failed to get parent directory")
            .to_string_lossy()
            .to_string()
    } else {
        let normalized_path = config
            .details
            .config
            .archive_path_directory
            .trim_start_matches(['/', '\\'])
            .replace("/", "\\");
        format!("{}\\{}", install_path, normalized_path)
    };

    // Setup system integration
    let env = Env::read().await?;

    // Create shortcuts
    if config.details.config.create_start_menu_shortcut {
        create_start_menu_shortcut(
            &env.system_drive_letter,
            &env.username,
            &config.details.info.name,
            &full_path,
            config.details.config.current_user_only,
        )?;
    }

    if config.details.config.create_desktop_shortcut {
        create_desktop_shortcut(&full_path, &config.details.info.name)?;
    }

    // Create registry entries if requested
    if config.details.config.create_registry_key {
        create_registry_entries(&config, &full_path, &install_path, timestamp)?;
    }

    // Add to PATH if requested
    if config.details.config.add_to_path {
        add_to_path(
            &full_path_directory,
            config.details.config.current_user_only,
        )?;
    }

    // Update app list
    update_app_list(config, full_path.clone(), full_path_directory, timestamp).await?;

    Ok(full_path)
}

async fn update_app_list(
    config: AppInstallConfig,
    full_path: String,
    full_path_directory: String,
    timestamp: i64,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut app_list = AppList::read().await?;
    let mut updated_details = config.details.clone();
    updated_details.paths.full_path = full_path;
    updated_details.config.full_path_directory = full_path_directory;
    updated_details.validation_status = AppValidationStatus {
        file_exists: true,
        registry_valid: true,
        path_exists: true,
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
            .apps
            .iter_mut()
            .find(|app| app.timestamp == config.timestamp)
        {
            existing_app.installed = true;
            existing_app.details = new_app.details;
        }
    } else {
        // Remove existing similar app and add new one
        app_list.apps.retain(|existing_app| {
            let mut app1 = existing_app.clone();
            let mut app2 = new_app.clone();
            app1.timestamp = 0;
            app2.timestamp = 0;
            app1.details.info.version = String::new();
            app2.details.info.version = String::new();
            app1 != app2
        });
        app_list.apps.push(new_app);
    }

    app_list.save().await?;
    Ok(())
}

async fn extract_files(
    zip_path: &str,
    install_path: &str,
    app: AppHandle,
    password: Option<&str>,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let path_7z = get_7z_path()?;
    let path_7z_str = path_7z.to_string_lossy();
    let password_arg = password.map(|p| format!("-p{}", p)).unwrap_or_default();

    // List command to validate archive
    let list_args = vec!["l", zip_path, "-y", &password_arg];
    let output_list = tokio::process::Command::new(&*path_7z_str)
        .args(list_args)
        .creation_flags(0x08000000) // CREATE_NO_WINDOW
        .output()
        .await?;

    if !output_list.status.success() {
        let error_str = String::from_utf8_lossy(&output_list.stderr);
        if error_str.contains("Cannot open encrypted archive. Wrong password?") {
            return Err("Wrong password".into());
        }
        return Err("Failed to list archive contents".into());
    }

    let output_str = String::from_utf8_lossy(&output_list.stdout);
    let paths = parse_7z_list_output(&output_str);

    // Validate all paths to make sure there's no path traversal attack
    let canonical_install_path = std::fs::canonicalize(install_path)?;
    for path in &paths {
        let target_path = Path::new(install_path).join(sanitize_path(path));

        // Try to get the canonical path (resolving any ../ etc)
        if let Ok(canonical_path) =
            std::fs::canonicalize(target_path.parent().unwrap_or(Path::new(install_path)))
        {
            // Check if this file would extract outside our target directory
            if !canonical_path.starts_with(&canonical_install_path) {
                return Err(format!(
                    "Security violation: Path traversal detected in archive: {}",
                    path
                )
                .into());
            }
        }
    }

    let dir = format!("-o{}", install_path);

    // If all paths are safe, proceed with extraction
    let mut extract_args = vec![
        "-bsp2", // set output stream
        "x",     // Extract files with full paths
        zip_path, &dir, "-y",   // Yes to all prompts
        "-aoa", // Overwrite all existing files without prompt
        "-snl", // Disable symbolic links
    ];
    extract_args.push(&password_arg);

    let mut child = std::process::Command::new(&*path_7z_str)
        .args(extract_args)
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
                        let _ = app_clone.emit("install_extract", percent);
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

fn add_to_path(
    path_directory: &str,
    current_user_only: bool,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let (key, path_key) = if current_user_only {
        (CURRENT_USER.create("Environment")?, "Path")
    } else {
        (
            LOCAL_MACHINE
                .create(r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment")?,
            "path",
        )
    };

    let current_path = key.get_string(path_key)?;

    if !current_path
        .split(';')
        .any(|p| p.trim() == path_directory.trim())
    {
        let new_path = format!("{};{}", current_path, path_directory);
        key.set_expand_string(path_key, new_path)?;
    }

    Ok(())
}

fn create_registry_entries(
    config: &AppInstallConfig,
    full_path: &str,
    install_path: &str,
    timestamp: i64,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let key = if config.details.config.current_user_only {
        CURRENT_USER.create(format!(
            r"Software\Microsoft\Windows\CurrentVersion\Uninstall\{}",
            config.details.info.name
        ))?
    } else {
        LOCAL_MACHINE.create(format!(
            r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\{}",
            config.details.info.name
        ))?
    };

    key.set_string("Comments", "Installed with AppPorter")?;
    key.set_string("DisplayIcon", full_path)?;
    key.set_string("DisplayName", &config.details.info.name)?;
    key.set_string("DisplayVersion", &config.details.info.version)?;
    key.set_string("InstallLocation", install_path)?;
    key.set_u32("NoModify", 1)?;
    key.set_u32("NoRemove", 0)?;
    key.set_u32("NoRepair", 1)?;
    key.set_string("Publisher", &config.details.info.publisher)?;
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

// Detect if extraction created a single root folder
async fn find_single_root_folder(
    install_path: &str,
) -> Result<Option<String>, Box<dyn Error + Send + Sync>> {
    let mut entries = tokio::fs::read_dir(install_path).await?;
    let mut items = Vec::new();

    while let Some(entry) = entries.next_entry().await? {
        items.push(entry.file_name().to_string_lossy().to_string());
    }

    if items.len() == 1 {
        let single_item = &items[0];
        let full_path = Path::new(install_path).join(single_item);
        if tokio::fs::metadata(full_path).await?.is_dir() {
            return Ok(Some(single_item.clone()));
        }
    }

    Ok(None)
}

fn get_full_executable_path(
    install_path: &str,
    archive_exe_path: &str,
    single_root: Option<&str>,
) -> String {
    if let Some(root) = single_root {
        let exe_path = if archive_exe_path.starts_with(&format!("{}/", root)) {
            archive_exe_path
                .strip_prefix(&format!("{}/", root))
                .unwrap_or(archive_exe_path)
                .to_string()
        } else {
            archive_exe_path.to_string()
        };
        format!(r"{}\{}", install_path, exe_path.replace("/", r"\"))
    } else {
        format!(r"{}\{}", install_path, archive_exe_path.replace("/", r"\"))
    }
}
