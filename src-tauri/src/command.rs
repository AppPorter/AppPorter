use crate::{configs::*, core::*, operations::*, utils::*};
use Command::*;
use anyhow::Result;
use erased_serde::Serialize as ErasedSerialize;
use serde::Deserialize;
use tauri::AppHandle;

#[derive(Deserialize, Clone)]
#[serde(tag = "name")]
pub enum Command<'a> {
    LoadEnv,
    LoadSettings,
    SaveSettings {
        settings: Box<Settings>,
    },
    StartThemeMonitoring,
    StopThemeMonitoring,
    LoadLibrary,
    SaveLibrary {
        library: Box<Library>,
    },

    Cli,
    RegisterContextMenu,
    UnregisterContextMenu,
    Elevate {
        revert: bool,
    },
    Exit {
        code: i32,
    },
    SetStartup,
    RemoveStartup,

    PreviewUrl {
        url: &'a str,
    },
    InstallApp {
        config: Box<AppInstallConfig<'a>>,
    },
    InstallTool {
        config: Box<ToolInstallConfig<'a>>,
    },
    UninstallApp {
        timestamp: i64,
    },
    UninstallTool {
        timestamp: i64,
    },

    OpenApp {
        path: &'a str,
    },
    OpenFolder {
        path: &'a str,
    },
    OpenRegistry {
        app_name: &'a str,
        current_user_only: bool,
    },

    GetDetails {
        path: ExePath,
    },
    RunInstaller {
        path: ExePath,
    },
    ConvertIconToBase64 {
        path: &'a str,
    },
    ValidatePath {
        path: &'a str,
    },
    GetArchiveContent {
        path: &'a str,
        password: Option<&'a str>,
    },
    CheckPathEmpty {
        path: &'a str,
    },
    GetArchiveTree {
        path: &'a str,
        password: Option<&'a str>,
    },
    GetTimestamp,
}

impl<'a> Command<'a> {
    fn ser<T: ErasedSerialize + Send + 'static>(v: T) -> Result<CommandResult> {
        if let Some(s) = (&v as &dyn std::any::Any).downcast_ref::<String>() {
            return Ok(CommandResult::String(s.clone()));
        }
        Ok(CommandResult::Serializable(Box::new(v)))
    }

    async fn execute(self, app: AppHandle) -> Result<CommandResult> {
        match self {
            LoadEnv => Self::ser(Env::read().await?),
            LoadSettings => Self::ser(Settings::read().await?),
            SaveSettings { settings } => Self::ser((*settings).save().await?),
            StartThemeMonitoring => {
                Settings::start_theme_monitoring(app);
                Self::ser(())
            }
            StopThemeMonitoring => {
                Settings::stop_theme_monitoring();
                Self::ser(())
            }
            LoadLibrary => Self::ser(Library::load().await?),
            SaveLibrary { library } => Self::ser((*library).save().await?),

            Cli => Self::ser(cli(&app).await?),
            RegisterContextMenu => Self::ser(register_context_menu()?),
            UnregisterContextMenu => Self::ser(unregister_context_menu()?),
            Elevate { revert } => Self::ser(elevate(revert).await?),
            Exit { code } => Self::ser(exit(code).await?),
            SetStartup => Self::ser(set_startup()?),
            RemoveStartup => Self::ser(remove_startup()?),

            PreviewUrl { url } => Self::ser(preview_url(&app, url).await?),
            InstallApp { config } => Self::ser(install_app(*config, &app).await?),
            InstallTool { config } => Self::ser(install_tool(*config, &app).await?),
            UninstallApp { timestamp } => Self::ser(uninstall_app(timestamp, &app).await?),
            UninstallTool { timestamp } => Self::ser(uninstall_tool(timestamp, &app).await?),

            OpenApp { path } => Self::ser(open_app(path).await?),
            OpenFolder { path } => Self::ser(open_folder(path).await?),
            OpenRegistry {
                app_name,
                current_user_only,
            } => Self::ser(open_registry(app_name, current_user_only).await?),

            GetDetails { path } => Self::ser(get_details(path).await?),
            RunInstaller { path } => Self::ser(
                run_installer(
                    &path.zip_path,
                    &path.executable_path,
                    path.password.as_deref(),
                    &app,
                )
                .await?,
            ),
            ConvertIconToBase64 { path } => Self::ser(convert_icon_to_base64(path).await?),
            ValidatePath { path } => Self::ser(validate_path(path).await?),
            GetArchiveContent { path, password } => {
                Self::ser(get_archive_content(path, password).await?)
            }
            CheckPathEmpty { path } => Self::ser(check_path_empty(path).await?),
            GetArchiveTree { path, password } => Self::ser(get_archive_tree(path, password).await?),
            GetTimestamp => Self::ser(get_timestamp()?),
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

#[tauri::command(async)]
pub async fn execute_command(command: Command<'_>, app: AppHandle) -> Result<String, String> {
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
