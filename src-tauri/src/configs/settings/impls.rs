use super::{AppInstall, InstallSettings, LanguageType, Settings, ThemeType, ToolInstall};
use crate::configs::{env::Env, ConfigFile};
use std::error::Error;

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
            color: String::new(),
            minimize_to_tray_on_close: false,
            run_as_admin: false,
            app_install: AppInstall {
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
            tool_install: ToolInstall {
                install_path: dirs::home_dir()
                    .map(|p| p.to_string_lossy().to_string())
                    .unwrap_or_else(|| String::from("C:\\")),
                add_to_path: true,
            },
        }
    }
}

impl Settings {
    pub async fn initialization(&mut self, env: Env) -> Result<(), Box<dyn Error + Send + Sync>> {
        self.run_as_admin = self.check_run_as_admin(env.user_sid)?;
        self.update_color_settings().await?;
        self.update_install_paths(env.system_drive_letter, env.username);
        self.save().await?;
        Ok(())
    }

    fn check_run_as_admin(&self, user_sid: String) -> Result<bool, Box<dyn Error + Send + Sync>> {
        let registry_path = format!(
            r"{}\Software\Microsoft\Windows NT\CurrentVersion\AppCompatFlags\Layers",
            user_sid
        );
        let exe_path = std::env::current_exe()?.to_string_lossy().to_string();

        Ok(windows_registry::USERS
            .open(registry_path)?
            .get_string(exe_path)
            .is_ok_and(|value| value.contains("RUNASADMIN")))
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

    fn update_install_paths(&mut self, system_drive_letter: String, username: String) {
        if self.app_install.all_users.install_path.is_empty() {
            self.app_install.all_users.install_path =
                format!(r"{}:\Program Files", system_drive_letter);
        }

        if self.app_install.current_user.install_path.is_empty() {
            self.app_install.current_user.install_path = format!(
                r"{}:\Users\{}\AppData\Local\Programs",
                system_drive_letter, username
            );
        }
    }
}
