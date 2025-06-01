use super::{AppList, AppValidationStatus, LibValidationStatus};
use crate::configs::app_list::{AppBasicInformation, AppConfig, AppPaths};
use crate::configs::ConfigFile;
use crate::configs::{app_list::AppDetails, settings::Settings};
use base64::{engine::general_purpose::STANDARD, Engine};
use std::{error::Error, path::Path};
use systemicons::get_icon;
use windows_registry::{CURRENT_USER, LOCAL_MACHINE};

#[async_trait::async_trait]
impl ConfigFile for AppList {
    fn get_filename() -> &'static str {
        "AppList.json"
    }
}

impl AppList {
    pub fn has_link(&self, url: &str) -> bool {
        self.apps.iter().any(|app| app.url == url && app.installed)
            || self.libs.iter().any(|lib| lib.url == url && lib.installed)
    }

    pub async fn validate_installs(&mut self) -> Result<(), Box<dyn Error + Send + Sync>> {
        for app in &mut self.apps {
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

            // Check PATH environment variable
            let path_valid = if app.details.config.add_to_path {
                let path_to_check = &app.details.config.full_path_directory;

                if app.details.config.current_user_only {
                    if let Ok(key) = CURRENT_USER.open("Environment") {
                        if let Ok(path) = key.get_string("Path") {
                            path.split(';').any(|p| p.trim() == path_to_check.trim())
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                } else {
                    if let Ok(key) = LOCAL_MACHINE
                        .open(r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment")
                    {
                        if let Ok(path) = key.get_string("path") {
                            path.split(';').any(|p| p.trim() == path_to_check.trim())
                        } else {
                            false
                        }
                    } else {
                        false
                    }
                }
            } else {
                true
            };

            app.details.validation_status = AppValidationStatus {
                file_exists,
                registry_valid,
                path_exists: path_valid,
            };
        }

        for lib in &mut self.libs {
            if !lib.installed {
                continue;
            }

            // Check if executable exists
            let file_exists = tokio::fs::try_exists(&lib.details.paths.install_path)
                .await
                .unwrap_or(false);

            // Check PATH environment variable
            let path_valid = if lib.details.config.add_to_path {
                let path_to_check = &lib.details.config.full_path_directory;

                // Libs are always added to CURRENT_USER environment
                if let Ok(key) = CURRENT_USER.open("Environment") {
                    if let Ok(path) = key.get_string("Path") {
                        path.split(';').any(|p| p.trim() == path_to_check.trim())
                    } else {
                        false
                    }
                } else {
                    false
                }
            } else {
                true
            };

            lib.details.validation_status = LibValidationStatus {
                file_exists,
                path_exists: path_valid,
            };
        }
        Ok(())
    }

    pub fn remove_duplicates(&mut self) {
        // Remove duplicates from apps
        let mut i = 0;
        while i < self.apps.len() {
            let mut j = i + 1;
            while j < self.apps.len() {
                if self.apps[i].details.paths.full_path == self.apps[j].details.paths.full_path {
                    // Check if all other fields except timestamp and version are identical
                    let mut app1 = self.apps[i].clone();
                    let mut app2 = self.apps[j].clone();

                    // Reset timestamp and version for comparison
                    app1.timestamp = 0;
                    app2.timestamp = 0;
                    app1.details.info.version = String::new();
                    app2.details.info.version = String::new();

                    if app1 == app2 {
                        // Remove the entry with earlier timestamp
                        if self.apps[i].timestamp < self.apps[j].timestamp {
                            self.apps.remove(i);
                            j = i + 1;
                            continue;
                        } else {
                            self.apps.remove(j);
                            continue;
                        }
                    }
                }
                j += 1;
            }
            i += 1;
        }

        // Remove duplicates from libs
        let mut i = 0;
        while i < self.libs.len() {
            let mut j = i + 1;
            while j < self.libs.len() {
                if self.libs[i].details.paths.install_path
                    == self.libs[j].details.paths.install_path
                {
                    // Check if all other fields except timestamp and version are identical
                    let mut lib1 = self.libs[i].clone();
                    let mut lib2 = self.libs[j].clone();

                    // Reset timestamp and version for comparison
                    lib1.timestamp = 0;
                    lib2.timestamp = 0;

                    if lib1 == lib2 {
                        // Remove the entry with earlier timestamp
                        if self.libs[i].timestamp < self.libs[j].timestamp {
                            self.libs.remove(i);
                            j = i + 1;
                            continue;
                        } else {
                            self.libs.remove(j);
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
            let app_exists = self.apps.iter().any(|app| {
                app.installed
                    && app.details.info.name == reg_app.info.name
                    && app.details.paths.full_path == reg_app.paths.full_path
            });

            // If not in our list, add it
            if !app_exists {
                self.apps.push(super::structs::App {
                    timestamp: chrono::Utc::now().timestamp(),
                    installed: true,
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
                &settings.app_install.current_user
            } else {
                &settings.app_install.all_users
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
                    archive_path_directory: String::new(),
                    full_path_directory: install_path.clone(),
                },
                paths: AppPaths {
                    parent_install_path: install_path.clone(),
                    install_path: install_path.clone(),
                    full_path,
                },
                validation_status: AppValidationStatus {
                    file_exists: true,
                    registry_valid: true,
                    path_exists: true,
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
    app_list.validate_installs().await?;
    app_list.remove_duplicates();
    app_list.save().await?;

    Ok(app_list)
}
