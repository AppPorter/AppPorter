use check_elevation::is_elevated;
use config::Config;
use serde::{Deserialize, Serialize};
use std::{error::Error, path::PathBuf};
use tokio::process::Command;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Settings {
    #[serde(default)]
    pub language: String,
    #[serde(default)]
    pub theme: String,
    #[serde(default)]
    pub minimize_to_tray_on_close: bool,

    #[serde(default)]
    pub color: String,
    #[serde(default)]
    pub debug: bool,
    #[serde(default)]
    pub elevated: bool,
    #[serde(default)]
    pub run_as_admin: bool,
    #[serde(default)]
    pub system_drive_letter: String,
    #[serde(default)]
    pub user_sid: String,
    #[serde(default)]
    pub username: String,

    #[serde(default)]
    pub installation: Installation,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Installation {
    #[serde(default)]
    pub current_user_only: bool,

    #[serde(default)]
    pub all_users: InstallSettings,
    #[serde(default)]
    pub current_user: InstallSettings,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct InstallSettings {
    #[serde(default)]
    pub create_desktop_shortcut: bool,
    #[serde(default)]
    pub create_registry_key: bool,
    #[serde(default)]
    pub create_start_menu_shortcut: bool,
    #[serde(default)]
    pub install_path: String,
}

impl Settings {
    async fn get_config_path() -> Result<PathBuf, Box<dyn Error>> {
        let exe_dir = tokio::fs::canonicalize(
            std::env::current_exe()?
                .parent()
                .ok_or("Failed to get exe directory")?
                .to_path_buf(),
        )
        .await?;
        Ok(exe_dir.join("Settings.toml"))
    }

    async fn create_default_config() -> Result<Self, Box<dyn Error>> {
        let default_settings = Self {
            language: String::from("en"),
            theme: String::from("system"),
            minimize_to_tray_on_close: false,

            color: String::new(),
            debug: false,
            elevated: false,
            run_as_admin: false,
            system_drive_letter: String::new(),
            user_sid: String::new(),
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

        let config_path = Self::get_config_path().await?;
        let settings_clone = default_settings.clone();
        let content =
            tokio::task::spawn_blocking(move || toml::to_string_pretty(&settings_clone)).await??;
        tokio::fs::write(config_path, content).await?;

        Ok(default_settings)
    }

    pub async fn read() -> Result<Self, Box<dyn Error>> {
        let config_path = Self::get_config_path().await?;

        let settings = if !tokio::fs::try_exists(&config_path).await? {
            Self::create_default_config().await?
        } else {
            let content = tokio::fs::read_to_string(&config_path).await?;
            let content_clone = content.clone();
            tokio::task::spawn_blocking(move || {
                Config::builder()
                    .add_source(config::File::from_str(
                        &content_clone,
                        config::FileFormat::Toml,
                    ))
                    .build()?
                    .try_deserialize::<Settings>()
            })
            .await??
        };

        Ok(settings)
    }

    pub async fn save(&self) -> Result<(), Box<dyn Error>> {
        let config_path = Self::get_config_path().await?;
        let settings_str = tokio::task::spawn_blocking({
            let settings = self.clone();
            move || toml::to_string_pretty(&settings)
        })
        .await??;
        tokio::fs::write(config_path, settings_str).await?;
        Ok(())
    }

    pub async fn initialization(&mut self) -> Result<(), Box<dyn Error>> {
        #[cfg(debug_assertions)]
        {
            self.debug = true;
        }
        #[cfg(not(debug_assertions))]
        {
            self.debug = false;
        }

        let output = tokio::process::Command::new("powershell")
            .args([
                "-NoProfile",
                "-NonInteractive",
                "-ExecutionPolicy",
                "Bypass",
                "-Command",
                r#"Get-WmiObject Win32_UserAccount -Filter "Name='$env:USERNAME'" | Select-Object -ExpandProperty SID"#,
            ])
            .output()
            .await?;
        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into());
        }
        self.user_sid = String::from_utf8_lossy(&output.stdout).trim().to_string();

        match windows_registry::USERS
            .open(format!(
                r"{}\Software\Microsoft\Windows NT\CurrentVersion\AppCompatFlags\Layers",
                self.user_sid
            ))?
            .get_string(std::env::current_exe()?.to_string_lossy().to_string())
        {
            Ok(value) => self.run_as_admin = value.contains("RUNASADMIN"),
            Err(_) => self.run_as_admin = false,
        }

        let accent_color_bgr = format!(
            "{:x}",
            windows_registry::CURRENT_USER
                .open(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Accent")?
                .get_u32("AccentColorMenu")?
        );
        let accent_color_b = &accent_color_bgr[2..4];
        let accent_color_g: &_ = &accent_color_bgr[4..6];
        let accent_color_r = &accent_color_bgr[6..8];
        let accent_color_rgb = format!("#{}{}{}", accent_color_r, accent_color_g, accent_color_b);
        self.color = accent_color_rgb;

        self.elevated = is_elevated()?;
        self.system_drive_letter = std::env::var("windir")?[..1].to_string();
        self.username = std::env::var("USERNAME")?.to_string();

        if self.installation.all_users.install_path.is_empty() {
            self.installation.all_users.install_path =
                format!(r"{}:\Program Files", self.system_drive_letter);
        }

        if self.installation.current_user.install_path.is_empty() {
            self.installation.current_user.install_path = format!(
                r"{}:\Users\{}\AppData\Local\Programs",
                self.system_drive_letter, self.username
            );
        }

        self.save().await?;
        Ok(())
    }
}

pub async fn load_settings() -> Result<String, Box<dyn Error>> {
    let mut settings = Settings::read().await?;
    settings.initialization().await?;
    Ok(serde_json::to_string(&settings)?)
}
