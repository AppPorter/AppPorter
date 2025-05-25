use crate::{
    configs::{
        app_list::{load_app_list, AppList},
        *,
    },
    core::*,
    operations::*,
};
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
    Install {
        config: InstallConfig,
    },
    Uninstall {
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

pub enum CommandResult {
    String(String),
    Serializable(Box<dyn ErasedSerialize + Send>),
}

impl CommandResult {
    pub fn as_string(&self) -> Option<&String> {
        if let CommandResult::String(s) = self {
            Some(s)
        } else {
            None
        }
    }
}

impl Command {
    // Helper function to wrap serializable results
    fn ser<T: ErasedSerialize + Send + 'static>(
        v: T,
    ) -> Result<CommandResult, Box<dyn Error + Send + Sync>> {
        // If the value is String, return CommandResult::String directly
        if let Some(s) = (&v as &dyn std::any::Any).downcast_ref::<String>() {
            return Ok(CommandResult::String(s.clone()));
        }
        Ok(CommandResult::Serializable(Box::new(v)))
    }

    // Routes command to appropriate handler function
    // Returns JSON-formatted response string or error message
    async fn execute(self, app: AppHandle) -> Result<CommandResult, Box<dyn Error + Send + Sync>> {
        use Command::*;
        match self {
            LoadSettings => Self::ser(Settings::read().await?),
            GetDetails { path } => Self::ser(get_details(path).await?),
            Install { config } => Self::ser(install(config, app).await?),
            Uninstall { timestamp } => Self::ser(uninstall(timestamp, app).await?),
            Elevate { revert } => Self::ser(elevate(revert).await?),
            ValidatePath { path } => Self::ser(validate_path(path).await?),
            SaveSettings { settings } => Self::ser(settings.save().await?),
            LoadAppList => Self::ser(load_app_list().await?),
            SaveAppList { app_list } => Self::ser(app_list.save().await?),
            GetArchiveContent { path, password } => {
                Self::ser(get_archive_content(path, password).await?)
            }
            Open { path } => Self::ser(open_app(&path).await?),
            OpenFolder { path } => Self::ser(open_folder(&path).await?),
            OpenRegistry {
                app_name,
                current_user_only,
            } => Self::ser(open_registry(&app_name, current_user_only).await?),
            CheckPathEmpty { path } => Self::ser(check_path_empty(&path).await?),
            Cli => Self::ser(cli(app).await?),
            RegisterContextMenu => Self::ser(register_context_menu()?),
            UnregisterContextMenu => Self::ser(unregister_context_menu()?),
            InstallWithLink { url, timestamp } => {
                Self::ser(install_with_link(url, timestamp).await?)
            }
            SetStartup => Self::ser(set_startup()?),
            RemoveStartup => Self::ser(remove_startup()?),
            CopyOnly { config } => Self::ser(copy_only(config, app).await?),
            Exit => {
                exit().await;
                Self::ser(())
            }
        }
    }
}

// Frontend-to-backend command bridge
#[tauri::command(async)]
pub async fn execute_command(command: Command, app: AppHandle) -> Result<String, String> {
    match command.execute(app).await {
        Ok(res) => {
            if let Some(s) = res.as_string() {
                Ok(s.clone())
            } else if let CommandResult::Serializable(val) = res {
                serde_json::to_string(&val).map_err(|e| e.to_string())
            } else {
                Ok(String::new())
            }
        }
        Err(e) => Err(e.to_string()),
    }
}
