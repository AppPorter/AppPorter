use crate::{
    configs::{
        library::{Library, Tool, ToolConfig, ToolDetails, ToolPaths, ToolValidationStatus},
        ConfigFile,
    },
    operations::extract_archive_files,
};
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::path::Path;
use tauri::{AppHandle, Emitter};

#[derive(Deserialize, Serialize, Debug, Clone)]
pub struct ToolInstallConfig {
    zip_path: String,
    password: Option<String>,
    extract_path: String,
    name: String,
    timestamp: i64,
}

pub async fn install_tool(
    config: ToolInstallConfig,
    app: AppHandle,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Send install start event
    app.emit("tool_install", 0)?;
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
        app.clone(),
        config.password.as_deref(),
        "tool_install_extract",
    )
    .await?;

    // Add to app list
    let mut app_list = Library::read().await?;
    let timestamp = if config.timestamp != 0 {
        config.timestamp
    } else {
        chrono::Utc::now().timestamp()
    };
    let details = ToolDetails {
        name: config.name,
        config: ToolConfig {
            archive_password: config.password.unwrap_or_default(),
            add_to_path: false,
            archive_path_directory: String::new(),
            full_path_directory: String::new(),
        },
        paths: ToolPaths {
            parent_install_path: extract_path.clone(),
            install_path: install_path.clone(),
        },
        validation_status: ToolValidationStatus {
            file_exists: true,
            path_exists: true,
        },
    };
    let app_item = Tool {
        timestamp,
        installed: true,
        details,
        url: String::new(),
    };
    if config.timestamp != 0 {
        // Update existing tool with matching timestamp
        if let Some(existing_tool) = app_list
            .tools
            .iter_mut()
            .find(|tool| tool.timestamp == config.timestamp)
        {
            existing_tool.installed = true;
            existing_tool.details = app_item.details;
        } else {
            // If tool doesn't exist, add it as new
            app_list.tools.push(app_item);
        }
    } else {
        // Remove existing similar tool and add new one
        app_list.tools.retain(|existing_tool| {
            let mut tool1 = existing_tool.clone();
            let mut tool2 = app_item.clone();
            tool1.timestamp = 0;
            tool2.timestamp = 0;
            tool1 != tool2
        });
        app_list.tools.push(app_item);
    }
    app_list.save().await?;

    // Send completion event
    app.emit("tool_install", 101)?;

    Ok(extract_path)
}
