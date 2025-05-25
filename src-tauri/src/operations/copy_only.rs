use super::get_7z_path;
use crate::configs::{
    app_list::{AppBasicInformation, AppConfig, AppDetails, AppList, AppPaths},
    ConfigFile,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::io::Read;
use std::os::windows::process::CommandExt;
use std::path::Path;
use std::process::Stdio;
use tauri::{AppHandle, Emitter};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct CopyOnlyConfig {
    zip_path: String,
    password: Option<String>,
    extract_path: String,
    name: String,
    timestamp: i64,
}

pub async fn copy_only(
    config: CopyOnlyConfig,
    app: AppHandle,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let zip_path = config.zip_path.clone();
    let extract_path = config.extract_path.clone();

    // Notify frontend preparing
    app.emit("copyonly", 0)?;

    // Ensure extract directory exists
    if !Path::new(&extract_path).exists() {
        tokio::fs::create_dir_all(&extract_path).await?;
    }

    // Create full path by combining extract_path and app name
    let app_path = format!("{}\\{}", extract_path, config.name.replace(" ", "-"));

    let target_path = Path::new(&app_path);
    if !target_path.exists() {
        tokio::fs::create_dir_all(target_path).await?;
    }

    // Prepare 7z extraction args
    let path_7z = get_7z_path()?;
    let arg = format!("-o{}", app_path);
    let mut args = vec![
        "-bsp2", // output stream for progress
        "x",     // extract with full paths
        &zip_path, &arg, "-y",   // yes to all
        "-aoa", // overwrite all
        "-snl", // disable symbolic links
    ];

    let mut pw = String::new();
    if let Some(pass) = &config.password {
        pw = format!("-p{}", pass);
    }
    args.push(&pw);

    let mut child = std::process::Command::new(&path_7z)
        .args(&args)
        .creation_flags(0x08000000)
        .stderr(Stdio::piped())
        .spawn()?;

    // Spawn thread to read progress and emit events
    if let Some(mut stderr) = child.stderr.take() {
        let app_clone = app.clone();
        std::thread::spawn(move || {
            let mut buffer = [0; 1024];
            while let Ok(n) = stderr.read(&mut buffer) {
                if n == 0 {
                    break;
                }
                if let Ok(output) = String::from_utf8(buffer[..n].to_vec()) {
                    if output.contains('%') {
                        if let Ok(percent) = output
                            .trim()
                            .split('%')
                            .next()
                            .unwrap_or("")
                            .trim()
                            .parse::<u32>()
                        {
                            let _ = app_clone.emit("copyonly_extract", percent);
                        }
                    }
                }
            }
        });
    }

    let status = child.wait()?;
    if !status.success() {
        return Err("7-Zip extraction failed".into());
    }

    // Notify frontend completed
    app.emit("copyonly", 101)?;

    // Add to app list
    let mut app_list = AppList::read().await?;
    let timestamp = if config.timestamp != 0 {
        config.timestamp
    } else {
        chrono::Utc::now().timestamp()
    };
    let details = AppDetails {
        info: AppBasicInformation {
            name: config.name,
            icon: String::new(),
            publisher: String::new(),
            version: String::new(),
        },
        config: AppConfig {
            archive_exe_path: String::new(),
            archive_password: config.password.unwrap_or_default(),
            current_user_only: true,
            create_desktop_shortcut: false,
            create_start_menu_shortcut: false,
            create_registry_key: false,
            add_to_path: false,
            path_directory: String::new(),
        },
        paths: AppPaths {
            parent_install_path: extract_path.clone(),
            install_path: extract_path.clone(),
            full_path: app_path,
        },
        validation_status: LibValidationStatus {
            file_exists: true,
            path_exists: true,
        },
    };
    let app_item = Lib {
        timestamp,
        installed: true,
        details,
        url: String::new(),
    };
    if config.timestamp != 0 {
        // Update existing app with matching timestamp
        if let Some(existing_app) = app_list
            .libs
            .iter_mut()
            .find(|app| app.timestamp == config.timestamp)
        {
            existing_app.installed = true;
            existing_app.details = app_item.details;
        }
    } else {
        // Remove existing similar app and add new one
        app_list.libs.retain(|existing_app| {
            let mut app1 = existing_app.clone();
            let mut app2 = app_item.clone();
            app1.timestamp = 0;
            app2.timestamp = 0;
            app1.details.info.version = String::new();
            app2.details.info.version = String::new();
            app1 != app2
        });
        app_list.libs.push(app_item);
    }
    app_list.save().await?;

    Ok(extract_path)
}
