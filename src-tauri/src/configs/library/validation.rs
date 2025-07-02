use super::{AppValidationStatus, Library, ToolValidationStatus};
use anyhow::Result;
use windows_registry::{CURRENT_USER, LOCAL_MACHINE};

impl Library {
    pub async fn validate_installs(&mut self) -> Result<()> {
        for app in &mut self.apps {
            if !app.installed {
                continue;
            }

            let file_exists = tokio::fs::try_exists(&app.details.full_path)
                .await
                .unwrap_or(false);

            let registry_valid = if app.details.config.create_registry_key {
                let registry_path = if app.details.current_user_only {
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

            let path_valid = if app.details.config.add_to_path.0 {
                let path_to_check = &app.details.config.add_to_path.1;

                if app.details.current_user_only {
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

            app.validation_status = AppValidationStatus {
                file_exists,
                registry_valid,
                path_exists: path_valid,
            };
        }

        for tool in &mut self.tools {
            if !tool.installed {
                continue;
            }

            let file_exists = tokio::fs::try_exists(&tool.details.install_path)
                .await
                .unwrap_or(false);

            let path_valid = if tool.details.add_to_path.0 {
                let path_to_check = &tool.details.add_to_path.1;

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

            tool.validation_status = ToolValidationStatus {
                file_exists,
                path_exists: path_valid,
            };
        }
        Ok(())
    }
}
