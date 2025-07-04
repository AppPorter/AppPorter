use serde::{Deserialize, Serialize};
use ts_rs::TS;

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, TS)]
#[ts(export)]
#[serde(default)]
pub struct Settings {
    pub first_run: bool,
    pub language: LanguageType,
    pub theme: ThemeType,
    pub minimize_to_tray_on_close: bool,
    pub context_menu: bool,
    pub auto_startup: bool,
    pub color: String,
    pub run_as_admin: bool,
    pub app_install: AppInstall,
    pub tool_install: ToolInstall,
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, TS)]
#[ts(export)]
#[serde(rename_all = "lowercase")]
pub enum LanguageType {
    En,
    #[serde(rename = "zh-hans")]
    ZhHans,
    #[serde(rename = "zh-hant")]
    ZhHant,
    Fr,
    De,
    Es,
    Ja,
    Ko,
    Ru,
}

impl Default for LanguageType {
    fn default() -> Self {
        Self::En
    }
}

#[derive(Debug, Deserialize, Serialize, Clone, PartialEq, Eq, TS)]
#[ts(export)]
#[serde(rename_all = "lowercase")]
pub enum ThemeType {
    System,
    Light,
    Dark,
}

impl Default for ThemeType {
    fn default() -> Self {
        Self::System
    }
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq, TS)]
#[ts(export)]
#[serde(default)]
pub struct AppInstall {
    pub current_user_only: bool,
    pub all_users: InstallSettings,
    pub current_user: InstallSettings,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq, TS)]
#[ts(export)]
#[serde(default)]
pub struct InstallSettings {
    pub create_desktop_shortcut: bool,
    pub create_registry_key: bool,
    pub create_start_menu_shortcut: bool,
    pub install_path: String,
    pub add_to_path: bool,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq, TS)]
#[ts(export)]
#[serde(default)]
pub struct ToolInstall {
    pub install_path: String,
    pub add_to_path: bool,
}
