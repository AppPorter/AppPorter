use crate::{
    configs::{
        app_list::{AppList, Lib, LibConfig, LibDetails, LibPaths, LibValidationStatus},
        ConfigFile,
    },
    operations::extract_archive_files,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;
use tauri::AppHandle;

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct LibInstallConfig {
    zip_path: String,
    password: Option<String>,
    extract_path: String,
    name: String,
    timestamp: i64,
}

pub async fn install_lib(
    config: LibInstallConfig,
    app: AppHandle,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let zip_path = config.zip_path.clone();
    let extract_path = config.extract_path.clone();

    // Ensure extract directory exists
    if !Path::new(&extract_path).exists() {
        tokio::fs::create_dir_all(&extract_path).await?;
    }

    // Create full path by combining extract_path and app name
    let install_path = format!("{}\\{}", extract_path, config.name.replace(" ", "-"));

    let target_path = Path::new(&install_path);
    if !target_path.exists() {
        tokio::fs::create_dir_all(target_path).await?;
    }

    // Extract files using shared function
    extract_archive_files(
        &zip_path,
        &install_path,
        app,
        config.password.as_deref(),
        "install_lib_extract",
    )
    .await?;

    // Add to app list
    let mut app_list = AppList::read().await?;
    let timestamp = if config.timestamp != 0 {
        config.timestamp
    } else {
        chrono::Utc::now().timestamp()
    };
    let details = LibDetails {
        name: config.name,
        config: LibConfig {
            archive_password: config.password.unwrap_or_default(),
            add_to_path: false,
            archive_path_directory: String::new(),
            full_path_directory: String::new(),
        },
        paths: LibPaths {
            parent_install_path: extract_path.clone(),
            install_path: install_path.clone(),
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
            app1 != app2
        });
        app_list.libs.push(app_item);
    }
    app_list.save().await?;

    Ok(extract_path)
}
