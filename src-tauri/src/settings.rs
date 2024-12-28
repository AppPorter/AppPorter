pub(crate) use config::Config;
use serde::Deserialize;
use std::{error::Error, result::Result};

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub download_dir: String,
    pub system_drive_letter: String,
    pub username: String,
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
    pub fn complete(&mut self) -> Result<(), Box<dyn Error>> {
        let system_drive_letter = &std::env::var("windir")?[..1];
        let username = &std::env::var("USERNAME")?;

        let download_dir = dirs::download_dir()
            .ok_or("Failed to get download directory")?
            .to_string_lossy()
            .to_string();
        if self.installation.start_path.is_empty()
            || ((self.installation.start_path == self.download_dir)
                && self.download_dir != download_dir)
        {
            Config::builder()
                .set_override("download_dir", &*download_dir)?
                .set_override("installation.start_path", download_dir)?
                .build()?;
        }

        if self.installation.all_users.install_path
            == format!(r"{}:\Program Files", self.system_drive_letter)
            && self.system_drive_letter != system_drive_letter
        {
            Config::builder()
                .set_override(
                    "installation.all_users.install_path",
                    format!(r"{}:\Program Files", system_drive_letter),
                )?
                .set_override(r"system_drive_letter", system_drive_letter)?
                .build()?;
        }

        if self.installation.current_user.install_path
            == format!(
                r"{}:\Users\{}\AppData\Local\Programs",
                self.system_drive_letter, self.username
            )
            && (self.system_drive_letter != system_drive_letter || &self.username != username)
        {
            Config::builder()
                .set_override(
                    "installation.current_user.install_path",
                    format!(
                        r"{}:\Users\{}\AppData\Local\Programs",
                        system_drive_letter, username
                    ),
                )?
                .set_override(r"system_drive_letter", system_drive_letter)?
                .set_override(r"username", username.as_str())?
                .build()?;
        }

        Ok(())
    }
}
