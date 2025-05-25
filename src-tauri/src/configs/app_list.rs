use super::settings::Settings;
use crate::configs::ConfigFile;
use base64::{engine::general_purpose::STANDARD, Engine};
use serde::{Deserialize, Serialize};
use std::{error::Error, path::Path};
use systemicons::get_icon;

#[derive(Debug, Deserialize, Serialize, Default, Clone)]
#[serde(default)]
pub struct AppList {
    pub links: Vec<App>,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct App {
    pub timestamp: i64,
    pub installed: bool,
    pub copy_only: bool,
    pub url: String,
    pub details: AppDetails,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct AppDetails {
    pub info: AppBasicInformation,
    pub config: AppConfig,
    pub paths: AppPaths,
    pub validation_status: AppValidationStatus,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct AppBasicInformation {
    pub name: String,
    pub icon: String,
    pub publisher: String,
    pub version: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct AppConfig {
    pub archive_exe_path: String,
    pub archive_password: String,
    pub current_user_only: bool,
    pub create_desktop_shortcut: bool,
    pub create_start_menu_shortcut: bool,
    pub create_registry_key: bool,
    pub add_to_path: bool,
    pub path_directory: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct AppPaths {
    pub parent_install_path: String,
    pub install_path: String,
    pub full_path: String,
}

#[derive(Debug, Deserialize, Serialize, Default, Clone, PartialEq, Eq)]
#[serde(default)]
pub struct AppValidationStatus {
    pub file_exists: bool,
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
            let file_exists = tokio::fs::try_exists(&app.details.paths.full_path)
                .await
                .unwrap_or(false);

            // Check registry Comments value
            let registry_valid = if app.details.config.create_registry_key {
                let registry_path = if app.details.config.current_user_only {
                    format!(
                        r"Software\Microsoft\Windows\CurrentVersion\Uninstall\{}",
                        app.details.info.name
                    )
                } else {
                    format!(
                        r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall\{}",
                        app.details.info.name
                    )
                };

                let reg_key = if app.details.config.current_user_only {
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

            app.details.validation_status = AppValidationStatus {
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
                if self.links[i].details.paths.full_path == self.links[j].details.paths.full_path {
                    // Check if all other fields except timestamp and version are identical
                    let mut app1 = self.links[i].clone();
                    let mut app2 = self.links[j].clone();

                    // Reset timestamp and version for comparison
                    app1.timestamp = 0;
                    app2.timestamp = 0;
                    app1.details.info.version = String::new();
                    app2.details.info.version = String::new();

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
                    && app.details.info.name == reg_app.info.name
                    && app.details.paths.full_path == reg_app.paths.full_path
            });

            // If not in our list, add it
            if !app_exists {
                self.links.push(App {
                    timestamp: chrono::Utc::now().timestamp(),
                    installed: true,
                    copy_only: false,
                    url: String::new(),
                    details: reg_app,
                });
            }
        }

        fn find_registry_apps() -> Result<Vec<AppDetails>, Box<dyn Error + Send + Sync>> {
            let mut result = Vec::new();

            // Search in both HKEY_CURRENT_USER and HKEY_LOCAL_MACHINE
            search_registry(
                &mut result,
                windows_registry::CURRENT_USER,
                r"Software\Microsoft\Windows\CurrentVersion\Uninstall",
                true,
            )?;
            search_registry(
                &mut result,
                windows_registry::LOCAL_MACHINE,
                r"SOFTWARE\Microsoft\Windows\CurrentVersion\Uninstall",
                false,
            )?;

            Ok(result)
        }

        fn search_registry(
            result: &mut Vec<AppDetails>,
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
        ) -> Result<Option<AppDetails>, Box<dyn Error + Send + Sync>> {
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

            // Create a new AppDetails
            let app = AppDetails {
                info: AppBasicInformation {
                    name,
                    icon,
                    publisher,
                    version,
                },
                config: AppConfig {
                    archive_exe_path: String::new(),
                    archive_password: String::new(),
                    current_user_only,
                    create_desktop_shortcut,
                    create_start_menu_shortcut,
                    create_registry_key,
                    add_to_path,
                    path_directory: install_path.clone(),
                },
                paths: AppPaths {
                    parent_install_path: install_path.clone(),
                    install_path: install_path.clone(),
                    full_path,
                },
                validation_status: AppValidationStatus {
                    file_exists: true,
                    registry_valid: true,
                },
            };

            Ok(Some(app))
        }

        Ok(())
    }
}

pub async fn load_app_list() -> Result<AppList, Box<dyn Error + Send + Sync>> {
    let mut app_list = AppList::read().await?;
    app_list.sync_from_registry().await?;
    app_list.validate_installations().await?;
    app_list.remove_duplicates();
    app_list.save().await?;

    Ok(app_list)
}
