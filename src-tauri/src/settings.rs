use config::Config;
use serde::Deserialize;
use std::{error::Error, result::Result};

#[derive(Debug, Deserialize)]
pub struct Settings {
    installation: Installation,
}

#[derive(Debug, Deserialize)]
struct Installation {
    default_install_mode: InstallMode,
    default_zip_path: String,
    all_users: InstallSettings,
    current_user: InstallSettings,
}

#[derive(Debug, Deserialize)]
struct InstallSettings {
    create_desktop_shortcut: bool,
    create_start_menu_shortcut: bool,
    create_taskbar_shortcut: bool,
    enable_installation_screen: bool,
    install_path: String,
    registry: RegistrySettings,
}

#[derive(Debug, Deserialize, PartialEq)]
enum InstallMode {
    #[serde(rename = "All_Users")]
    AllUsers,
    #[serde(rename = "Current_User")]
    CurrentUser,
}

#[derive(Debug, Deserialize)]
struct RegistrySettings {
    create_comments: bool,
    create_display_icon: bool,
    create_display_name: bool,
    create_display_version: bool,
    create_estimated_size: bool,
    create_install_location: bool,
    create_no_modify: bool,
    create_no_remove: bool,
    create_no_repair: bool,
    create_publisher: bool,
    create_uninstall_string: bool,
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
