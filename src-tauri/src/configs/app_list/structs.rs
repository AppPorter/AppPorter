use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(default)]
pub struct AppList {
    pub apps: Vec<App>,
    pub libs: Vec<Lib>,
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
    pub add_to_path: bool,
    pub path_directory: String,
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
pub struct Lib {
    pub timestamp: i64,
    pub installed: bool,
    pub url: String,
    pub details: LibDetails,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct LibDetails {
    pub name: String,
    pub config: LibConfig,
    pub paths: LibPaths,
    pub validation_status: LibValidationStatus,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct LibConfig {
    pub archive_password: String,
    pub add_to_path: bool,
    pub path_directory: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct LibPaths {
    pub parent_install_path: String,
    pub install_path: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct LibValidationStatus {
    pub file_exists: bool,
    pub path_exists: bool,
}
