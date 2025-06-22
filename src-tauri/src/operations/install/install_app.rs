use crate::configs::env::Env;
use crate::configs::library::*;
use crate::configs::ConfigFile;
use crate::operations::extract_archive_files;
use crate::operations::install::flatten_nested_folders;
use mslnk::ShellLink;
use serde::{Deserialize, Serialize};
use std::{error::Error, path::Path};
use tauri::{AppHandle, Emitter};
use windows_registry::{CURRENT_USER, LOCAL_MACHINE};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct AppInstallConfig {
    zip_path: String,
    password: Option<String>,
    timestamp: i64,
    details: AppDetails,
}

pub async fn install_app(
    config: AppInstallConfig,
    app: AppHandle,
) -> Result<(String, String), Box<dyn Error + Send + Sync>> {
    let timestamp = chrono::Utc::now().timestamp();

    // Send initial install progress event
    app.emit("app_install_progress", 0)?;

    let install_path = format!(
        "{}\\{}",
        config.details.paths.parent_install_path, config.details.info.name
    );

    // Setup installation directory and extract files
    tokio::fs::create_dir_all(&install_path).await?;
    extract_archive_files(
        &config.zip_path,
        &install_path,
        app.clone(),
        config.password.as_deref(),
        "app_install_progress",
    )
    .await?;

    // Flatten nested single folders to avoid deep nesting
    flatten_nested_folders(&install_path).await?;

    // Determine executable paths
    let full_path = format!(
        r"{}\{}",
        install_path,
        config.details.config.archive_exe_path.replace("/", r"\")
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

    // Create shortcuts
    let shell_link = ShellLink::new(&full_path)?;

    if config.details.config.create_desktop_shortcut {
        create_desktop_shortcut(&shell_link, &config.details.info.name)?;
    }

    if config.details.config.create_start_menu_shortcut {
        create_start_menu_shortcut(
            &shell_link,
            config.details.config.current_user_only,
            &config.details.info.name,
        )
        .await?;
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

    app.emit("app_install_progress", 101)?;

    Ok((install_path, full_path))
}

fn create_desktop_shortcut(
    shell_link: &ShellLink,
    app_name: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    shell_link.create_lnk(format!(
        r"{}\{}.lnk",
        dirs::desktop_dir()
            .ok_or("Failed to get desktop directory")?
            .to_string_lossy(),
        app_name
    ))?;
    Ok(())
}

async fn create_start_menu_shortcut(
    shell_link: &ShellLink,
    current_user_only: bool,
    app_name: &str,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let env = Env::read().await?;
    let lnk_path = if current_user_only {
        format!(
            r"{}:\Users\{}\AppData\Roaming\Microsoft\Windows\Start Menu\Programs\{}.lnk",
            env.system_drive_letter, env.username, app_name
        )
    } else {
        format!(
            r"{}:\ProgramData\Microsoft\Windows\Start Menu\Programs\{}.lnk",
            env.system_drive_letter, app_name
        )
    };
    shell_link.create_lnk(lnk_path)?;
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

async fn update_app_list(
    config: AppInstallConfig,
    full_path: String,
    full_path_directory: String,
    timestamp: i64,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut app_list = Library::read().await?;
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
