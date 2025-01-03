pub(crate) use config::Config;
use serde::{Deserialize, Serialize};
use std::{error::Error, path::PathBuf};

#[derive(Debug, Deserialize, Serialize)]
pub struct Settings {
    pub language: String,
    pub minimize_to_tray_on_close: bool,
    pub system_drive_letter: String,
    pub theme: String,
    pub username: String,
    pub installation: Installation,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct Installation {
    pub current_user_only: bool,
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

impl Settings {
    fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
        let exe_dir = std::env::current_exe()?
            .parent()
            .ok_or("Failed to get exe directory")?
            .to_path_buf();
        Ok(exe_dir.join("Settings.toml"))
    }

    fn create_default_config() -> Result<Self, Box<dyn Error>> {
        let default_settings = Self {
            language: String::from("en"),
            minimize_to_tray_on_close: false,
            system_drive_letter: String::new(),
            theme: String::from("system"),
            username: String::new(),
            installation: Installation {
                current_user_only: false,
                all_users: InstallSettings {
                    create_desktop_shortcut: false,
                    create_registry_key: true,
                    create_start_menu_shortcut: true,
                    install_path: String::new(),
                },
                current_user: InstallSettings {
                    create_desktop_shortcut: false,
                    create_registry_key: true,
                    create_start_menu_shortcut: true,
                    install_path: String::new(),
                },
            },
        };

        let config_path = Self::get_config_path()?;
        let content = toml::to_string_pretty(&default_settings)?;
        std::fs::write(config_path, content)?;

        Ok(default_settings)
    }

    pub fn read() -> Result<Self, Box<dyn Error>> {
        let config_path = Self::get_config_path()?;

        let settings = if !config_path.exists() {
            Self::create_default_config()?
        } else {
            Config::builder()
                .add_source(config::File::from(config_path))
                .build()?
                .try_deserialize::<Settings>()?
        };

        Ok(settings)
    }

    pub fn save(&self) -> Result<(), Box<dyn Error>> {
        let config_path = Self::get_config_path()?;
        let content = toml::to_string_pretty(&self)?;
        std::fs::write(config_path, content)?;
        Ok(())
    }

    pub fn initialization(&mut self) -> Result<(), Box<dyn Error>> {
        let system_drive_letter = &std::env::var("windir")?[..1];
        let username = &std::env::var("USERNAME")?;

        self.system_drive_letter = system_drive_letter.to_string();
        self.username = username.to_string();

        if self.installation.all_users.install_path.is_empty() {
            self.installation.all_users.install_path =
                format!(r"{}:\Program Files", system_drive_letter);
        }

        if self.installation.current_user.install_path.is_empty() {
            self.installation.current_user.install_path = format!(
                r"{}:\Users\{}\AppData\Local\Programs",
                system_drive_letter, username
            );
        }

        self.save()?;
        Ok(())
    }
}

pub fn load_settings() -> Result<String, Box<dyn Error>> {
    let mut settings = Settings::read()?;
    settings.initialization()?;
    Ok(serde_json::to_string(&settings)?)
}
