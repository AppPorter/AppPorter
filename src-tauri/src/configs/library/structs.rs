use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq, TS)]
#[ts(export)]
#[serde(default)]
pub struct Library {
    pub apps: Vec<App>,
    pub tools: Vec<Tool>,
    pub urls: Vec<Url>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq, TS)]
#[ts(export)]
#[serde(default)]
pub struct App {
    pub id: String,
    pub timestamp_add: String,
    pub timestamp_update: String,
    pub installed: bool,
    pub url: String,
    pub archive_password: String,
    pub details: AppDetails,
    pub validation_status: AppValidationStatus,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq, TS)]
#[ts(export)]
#[serde(default)]
pub struct AppDetails {
    pub current_user_only: bool,
    pub info: AppInfo,
    pub config: AppConfig,
    pub install_path: String,
    pub full_path: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq, TS)]
#[ts(export)]
#[serde(default)]
pub struct AppInfo {
    pub name: String,
    pub icon: String,
    pub publisher: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq, TS)]
#[ts(export)]
#[serde(default)]
pub struct AppConfig {
    pub custom_icon: bool,
    pub create_desktop_shortcut: bool,
    pub create_start_menu_shortcut: bool,
    pub create_registry_key: bool,
    pub add_to_path: (bool, String),
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq, TS)]
#[ts(export)]
#[serde(default)]
pub struct AppValidationStatus {
    pub file_exists: bool,
    pub registry_valid: bool,
    pub path_exists: bool,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq, TS)]
#[ts(export)]
#[serde(default)]
pub struct Tool {
    pub id: String,
    pub timestamp_add: String,
    pub timestamp_update: String,
    pub installed: bool,
    pub url: String,
    pub archive_password: String,
    pub details: ToolDetails,
    pub validation_status: ToolValidationStatus,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq, TS)]
#[ts(export)]
#[serde(default)]
pub struct ToolDetails {
    pub name: String,
    pub add_to_path: (bool, String),
    pub install_path: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq, TS)]
#[ts(export)]
#[serde(default)]
pub struct ToolValidationStatus {
    pub file_exists: bool,
    pub path_exists: bool,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq, TS)]
#[ts(export)]
#[serde(default)]
pub struct Url {
    pub id: String,
    pub url: String,
    pub timestamp: String,
}
