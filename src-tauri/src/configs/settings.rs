use crate::configs::ConfigFile;
use check_elevation::is_elevated;
use serde::{Deserialize, Serialize};
use std::error::Error;
use tokio::process::Command;

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

#[serde(default)]
#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Settings {
    pub language: LanguageType,
    pub theme: ThemeType,
    pub minimize_to_tray_on_close: bool,
    pub context_menu: bool,
    pub auto_startup: bool,
    pub first_run: bool,
    pub color: String,
    pub debug: bool,
    pub elevated: bool,
    pub run_as_admin: bool,
    pub system_drive_letter: String,
    pub user_sid: String,
    pub username: String,
    pub installation: Installation,
    pub copy_only: CopyOnly,
}

#[serde(default)]
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct Installation {
    pub current_user_only: bool,
    pub all_users: InstallSettings,
    pub current_user: InstallSettings,
}

#[serde(default)]
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct InstallSettings {
    pub create_desktop_shortcut: bool,
    pub create_registry_key: bool,
    pub create_start_menu_shortcut: bool,
    pub install_path: String,
    pub add_to_path: bool,
}

#[serde(default)]
#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct CopyOnly {
    pub install_path: String,
    pub add_to_path: bool,
}

#[async_trait::async_trait]
impl ConfigFile for Settings {
    fn get_filename() -> &'static str {
        "Settings.json"
    }
}

impl Default for Settings {
    fn default() -> Self {
        let system_drive = std::env::var("windir")
            .map(|s| s[..1].to_string())
            .unwrap_or_else(|_| "C".to_string());
        let username = std::env::var("USERNAME").unwrap_or_else(|_| "user".to_string());
        Self {
            language: LanguageType::En,
            theme: ThemeType::System,
            context_menu: true,
            auto_startup: false,
            minimize_to_tray_on_close: false,
            first_run: true,
            color: String::new(),
            debug: cfg!(debug_assertions),
            elevated: is_elevated().unwrap_or(false),
            run_as_admin: false,
            system_drive_letter: system_drive.clone(),
            user_sid: String::new(),
            username: username.clone(),
            installation: Installation {
                current_user_only: false,
                all_users: InstallSettings {
                    create_desktop_shortcut: false,
                    create_registry_key: true,
                    create_start_menu_shortcut: true,
                    install_path: format!(r"{}:\Program Files", system_drive),
                    add_to_path: false,
                },
                current_user: InstallSettings {
                    create_desktop_shortcut: false,
                    create_registry_key: true,
                    create_start_menu_shortcut: true,
                    install_path: format!(
                        r"{}:\Users\{}\AppData\Local\Programs",
                        system_drive, username
                    ),
                    add_to_path: false,
                },
            },
            copy_only: CopyOnly {
                install_path: dirs::home_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| String::from("C:\\")),
                add_to_path: true,
            },
        }
    }
}

impl Settings {
    pub async fn initialization(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.debug = cfg!(debug_assertions);
        self.elevated = is_elevated()?;
        self.user_sid = self.get_user_sid().await?;
        self.run_as_admin = self.check_run_as_admin()?;
        self.update_system_info()?;
        self.update_color_settings().await?;
        self.update_install_paths();
        self.save().await?;
        Ok(())
    }

    async fn get_user_sid(&self) -> Result<String, Box<dyn Error + Send + Sync>> {
        let output = Command::new("powershell")
            .args([
                "-NoProfile", "-NonInteractive", "-ExecutionPolicy", "Bypass",
                "-Command",
                r#"Get-WmiObject Win32_UserAccount -Filter "Name='$env:USERNAME'" | Select-Object -ExpandProperty SID"#,
            ])
            .creation_flags(0x08000000) // CREATE_NO_WINDOW flag
            .output()
            .await?;

        if !output.status.success() {
            return Err(String::from_utf8_lossy(&output.stderr).into());
        }
        Ok(String::from_utf8_lossy(&output.stdout).trim().to_string())
    }

    fn check_run_as_admin(&self) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let registry_path = format!(
            r"{}\Software\Microsoft\Windows NT\CurrentVersion\AppCompatFlags\Layers",
            self.user_sid
        );
        let exe_path = std::env::current_exe()?.to_string_lossy().to_string();

        Ok(windows_registry::USERS
            .open(registry_path)?
            .get_string(exe_path)
            .is_ok_and(|value| value.contains("RUNASADMIN")))
    }

    fn update_system_info(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.system_drive_letter = std::env::var("windir")?[..1].to_string();
        self.username = std::env::var("USERNAME")?;
        Ok(())
    }

    // Extract Windows accent color from registry
    async fn update_color_settings(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        let accent_color = windows_registry::CURRENT_USER
            .open(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Accent")?
            .get_u32("AccentColorMenu")?;

        // Convert ABGR format to RGB hex
        let accent_color_str = format!("{:08x}", accent_color);
        let (b, g, r) = (
            &accent_color_str[2..4],
            &accent_color_str[4..6],
            &accent_color_str[6..8],
        );
        self.color = format!("#{}{}{}", r, g, b);
        Ok(())
    }

    fn update_install_paths(&mut self) {
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
    }
}
