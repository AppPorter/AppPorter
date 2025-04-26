use crate::{configs::*, core::*, operations::*};
use erased_serde::Serialize as ErasedSerialize;
use serde::Deserialize;
use std::error::Error;
use tauri::AppHandle;

// Available frontend-to-backend commands
#[derive(Deserialize, Clone)]
#[serde(tag = "name")]
pub enum Command {
    LoadSettings,
    GetDetails {
        path: ExePath,
    },
    Installation {
        config: InstallationConfig,
    },
    Uninstallation {
        timestamp: i64,
    },
    Elevate {
        revert: bool,
    },
    ValidatePath {
        path: String,
    },
    SaveSettings {
        settings: Settings,
    },
    LoadAppList,
    SaveAppList {
        app_list: AppList,
    },
    GetArchiveContent {
        path: String,
        password: Option<String>,
    },
    Open {
        path: String,
    },
    OpenFolder {
        path: String,
    },
    OpenRegistry {
        app_name: String,
        current_user_only: bool,
    },
    CheckPathEmpty {
        path: String,
    },
    Cli,
    RegisterContextMenu,
    UnregisterContextMenu,
    InstallWithLink {
        url: String,
        timestamp: i64,
    },
    SetStartup,
    RemoveStartup,
    CopyOnly {
        config: CopyOnlyConfig,
    },
    Exit,
}

impl Command {
    // Routes command to appropriate handler function
    // Returns JSON-formatted response string or error message
    async fn execute(
        self,
        app: AppHandle,
    ) -> Result<Box<dyn ErasedSerialize + Send>, Box<dyn Error + Send + Sync>> {
        match self {
            Self::LoadSettings => Ok(Box::new(Settings::read().await?)),
            Self::GetDetails { path } => Ok(Box::new(get_details(path).await?)),
            Self::Installation { config } => Ok(Box::new(installation(config, app).await?)),
            Self::Uninstallation { timestamp } => {
                Ok(Box::new(uninstallation(timestamp, app).await?))
            }
            Self::Elevate { revert } => Ok(Box::new(elevate(revert).await?)),
            Self::ValidatePath { path } => Ok(Box::new(validate_path(path).await?)),
            Self::SaveSettings { settings } => Ok(Box::new(settings.save().await?)),
            Self::LoadAppList => Ok(Box::new(load_app_list().await?)),
            Self::SaveAppList { app_list } => Ok(Box::new(app_list.save().await?)),
            Self::GetArchiveContent { path, password } => {
                Ok(Box::new(get_archive_content(path, password).await?))
            }
            Self::Open { path } => Ok(Box::new(open_app(&path).await?)),
            Self::OpenFolder { path } => Ok(Box::new(open_folder(&path).await?)),
            Self::OpenRegistry {
                app_name,
                current_user_only,
            } => Ok(Box::new(open_registry(&app_name, current_user_only).await?)),
            Self::CheckPathEmpty { path } => Ok(Box::new(check_path_empty(&path).await?)),
            Self::Cli => Ok(Box::new(cli(app).await?)),
            Self::RegisterContextMenu => Ok(Box::new(register_context_menu()?)),
            Self::UnregisterContextMenu => Ok(Box::new(unregister_context_menu()?)),
            Self::InstallWithLink { url, timestamp } => {
                Ok(Box::new(install_with_link(url, timestamp).await?))
            }
            Self::SetStartup => Ok(Box::new(set_startup()?)),
            Self::RemoveStartup => Ok(Box::new(remove_startup()?)),
            Self::CopyOnly { config } => Ok(Box::new(copy_only(config, app).await?)),
            Self::Exit => {
                exit().await;
                Ok(Box::new(()))
            }
        }
    }
}

// Frontend-to-backend command bridge
#[tauri::command(async)]
pub async fn execute_command(command: Command, app: AppHandle) -> Result<String, String> {
    command
        .execute(app)
        .await
        .map(|res| serde_json::to_string(&res).unwrap())
        .map_err(|e| e.to_string())
}
