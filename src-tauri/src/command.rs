use crate::{configs::*, core::*, operations::*};
use erased_serde::Serialize as ErasedSerialize;
use serde::Deserialize;
use std::error::Error;
use tauri::AppHandle;
use Command::*;

// Available frontend-to-backend commands
#[derive(Deserialize, Clone)]
#[serde(tag = "name")]
pub enum Command {
    // Configs
    LoadEnv,
    SaveEnv {
        env: Env,
    },
    LoadSettings,
    SaveSettings {
        settings: Settings,
    },
    LoadAppList,
    SaveAppList {
        app_list: AppList,
    },

    // Core
    Cli,
    RegisterContextMenu,
    UnregisterContextMenu,
    Elevate {
        revert: bool,
    },
    Exit,
    SetStartup,
    RemoveStartup,

    // Operations - Install/Uninstall
    InstallApp {
        config: AppInstallConfig,
    },
    InstallLib {
        config: LibInstallConfig,
    },
    UninstallApp {
        timestamp: i64,
    },
    UninstallLib {
        timestamp: i64,
    },

    // Operations - Launcher
    OpenApp {
        path: String,
    },
    OpenFolder {
        path: String,
    },
    OpenRegistry {
        app_name: String,
        current_user_only: bool,
    },

    // Operations - Misc
    GetDetails {
        path: ExePath,
    },

    ValidatePath {
        path: String,
    },

    GetArchiveContent {
        path: String,
        password: Option<String>,
    },

    CheckPathEmpty {
        path: String,
    },
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
        match self {
            // Configs
            LoadEnv => Self::ser(Env::read().await?),
            SaveEnv { env } => Self::ser(env.save().await?),
            LoadSettings => Self::ser(Settings::read().await?),
            SaveSettings { settings } => Self::ser(settings.save().await?),
            LoadAppList => Self::ser(load_app_list().await?),
            SaveAppList { app_list } => Self::ser(app_list.save().await?),

            // Core
            Cli => Self::ser(cli(app).await?),
            RegisterContextMenu => Self::ser(register_context_menu()?),
            UnregisterContextMenu => Self::ser(unregister_context_menu()?),
            Elevate { revert } => Self::ser(elevate(revert).await?),
            Exit => {
                exit().await;
                Self::ser(())
            }
            SetStartup => Self::ser(set_startup()?),
            RemoveStartup => Self::ser(remove_startup()?),

            // Operations - Install/Uninstall
            InstallApp { config } => Self::ser(install_app(config, app).await?),
            InstallLib { config } => Self::ser(install_lib(config, app).await?),
            UninstallApp { timestamp } => Self::ser(uninstall_app(timestamp, app).await?),
            UninstallLib { timestamp } => Self::ser(uninstall_lib(timestamp, app).await?),

            // Operations - Launcher
            OpenApp { path } => Self::ser(open_app(&path).await?),
            OpenFolder { path } => Self::ser(open_folder(&path).await?),
            OpenRegistry {
                app_name,
                current_user_only,
            } => Self::ser(open_registry(&app_name, current_user_only).await?),

            // Operations - Misc
            GetDetails { path } => Self::ser(get_details(path).await?),
            ValidatePath { path } => Self::ser(validate_path(path).await?),
            GetArchiveContent { path, password } => {
                Self::ser(get_archive_content(path, password).await?)
            }
            CheckPathEmpty { path } => Self::ser(check_path_empty(&path).await?),
        }
    }
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
