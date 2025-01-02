pub(crate) use config::Config;
use serde::{Deserialize, Serialize};
use std::{error::Error, result::Result};

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub download_dir: String,
    pub language: String,
    pub minimize_to_tray_on_close: bool,
    pub system_drive_letter: String,
    pub theme: String,
    pub username: String,
    pub installation: Installation,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Installation {
    pub install_mode: InstallMode,
    pub start_path: String,
    pub zip_path: String,
    pub all_users: InstallSettings,
    pub current_user: InstallSettings,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct InstallSettings {
    pub create_desktop_shortcut: bool,
    pub create_registry_key: bool,
    pub create_start_menu_shortcut: bool,
    pub install_path: String,
}

#[derive(Debug, Deserialize, Serialize)]
pub enum InstallMode {
    AllUsers,
    CurrentUser,
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
        if self.installation.start_path.is_empty() {
            Config::builder()
                .set_override("download_dir", &*download_dir)?
                .set_override("installation.start_path", download_dir)?
                .build()?;
        }

        if self.installation.all_users.install_path.is_empty() {
            Config::builder()
                .set_override(
                    "installation.all_users.install_path",
                    format!(r"{}:\Program Files", system_drive_letter),
                )?
                .set_override(r"system_drive_letter", system_drive_letter)?
                .build()?;
        }

        if self.installation.current_user.install_path.is_empty() {
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

pub fn load_settings() -> Result<Settings, Box<dyn Error>> {
    let settings = Settings::read()?;
    Ok(settings)
}
