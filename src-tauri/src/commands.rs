use crate::{configs::*, core::*, operations::*, utils::*};
use Command::*;
use anyhow::Result;
use serde::Deserialize;
use serde_json::to_string;
use tauri::AppHandle;

macro_rules! json {
    ($expr:expr) => {
        Ok(to_string(&$expr)?)
    };
}

#[derive(Deserialize, Clone)]
#[serde(tag = "name")]
pub enum Command<'a> {
    LoadEnv,
    LoadSettings,
    SaveSettings {
        settings: Settings,
    },
    StartThemeMonitoring,
    StopThemeMonitoring,
    LoadLibrary,
    SaveLibrary {
        library: Library,
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
        config: App,
        zip_path: &'a str,
    },
    InstallTool {
        config: Tool,
        zip_path: &'a str,
    },
    UninstallApp {
        timestamp: i64,
    },
    UninstallTool {
        timestamp: i64,
    },
    RepairApp {
        timestamp: i64,
    },
    RepairTool {
        timestamp: i64,
    },
    ReinstallApp {
        timestamp: i64,
        zip_path: &'a str,
    },
    ReinstallTool {
        timestamp: i64,
        zip_path: &'a str,
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
    DetermineInputType {
        input: &'a str,
    },
}

impl<'a> Command<'a> {
    async fn execute(self, app: AppHandle) -> Result<String> {
        match self {
            LoadEnv => json!(Env::read().await?),
            LoadSettings => json!(Settings::read().await?),
            SaveSettings { settings } => json!(settings.save().await?),
            StartThemeMonitoring => json!(Settings::start_theme_monitoring(app)?),
            StopThemeMonitoring => json!(Settings::stop_theme_monitoring()?),
            LoadLibrary => json!(Library::load().await?),
            SaveLibrary { library } => json!(library.save().await?),

            Cli => json!(cli(&app).await?),
            RegisterContextMenu => json!(register_context_menu()?),
            UnregisterContextMenu => json!(unregister_context_menu()?),
            Elevate { revert } => json!(elevate(revert).await?),
            Exit { code } => json!(exit(code).await?),
            SetStartup => json!(set_startup()?),
            RemoveStartup => json!(remove_startup()?),

            PreviewUrl { url } => json!(preview_url(&app, url).await?),
            InstallApp { config, zip_path } => {
                json!(install_app(config, zip_path, &app).await?)
            }
            InstallTool { config, zip_path } => {
                json!(install_tool(config, zip_path, &app).await?)
            }
            UninstallApp { timestamp } => json!(uninstall_app(timestamp).await?),
            UninstallTool { timestamp } => json!(uninstall_tool(timestamp).await?),
            RepairApp { timestamp } => json!(repair_app(timestamp).await?),
            RepairTool { timestamp } => json!(repair_tool(timestamp).await?),
            ReinstallApp {
                timestamp,
                zip_path,
            } => {
                json!(reinstall_app(timestamp, zip_path).await?)
            }
            ReinstallTool {
                timestamp,
                zip_path,
            } => {
                json!(reinstall_tool(timestamp, zip_path).await?)
            }

            OpenApp { path } => json!(open_app(path).await?),
            OpenFolder { path } => json!(open_folder(path).await?),
            OpenRegistry {
                app_name,
                current_user_only,
            } => json!(open_registry(app_name, current_user_only).await?),

            GetDetails { path } => json!(get_details(path).await?),
            RunInstaller { path } => json!(
                run_installer(
                    &path.zip_path,
                    &path.executable_path,
                    path.password.as_deref().unwrap_or_default(),
                    &app,
                )
                .await?
            ),
            ConvertIconToBase64 { path } => json!(convert_icon_to_base64(path).await?),
            ValidatePath { path } => json!(validate_path(path).await?),
            GetArchiveContent { path, password } => {
                json!(get_archive_content(path, password).await?)
            }
            CheckPathEmpty { path } => json!(check_path_empty(path).await?),
            GetArchiveTree { path, password } => {
                json!(get_archive_tree(path, password).await?)
            }
            GetTimestamp => json!(chrono::Utc::now().timestamp()),
            DetermineInputType { input } => json!(determine_input_type(input).await?),
        }
    }
}

#[tauri::command(async)]
pub async fn exec(cmd: Command<'_>, app: AppHandle) -> Result<String, String> {
    match cmd.execute(app).await {
        Ok(result) => Ok(result),
        Err(e) => Err(e.to_string()),
    }
}
