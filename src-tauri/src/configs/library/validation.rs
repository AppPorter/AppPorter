use super::{AppValidationStatus, Library, ToolValidationStatus};
use anyhow::Result;
use windows_registry::{CURRENT_USER, LOCAL_MACHINE};

impl Library {
    pub async fn validate_installs(&mut self) -> Result<()> {
        for app in &mut self.apps {
            if !app.installed {
                continue;
            }

            let file_exists = tokio::fs::try_exists(&app.details.paths.full_path)
                .await
                .unwrap_or(false);

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

            let path_valid = if app.details.config.add_to_path {
                let path_to_check = &app.details.config.full_path_directory;

                if app.details.config.current_user_only {
                    match CURRENT_USER.open("Environment") {
                        Ok(key) => match key.get_string("Path") {
                            Ok(path) => path.split(';').any(|p| p.trim() == path_to_check.trim()),
                            _ => false,
                        },
                        _ => false,
                    }
                } else {
                    match LOCAL_MACHINE
                        .open(r"SYSTEM\CurrentControlSet\Control\Session Manager\Environment")
                    {
                        Ok(key) => match key.get_string("path") {
                            Ok(path) => path.split(';').any(|p| p.trim() == path_to_check.trim()),
                            _ => false,
                        },
                        _ => false,
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

        for tool in &mut self.tools {
            if !tool.installed {
                continue;
            }

            let file_exists = tokio::fs::try_exists(&tool.details.paths.install_path)
                .await
                .unwrap_or(false);

            let path_valid = if tool.details.config.add_to_path {
                let path_to_check = &tool.details.config.full_path_directory;

                match CURRENT_USER.open("Environment") {
                    Ok(key) => match key.get_string("Path") {
                        Ok(path) => path.split(';').any(|p| p.trim() == path_to_check.trim()),
                        _ => false,
                    },
                    _ => false,
                }
            } else {
                true
            };

            tool.details.validation_status = ToolValidationStatus {
                file_exists,
                path_exists: path_valid,
            };
        }
        Ok(())
    }

    pub fn remove_duplicates(&mut self) {
        let mut i = 0;
        while i < self.apps.len() {
            let mut j = i + 1;
            while j < self.apps.len() {
                if self.apps[i].details.paths.full_path == self.apps[j].details.paths.full_path {
                    let mut app1 = self.apps[i].clone();
                    let mut app2 = self.apps[j].clone();

                    app1.timestamp = 0;
                    app2.timestamp = 0;
                    app1.details.info.version = String::new();
                    app2.details.info.version = String::new();

                    if app1 == app2 {
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

        let mut i = 0;
        while i < self.tools.len() {
            let mut j = i + 1;
            while j < self.tools.len() {
                if self.tools[i].details.paths.install_path
                    == self.tools[j].details.paths.install_path
                {
                    let mut tool1 = self.tools[i].clone();
                    let mut tool2 = self.tools[j].clone();

                    tool1.timestamp = 0;
                    tool2.timestamp = 0;

                    if tool1 == tool2 {
                        if self.tools[i].timestamp < self.tools[j].timestamp {
                            self.tools.remove(i);
                            j = i + 1;
                            continue;
                        } else {
                            self.tools.remove(j);
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
