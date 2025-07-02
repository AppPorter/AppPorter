use crate::{configs::*, core::*, operations::*, utils::*};
use Command::*;
use anyhow::{Result, anyhow};
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

    InitApp {
        config: App,
    },
    InitTool {
        config: Tool,
    },
    InitUrl {
        config: Url,
    },
    Remove {
        id: &'a str,
    },
    HasLink {
        url: &'a str,
    },
    GetApp {
        id: &'a str,
    },
    GetTool {
        id: String,
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
        config: AppInstallConfig,
    },
    InstallTool {
        config: ToolInstallConfig,
    },
    UninstallApp {
        id: String,
    },
    UninstallTool {
        id: String,
    },
    RepairApp {
        id: String,
    },
    RepairTool {
        id: String,
    },
    ReinstallApp {
        id: String,
        zip_path: &'a str,
    },
    ReinstallTool {
        id: String,
        zip_path: &'a str,
    },
    ModifyApp {
        new_app: App,
        id: String,
    },
    ModifyTool {
        new_tool: Tool,
        id: String,
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
            StartThemeMonitoring => json!(start_theme_monitoring(app)?),
            StopThemeMonitoring => json!(stop_theme_monitoring()?),
            LoadLibrary => json!(Library::load().await?),
            SaveLibrary { library } => json!(library.save().await?),

            InitApp { config } => {
                let mut config = config;
                json!(Library::init_app(&mut config).await?)
            }
            InitTool { config } => {
                let mut config = config;
                json!(Library::init_tool(&mut config).await?)
            }
            InitUrl { config } => {
                let mut config = config;
                json!(Library::init_url(&mut config).await?)
            }
            Remove { id } => {
                json!(Library::load().await?.remove(id).await?)
            }
            HasLink { url } => json!(Library::load().await?.has_link(url).await),
            GetApp { id } => {
                json!(
                    Library::load()
                        .await?
                        .get_app(id)
                        .await
                        .ok_or(anyhow!("App with ID '{}' not found", id))?
                )
            }
            GetTool { id } => {
                json!(
                    Library::load()
                        .await?
                        .get_tool(&id)
                        .await
                        .ok_or(anyhow!("Tool with ID '{}' not found", id))?
                )
            }

            Cli => json!(cli(&app).await?),
            RegisterContextMenu => json!(register_context_menu()?),
            UnregisterContextMenu => json!(unregister_context_menu()?),
            Elevate { revert } => json!(elevate(revert).await?),
            Exit { code } => json!(exit(code).await?),
            SetStartup => json!(set_startup()?),
            RemoveStartup => json!(remove_startup()?),

            PreviewUrl { url } => json!(preview_url(&app, url).await?),
            InstallApp { config } => {
                json!(install_app(config, &app).await?)
            }
            InstallTool { config } => {
                json!(install_tool(config, &app).await?)
            }
            UninstallApp { id } => json!(uninstall_app(&id).await?),
            UninstallTool { id } => json!(uninstall_tool(&id).await?),
            RepairApp { id } => json!(repair_app(&id).await?),
            RepairTool { id } => json!(repair_tool(&id).await?),
            ReinstallApp { id, zip_path } => {
                json!(reinstall_app(&id, zip_path).await?)
            }
            ReinstallTool { id, zip_path } => {
                json!(reinstall_tool(&id, zip_path).await?)
            }
            ModifyApp { new_app, id } => {
                json!(modify_app(new_app, &id).await?)
            }
            ModifyTool { new_tool, id } => {
                json!(modify_tool(new_tool, &id).await?)
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
            GetTimestamp => json!(chrono::Utc::now().to_rfc3339()),
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
