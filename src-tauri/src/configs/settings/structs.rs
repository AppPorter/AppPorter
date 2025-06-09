use serde::{Deserialize, Serialize};

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(default)]
pub struct Settings {
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

#[derive(Debug, Deserialize, Serialize, Clone)]
#[serde(rename_all = "lowercase")]
pub enum LanguageType {
    En,
    Zh,
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

#[derive(Debug, Deserialize, Serialize, Clone)]
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

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(default)]
pub struct AppInstall {
    pub current_user_only: bool,
    pub all_users: InstallSettings,
    pub current_user: InstallSettings,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(default)]
pub struct InstallSettings {
    pub create_desktop_shortcut: bool,
    pub create_registry_key: bool,
    pub create_start_menu_shortcut: bool,
    pub install_path: String,
    pub add_to_path: bool,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(default)]
pub struct ToolInstall {
    pub install_path: String,
    pub add_to_path: bool,
}
