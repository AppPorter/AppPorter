use crate::configs::ConfigFile;
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};
use std::{error::Error, path::Path};
use systemicons::get_icon;

use super::settings::Settings;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
pub struct AppList {
    #[serde(default)]
    pub links: Vec<App>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct App {
    #[serde(default)]
    pub timestamp: i64,
    #[serde(default)]
    pub installed: bool,
    #[serde(default)]
    pub url: String,
    #[serde(default)]
    pub details: InstalledApp,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct InstalledApp {
    #[serde(default)]
    pub name: String,
    #[serde(default)]
    pub icon: String,
    #[serde(default)]
    pub publisher: String,
    #[serde(default)]
    pub version: String,

    #[serde(default)]
    pub install_path: String,
    #[serde(default)]
    pub executable_path: String,
    #[serde(default)]
    pub full_path: String,

    #[serde(default)]
    pub current_user_only: bool,
    #[serde(default)]
    pub create_desktop_shortcut: bool,
    #[serde(default)]
    pub create_start_menu_shortcut: bool,
    #[serde(default)]
    pub create_registry_key: bool,
    #[serde(default)]
    pub add_to_path: bool,

    #[serde(default)]
    pub validation_status: ValidationStatus,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
pub struct ValidationStatus {
    #[serde(default)]
    pub file_exists: bool,
    #[serde(default)]
    pub registry_valid: bool,
}

#[async_trait::async_trait]
impl ConfigFile for AppList {
    fn get_filename() -> &'static str {
        "AppList.json"
    }
}

impl AppList {
    pub fn has_link(&self, url: &str) -> bool {
        self.links
            .iter()
            .any(|link| link.url == url && link.installed)
    }

    pub async fn validate_installations(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        for app in &mut self.links {
            if !app.installed {
                continue;
            }

            // Check if executable exists
            let file_exists = tokio::fs::try_exists(&app.details.full_path)
                .await
                .unwrap_or(false);

            // Check registry Comments value
            let registry_valid = if app.details.create_registry_key {
                let registry_path = if app.details.current_user_only {
                    format!(
                        r"Software\Microsoft\Windows\CurrentVersion\Uninstall\{}",
                        app.details.name
                    )
                } else {
                    format!(
                        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\{}",
                        app.details.name
                    )
                };

                let reg_key = if app.details.current_user_only {
                    windows_registry::CURRENT_USER.open(&registry_path)
                } else {
                    windows_registry::LOCAL_MACHINE.open(&registry_path)
                };

                reg_key
                    .and_then(|key| key.get_string("Comments"))
                    .is_ok_and(|v| v == "Installed with AppPorter")
            } else {
                true
            };

            app.details.validation_status = ValidationStatus {
                file_exists,
                registry_valid,
            };
        }
        Ok(())
    }

    pub fn remove_duplicates(&mut self) {
        let mut i = 0;
        while i < self.links.len() {
            let mut j = i + 1;
            while j < self.links.len() {
                if self.links[i].details.full_path == self.links[j].details.full_path {
                    // Check if all other fields except timestamp and version are identical
                    let mut app1 = self.links[i].clone();
                    let mut app2 = self.links[j].clone();

                    // Reset timestamp and version for comparison
                    app1.timestamp = 0;
                    app2.timestamp = 0;
                    app1.details.version = String::new();
                    app2.details.version = String::new();

                    if app1 == app2 {
                        // Remove the entry with earlier timestamp
                        if self.links[i].timestamp < self.links[j].timestamp {
                            self.links.remove(i);
                            j = i + 1;
                            continue;
                        } else {
                            self.links.remove(j);
                            continue;
                        }
                    }
                }
                j += 1;
            }
            i += 1;
        }
    }

    pub async fn sync_from_registry(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        // Get list of installed apps from registry
        let registry_apps = find_registry_apps()?;

        // For each app found in registry
        for reg_app in registry_apps {
            // Check if it's already in our list
            let app_exists = self.links.iter().any(|app| {
                app.installed
                    && app.details.name == reg_app.name
                    && app.details.full_path == reg_app.full_path
            });

            // If not in our list, add it
            if !app_exists {
                self.links.push(App {
                    timestamp: chrono::Utc::now().timestamp(),
                    installed: true,
                    url: String::new(),
                    details: reg_app,
                });
            }
        }

        fn find_registry_apps() -> Result<Vec<InstalledApp>, Box<dyn Error + Send + Sync>> {
            let mut result = Vec::new();

            // Search in both HKEY_CURRENT_USER and HKEY_LOCAL_MACHINE
            search_registry(
                &mut result,
                &windows_registry::CURRENT_USER,
                r"Software\Microsoft\Windows\CurrentVersion\Uninstall",
                true,
            )?;
            search_registry(
                &mut result,
                &windows_registry::LOCAL_MACHINE,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
                false,
            )?;

            Ok(result)
        }

        fn search_registry(
            result: &mut Vec<InstalledApp>,
            hkey: &windows_registry::Key,
            path: &str,
            current_user_only: bool,
        ) -> Result<(), Box<dyn Error + Send + Sync>> {
            if let Ok(uninstall_key) = hkey.open(path) {
                if let Ok(keys) = uninstall_key.keys() {
                    for key_result in keys {
                        let app_key_name = key_result;
                        if let Ok(app_key) = uninstall_key.open(&app_key_name) {
                            if let Ok(comments) = app_key.get_string("Comments") {
                                if comments == "Installed with AppPorter" {
                                    if let Ok(Some(app)) = tokio::task::block_in_place(|| {
                                        tokio::runtime::Handle::current()
                                            .block_on(extract_app_info(&app_key, current_user_only))
                                    }) {
                                        result.push(app);
                                    }
                                }
                            }
                        }
                    }
                }
            }
            Ok(())
        }

        async fn extract_app_info(
            app_key: &windows_registry::Key,
            current_user_only: bool,
        ) -> Result<Option<InstalledApp>, Box<dyn Error + Send + Sync>> {
            // Extract required values
            let name = match app_key.get_string("DisplayName") {
                Ok(name) => name,
                Err(_) => return Ok(None), // Skip if no display name
            };

            let full_path = match app_key.get_string("DisplayIcon") {
                Ok(path) => path,
                Err(_) => return Ok(None), // Skip if no path
            };

            // Extract optional values with defaults
            let version = app_key.get_string("DisplayVersion").unwrap_or_default();
            let publisher = app_key.get_string("Publisher").unwrap_or_default();
            let install_path = app_key
                .get_string("InstallLocation")
                .map(|path| {
                    Path::new(&path)
                        .parent()
                        .and_then(|p| p.to_str())
                        .unwrap_or(&path)
                        .to_string()
                })
                .unwrap_or_default();

            let raw_icon = get_icon(&full_path, 64).unwrap_or_default();
            let icon = format!("data:image/png;base64,{}", STANDARD.encode(&raw_icon));

            let settings = Settings::read().await?;

            let config = if current_user_only {
                &settings.installation.current_user
            } else {
                &settings.installation.all_users
            };

            let create_desktop_shortcut = config.create_desktop_shortcut;
            let create_start_menu_shortcut = config.create_start_menu_shortcut;
            let create_registry_key = config.create_registry_key;
            let add_to_path = config.add_to_path;

            // Create a new InstalledApp
            let app = InstalledApp {
                name,
                version,
                publisher,
                install_path,
                executable_path: String::new(),
                full_path,
                icon, // Will be filled later during validation
                current_user_only,
                create_desktop_shortcut,
                create_start_menu_shortcut,
                create_registry_key,
                add_to_path,
                validation_status: ValidationStatus {
                    file_exists: true,
                    registry_valid: true,
                },
            };

            Ok(Some(app))
        }

        Ok(())
    }
}

pub async fn load_app_list() -> Result<String, Box<dyn Error + Send + Sync>> {
    let mut app_list = AppList::read().await?;
    app_list.sync_from_registry().await?;
    app_list.validate_installations().await?;
    app_list.remove_duplicates();
    app_list.save().await?;

    Ok(serde_json::to_string(&app_list)?)
}

pub async fn save_app_list(app_list: AppList) -> Result<String, Box<dyn Error + Send + Sync>> {
    app_list.save().await?;
    Ok("AppList saved successfully".to_owned())
}
