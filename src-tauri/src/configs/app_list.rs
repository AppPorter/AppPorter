use crate::configs::ConfigFile;
use serde::{Deserialize, Serialize};
use std::error::Error;

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
    pub fn add_link(&mut self, url: String) {
        self.links.push(App {
            timestamp: chrono::Utc::now().timestamp(),
            url,
            ..Default::default()
        });
    }

    pub fn has_link(&self, url: &str) -> bool {
        self.links.iter().any(|link| link.url == url)
    }

    pub async fn validate_installations(&mut self) -> Result<(), Box<dyn Error>> {
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
}

pub async fn load_app_list() -> Result<String, Box<dyn Error>> {
    let mut app_list = AppList::read().await?;
    app_list.validate_installations().await?;
    app_list.remove_duplicates();
    app_list.save().await?;
    Ok(serde_json::to_string(&app_list)?)
}

pub async fn save_app_list(app_list: AppList) -> Result<String, Box<dyn Error>> {
    app_list.save().await?;
    Ok("AppList saved successfully".to_owned())
}
