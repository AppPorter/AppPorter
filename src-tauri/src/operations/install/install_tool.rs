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
    timestamp: i64,
    name: String,
    extract_path: String,
}

pub async fn install_tool(
    config: ToolInstallConfig,
    app: AppHandle,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    let timestamp = chrono::Utc::now().timestamp();

    // Send install start event
    app.emit("tool_install_progress", 0)?;

    // Setup installation directory and extract files
    let install_path = setup_installation_directory(&config).await?;
    extract_archive_files(
        &config.zip_path,
        &install_path,
        app.clone(),
        config.password.as_deref(),
        "tool_install_progress",
    )
    .await?;

    // Store extract path before moving config
    let extract_path = config.extract_path.clone();

    // Update tool list
    update_tool_list(config, install_path.clone(), timestamp).await?;

    // Send completion event
    app.emit("tool_install_progress", 101)?;

    Ok(extract_path)
}

async fn setup_installation_directory(
    config: &ToolInstallConfig,
) -> Result<String, Box<dyn Error + Send + Sync>> {
    // Ensure extract directory exists
    if !Path::new(&config.extract_path).exists() {
        tokio::fs::create_dir_all(&config.extract_path).await?;
    }

    // Create full path by combining extract_path and tool name
    let install_path = format!("{}\\{}", config.extract_path, config.name.replace(" ", "-"));

    let target_path = Path::new(&install_path);
    if !target_path.exists() {
        tokio::fs::create_dir_all(target_path).await?;
    }

    Ok(install_path)
}

async fn update_tool_list(
    config: ToolInstallConfig,
    install_path: String,
    timestamp: i64,
) -> Result<(), Box<dyn Error + Send + Sync>> {
    let mut app_list = Library::read().await?;
    let tool_timestamp = if config.timestamp != 0 {
        config.timestamp
    } else {
        timestamp
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
            parent_install_path: config.extract_path.clone(),
            install_path: install_path.clone(),
        },
        validation_status: ToolValidationStatus {
            file_exists: true,
            path_exists: true,
        },
    };

    let new_tool = Tool {
        timestamp: tool_timestamp,
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
            existing_tool.details = new_tool.details;
        } else {
            // If tool doesn't exist, add it as new
            app_list.tools.push(new_tool);
        }
    } else {
        // Remove existing similar tool and add new one
        app_list.tools.retain(|existing_tool| {
            let mut tool1 = existing_tool.clone();
            let mut tool2 = new_tool.clone();
            tool1.timestamp = 0;
            tool2.timestamp = 0;
            tool1 != tool2
        });
        app_list.tools.push(new_tool);
    }

    app_list.save().await?;
    Ok(())
}
