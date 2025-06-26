use super::{AppInstall, InstallSettings, LanguageType, Settings, ThemeType, ToolInstall};
use crate::configs::{ConfigFile, env::Env};
use crate::core::{context_menu, startup};
use anyhow::{Result, anyhow};
use std::path::PathBuf;
use std::sync::Arc;
use std::sync::atomic::{AtomicBool, Ordering};
use std::thread;
use std::time::Duration;
use tauri::{AppHandle, Emitter};
use windows_registry::CURRENT_USER;

static THEME_MONITORING_ACTIVE: AtomicBool = AtomicBool::new(false);

#[async_trait::async_trait]
impl ConfigFile for Settings {
    fn get_file_path() -> Result<PathBuf> {
        Ok(dirs::config_dir()
            .ok_or_else(|| anyhow!("Failed to get config directory"))?
            .join("AppPorter")
            .join("Settings.json"))
    }
}

impl Default for Settings {
    fn default() -> Self {
        let system_drive = std::env::var("windir")
            .map(|s| s[..1].to_owned())
            .unwrap_or("C".to_owned());
        let username = std::env::var("USERNAME").unwrap_or("user".to_owned());
        Self {
            first_run: true,
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
                    .unwrap_or("C:\\".to_owned()),
                add_to_path: true,
            },
        }
    }
}

impl Settings {
    pub async fn initialize() -> Result<()> {
        let mut settings = Settings::read().await?;
        let env = Env::read().await?;

        settings.run_as_admin = settings.check_run_as_admin(&env.user_sid)?;
        settings.color = Self::get_system_accent_color().unwrap_or("ff8c00".to_owned());
        settings.update_install_paths(&env.system_drive_letter, &env.username);
        settings.context_menu = context_menu::check_and_fix_context_menu()?;
        settings.auto_startup = startup::check_and_fix_startup()?;

        settings.save().await?;
        Ok(())
    }

    pub fn start_theme_monitoring(app_handle: AppHandle) {
        if THEME_MONITORING_ACTIVE.swap(true, Ordering::SeqCst) {
            return;
        }

        let app_handle = Arc::new(app_handle);
        thread::spawn(move || {
            let mut last_color = Self::get_system_accent_color().unwrap_or("ff8c00".to_owned());

            while THEME_MONITORING_ACTIVE.load(Ordering::SeqCst) {
                thread::sleep(Duration::from_secs(1));

                if let Ok(current_color) = Self::get_system_accent_color() {
                    if current_color != last_color {
                        last_color = current_color.clone();
                        let _ = app_handle.emit("theme-color-changed", &current_color);
                    }
                }
            }

            THEME_MONITORING_ACTIVE.store(false, Ordering::SeqCst);
        });
    }

    pub fn stop_theme_monitoring() {
        THEME_MONITORING_ACTIVE.store(false, Ordering::SeqCst);
    }

    pub fn get_system_accent_color() -> Result<String> {
        let accent_color = CURRENT_USER
            .open(r"Software\Microsoft\Windows\CurrentVersion\Explorer\Accent")?
            .get_u32("StartColorMenu")?;

        let accent_color_str = format!("{:08x}", accent_color);
        let (b, g, r) = (
            &accent_color_str[2..4],
            &accent_color_str[4..6],
            &accent_color_str[6..8],
        );
        Ok(format!("#{}{}{}", r, g, b))
    }

    fn check_run_as_admin(&self, user_sid: &str) -> Result<bool> {
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

    fn update_install_paths(&mut self, system_drive_letter: &str, username: &str) {
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
