pub(crate) use config::Config;
use serde::Deserialize;
use std::{error::Error, result::Result};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub installation: Installation,
}

#[derive(Debug, Deserialize)]
pub struct Installation {
    pub install_mode: InstallMode,
    pub start_path: String,
    pub all_users: InstallSettings,
    pub current_user: InstallSettings,
}

#[derive(Debug, Deserialize)]
pub struct InstallSettings {
    pub create_desktop_shortcut: bool,
    pub create_registry_key: bool,
    pub create_start_menu_shortcut: bool,
    pub enable_installation_screen: bool,
    pub install_path: String,
    pub registry: RegistrySettings,
}

#[derive(Debug, Deserialize)]
pub enum InstallMode {
    AllUsers,
    CurrentUser,
}

#[derive(Debug, Deserialize)]
pub struct RegistrySettings {
    pub create_comments: bool,
    pub create_display_icon: bool,
    pub create_display_name: bool,
    pub create_display_version: bool,
    pub create_estimated_size: bool,
    pub create_install_location: bool,
    pub create_no_modify: bool,
    pub create_no_remove: bool,
    pub create_no_repair: bool,
    pub create_publisher: bool,
    pub create_uninstall_string: bool,
}

impl Settings {
    pub fn read() -> Result<Self, Box<dyn Error>> {
        let settings = Config::builder()
            .add_source(config::File::with_name("Settings"))
            .build()?
            .try_deserialize::<Settings>()?;
        Ok(settings)
    }
}
