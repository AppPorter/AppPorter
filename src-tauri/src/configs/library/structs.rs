use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(default)]
pub struct Library {
    pub apps: Vec<App>,
    pub tools: Vec<Tool>,
    pub urls: Vec<Url>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct App {
    pub timestamp: i64,
    pub installed: bool,
    pub url: String,
    pub details: AppDetails,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct AppDetails {
    pub info: AppBasicInformation,
    pub config: AppConfig,
    pub paths: AppPaths,
    pub validation_status: AppValidationStatus,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct AppBasicInformation {
    pub name: String,
    pub icon: String,
    pub publisher: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct AppConfig {
    pub archive_exe_path: String,
    pub archive_password: String,
    pub current_user_only: bool,
    pub create_desktop_shortcut: bool,
    pub create_start_menu_shortcut: bool,
    pub create_registry_key: bool,
    pub custom_icon: bool,
    pub add_to_path: bool,
    pub archive_path_directory: String,
    pub full_path_directory: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct AppPaths {
    pub parent_install_path: String,
    pub install_path: String,
    pub full_path: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct AppValidationStatus {
    pub file_exists: bool,
    pub registry_valid: bool,
    pub path_exists: bool,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct Tool {
    pub timestamp: i64,
    pub installed: bool,
    pub url: String,
    pub details: ToolDetails,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct ToolDetails {
    pub name: String,
    pub config: ToolConfig,
    pub paths: ToolPaths,
    pub validation_status: ToolValidationStatus,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct ToolConfig {
    pub archive_password: String,
    pub add_to_path: bool,
    pub archive_path_directory: String,
    pub full_path_directory: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct ToolPaths {
    pub parent_install_path: String,
    pub install_path: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct ToolValidationStatus {
    pub file_exists: bool,
    pub path_exists: bool,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct Url {
    pub url: String,
    pub timestamp: i64,
}
